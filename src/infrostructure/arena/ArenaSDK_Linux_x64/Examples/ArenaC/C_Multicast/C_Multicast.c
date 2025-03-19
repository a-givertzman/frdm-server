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
#include <time.h>     // defines time type and values
#include <string.h>   // defines string type and values

#if (!defined _WIN32 && !defined _WIN64)
#define scanf_s scanf
#endif

#define TAB1 "  "
#define TAB2 "    "

// Multicast: Introduction
//    This example demonstrates multicasting from the master's perspective.
//    Multicasting allows for the streaming of images and events to multiple
//    destinations. Multicasting requires nearly the same steps for both masters
//    and listeners. The only difference, as seen below, is that device features
//    can only be set by the master.

// =-=-=-=-=-=-=-=-=-
// =-=- SETTINGS =-=-
// =-=-=-=-=-=-=-=-=-

// Length of time to grab images (sec)
//    Note that the listener must be started while the master is still streaming,
//    and that the listener will not receive any more images once the master
//    stops streaming.
#define NUM_SECONDS 20

// image timeout
#define IMAGE_TIMEOUT 2000

// maximum buffer length
#define MAX_BUF 1024

// timeout for detecting camera devices (in milliseconds).
#define SYSTEM_TIMEOUT 100

// =-=-=-=-=-=-=-=-=-
// =-=- EXAMPLE -=-=-
// =-=-=-=-=-=-=-=-=-

// demonstrates acquisition
// (1) enable multicast
// (2) prepare settings on master, not on listener
// (3) stream regularly
AC_ERROR AcquireImages(acDevice hDevice)
{
	AC_ERROR err = AC_ERR_SUCCESS;

	// get node map
	acNodeMap hNodeMap = NULL;

	err = acDeviceGetNodeMap(hDevice, &hNodeMap);
	if (err != AC_ERR_SUCCESS)
		return err;

	// get TL stream node map
	acNodeMap hTLStreamNodeMap = NULL;

	err = acDeviceGetTLStreamNodeMap(hDevice, &hTLStreamNodeMap);
	if (err != AC_ERR_SUCCESS)
		return err;

	// get TL device node map
	acNodeMap hTLDeviceNodeMap = NULL;

	err = acDeviceGetTLDeviceNodeMap(hDevice, &hTLDeviceNodeMap);
	if (err != AC_ERR_SUCCESS)
		return err;

	// get node values that will be changed in order to return their values at
	// the end of the example
	char pAcquisitionModeInitial[MAX_BUF];
	size_t pAcquisitionModeBufLen = MAX_BUF;

	err = acNodeMapGetStringValue(hNodeMap, "AcquisitionMode", pAcquisitionModeInitial, &pAcquisitionModeBufLen);

	if (err != AC_ERR_SUCCESS)
		return err;

	// Enable multicast
	//    Multicast must be enabled on both the master and listener. A small
	//    number of transport layer features will remain writable even though a
	//    device's access mode might be read-only.
	printf("%sEnable multicast\n", TAB1);

	err = acNodeMapSetBooleanValue(hTLStreamNodeMap, "StreamMulticastEnable", true);
	if (err != AC_ERR_SUCCESS)
		return err;

	// Prepare settings on master, not on listener
	//    Device features must be set on the master rather than the listener.
	//    This is because the listener is opened with a read-only access mode.

	char pDeviceAccessStatus[MAX_BUF];
	size_t pDeviceAccessStatusBufLen = MAX_BUF;

	err = acNodeMapGetStringValue(hTLDeviceNodeMap, "DeviceAccessStatus", pDeviceAccessStatus, &pDeviceAccessStatusBufLen);
	if (err != AC_ERR_SUCCESS)
		return err;

	// master
	if (strcmp(pDeviceAccessStatus, "ReadWrite") == 0)
	{
		printf("%sHost streaming as 'master'\n", TAB1);

		// set acquisition mode
		printf("%sSet acquisition mode to 'Continuous'\n", TAB2);

		err = acNodeMapSetStringValue(hNodeMap, "AcquisitionMode", "Continuous");
		if (err != AC_ERR_SUCCESS)
			return err;

		// enable stream auto negotiate packet size
		err = acNodeMapSetBooleanValue(hTLStreamNodeMap, "StreamAutoNegotiatePacketSize", true);
		if (err != AC_ERR_SUCCESS)
			return err;

		// enable stream packet resend
		err = acNodeMapSetBooleanValue(hTLStreamNodeMap, "StreamPacketResendEnable", true);
		if (err != AC_ERR_SUCCESS)
			return err;
	}
	// listener
	else
	{
		printf("%sHost streaming as 'listener'\n", TAB2);
	}

	printf("%sStart stream\n", TAB1);
	err = acDeviceStartStream(hDevice);
	if (err != AC_ERR_SUCCESS)
		return err;

	// define image count to detect if all images are not received
	int imageCount = 0;
	int unreceivedImageCount = 0;

	// get images
	printf("%sGetting images for %d seconds\n", TAB1, NUM_SECONDS);

	// define start and latest time for timed image acquisition
	time_t startTime = time(0);
	time_t latestTime = time(0);

	// define buffer for device
	acBuffer hBuffer = NULL;

	while ((latestTime - startTime) < NUM_SECONDS)
	{
		// update time
		latestTime = time(0);

		// get image
		imageCount++;

		err = acDeviceGetBuffer(hDevice, IMAGE_TIMEOUT, &hBuffer);
		if (err == AC_ERR_TIMEOUT) {
			printf("%sNo image received\n", TAB2);
			unreceivedImageCount++;
		}
		else if (err == AC_ERR_SUCCESS) 
		{

			// Print identifying information
			//    Using the frame ID and timestamp allows for the comparison of
			//    images between multiple hosts.
			printf("%sImage retrieved (", TAB2);

			// get and display frame ID
			uint64_t frameID = 0;

			err = acBufferGetFrameId(hBuffer, &frameID);
			if (err != AC_ERR_SUCCESS)
				return err;

			printf("frame ID %" PRIu64 "; ", frameID);

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

	}
	if (unreceivedImageCount == imageCount) 
	{
		printf("\nNo images were received, this can be caused by firewall or VPN settings\n");
		printf("Please add the application to firewall exception\n\n");
	}

	// stop stream
	printf("%sStop stream\n", TAB1);

	err = acDeviceStopStream(hDevice);
	if (err != AC_ERR_SUCCESS)
		return err;

	if (strcmp(pDeviceAccessStatus, "ReadWrite") == 0)
	{
		// return node to its initial value
		err = acNodeMapSetStringValue(hNodeMap, "AcquisitionMode", pAcquisitionModeInitial);
		if (err != AC_ERR_SUCCESS)
			return err;
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
	printf("C_Multicast\n");
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
