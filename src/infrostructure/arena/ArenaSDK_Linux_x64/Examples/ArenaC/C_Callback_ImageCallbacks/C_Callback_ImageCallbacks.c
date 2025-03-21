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
#include <string.h>

#if (!defined _WIN32 && !defined _WIN64)
#define scanf_s scanf
#endif

#if defined _WIN32
#include <windows.h>
#elif defined linux
#include <unistd.h>
#endif

#define TAB1 "  "
#define TAB2 "    "

// Callback: Image Callbacks
//    This example demonstrates configuring an image callback for a device. Once
//    a callback is registered and the device is streaming, the user-implemented
//    OnBuffer() function will be called. OnBuffer will receive an acBuffer and a
//    pointer to optional user-specified callback data. OnBuffer will display
//    the frame id and timestamp of the image before returning.

// =-=-=-=-=-=-=-=-=-
// =-=- SETTINGS =-=-
// =-=-=-=-=-=-=-=-=-

//// system timeout
#define SYSTEM_TIMEOUT 100

// number of seconds to stream for
#define STREAM_TIME_SEC 5

#define LEN_SERIAL_MAX 32

// maximum buffer length
#define MAX_BUF 1024

#if defined(_WIN32)
#define portable_sleep(x) Sleep(x * 1000)
#define portable_strcpy(d, s) strcpy_s(d, LEN_SERIAL_MAX, (char*)s);
#elif defined(__linux__)
#define portable_sleep(x) sleep(x)
#define portable_strcpy(d, s) strcpy(d, (char*)s);
#endif

// =-=-=-=-=-=-=-=-=-
// =-=- EXAMPLE -=-=-
// =-=-=-=-=-=-=-=-=-

// A callback function returns void and takes an acBuffer and a void* as arguments.
//    The Arena library will call the user-implemented OnBuffer() when an image
//    has been received.
void OnBuffer(acBuffer buffer, void* pParam)
{
	static uint64_t lastTimestamp = 0;
	static uint64_t imageCount = 0;
	uint64_t currTimestamp = 0;
	uint64_t frameId = 0;

	char serialNumber[LEN_SERIAL_MAX] = { 0 };

	if (pParam != NULL)
	{
		portable_strcpy(serialNumber, (char*)pParam);
	}

	acBufferGetFrameId(buffer, &frameId);
	acImageGetTimestamp(buffer, &currTimestamp);

	double diffMillisecond = (currTimestamp - lastTimestamp) / 1000000.0;

	printf("Serial: [%s], Image#: [%" PRIu64 "], FrameId: [%" PRIu64 "], TimeStamp: [%" PRIu64 "], Diff: [%f ms]\n",
		serialNumber,
		++imageCount,
		frameId,
		currTimestamp,
		diffMillisecond);

	lastTimestamp = currTimestamp;
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
	printf("C_Callback_ImageCallbacks\n");
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
		printf("Error: no devices connected, example requires at least one device.\n");
		printf("Press enter to complete\n");
		getchar();
		return -1;
	}
	acDevice hDevice = NULL;
	size_t selection = 0;
	err = SelectDevice(hSystem, &numDevices, &selection);
	CHECK_RETURN;
	err = acSystemCreateDevice(hSystem, selection, &hDevice);
	CHECK_RETURN;

	char serial[LEN_SERIAL_MAX] = { 0 };
	size_t serialLen = LEN_SERIAL_MAX;

	err = acSystemGetDeviceSerial(hSystem, selection, serial, &serialLen);
	CHECK_RETURN;

	// register callback
	acCallback hCallback;
	err = acDeviceRegisterImageCallback(hDevice, &hCallback, OnBuffer, serial);
	CHECK_RETURN;

	// get stream node map
	acNodeMap hTLStreamNodeMap = NULL;

	err = acDeviceGetTLStreamNodeMap(hDevice, &hTLStreamNodeMap);
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

	// run example
	printf("Commence example\n\n");

	err = acDeviceStartStream(hDevice);
	CHECK_RETURN;

	portable_sleep(STREAM_TIME_SEC);

	err = acDeviceStopStream(hDevice);
	CHECK_RETURN;

	// deregister callback
	err = acDeviceDeregisterImageCallback(hDevice, &hCallback);
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
