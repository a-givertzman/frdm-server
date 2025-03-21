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
#include <stdbool.h>
#include <string.h>

#if (!defined _WIN32 && !defined _WIN64)
#define scanf_s scanf
#endif

#define TAB1 "  "
#define TAB2 "    "
#define TAB3 "      "

// Enumeration Unicast: Introduction
//    This example introduces adding unicast device. This includes opening and
//    closing the system, updating and retrieving the list of devices, adding
//    unicast devices using the IP address for the device, and chekcing connections
//    of the devices

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

// buffer size for IP address
#define IP_BUF 32

// =-=-=-=-=-=-=-=-=-
// =-=- EXAMPLE -=-=-
// =-=-=-=-=-=-=-=-=-

// enumerates device(s)
// (1) updates device list
// (2) gets device list
// (3) creates device
// (4) prints device information
// (5) checks connection for device
// (6) prints connection information
// (7) destroys device
AC_ERROR EnumerateDevices(acSystem hSystem)
{
	AC_ERROR err = AC_ERR_SUCCESS;

	// update devices
	err = acSystemUpdateDevices(hSystem, SYSTEM_TIMEOUT);
	if (err != AC_ERR_SUCCESS)
		return err;

	// number of devices
	size_t numDevices = 0;

	err = acSystemGetNumDevices(hSystem, &numDevices);
	if (err != AC_ERR_SUCCESS)
		return err;

	// display device information
	printf("%sGet device information\n", TAB1);
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
		printf("; IP %s)\n", pBuf);

		// create device
		acDevice hDevice = NULL;
		acSystemCreateDevice(hSystem, i, &hDevice);
		if (err != AC_ERR_SUCCESS)
			return err;

		// check connection
		bool isConnected;
		printf("%sCheck connection: ", TAB2);
		err = acDeviceIsConnected(hDevice, &isConnected);
		if (err != AC_ERR_SUCCESS)
			return err;

		if (isConnected)
		{	
			// display connection information
			printf("Device %zu is connected", i);

			acNodeMap hDeviceNodeMap = NULL;
			err = acDeviceGetNodeMap(hDevice, &hDeviceNodeMap);
			if (err != AC_ERR_SUCCESS)
				return err;

			acNode hPFNode = NULL;
			err = acNodeMapGetNode(hDeviceNodeMap, "PixelFormat", &hPFNode);
			if (err != AC_ERR_SUCCESS)
				return err;
			
			acNode hFRNode = NULL;
			err = acNodeMapGetNode(hDeviceNodeMap, "AcquisitionFrameRate", &hFRNode);
			if (err != AC_ERR_SUCCESS)
				return err;

			size_t valueBufLen = MAX_BUF;
			err = acValueToString(hPFNode, pBuf, &valueBufLen);
			if (err != AC_ERR_SUCCESS)
				return err;
			printf("(Pixel Format: %s; ", pBuf);

			err = acValueToString(hFRNode, pBuf, &valueBufLen);
			if (err != AC_ERR_SUCCESS)
				return err;
			printf("Frame Rate: %sfps)\n\n", pBuf);
		} 
		else
		{
			printf("Device %zu is not connected\n\n", i);
		}

		// destroy device
		acSystemDestroyDevice(hSystem, hDevice);
		if (err != AC_ERR_SUCCESS)
			return err;
	}
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

// adds unicast device(s)
// (1) opens system
// (2) enumerates devices before adding unicast device
// (3) takes IP address for device to be added from user
// (4) adds unicast discovery device(s)
// (5) enumerates devices after adding unicast device(s)
// (6) closes system
AC_ERROR AddUnicastDevice()
{
	AC_ERROR err = AC_ERR_SUCCESS;

	// open system
	printf("%sOpen system\n", TAB1);
	acSystem hSystem = NULL;
	err = acOpenSystem(&hSystem);
	if (err != AC_ERR_SUCCESS)
		return err;

	// enumerate devices before adding unicast device(s) 
	printf("%sDevice list before adding unicast device(s)\n", TAB1);
	err = EnumerateDevices(hSystem);
	if (err != AC_ERR_SUCCESS)
		return err;
	
	// stay in loop until exit
	while (true)
	{
		// 'x' to exit
		char ip[IP_BUF];
		printf("%sInput IP for device to be added ('x' to exit)\n%s", TAB2, TAB2);
		if ((fgets(ip, sizeof ip, stdin)) != NULL)
			;

		if (0 == strcmp(ip, "x\n"))
		{
			printf("%sExit\n\n", TAB2);
			break;
		}

		// change end of string character in ip so it is accepted as input
		size_t length = strlen(ip);
		if (length > 0 && ip[length - 1] == '\n')
		{
			ip[length - 1] = '\0';
		}
		
		// Add a unicast discovery device
		//    registers an IP address for a device on a different subnet
		//    than the host. Registered devices will be enumerated using
		//    unicast discovery messages. The list of remote devices will
		//    persist until they are removed using RemoveUnicastDiscoveryDevice()
		//    or until the application terminates. Unicast discovery's will be
		//    sent when UpdateDevices() is called.
		printf("%sAdd device with IP: %s \n\n", TAB1, ip);
		err = acSystemAddUnicastDiscoveryDevice(hSystem, ip);
		if (err != AC_ERR_SUCCESS)
			return err;
	}

	// enumerate devices after adding unicast device(s) 
	printf("%sDevice list after adding unicast device(s)\n", TAB1);
	err = EnumerateDevices(hSystem);
	if (err != AC_ERR_SUCCESS)
		return err;

	// close system
	printf("%sClose system\n", TAB1);
	err = acCloseSystem(hSystem);
	return err;
}

int main()
{
	printf("C_Enumeration_Unicast\n");
	AC_ERROR err = AC_ERR_SUCCESS;

	printf("Commence example\n\n");
	err = AddUnicastDevice();
	CHECK_RETURN;
	printf("\nExample complete\n");

	printf("Press enter to complete\n");
	while (getchar() != '\n') {};
	getchar();
	return -1;
}
