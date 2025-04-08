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

// Acquisition: Introduction
//    Some 10G higher bandwidth LUCID cameras support TCP streaming.The TCP protocol
//    implements a reliable connection-based stream at the hardware level,
//    eliminating the need for a software-based packet resend mechanism.

// =-=-=-=-=-=-=-=-=-
// =-=- SETTINGS =-=-
// =-=-=-=-=-=-=-=-=-

// image timeout
#define IMAGE_TIMEOUT 2000

// number of images to grab
#define NUM_IMAGES 25

// maximum buffer length
#define MAX_BUF 1024

// timeout for detecting camera devices (in milliseconds).
#define SYSTEM_TIMEOUT 100

// =-=-=-=-=-=-=-=-=-
// =-=- HELPER =-=-=-
// =-=-=-=-=-=-=-=-=-

// gets node value
// (1) gets node
// (2) checks access mode
// (3) gets value
AC_ERROR GetNodeValue(acNodeMap hNodeMap, const char* nodeName, char* pValue, size_t* pLen)
{
	AC_ERROR err = AC_ERR_SUCCESS;

	// get node
	acNode hNode = NULL;
	AC_ACCESS_MODE accessMode = 0;

	err = acNodeMapGetNodeAndAccessMode(hNodeMap, nodeName, &hNode, &accessMode);
	if (err != AC_ERR_SUCCESS)
		return err;

	// check access mode
	if (accessMode != AC_ACCESS_MODE_RO && accessMode != AC_ACCESS_MODE_RW)
		return AC_ERR_ERROR;

	// get value
	err = acValueToString(hNode, pValue, pLen);
	return err;
}

// sets node value
// (1) gets node
// (2) checks access mode
// (3) gets value
AC_ERROR SetNodeValue(acNodeMap hNodeMap, const char* nodeName, const char* pValue)
{
	AC_ERROR err = AC_ERR_SUCCESS;

	// get node
	acNode hNode = NULL;
	AC_ACCESS_MODE accessMode = 0;

	err = acNodeMapGetNodeAndAccessMode(hNodeMap, nodeName, &hNode, &accessMode);
	if (err != AC_ERR_SUCCESS)
		return err;

	// check access mode
	if (accessMode != AC_ACCESS_MODE_WO && accessMode != AC_ACCESS_MODE_RW)
		return AC_ERR_ERROR;

	// get value
	err = acValueFromString(hNode, pValue);
	return err;
}

// =-=-=-=-=-=-=-=-=-
// =-=- EXAMPLE -=-=-
// =-=-=-=-=-=-=-=-=-

// demonstrates acquisition
// (1) sets acquisition mode
// (2) sets buffer handling mode
// (3) set transport stream protocol to TCP
// (4) starts the stream
// (5) gets a number of images
// (6) prints information from images
// (7) requeues buffers
// (8) stops the stream
AC_ERROR AcquireImages(acDevice hDevice)
{
	AC_ERROR err = AC_ERR_SUCCESS;

	// get node map
	acNodeMap hNodeMap = NULL;

	err = acDeviceGetNodeMap(hDevice, &hNodeMap);
	if (err != AC_ERR_SUCCESS)
		return err;

	// get node values that will be changed in order to return their values at
	// the end of the example
	char pAcquisitionModeInitial[MAX_BUF];
	size_t len = MAX_BUF;

	err = GetNodeValue(hNodeMap, "AcquisitionMode", pAcquisitionModeInitial, &len);
	if (err != AC_ERR_SUCCESS)
		return err;

	// set acquisition mode
	printf("%sSet acquisition mode to 'Continuous'\n", TAB1);

	err = SetNodeValue(
		hNodeMap,
		"AcquisitionMode",
		"Continuous");

	if (err != AC_ERR_SUCCESS)
		return err;

	// set buffer handling mode
	printf("%sSet buffer handling mode to 'NewestOnly'\n", TAB1);

	// get stream node map
	acNodeMap hTLStreamNodeMap = NULL;

	err = acDeviceGetTLStreamNodeMap(hDevice, &hTLStreamNodeMap);
	if (err != AC_ERR_SUCCESS)
		return err;

	err = SetNodeValue(
		hTLStreamNodeMap,
		"StreamBufferHandlingMode",
		"NewestOnly");

	// The TransportStreamProtocol node can tell the camera to use the TCP datastream engine. When
	//    set to TCP - Arena will switch to using the TCP datastream engine. 
	//    There is no further necessary configuration, though to achieve maximum throughput 
	//    users may want to set the "DeviceLinkThroughputReserve" to 0 and 
	//    also set the stream channel packet delay "GevSCPD" to 0.

	acNode hTransportStreamProtocolNode = NULL;
	AC_ACCESS_MODE accessModeTransportStreamProtocol = 0;

	err = acNodeMapGetNodeAndAccessMode(hNodeMap, "TransportStreamProtocol", &hTransportStreamProtocolNode, &accessModeTransportStreamProtocol);
	if (err != AC_ERR_SUCCESS)
		return err;

	if (accessModeTransportStreamProtocol != AC_ACCESS_MODE_NI)
	{
		// get node value
		char pTransportStreamProtocolInitial[MAX_BUF];
		size_t len = MAX_BUF;

		err = GetNodeValue(hNodeMap, "TransportStreamProtocol", pTransportStreamProtocolInitial, &len);
		if (err != AC_ERR_SUCCESS)
			return err;

		printf("%sSet Transport Stream Protocol to TCP\n", TAB1);

		err = acNodeMapSetEnumerationValue(hNodeMap, "TransportStreamProtocol", "TCP");
		if (err != AC_ERR_SUCCESS)
			return err;

		// start stream
		printf("%sStart stream\n", TAB1);

		err = acDeviceStartStream(hDevice);
		if (err != AC_ERR_SUCCESS)
			return err;

		// get images
		printf("%sGetting %d images\n", TAB1, NUM_IMAGES);

		int i = 0;
		for (i = 0; i < NUM_IMAGES; i++)
		{
			// get image
			printf("%sGet image %d", TAB2, i);
			acBuffer hBuffer = NULL;

			err = acDeviceGetBuffer(hDevice, IMAGE_TIMEOUT, &hBuffer);
			if (err != AC_ERR_SUCCESS)
				return err;

			// get image information
			printf(" (");

			// get and display size filled
			size_t sizeFilled = 0;

			err = acBufferGetSizeFilled(hBuffer, &sizeFilled);
			if (err != AC_ERR_SUCCESS)
				return err;

			printf("%zu bytes; ", sizeFilled);

			// get and display width
			size_t width = 0;

			err = acImageGetWidth(hBuffer, &width);
			if (err != AC_ERR_SUCCESS)
				return err;

			printf("%zux", width);

			// get and display height
			size_t height = 0;

			err = acImageGetHeight(hBuffer, &height);
			if (err != AC_ERR_SUCCESS)
				return err;

			printf("%zu; ", height);

			// get and display timestamp
			uint64_t timestampNs = 0;

			err = acImageGetTimestampNs(hBuffer, &timestampNs);
			if (err != AC_ERR_SUCCESS)
				return err;

			printf("timestamp (ns): %" PRIu64 ")", timestampNs);

			// requeue image buffer
			printf(" and requeue\n");

			err = acDeviceRequeueBuffer(hDevice, hBuffer);
			if (err != AC_ERR_SUCCESS)
				return err;
		}

		// stop stream
		printf("%sStop stream\n", TAB1);

		err = acDeviceStopStream(hDevice);
		if (err != AC_ERR_SUCCESS)
			return err;

		// return node to its initial values
		err = SetNodeValue(
			hNodeMap,
			"TransportStreamProtocol",
			pTransportStreamProtocolInitial);

	} else {

		printf("%s\nConnected camera does not support TCP stream\n", TAB1);
	}

	// return node to its initial values
	err = SetNodeValue(
		hNodeMap,
		"AcquisitionMode",
		pAcquisitionModeInitial);

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
	printf("C_Acquisition_TCP\n");
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
	acDevice hDevice = NULL;
	size_t selection = 0;
	err = SelectDevice(hSystem, &numDevices, &selection);
	CHECK_RETURN;
	err = acSystemCreateDevice(hSystem, selection, &hDevice);
	CHECK_RETURN;

	// run example
	printf("Commence example\n\n");
	err = AcquireImages(hDevice);
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
	return -1;
}
