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
#include <stdbool.h> // defines boolean type and values

#if (!defined _WIN32 && !defined _WIN64)
#define scanf_s scanf
#endif

#define TAB1 "  "
#define TAB2 "    "
#define TAB3 "      "

// Trigger: WaitForNextLeader
//    WaitForNextLeader feature uses a first packet of every incoming image to
//    inform users that the camera is done integrating. This is an approximation
//    of what the Exposure End event does, but it simplifies the process because
//    we don't need to start a whole new event channel, and reuses data that has
//    to be transmitted already for the purpose of delivering the image to the
//    user.

// =-=-=-=-=-=-=-=-=-
// =-=- SETTINGS =-=-
// =-=-=-=-=-=-=-=-=-

// image timeout
#define TIMEOUT_MILLISEC 2000

// number of images to capture
#define NUMBER_IMAGES_TO_CAPTURE 10

// maximum buffer length
#define MAX_BUF 1024

// Wait for the next leader for each triggered image or every 3rd image.
//    In some cases, the user might need to reset WaitForNextleader device state
#define WAIT_FOR_LEADER_EVERY_3RD_IMAGE false

// =-=-=-=-=-=-=-=-=-
// =-=- EXAMPLE -=-=-
// =-=-=-=-=-=-=-=-=-

// demonstrates trigger configuration and WaitForNextleader use
// (1) sets trigger mode, source, and selector
// (2) starts stream
// (3) waits until trigger is armed
// (4) triggers image
// (5) wait for next leader
// (6) gets image
// (7) requeues buffer
// (8) stops stream
AC_ERROR ConfigureTriggerAndAcquireImage(acDevice hDevice)
{
	AC_ERROR err = AC_ERR_SUCCESS;

	// get node map and retrieve initial node values that will be changed in
	// order to return their values at the end of the example
	acNodeMap hNodeMap = NULL;
	char triggerModeInitial[MAX_BUF];
	size_t triggerModeBufLen = MAX_BUF;
	char triggerSourceInitial[MAX_BUF];
	size_t triggerSourceBufLen = MAX_BUF;
	char triggerSelectorInitial[MAX_BUF];
	size_t triggerSelectorBufLen = MAX_BUF;

	err = acDeviceGetNodeMap(hDevice, &hNodeMap);
	if (err != AC_ERR_SUCCESS)
		return err;

	err = acNodeMapGetEnumerationValue(hNodeMap, "TriggerSelector", triggerSelectorInitial, &triggerSelectorBufLen) |
		acNodeMapGetEnumerationValue(hNodeMap, "TriggerMode", triggerModeInitial, &triggerModeBufLen) |
		acNodeMapGetEnumerationValue(hNodeMap, "TriggerSource", triggerSourceInitial, &triggerSourceBufLen);
	if (err != AC_ERR_SUCCESS)
		printf("\nWarning: failed to retrieve one or more initial node values.\n");

	// set trigger selector
	printf("%sSet trigger selector to FrameStart\n", TAB1);

	err = acNodeMapSetEnumerationValue(hNodeMap, "TriggerSelector", "FrameStart");
	if (err != AC_ERR_SUCCESS)
		return err;

	// set trigger mode
	printf("%sEnable trigger mode\n", TAB1);

	err = acNodeMapSetEnumerationValue(hNodeMap, "TriggerMode", "On");
	if (err != AC_ERR_SUCCESS)
		return err;

	// set trigger source
	printf("%sSet trigger source to Software\n", TAB1);

	err = acNodeMapSetEnumerationValue(hNodeMap, "TriggerSource", "Software");
	if (err != AC_ERR_SUCCESS)
		return err;

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

	// start stream
	printf("%sStart stream\n\n", TAB1);
	err = acDeviceStartStream(hDevice);
	if (err != AC_ERR_SUCCESS)
		return err;

	for (uint32_t i = 1; i <= NUMBER_IMAGES_TO_CAPTURE; i++)
	{

		// trigger armed
		printf("%sWait until trigger is armed\n", TAB2);

		bool8_t triggerArmed = false;

		do
		{
			acNodeMapGetBooleanValue(hNodeMap, "TriggerArmed", &triggerArmed);
		} while (triggerArmed == false);

		// trigger an image
		printf("%sTrigger image %d\n", TAB2, i);

		// This waits for the next leader for each triggered image
		if (WAIT_FOR_LEADER_EVERY_3RD_IMAGE == false)
		{
			err = acNodeMapExecute(hNodeMap, "TriggerSoftware");
			if (err != AC_ERR_SUCCESS)
				return err;

			printf("%sWait for leader to arrive %d\n", TAB2, i);

			// Wait for next leader
			//    This will return when the leader for the next image arrives at
			//    the host if it arrives before the timeout. Otherwise, it will
			//    throw a timeout exception
			err = acDeviceWaitForNextLeader(hDevice, TIMEOUT_MILLISEC);
			if (err != AC_ERR_SUCCESS)
				return err;

			printf("%sLeader has arrived for image %d\n", TAB2, i);

		}
		// This waits for the leader of every 3rd image.
		//    Since Wait is not called for the other images, we call Reset to
		//    clear the current wait state before continuing. If we do not do a
		//    Reset, then the next Wait would return immediately for the last
		//    leader.
		else
		{
			if (i % 3 == 0)
			{
				printf("%sResetting WaitForNextLeader state\n", TAB2);

				err = acDeviceResetWaitForNextLeader(hDevice);
				if (err != AC_ERR_SUCCESS)
					return err;
			}

			err = acNodeMapExecute(hNodeMap, "TriggerSoftware");
			if (err != AC_ERR_SUCCESS)
				return err;

			if (i % 3 == 0)
			{
				printf("%sWait for leader to arrive\n", TAB2);

				err = acDeviceWaitForNextLeader(hDevice, TIMEOUT_MILLISEC);
				if (err != AC_ERR_SUCCESS)
					return err;

				printf("%sLeader has arrived for image %d\n", TAB2, i);
			}
		}

		// get image
		acBuffer hBuffer = NULL;

		err = acDeviceGetBuffer(hDevice, TIMEOUT_MILLISEC, &hBuffer);
		if (err != AC_ERR_SUCCESS)
			return err;

		// get image width and height
		size_t imageWidth = 0;
		size_t imageHeight = 0;

		err = acImageGetWidth(hBuffer, &imageWidth);
		if (err != AC_ERR_SUCCESS)
			return err;

		err = acImageGetHeight(hBuffer, &imageHeight);
		if (err != AC_ERR_SUCCESS)
			return err;

		printf("%sGet image (%zux%zu)\n", TAB2, imageWidth, imageHeight);

		// requeue image buffer
		printf("%sRequeue  buffer\n\n", TAB2);

		err = acDeviceRequeueBuffer(hDevice, hBuffer);
		if (err != AC_ERR_SUCCESS)
			return err;
	}

	// stop the stream
	printf("%sStop stream\n", TAB1);

	err = acDeviceStopStream(hDevice);
	if (err != AC_ERR_SUCCESS)
		return err;

	// return nodes to their initial values
	err = acNodeMapSetEnumerationValue(hNodeMap, "TriggerSource", triggerSourceInitial) |
		acNodeMapSetEnumerationValue(hNodeMap, "TriggerMode", triggerModeInitial) |
		acNodeMapSetEnumerationValue(hNodeMap, "TriggerSelector", triggerSelectorInitial);
	if (err != AC_ERR_SUCCESS)
		printf("\nWarning: failed to set one or more node values back to its initial value.\n");

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
	printf("C_Trigger_NextLeader\n");
	AC_ERROR err = AC_ERR_SUCCESS;

	// prepare example
	acSystem hSystem = NULL;
	err = acOpenSystem(&hSystem);
	CHECK_RETURN;
	err = acSystemUpdateDevices(hSystem, TIMEOUT_MILLISEC);
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
	err = ConfigureTriggerAndAcquireImage(hDevice);
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
