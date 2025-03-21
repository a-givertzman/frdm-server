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

// Exposure: Introduction
//    This example introduces the exposure feature. An image's exposure time
//    refers to the amount of time that a device's sensor is exposed to a scene
//    before the data is collected. The exposure can be handled automatically or
//    manually.

// =-=-=-=-=-=-=-=-=-
// =-=- SETTINGS =-=-
// =-=-=-=-=-=-=-=-=-

// Exposure time
//    Time that the sensor will be exposed when grabbing an image (in
//    microseconds). Exposure time depends on the application, but generally the
//    less available the light, the higher the exposure time.
#define EXPOSURE_TIME 4000.0

// Image timeout
//    Timeout for grabbing images (in milliseconds). Make the timeout longer 
//    than the exposure time to avoid timeout exceptions.
#define IMAGE_TIMEOUT 2000

// system timeout
#define SYSTEM_TIMEOUT 100

// number of images to grab
#define NUM_IMAGES 25

// maximum buffer length
#define MAX_BUF 1024

// =-=-=-=-=-=-=-=-=-
// =-=- EXAMPLE -=-=-
// =-=-=-=-=-=-=-=-=-

// demonstrates basic exposure configuration
// (1) disables automatic exposure
// (2) gets exposure node
// (3) ensures exposure above min/below max
// (4) sets exposure
// (5) starts stream
// (6) acquires images
// (7) stops stream and returns nodes to initial values
AC_ERROR ConfigureExposureAndAcquireImages(acDevice hDevice)
{
	AC_ERROR err = AC_ERR_SUCCESS;

	// get node map
	acNodeMap hNodeMap = NULL;

	err = acDeviceGetNodeMap(hDevice, &hNodeMap);
	if (err != AC_ERR_SUCCESS)
		return err;

	// Disable automatic exposure
	//    Disable automatic exposure before setting an exposure time. Automatic
	//    exposure controls whether the exposure time is set manually or
	//    automatically by the device. Setting automatic exposure to 'Off' stops
	//    the device from automatically updating the exposure time while
	//    streaming.
	printf("%sDisable automatic exposure\n", TAB1);
	char exposureAutoInitial[MAX_BUF];
	size_t exposureBufLen = MAX_BUF;

	err = acNodeMapGetEnumerationValue(hNodeMap, "ExposureAuto", exposureAutoInitial, &exposureBufLen);
	if (err != AC_ERR_SUCCESS)
		return err;

	err = acNodeMapSetEnumerationValue(hNodeMap, "ExposureAuto", "Off");
	if (err != AC_ERR_SUCCESS)
		return err;

	// Get exposure time node
	//    In order to get the exposure time maximum and minimum values, get the
	//    exposure time node. Because we expect to set its value, check that the
	//    exposure time node is writable.
	printf("%sGet exposure time node\n", TAB1);
	acNode hExposureTimeNode = NULL;

	err = acNodeMapGetNode(hNodeMap, "ExposureTime", &hExposureTimeNode);
	if (err != AC_ERR_SUCCESS)
		return err;

	// check if node is writable
	bool8_t isWritable = false;

	err = acIsWritable(hExposureTimeNode, &isWritable);
	if (err != AC_ERR_SUCCESS)
		return err;
	if (!isWritable)
	{
		printf("ExposureTime node not writable\n");
		return err;
	}

	// get initial node values that will be changed in order to return their
	// values at the end of the example
	double exposureTimeInitial = 0;

	err = acFloatGetValue(hExposureTimeNode, &exposureTimeInitial);
	if (err != AC_ERR_SUCCESS)
		return err;

	// retrieve the exposure time range for the node.
	double minimum = 0;
	double maximum = 0;

	err = acFloatGetMin(hExposureTimeNode, &minimum);
	if (err != AC_ERR_SUCCESS)
		return err;

	err = acFloatGetMax(hExposureTimeNode, &maximum);
	if (err != AC_ERR_SUCCESS)
		return err;

	// Set exposure time
	//    Before setting the exposure time, check that the new exposure time is not
	//    outside of the exposure time's acceptable range. If the value is above the
	//    maximum or below the minimum, update the value to be within range. Lastly,
	//    set new the new exposure time.
	double exposureTime = EXPOSURE_TIME;

	if (exposureTime < minimum)
	{
		exposureTime = minimum;
	}
	if (exposureTime > maximum)
	{
		exposureTime = maximum;
	}
	err = acFloatSetValue(hExposureTimeNode, exposureTime);
	if (err != AC_ERR_SUCCESS)
		return err;
	printf("%sSet exposure time to %.1f us\n", TAB1, exposureTime);

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
	err = acDeviceStartStream(hDevice);
	if (err != AC_ERR_SUCCESS)
		return err;

	// get images
	printf("%sGet %d images\n", TAB1, NUM_IMAGES);

	for (int i = 0; i < NUM_IMAGES; i++)
	{
		// Grab images with new exposure time
		//    When getting images, ensure the timeout is longer than the exposure
		//    time to avoid returning an error. The timeout is in milliseconds
		//    while the exposure time is in microseconds, so a timeout of 2000 is
		//    quite a bit longer than an exposure time of 4000.
		acBuffer hBuffer = NULL;
		uint64_t timestampNs = 0;

		err = acDeviceGetBuffer(hDevice, IMAGE_TIMEOUT, &hBuffer);
		if (err != AC_ERR_SUCCESS)
			return err;

		err = acImageGetTimestampNs(hBuffer, &timestampNs);
		if (err != AC_ERR_SUCCESS)
			return err;
		printf("%sImage %3i (timestamp %" PRIu64 " ns) \n", TAB2, i, timestampNs);

		// requeue image buffer
		err = acDeviceRequeueBuffer(hDevice, hBuffer);
		if (err != AC_ERR_SUCCESS)
			return err;
	}

	// stop stream
	err = acDeviceStopStream(hDevice);
	if (err != AC_ERR_SUCCESS)
		return err;

	// return nodes to their initial values
	err = acFloatSetValue(hExposureTimeNode, exposureTimeInitial);
	if (err != AC_ERR_SUCCESS)
		return err;

	err = acNodeMapSetEnumerationValue(hNodeMap, "ExposureAuto", exposureAutoInitial);
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
	printf("C_Exposure\n");
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
	err = ConfigureExposureAndAcquireImages(hDevice);
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
