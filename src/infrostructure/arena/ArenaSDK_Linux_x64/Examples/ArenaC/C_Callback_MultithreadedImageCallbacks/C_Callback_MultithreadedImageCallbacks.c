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

// Multithreading is implemented differently for Windows and Linux
//    systems, so headers, functions and macros are defined according to the
//    operating system being used.
#ifdef _WIN32
#include "ArenaCThreadWindows.h"
#elif defined linux
#include "ArenaCThreadLinux.h"
#endif

#define TAB1 "  "
#define TAB2 "    "

// Callback: Multithreaded Image Callbacks
//    This example demonstrates configuring a callback within a thread. Once the
//    thread has been launched, each new image is acquired and the callback is
//    triggered to retrieve the image's frame ID. Once the callback function
//    exits, the image buffer is requeued. After all images have been acquired,
//    the thread exits and memory is cleaned up.

// =-=-=-=-=-=-=-=-=-
// =-=- SETTINGS =-=-
// =-=-=-=-=-=-=-=-=-

// image timeout
#define IMAGE_TIMEOUT 2000

// number of images to grab
#define NUM_IMAGES 25

// system timeout
#define SYSTEM_TIMEOUT 100

// maximum buffer length
#define MAX_BUF 1024

// =-=-=-=-=-=-=-=-=-
// =-=- EXAMPLE -=-=-
// =-=-=-=-=-=-=-=-=-

typedef struct _MsgData
{
	acBuffer buffer;
	int count;
} MsgData;

// callback declaration
void (*callback)(MsgData*);

// return error
// (1) exit current thread
// (2) notify other thread to exit
// (3) return error
AC_ERROR exitThreads(AC_ERROR err)
{
	return err;
}
// implements callback message
void OnImage(MsgData* msg)
{

	// Get frameId on callback
	uint64_t frameId = 0;

	acBufferGetFrameId(msg->buffer, &frameId);
	printf("%sImage callback triggered (frame ID %" PRIu64 ")\n", TAB2, frameId);

	// increment count
	msg->count = msg->count + 1;
}

// implements exit condition
bool8_t ContinueAcquiring(MsgData* msg)
{
	int imageCount = msg->count;
	const int k_numImages = NUM_IMAGES;

	return imageCount < k_numImages;
}

// acquisition thread
THREAD_FUNCTION_SIGNATURE(AcquisitionThread)
{
	AC_ERROR err = AC_ERR_SUCCESS;
	acDevice hDevice = (acDevice*)lpParam;

	// get stream node map
	acNodeMap hTLStreamNodeMap = NULL;

	err = acDeviceGetTLStreamNodeMap(hDevice, &hTLStreamNodeMap);
	if (err != AC_ERR_SUCCESS)
		exitThreads(err);

	// enable stream auto negotiate packet size
	err = acNodeMapSetBooleanValue(hTLStreamNodeMap, "StreamAutoNegotiatePacketSize", true);
	if (err != AC_ERR_SUCCESS)
		exitThreads(err);

	// enable stream packet resend
	err = acNodeMapSetBooleanValue(hTLStreamNodeMap, "StreamPacketResendEnable", true);
	if (err != AC_ERR_SUCCESS)
		exitThreads(err);

	// start stream upon entering acquisition thread
	printf("%sEnter acquisition thread\n", TAB1);
	MsgData callbackMsg;
	callbackMsg.count = 0;

	err = acDeviceStartStream(hDevice);
	if (err != AC_ERR_SUCCESS)
		exitThreads(err);

	// acquire images
	while (ContinueAcquiring(&callbackMsg))
	{
		// get image buffer
		printf("%sImage retrieved\n", TAB2);
		acBuffer hBuffer = NULL;

		err = acDeviceGetBuffer(hDevice, IMAGE_TIMEOUT, &hBuffer);
		if (err != AC_ERR_SUCCESS)
			exitThreads(err);

		callbackMsg.buffer = hBuffer;

		// Trigger callback
		callback = &OnImage;
		(*callback)(&callbackMsg);

		// requeue image buffer
		err = acDeviceRequeueBuffer(hDevice, hBuffer);
		if (err != AC_ERR_SUCCESS)
			exitThreads(err);
	};

	// stop stream before exiting acquisition thread
	printf("%sExit acquisition thread\n", TAB1);

	err = acDeviceStopStream(hDevice);
	if (err != AC_ERR_SUCCESS)
		exitThreads(err);

	THREAD_RETURN(err);
}

// main thread
AC_ERROR MainThread(acDevice hDevice)
{
	AC_ERROR err = AC_ERR_SUCCESS;

	// create thread
	THREAD_ID(hChildThread);
	acThreadCreate(AcquisitionThread, hDevice, &hChildThread);

	// wait for threads to complete and close threads
	acThreadDestroy(hChildThread);

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
	printf("C_Callback_MultithreadedImageCallbacks\n");
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
	err = MainThread(hDevice);
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
