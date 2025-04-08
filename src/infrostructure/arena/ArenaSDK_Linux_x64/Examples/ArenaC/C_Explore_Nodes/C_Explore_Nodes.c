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

// Explore: Nodes
//    This example explores traversing the nodes as a tree and fundamental node
//    information including display name, node name, access mode visibility,
//	  interface type, and value.

// =-=-=-=-=-=-=-=-=-
// =-=- SETTINGS =-=-
// =-=-=-=-=-=-=-=-=-

// Choose node types to explore
#define EXPLORE_ACCESS true
#define EXPLORE_VISIBILITY true
#define EXPLORE_TYPE true
#define EXPLORE_VALUE true

// Device Timeout
//    Timeout for detecting camera devices (in milliseconds). If no device is
//    discovered at the end of the timeout, the example will return an error. The
//    timeout is the maximum time to wait to detect a device; the example will
//    resume once the internal list of devices has been updated, not waiting the
//    full extent of the timeout.
#define DEVICE_TIMEOUT 100

// maximum buffer length
#define MAX_BUF 1024

// maintains display indentation
void Indent(size_t depth)
{
	for (size_t i = 0; i < depth; i++)
		printf(" ");
}
// =-=-=-=-=-=-=-=-=-
// =-=- EXAMPLE -=-=-
// =-=-=-=-=-=-=-=-=-

// explores node
// (1) retrieves display name
// (2) retrieves node name
// (3) retrieves accessibility
// (4) retrieves visibility
// (5) retrieves interface type
// (6) retrieves value
AC_ERROR ExploreNode(acNode hNode, size_t depth)
{
	AC_ERROR err = AC_ERR_SUCCESS;

	// Retrieve display name
	char pDisplayNameBuf[MAX_BUF];
	size_t bufLen = MAX_BUF;
	err = acNodeGetDisplayName(hNode, pDisplayNameBuf, &bufLen);
	if (err != AC_ERR_SUCCESS)
		return err;
	
	// Retrieve node name
	char pNodeNameBuff[MAX_BUF];
	size_t nodeBufLen = MAX_BUF;
	err = acNodeGetName(hNode, pNodeNameBuff, &nodeBufLen);
	if (err != AC_ERR_SUCCESS)
		return err;

	// Retrieve accessibility
	AC_ACCESS_MODE accessMode = 0;
	err = acNodeGetAccessMode(hNode, &accessMode);
	if (err != AC_ERR_SUCCESS)
		return err;

	// Retrieve visibility
	AC_VISIBILITY visibility = 0;
	err = acNodeGetVisibility(hNode, &visibility);
	if (err != AC_ERR_SUCCESS)
		return err;

	// Retrieve interface type
	AC_INTERFACE_TYPE interfaceType = 0;
	err = acNodeGetPrincipalInterfaceType(hNode, &interfaceType);
	if (err != AC_ERR_SUCCESS)
		return err;

	// Retrieve value
	bool8_t isReadable = true;
	err = acIsReadable(hNode, &isReadable);
	if (err != AC_ERR_SUCCESS)
		return err;

	char pValueBuf[MAX_BUF];
	size_t valueBufLen = MAX_BUF;
	err = acValueToString(hNode, pValueBuf, &valueBufLen);
	if (err != AC_ERR_SUCCESS)
		return err;

	// print node information
	Indent(depth);
	printf("%s (%s)", pDisplayNameBuf, pNodeNameBuff);

	if (EXPLORE_ACCESS)
	{
		Indent(90 - (depth + bufLen + nodeBufLen));
		printf("%" PRId32, accessMode);
	}

	if (EXPLORE_VISIBILITY)
	{
		Indent(10);
		printf("%" PRId32, visibility);
	}

	if (EXPLORE_TYPE)
	{
		Indent(10);
		printf("%" PRId32, interfaceType);
	}

	if (EXPLORE_VALUE)
	{
		Indent(10);

		if (!isReadable)
			printf("-");
		else if (valueBufLen > 90)
			printf("...");
		else
			printf("%s", pValueBuf);
	}
	
	printf("\n");

	if (interfaceType == AC_INTERFACE_TYPE_CATEGORY && accessMode != AC_ACCESS_MODE_NI)
	{
		size_t numFeatures = 0;
		err = acCategoryGetNumFeatures(hNode, &numFeatures);
		if (err != AC_ERR_SUCCESS)
			return err;
		acNode hFeatureNode = NULL;
		depth = (depth + 1);
		for (size_t index = 0; index < numFeatures; index++)
		{

			err = acCategoryGetFeature(hNode, index, &hFeatureNode);
			if (err != AC_ERR_SUCCESS)
				return err;

			ExploreNode(hFeatureNode, depth);
		}
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
	printf("C_Explore_Nodes\n");
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

	// get nodemaps
	acNodeMap hDeviceNodeMap = NULL;
	err = acDeviceGetNodeMap(hDevice, &hDeviceNodeMap);
	if (err != AC_ERR_SUCCESS)
		return err;

	acNodeMap pTLDeviceNodeMap = NULL;
	err = acDeviceGetTLDeviceNodeMap(hDevice, &pTLDeviceNodeMap);
	if (err != AC_ERR_SUCCESS)
		return err;

	acNodeMap pTLStreamNodeMap = NULL;
	err = acDeviceGetTLStreamNodeMap(hDevice, &pTLStreamNodeMap);
	if (err != AC_ERR_SUCCESS)
		return err;

	acNodeMap pTLInterfaceNodeMap = NULL;
	err = acDeviceGetTLInterfaceNodeMap(hDevice, &pTLInterfaceNodeMap);
	if (err != AC_ERR_SUCCESS)
		return err;

	acNodeMap pTLSystemNodeMap = NULL;
	err = acSystemGetTLSystemNodeMap(hSystem, &pTLSystemNodeMap);
	if (err != AC_ERR_SUCCESS)
		return err;


	// get nodes by index
	size_t index = 0;
	
	acNode hDeviceRootNode = NULL;
	err = acNodeMapGetNodeByIndex(hDeviceNodeMap, index, &hDeviceRootNode);
	if (err != AC_ERR_SUCCESS)
		return err;

	acNode hTLDeviceRootNode = NULL;
	err = acNodeMapGetNodeByIndex(pTLDeviceNodeMap, index, &hTLDeviceRootNode);
	if (err != AC_ERR_SUCCESS)
		return err;

	acNode hTLStreamRootNode = NULL;
	err = acNodeMapGetNodeByIndex(pTLStreamNodeMap, index, &hTLStreamRootNode);
	if (err != AC_ERR_SUCCESS)
		return err;

	acNode hTLInterfaceRootNode = NULL;
	err = acNodeMapGetNodeByIndex(pTLInterfaceNodeMap, index, &hTLInterfaceRootNode);
	if (err != AC_ERR_SUCCESS)
		return err;
	
	acNode hTLSystemRootNode = NULL;
	err = acNodeMapGetNodeByIndex(pTLSystemNodeMap, index, &hTLSystemRootNode);
	if (err != AC_ERR_SUCCESS)
		return err;

	// run explore node example
	printf("Commence example\n");
	size_t depth = 0;
	
	printf("\nDevice Nodemap\n");
	err = ExploreNode(hDeviceRootNode, depth);
	CHECK_RETURN;

	printf("\nTL Device Nodemap\n");
	err = ExploreNode(hTLDeviceRootNode, depth);
	CHECK_RETURN;

	printf("\nTL Stream Nodemap\n");
	err = ExploreNode(hTLStreamRootNode, depth);
	CHECK_RETURN;

	printf("\nTL Interface Nodemap\n");
	err = ExploreNode(hTLInterfaceRootNode, depth);
	CHECK_RETURN;

	printf("\nTL System Nodemap\n");
	err = ExploreNode(hTLSystemRootNode, depth);
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
