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

#if (!defined _WIN32 && !defined _WIN64)
#define scanf_s scanf
#endif

#define TAB1 "  "
#define TAB2 "    "

// Explore: Node Maps
//    This example explores the 5 available node maps of Arena, 4 retrieved from
//    any devices and 1 from the system. It demonstrates traversing the retrieved
//    nodes.

// =-=-=-=-=-=-=-=-=-
// =-=- SETTINGS =-=-
// =-=-=-=-=-=-=-=-=-

// Choose node maps to explore
#define EXPLORE_DEVICE_NODEMAP true
#define EXPLORE_TL_DEVICE_NODEMAP true
#define EXPLORE_TL_STREAM_NODEMAP true
#define EXPLORE_TL_INTERFACE_NODEMAP true
#define EXPLORE_TL_SYSTEM_NODEMAP true

// Device Timeout
//    Timeout for detecting camera devices (in milliseconds). If no device is
//    discovered at the end of the timeout, the example will return an error. The
//    timeout is the maximum time to wait to detect a device; the example will
//    resume once the internal list of devices has been updated, not waiting the
//    full extent of the timeout.
#define DEVICE_TIMEOUT 100

// maximum buffer length
#define MAX_BUF 1024

// =-=-=-=-=-=-=-=-=-
// =-=- EXAMPLE -=-=-
// =-=-=-=-=-=-=-=-=-

// explores a node map
// (1) total number of nodes in a node map
// (2) retrieves category nodes
AC_ERROR ExploreNodeMap(acNodeMap hNodeMap)
{
	AC_ERROR err = AC_ERR_SUCCESS;

	// Get number of nodes
	printf("%sNumber of nodes: ", TAB2);
	uint64_t numNodes = 0;

	err = acNodeMapGetNumNodes(hNodeMap, &numNodes);
	if (err != AC_ERR_SUCCESS)
		return err;

	printf("%" PRIu64 "\n", numNodes);

	// print category nodes
	printf("%sCategory nodes: ", TAB2);

	bool firstPass = true;

	for (uint64_t index = 0; index < numNodes; index++)
	{
		size_t index_size_t = (size_t)index;
		AC_INTERFACE_TYPE interfaceType = 0;
		acNode hNode = NULL;
		char pDisplayNameBuf[MAX_BUF];
		size_t bufLen = MAX_BUF;

		err = acNodeMapGetNodeByIndex(hNodeMap, index_size_t, &hNode);
		if (err != AC_ERR_SUCCESS)
			return err;

		err = acNodeGetPrincipalInterfaceType(hNode, &interfaceType);
		if (err != AC_ERR_SUCCESS)
			return err;

		AC_ACCESS_MODE accessModeCategoryNode = 0;
		err = acNodeGetAccessMode(hNode, &accessModeCategoryNode);
		if (err != AC_ERR_SUCCESS)
			return err;

		// get node display name
		if (interfaceType == AC_INTERFACE_TYPE_CATEGORY && accessModeCategoryNode != AC_ACCESS_MODE_NI)
		{
			err = acNodeGetDisplayName(hNode, pDisplayNameBuf, &bufLen);
			if (err != AC_ERR_SUCCESS)
				return err;

			if (firstPass)
			{
				printf("%s", pDisplayNameBuf);
				firstPass = false;
			}
			else
				printf(", %s", pDisplayNameBuf);
		}
		else
			continue;
	}
	printf("\n");

	return err;
}

// explores node maps
// (1) retrieves node maps from device
// (2) explores the device node map from device
// (3) explores the transport layer device node map from device
// (4) explores the transport layer stream node map from device
// (5) explores the transport layer interface node map from device
// (6) explores the transport layer system node map from system
AC_ERROR ExploreNodeMaps(acSystem hSystem, acDevice hDevice)
{
	AC_ERROR err = AC_ERR_SUCCESS;
	acNodeMap hDeviceNodeMap = NULL;
	acNodeMap hTLDeviceNodeMap = NULL;
	acNodeMap hTLStreamNodeMap = NULL;
	acNodeMap hTLInterfaceNodeMap = NULL;
	acNodeMap hTLSystemNodeMap = NULL;

	printf("%sRetrieve node maps \n", TAB1);

	// Explore device node map
	if (EXPLORE_DEVICE_NODEMAP)
	{
		err = acDeviceGetNodeMap(hDevice, &hDeviceNodeMap);

		printf("%sExplore device node map \n", TAB1);
		ExploreNodeMap(hDeviceNodeMap);
	}

	// Explore transport layer device node map
	if (EXPLORE_TL_DEVICE_NODEMAP)
	{
		err = acDeviceGetTLDeviceNodeMap(hDevice, &hTLDeviceNodeMap);

		printf("%sExplore transport layer device node map \n", TAB1);
		ExploreNodeMap(hTLDeviceNodeMap);
	}

	// Explore transport layer stream node map
	if (EXPLORE_TL_STREAM_NODEMAP)
	{
		err = acDeviceGetTLStreamNodeMap(hDevice, &hTLStreamNodeMap);

		printf("%sExplore transport layer stream node map \n", TAB1);
		ExploreNodeMap(hTLStreamNodeMap);
	}

	// Explore transport layer interface node map
	if (EXPLORE_TL_INTERFACE_NODEMAP)
	{
		err = acDeviceGetTLInterfaceNodeMap(hDevice, &hTLInterfaceNodeMap);

		printf("%sExplore transport layer interface node map \n", TAB1);
		ExploreNodeMap(hTLInterfaceNodeMap);
	}

	// Explore transport layer system node map
	if (EXPLORE_TL_SYSTEM_NODEMAP)
	{
		err = acSystemGetTLSystemNodeMap(hSystem, &hTLSystemNodeMap);

		printf("%sExplore transport layer system node map \n", TAB1);
		ExploreNodeMap(hTLSystemNodeMap);
	}

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
	printf("C_Explore_NodeMaps\n");
	AC_ERROR err = AC_ERR_SUCCESS;

	// prepare example

	acSystem hSystem = NULL;
	err = acOpenSystem(&hSystem);
	CHECK_RETURN;
	err = acSystemUpdateDevices(hSystem, DEVICE_TIMEOUT);
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
	acDevice hDevice = NULL;
	size_t selection = 0;
	err = SelectDevice(hSystem, &numDevices, &selection);
	CHECK_RETURN;
	err = acSystemCreateDevice(hSystem, selection, &hDevice);
	CHECK_RETURN;

	// run example
	printf("Commence example\n\n");
	err = ExploreNodeMaps(hSystem, hDevice);
	CHECK_RETURN;
	printf("\nExample complete\n");

	// clean up example
	err = acSystemDestroyDevice(hSystem, hDevice);
	CHECK_RETURN;
	err = acCloseSystem(hSystem);
	CHECK_RETURN;

	printf("Press enter to complete\n");
	while (getchar() != '\n') {};
	getchar();
	return 0;
}
