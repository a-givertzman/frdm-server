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
#include <inttypes.h> // defines macros for printf functions
#include <stdbool.h>  // defines boolean type and values
#include <time.h>
#include <stdlib.h>

#if (!defined _WIN32 && !defined _WIN64)
#define scanf_s scanf
#endif

#define TAB1 "  "
#define TAB2 "    "

// IpConfig: Auto
//    This example displays the code to automatically configure the IP address.
//    The system cannot communicate with the device if the IP address and subnet
//    mask are configured for different networks. In this case, we force the
//    device's IP to establish a connection.

// =-=-=-=-=-=-=-=-=-
// =-=- SETTINGS =-=-
// =-=-=-=-=-=-=-=-=-

// timeout for detecting camera devices (in milliseconds)
#define SYSTEM_TIMEOUT 100

// maximum buffer length
#define MAX_BUF 1024

// =-=-=-=-=-=-=-=-=-
// =-=- HELPER =-=-=-
// =-=-=-=-=-=-=-=-=-

uint32_t generateNewIp(uint32_t ifSubnet, uint32_t ifIp)
{
	uint32_t newIp = 0;
	uint32_t randNum = (uint32_t)rand() & (~ifSubnet);
	while (randNum == 0 || randNum == 0xFFFFFFFF || newIp == 0 || newIp == ifIp)
	{
		randNum = (uint32_t)rand() & (~ifSubnet);
		newIp = (ifIp & ifSubnet) | (randNum);
	}

	return newIp;
}

void print_ip(uint32_t ip)
{
	unsigned char bytes[4];
	bytes[0] = ip & 0xFF;
	bytes[1] = (ip >> 8) & 0xFF;
	bytes[2] = (ip >> 16) & 0xFF;
	bytes[3] = (ip >> 24) & 0xFF;

	printf("%d.%d.%d.%d\n", bytes[3], bytes[2], bytes[1], bytes[0]);
}

// =-=-=-=-=-=-=-=-=-
// =-=- EXAMPLE -=-=-
// =-=-=-=-=-=-=-=-=-

AC_ERROR SelectDevice(acSystem hSystem, size_t* pNumDevices, size_t* pSelection)
{
	AC_ERROR err = AC_ERR_SUCCESS;

	if (*pNumDevices == 1)
	{
		printf(TAB1 "Only one device detected, automatically selecting this device.\n");
		*pSelection = 0;
		return AC_ERR_SUCCESS;
	}

	printf(TAB1 "Select device to force IP:\n");
	for (size_t i = 0; i < *pNumDevices; i++)
	{
		// get device model
		char pDeviceModel[MAX_BUF];
		size_t pDeviceModelLen = MAX_BUF;
		err = acSystemGetDeviceModel(hSystem, i, pDeviceModel, &pDeviceModelLen);
		if (err != AC_ERR_SUCCESS)
			return err;

		// get device serial
		char pDeviceSerial[MAX_BUF];
		size_t pDeviceSerialLen = MAX_BUF;
		err = acSystemGetDeviceSerial(hSystem, i, pDeviceSerial, &pDeviceSerialLen);
		if (err != AC_ERR_SUCCESS)
			return err;

		// get device IP address
		char pIpAddressStr[MAX_BUF];
		size_t pIpAddressStrBufLen = MAX_BUF;
		err = acSystemGetDeviceIpAddressStr(hSystem, i, pIpAddressStr, &pIpAddressStrBufLen);
		if (err != AC_ERR_SUCCESS)
			return err;

		printf(TAB2 "%zu. %s%s%s%s%s\n", i + 1, pDeviceModel, TAB1, pDeviceSerial, TAB1, pIpAddressStr);
	}

	do
	{
		printf(TAB1 "Make selection (1-%zu): ", *pNumDevices);

		if (scanf_s("%zu", pSelection) != 1)
		{
			while (getchar() != '\n');
			printf(TAB1 "Invalid input. Please enter a number.\n");
			continue;
		}

		if (*pSelection <= 0 || *pSelection > *pNumDevices)
		{
			printf(TAB1 "Invalid device selected. Please select a device in the range (1-%zu).\n", *pNumDevices);
		}
	} while (*pSelection <= 0 || *pSelection > *pNumDevices);

	*pSelection -= 1;
	return AC_ERR_SUCCESS;
}

// demonstrates Auto IP Config
// (1) Get number of devices; if greater than 0, try creating a device
// (2) If creating a device is unsuccessful, camera is not on the correct network
// (3) Generate a new IP using the helper function
// (4) Force camera to the new IP address to establish a connection
AC_ERROR AutoIPConfig(acSystem hSystem)
{
	AC_ERROR err = AC_ERR_SUCCESS;

	// prepare system
	err = acSystemUpdateDevices(hSystem, SYSTEM_TIMEOUT);
	if (err != AC_ERR_SUCCESS)
		return err;

	size_t numDevices = 0;
	err = acSystemGetNumDevices(hSystem, &numDevices);
	if (err != AC_ERR_SUCCESS)
		return err;
	if (numDevices == 0)
	{
		printf("\nNo camera connected\nPress enter to complete\n");
		getchar();
		return -1;
	}

	printf("%sDevices(s) Available : %d\n", TAB1, (int)numDevices);

	size_t selection = 0;
	err = SelectDevice(hSystem, &numDevices, &selection);
	if (err != AC_ERR_SUCCESS)
		return err;

	// get device information; this is used to force IP if needed
	uint64_t macAddress = 0;
	uint32_t ipAddress = 0;
	uint32_t subnetMask = 0;
	uint32_t defaultGateway = 0;

	err = acSystemGetDeviceMacAddress(hSystem, selection, &macAddress) |
		  acSystemGetDeviceIpAddress(hSystem, selection, &ipAddress) |
		  acSystemGetDeviceSubnetMask(hSystem, selection, &subnetMask) |
		  acSystemGetDeviceDefaultGateway(hSystem, selection, &defaultGateway);
	if (err != AC_ERR_SUCCESS)
		printf("\nWarning: failed to retrieve one or more initial address integer values\n");

	printf("%sCurrent IP Address is ", TAB2);
	print_ip(ipAddress);

	printf("%sCurrent Subnet Mask is ", TAB2);
	print_ip(subnetMask);
	// try creating a device, if unsuccessful then the device is on the wrong
	// network, force IP
	acDevice hDevice;
	err = acSystemCreateDevice(hSystem, selection, &hDevice);
	if (err != AC_ERR_SUCCESS)
	{
		printf("%sDevice is on an incorrect network, Force Ip\n", TAB1);

		uint32_t ifIntSubnet = 0;
		err = acSystemGetInterfaceSubnetMask(hSystem, selection, &ifIntSubnet);
		if (err != AC_ERR_SUCCESS)
			return err;

		uint32_t ifIntIp = 0;
		err = acSystemGetInterfaceIpAddress(hSystem, selection, &ifIntIp);
		if (err != AC_ERR_SUCCESS)
			return err;

		// helper function generates new ip
		uint32_t newIp = generateNewIp(ifIntSubnet, ifIntIp);

		printf("%sNew IP is ", TAB2);
		print_ip(newIp);

		// If a newIp is generated, the IP and subnet were not on the correct network
		//    ForceIp is used to change the IP to newIp, which allows the camera
		//    to connect
		err = acSystemForceIpAddress(hSystem, macAddress, newIp, subnetMask, defaultGateway);
		if (err != AC_ERR_SUCCESS)
			return err;

		printf("%sForced Camera to the correct network\n", TAB1);
	}
	else
	{
		printf("%sDevice already on correct network", TAB1);

		// Destroy and clean up the internal memory of the device
		acSystemDestroyDevice(hSystem, hDevice);
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

int main()
{
	printf("C_IpConfig_Auto\n");
	AC_ERROR err = AC_ERR_SUCCESS;

	// prepare example
	acSystem hSystem = NULL;
	err = acOpenSystem(&hSystem);
	CHECK_RETURN;
	srand( (uint32_t) time(NULL));

	// run example
	printf("Commence example\n\n");
	err = AutoIPConfig(hSystem);
	CHECK_RETURN;
	printf("\nExample complete\n");

	// clean up example
	err = acCloseSystem(hSystem);
	CHECK_RETURN;

	printf("Press enter to complete\n");
	while (getchar() != '\n') {};
	getchar();
	return -1;
}
