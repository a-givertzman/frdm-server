/***************************************************************************************
 ***                                                                                 ***
 ***  Copyright (c) 2024, Lucid Vision Labs, Inc.                                    ***
 ***                                                                                 ***
 ***  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR     ***
 ***  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,       ***
 ***  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE    ***
 ***  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER         ***
 ***  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,  ***
 ***  OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE  ***
 ***  SOFTWARE.                                                                      ***
 ***                                                                                 ***
 ***************************************************************************************/

#include "stdafx.h"
#include "ArenaCApi.h"
#include <stdbool.h>  // defines boolean type and values
#include <string.h>   // defines string type and values

#if (!defined _WIN32 && !defined _WIN64)
#define scanf_s scanf
#endif

#define TAB1 "  "
#define TAB2 "    "
#define TAB3 "      "

 // Enumeration: CcpSwitchover
 //    This example introduces device enumeration with the ability to hand over control 
 //    to another process. This includes opening and closing the system, updating
 //    and retrieving the list of devices, searching for devices, and creating
 //    and destroying a device. In this example, we will also set a special key
 //    to the device that another process can use to acquire control of the
 //    device when running the example for the first time. Running the example a 
 //    second time while the first instance is still running will try to use the
 //    special key to gain control of the device.
 //

// =-=-=-=-=-=-=-=-=-
// =-=- SETTINGS =-=-
// =-=-=-=-=-=-=-=-=-

// Update timeout
//    Timeout for updating the list of devices (in milliseconds). Because it is
//    unclear how many devices are expected, this timeout waits the entire
//    timeout, not returning early as devices are found.
#define SYSTEM_TIMEOUT 100

// maximum buffer length
#define MAX_BUF 1024

// =-=-=-=-=-=-=-=-=-
// =-=- EXAMPLE -=-=-
// =-=-=-=-=-=-=-=-=-

// checks access status to camera
// (1) read DeviceAccessStatus node
// (2) check if access status is ReadWrite
bool hasControl(acNodeMap hTLDeviceNodeMap)
{
	AC_ERROR err = AC_ERR_ERROR;

	char pDeviceAccessStatus[MAX_BUF];
	size_t pDeviceAccessStatusBufLen = MAX_BUF;

	err = acNodeMapGetStringValue(hTLDeviceNodeMap, "DeviceAccessStatus", pDeviceAccessStatus, &pDeviceAccessStatusBufLen);
	if (err != AC_ERR_SUCCESS)
		return false;  //if we cannot read access status assume we do not have control

	return strcmp(pDeviceAccessStatus, "ReadWrite") == 0;
}


// demonstrates enumeration
// (1) opens system
// (2) updates device list
// (3) gets device list
// (4) prints device information
// (5) creates device
// (6) destroys device
AC_ERROR EnumerateDevices()
{
	AC_ERROR err = AC_ERR_SUCCESS;

	// Open system
	//    Open the system in order to update and retrieve the device list.
	//    Opening the system is the entry point to the rest of the Arena SDK.
	//    Only one system can be opened at a time.
	printf("%sOpen system\n", TAB1);
	acSystem hSystem = NULL;

	err = acOpenSystem(&hSystem);
	if (err != AC_ERR_SUCCESS)
		return err;

	// Update and retrieve number of devices
	//    Update and retrieve the list of connected devices. Failing to update
	//    results in an empty list being retrieved, even if devices are
	//    connected. Getting the devices does not update or otherwise affect
	//    the list.
	printf("%sUpdate and retrieve number of devices\n", TAB1);

	// update devices
	err = acSystemUpdateDevices(hSystem, SYSTEM_TIMEOUT);
	if (err != AC_ERR_SUCCESS)
		return err;

	// number of devices
	size_t numDevices = 0;

	err = acSystemGetNumDevices(hSystem, &numDevices);
	if (err != AC_ERR_SUCCESS)
		return err;

	// Get and display device information
	//    The list of devices is kept as a std::vector of device information
	//    objects. Device information objects provide access to devices'
	//    discovery information. This information is provided without having to
	//    create the device and includes information related to identification
	//    and network settings.
	printf("%sGet device information\n", TAB1);

	if (numDevices == 0)
	{
		printf("%sNo camera connected\n", TAB1);
		return AC_ERR_INVALID_VALUE;
	}

	size_t i = 0;
	for (i = 0; i < numDevices; i++)
	{
		printf("%sInformation for device %zu ", TAB2, i);

		// get and display vendor name
		char pBuf[MAX_BUF];
		size_t len = MAX_BUF;

		err = acSystemGetDeviceVendor(hSystem, i, pBuf, &len);
		if (err != AC_ERR_SUCCESS)
			return err;

		printf("(%s", pBuf);

		// get and display model name
		len = MAX_BUF;

		err = acSystemGetDeviceModel(hSystem, i, pBuf, &len);
		if (err != AC_ERR_SUCCESS)
			return err;

		printf("; %s", pBuf);

		// get and display serial number
		len = MAX_BUF;

		err = acSystemGetDeviceSerial(hSystem, i, pBuf, &len);
		if (err != AC_ERR_SUCCESS)
			return err;

		printf("; serial %s", pBuf);

		// get and display MAC address
		len = MAX_BUF;

		err = acSystemGetDeviceMacAddressStr(hSystem, i, pBuf, &len);
		if (err != AC_ERR_SUCCESS)
			return err;

		printf("; MAC %s", pBuf);

		// get and display IP address
		len = MAX_BUF;

		err = acSystemGetDeviceIpAddressStr(hSystem, i, pBuf, &len);
		if (err != AC_ERR_SUCCESS)
			return err;

		printf("; IP %s\n", pBuf);

		// Create device
		//    Create device in order to configure it and grab images. Creating a
		//    device requires a device information object to be passed in.
		//    Created objects need to be destroyed. A device can only be created
		//    once per process, and can only be opened with read-write access
		//    once.
		printf("%sCreate device\n", TAB3);
		acDevice hDevice = NULL;

		acSystemCreateDevice(hSystem, i, &hDevice);
		if (err != AC_ERR_SUCCESS)
			return err;

		// get device TL node map
		acNodeMap hTLDeviceNodeMap = NULL;

		err = acDeviceGetTLDeviceNodeMap(hDevice, &hTLDeviceNodeMap);
		if (err != AC_ERR_SUCCESS)
			return err;

		// static key used to acquire control between applications
		static int64_t switchoverKey = 0x1234;

		// check if we were able to get control of application
		if (hasControl(hTLDeviceNodeMap))
		{
			// this means we are running with control
			// we set a unique key in case someone wants to gain control 

			err = acNodeMapSetIntegerValue(hTLDeviceNodeMap, "CcpSwitchoverKey", switchoverKey);
			if (err != AC_ERR_SUCCESS)
			{
				printf("%sFailed to set CcpSwitchoverKey\n", TAB2);
				return err;
			}

			// wait for user to input something
			printf("%sPlease press enter to continue:\n", TAB2);
			getchar();

		}
		else
		{
			// set a unique key in case someone wants to gain control 
			err = acNodeMapSetIntegerValue(hTLDeviceNodeMap, "CcpSwitchoverKey", switchoverKey);
			if (err != AC_ERR_SUCCESS)
			{
				printf("%sFailed to set CcpSwitchoverKey\n", TAB2);
				return err;
			}

			// try to set the access status to read/write
			err = acNodeMapSetStringValue(hTLDeviceNodeMap, "DeviceAccessStatus", "ReadWrite");
			if (err != AC_ERR_SUCCESS)
				return err;

			if (hasControl(hTLDeviceNodeMap))
			{
				printf("%sCreate device succeeded with acquiring control\n", TAB2);
			}
			else
			{
				printf("%sCreate device failed to acquire control\n", TAB2);
			}

		}


		// Destroy device
		//    Destroy the device before closing the system. Destroying devices cleans
		//    up allocated memory.
		printf("%sDestroy device\n", TAB3);

		acSystemDestroyDevice(hSystem, hDevice);
		if (err != AC_ERR_SUCCESS)
			return err;
	}

	// Release system
	//    Release the system to end use of Arena SDK. Releasing the system cleans
	//    up allocated memory. Failing to release the system causes memory to
	//    leak.
	printf("%sClose system\n", TAB1);

	err = acCloseSystem(hSystem);
	return err;
}

// =-=-=-=-=-=-=-=-=-
// =- PREPARATION -=-
// =- & CLEAN UP =-=-
// =-=-=-=-=-=-=-=-=-

// error buffer length
#define ERR_BUF 512

#define CHECK_RETURN                                  \
	if (err != AC_ERR_SUCCESS)                        \
	{                                                 \
		char pMessageBuf[ERR_BUF];                    \
		size_t pBufLen = ERR_BUF;                     \
		acGetLastErrorMessage(pMessageBuf, &pBufLen); \
		printf("\nError: %s", pMessageBuf);           \
		printf("\n\nPress enter to complete\n");      \
		getchar();                                    \
		return -1;                                    \
	}

int main()
{
	printf("C_Enumeration_CcpSwitchover\n");
	AC_ERROR err = AC_ERR_SUCCESS;

	printf("Commence example\n\n");
	err = EnumerateDevices();
	CHECK_RETURN;
	printf("\nExample complete\n");

	printf("Press enter to complete\n");
	while (getchar() != '\n') {};
	getchar();
	return -1;
}
