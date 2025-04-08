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
#include <inttypes.h>
#include <stdbool.h>  // defines boolean type and values
#include <math.h>
#include <string.h>

#if (!defined _WIN32 && !defined _WIN64)
#define scanf_s scanf
#endif

#define TAB1 "  "
#define TAB2 "    "

// IP Config Manual
//    This example sets persistent IP on the camera. 5 parts:
//    1) Persistent IP address to 169.254.3.2
//    2) Subnet mask to 255.255.0.0
//    3) Enables persistent IP
//    4) Disables DHCP
//    5) Disables ARP conflict detection

// =-=-=-=-=-=-=-=-=-
// =-=- SETTINGS =-=-
// =-=-=-=-=-=-=-=-=-

// image timeout
#define TIMEOUT 2000

// update Timeout full extent of the timeout.
#define SYSTEM_TIMEOUT 100

// maximum buffer length
#define MAX_BUF 1024

// =-=-=-=-=-=-=-=-=-
// =-=- EXAMPLE -=-=-
// =-=-=-=-=-=-=-=-=-

AC_ERROR SetIpConfig(acDevice hDevice)
{
	AC_ERROR err = AC_ERR_SUCCESS;
	if (err != AC_ERR_SUCCESS)
		return err;

	// get node maps
	acNodeMap hNodeMap;

	err = acDeviceGetNodeMap(hDevice, &hNodeMap);
	if (err != AC_ERR_SUCCESS)
		return err;

	// Calculate IP addresses as integers.
	//    Each octet is shifted individually and added to the address. They must
	//    be received as 64-bit integers.
	int64_t address = (int64_t)(169 * pow(2, 24) + 254 * pow(2, 16) + 3 * pow(2, 8) + 2);
	int64_t subnet_mask = (int64_t)(255 * pow(2, 24) + 255 * pow(2, 16) + 0 * pow(2, 8) + 0);

	printf("%sSet persistent IP address to %d.%d.%d.%d\n",
		TAB1,
		(int)(address / pow(2, 24)),
		(int)((address % (int)pow(2, 24)) / pow(2, 16)),
		(int)((address % (int)pow(2, 16)) / pow(2, 8)),
		(int)((address % (int)pow(2, 8)))
	);

	err = acNodeMapSetIntegerValue(hNodeMap,
		"GevPersistentIPAddress",
		address);

	if (err != AC_ERR_SUCCESS)
		return err;

	printf("%sSet persistent subnet mask to %d.%d.%d.%d\n",
		TAB1,
		(int)(subnet_mask / pow(2, 24)),
		(int)((subnet_mask % (int)pow(2, 24)) / pow(2, 16)),
		(int)((subnet_mask % (int)pow(2, 16)) / pow(2, 8)),
		(int)((subnet_mask % (int)pow(2, 8)))
	);

	err = acNodeMapSetIntegerValue(hNodeMap,
		"GevPersistentSubnetMask",
		subnet_mask);

	if (err != AC_ERR_SUCCESS)
		return err;

	printf("%sEnabling persistent IP\n", TAB1);
	err = acNodeMapSetBooleanValue(hNodeMap,
		"GevCurrentIPConfigurationPersistentIP",
		true);

	if (err != AC_ERR_SUCCESS)
		return err;

	printf("%sDisabling DHCP\n", TAB1);
	err = acNodeMapSetBooleanValue(hNodeMap,
		"GevCurrentIPConfigurationDHCP",
		false);

	if (err != AC_ERR_SUCCESS)
		return err;

	printf("%sDisabling ARP conflict detection\n", TAB1);
	err = acNodeMapSetBooleanValue(hNodeMap,
		"GevPersistentARPConflictDetectionEnable",
		false);

	if (err != AC_ERR_SUCCESS)
		return err;


	return AC_ERR_SUCCESS;
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

	printf(TAB1 "Select a device to set persistent IP on the camera:\n");
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
	printf("C_IpConfig_Manual\n");
	AC_ERROR err = AC_ERR_SUCCESS;

	// user prompt for possible device settings overwrite
	printf("Example may overwrite device settings saved -- proceed? ('y' to continue) ");
	char continueExample[MAX_BUF];

	if ((fgets(continueExample, sizeof continueExample, stdin)) != NULL)
		;
	if (0 == strcmp(continueExample, "y\n"))
	{

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
			return 0;
		}
		else
		{
			acDevice hDevice = NULL;
			size_t selection = 0;
			err = SelectDevice(hSystem, &numDevices, &selection);
			CHECK_RETURN;
			err = acSystemCreateDevice(hSystem, selection, &hDevice);
			CHECK_RETURN;

			// run example
			printf("Commence example\n\n");
			err = SetIpConfig(hDevice);
			CHECK_RETURN;

			// clean up example
			printf("%sClean Up Arena\n", TAB1);
			err = acSystemDestroyDevice(hSystem, hDevice);
			CHECK_RETURN;
			err = acCloseSystem(hSystem);
			CHECK_RETURN;

			printf("\nExample complete\n");
		}
	}

	printf("Press enter to complete\n");
	while (getchar() != '\n') {};
	getchar();
	return 0;
}
