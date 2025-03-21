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

#if (!defined _WIN32 && !defined _WIN64)
#define scanf_s scanf
#endif

#define TAB1 "  "
#define TAB2 "    "

// Force IP
//    This example demonstrates how to force network settings. It does this by
//    adding 1 to the final octet of the IP address. It leaves the subnet mask
//    and default gateway unchanged, although the same method is used to change these
//    as well.

// =-=-=-=-=-=-=-=-=-
// =-=- SETTINGS =-=-
// =-=-=-=-=-=-=-=-=-

// update timeout
#define SYSTEM_TIMEOUT 100

// maximum buffer length
#define MAX_BUF 1024

// =-=-=-=-=-=-=-=-=-
// =-=- EXAMPLE -=-=-
// =-=-=-=-=-=-=-=-=-

// demonstrates forcing a new IP address
// (1) discovers devices and gets information
// (2) prepares IP address by adding 1 to the last octet
// (3) forces new IP address
// (4) discovers devices and gets information again
AC_ERROR ForceNetworkSettings(acSystem hSystem, size_t* selection)
{
	AC_ERROR err = AC_ERR_SUCCESS;

	// discover devices
	printf("%sDiscover devices\n", TAB1);

	err = acSystemUpdateDevices(hSystem, SYSTEM_TIMEOUT);
	if (err != AC_ERR_SUCCESS)
		return err;

	// Get device information
	//    Forcing the IP address requires a device's MAC address to specify the
	//    device. This example grabs the IP address, subnet mask, and default
	//    gateway as well to display changes and return the device to its
	//    original IP address.
	printf("%sGet device information\n", TAB1);

	// get device information as integer values to set new IP and to return at
	// the end of the example
	uint64_t macAddress = 0;
	uint32_t ipAddress = 0;
	uint32_t subnetMask = 0;
	uint32_t defaultGateway = 0;

	err = acSystemGetDeviceMacAddress(hSystem, *selection, &macAddress) |
		  acSystemGetDeviceIpAddress(hSystem, *selection, &ipAddress) |
		  acSystemGetDeviceSubnetMask(hSystem, *selection, &subnetMask) |
		  acSystemGetDeviceDefaultGateway(hSystem, *selection, &defaultGateway);
	if (err != AC_ERR_SUCCESS)
		printf("\nWarning: failed to retrieve one or more initial address integer values\n");

	// Get device information as a string to print
	char pMacAddressStr[MAX_BUF];
	size_t macAddressStrBufLen = MAX_BUF;
	char pIpAddressStr[MAX_BUF];
	size_t ipAddressStrBufLen = MAX_BUF;
	char pSubnetMaskStr[MAX_BUF];
	size_t subnetMaskStrBufLen = MAX_BUF;
	char pDefaultGatewayStr[MAX_BUF];
	size_t defaultGatewayStrBufLen = MAX_BUF;

	err = acSystemGetDeviceMacAddressStr(hSystem, *selection, pMacAddressStr, &macAddressStrBufLen) |
		  acSystemGetDeviceIpAddressStr(hSystem, *selection, pIpAddressStr, &ipAddressStrBufLen) |
		  acSystemGetDeviceSubnetMaskStr(hSystem, *selection, pSubnetMaskStr, &subnetMaskStrBufLen) |
		  acSystemGetDeviceDefaultGatewayStr(hSystem, *selection, pDefaultGatewayStr, &defaultGatewayStrBufLen);
	if (err != AC_ERR_SUCCESS)
		printf("\nWarning: failed to retrieve one or more initial address string values\n");

	printf("%sMAC %s\n", TAB2, pMacAddressStr);
	printf("%sIP %s\n", TAB2, pIpAddressStr);
	printf("%sSubnet %s\n", TAB2, pSubnetMaskStr);
	printf("%sGateway %s\n", TAB2, pDefaultGatewayStr);

	// Add 1 to IP address
	//    The new IP address takes the current IP address and adds 1 to the final
	//    octet. If the final octet is 254, the final octet is set to 1 (to avoid
	//    0 and 255).
	uint32_t ipAddressToSet = 0;

	if ((ipAddress & 0x000000FF) == 0x000000FE)
	{
		ipAddressToSet = ipAddress & 0xFFFFFF01;
	}
	else
	{
		ipAddressToSet = ipAddress + 0x00000001;
	}
	printf("%sPrepare new IP address %" PRIu32 "."
		"%" PRIu32 "."
		"%" PRIu32 "."
		"%" PRIu32 "\n",
		TAB2,
		(ipAddressToSet >> 24 & 0xFF),
		(ipAddressToSet >> 16 & 0xFF),
		(ipAddressToSet >> 8 & 0xFF),
		(ipAddressToSet & 0xFF));

	// Force the new IP address
	//    Cast IP address, subnet mask, and default gateway to uint64_t type and
	//    set the new IP address
	ipAddressToSet = (uint64_t)ipAddressToSet;
	subnetMask = (uint64_t)subnetMask;
	defaultGateway = (uint64_t)defaultGateway;

	err = acSystemForceIpAddress(hSystem, macAddress, ipAddressToSet, subnetMask, defaultGateway);
	if (err != AC_ERR_SUCCESS)
		return err;

	// Discover devices again
	//    Once a device has been forced, it needs to be rediscovered with its new
	//    network settings. This is especially important if moving on to
	//    configuration and acquisition.
	printf("%sDiscover devices again\n", TAB1);

	err = acSystemUpdateDevices(hSystem, SYSTEM_TIMEOUT);
	if (err != AC_ERR_SUCCESS)
		return err;

	// Get device information again
	//    Notice that the MAC address, subnet mask, and default gateway all
	//    retain their initial values while the last octet of the IP address
	//    has been incremented by 1.
	printf("%sGet device information again\n", TAB1);

	err = acSystemGetDeviceMacAddressStr(hSystem, *selection, pMacAddressStr, &macAddressStrBufLen) |
		  acSystemGetDeviceIpAddressStr(hSystem, *selection, pIpAddressStr, &ipAddressStrBufLen) |
		  acSystemGetDeviceSubnetMaskStr(hSystem, *selection, pSubnetMaskStr, &subnetMaskStrBufLen) |
		  acSystemGetDeviceDefaultGatewayStr(hSystem, *selection, pDefaultGatewayStr, &defaultGatewayStrBufLen);
	if (err != AC_ERR_SUCCESS)
		printf("\nWarning: failed to retrieve one or more address string values\n");

	printf("%sMAC %s\n", TAB2, pMacAddressStr);
	printf("%sIP %s\n", TAB2, pIpAddressStr);
	printf("%sSubnet %s\n", TAB2, pSubnetMaskStr);
	printf("%sGateway %s\n", TAB2, pDefaultGatewayStr);

	// return IP address to its initial value
	err = acSystemForceIpAddress(hSystem, macAddress, ipAddress, subnetMask, defaultGateway);
	if (err != AC_ERR_SUCCESS)
		return err;

	return err;
}

// =-=-=-=-=-=-=-=-=-
// =- PREPARATION -=-
// =- & CLEAN UP =-=-
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

	printf(TAB1 "Select device:\n");
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
			while (getchar() != '\n')
				;
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
	printf("C_ForceIp\n");
	AC_ERROR err = AC_ERR_SUCCESS;

	// prepare example
	acSystem hSystem = NULL;
	err = acOpenSystem(&hSystem);
	CHECK_RETURN;
	err = acSystemUpdateDevices(hSystem, SYSTEM_TIMEOUT);
	CHECK_RETURN;
	size_t numDevices = 0;
	err = acSystemGetNumDevices(hSystem, &numDevices);
	CHECK_RETURN;
	if (numDevices == 0)
	{
		printf("\nNo camera connected\nPress enter to complete\n");
		getchar();
		return -1;
	}

	size_t selection = 0;
	err = SelectDevice(hSystem, &numDevices, &selection);
	CHECK_RETURN;

	// run example
	printf("Commence example\n\n");
	err = ForceNetworkSettings(hSystem, &selection);
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
