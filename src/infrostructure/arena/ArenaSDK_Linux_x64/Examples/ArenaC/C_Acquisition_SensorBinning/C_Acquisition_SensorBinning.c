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
#include <string.h>

#if (!defined _WIN32 && !defined _WIN64)
#define scanf_s scanf
#endif

#define TAB1 "  "
#define TAB2 "    "

// Acquisition: Sensor Binning
//    This example demonstrates how to configure device settings to enable
//    binning at the sensor level, so that the sensor will combine rectangles of
//    pixels into larger "bins". This reduces image  resolution, but
//    also reduces the amount of data sent to the software and networking
//    layers.

// =-=-=-=-=-=-=-=-=-
// =-=- SETTINGS =-=-
// =-=-=-=-=-=-=-=-=-

#define TIMEOUT 2000
#define SYSTEM_TIMEOUT 100
#define MAX_BUF 1024

#define BINTYPE "Sum"

// =-=-=-=-=-=-=-=-=-
// =-=- EXAMPLE -=-=-
// =-=-=-=-=-=-=-=-=-

// Activates sensor binning, then acquires images. 6 steps:
//    1) Configure network flow
//    2) Store initial values
//    3) Enable sensor binning
//    4) Maximize the size of bins
//    5) Acquire images
//    6) Restore initial values
AC_ERROR AcquireImages(acDevice hDevice)
{
	AC_ERROR err = AC_ERR_SUCCESS;
	bool8_t check;
	if (err != AC_ERR_SUCCESS)
		return err;

	// get stream node map
	acNodeMap hTLStreamNodeMap = NULL;

	err = acDeviceGetTLStreamNodeMap(hDevice, &hTLStreamNodeMap);
	if (err != AC_ERR_SUCCESS)
		return err;

	// enable stream auto negotiate packet size
	err = acNodeMapSetBooleanValue(hTLStreamNodeMap,
		"StreamAutoNegotiatePacketSize",
		true);

	if (err != AC_ERR_SUCCESS)
		return err;

	// enable stream packet resend
	err = acNodeMapSetBooleanValue(hTLStreamNodeMap,
		"StreamPacketResendEnable",
		true);

	if (err != AC_ERR_SUCCESS)
		return err;

	acNodeMap hNodeMap;
	err = acDeviceGetNodeMap(hDevice, &hNodeMap);
	if (err != AC_ERR_SUCCESS)
		return err;


	// store initial values so they can be restored when we finish
	int64_t initialBinningVertical;
	int64_t initialBinningHorizontal;

	char initialBinningSelector[MAX_BUF];
	size_t initialBinningSelectorBufSiz = MAX_BUF;

	char initialBinningVerticalMode[MAX_BUF];
	size_t initialBinningVerticalModeBufSiz = MAX_BUF;

	char initialBinningHorizontalMode[MAX_BUF];
	size_t initialBinningHorizontalModeBufSiz = MAX_BUF;

	// digital or sensor
	err = acNodeMapGetStringValue(hNodeMap, "BinningSelector", initialBinningSelector, &initialBinningSelectorBufSiz);
	if (err != AC_ERR_SUCCESS)
		return err;

	// some integer: height and width of bins
	err = acNodeMapGetIntegerValue(hNodeMap, "BinningVertical", &initialBinningVertical);
	if (err != AC_ERR_SUCCESS)
		return err;

	err = acNodeMapGetIntegerValue(hNodeMap, "BinningHorizontal", &initialBinningHorizontal);
	if (err != AC_ERR_SUCCESS)
		return err;

	// sum or average
	err = acNodeMapGetStringValue(hNodeMap, "BinningVerticalMode", initialBinningVerticalMode, &initialBinningVerticalModeBufSiz);
	if (err != AC_ERR_SUCCESS)
		return err;

	err = acNodeMapGetStringValue(hNodeMap, "BinningHorizontalMode", initialBinningHorizontalMode, &initialBinningHorizontalModeBufSiz);
	if (err != AC_ERR_SUCCESS)
		return err;

	acNode hNodeBinningSelector;
	acNode hBinningVerticalNode;
	acNode hBinningHorizontalNode;

	err = acNodeMapGetNode(hNodeMap, "BinningSelector", &hNodeBinningSelector);
	if (err != AC_ERR_SUCCESS)
		return err;

	err = acNodeMapGetNode(hNodeMap, "BinningHorizontal", &hBinningHorizontalNode);
	if (err != AC_ERR_SUCCESS)
		return err;

	err = acNodeMapGetNode(hNodeMap, "BinningVertical", &hBinningVerticalNode);
	if (err != AC_ERR_SUCCESS)
		return err;

	printf("%sSet BinningSelector to Sensor\n", TAB1);
	// sets BinningSelector to Sensor
	err = acNodeMapSetStringValue(hNodeMap,
		"BinningSelector",
		"Sensor");

	if (err != AC_ERR_SUCCESS)
		return err;

	printf("%sCheck configuration nodes for sensor binning\n", TAB1);
	// Check if parameter nodes (BinningVertical, BinningHorizontal) are available
	//    Secondary check if sensor binning is supported or not. In this case, we
	//    can set BinningSelector to Sensor, but the parameters are locked to 1.
	//    Sensor binning would then be unsupported. This case was probably just a
	//    bug in the firmware.
	err = acIsWritable(hBinningVerticalNode, &check);
	if (err != AC_ERR_SUCCESS)
		return err;
	if (!check) {
		printf("%sSensor binning is not supported: BinningVertical not writable. \n", TAB1);
		return 0;
	}

	err = acIsWritable(hBinningHorizontalNode, &check);
	if (err != AC_ERR_SUCCESS)
		return err;
	if (!check) {
		printf("%sSensor binning is not supported: BinningHorizontal not writable. \n", TAB1);
		return 0;
	}

	int64_t maxBinningVertical;
	int64_t maxBinningHorizontal;

	// Find max values for BinningVertical, BinningHorizontal.
	//    We then set BinningVertical and BinningHorizontal to their maximum
	//    values. This maximizes the size of the bins, and reduces the data by
	//    the maximum amount.
	printf("%sFind max binning values\n", TAB1);

	acIntegerGetMax(hBinningVerticalNode, &maxBinningVertical);
	acIntegerGetMax(hBinningHorizontalNode, &maxBinningHorizontal);

	printf("%sSet vertical and horizontal binning to %ld and %ld respectively\n", TAB1, (long)maxBinningVertical, (long)maxBinningHorizontal);
	err = acNodeMapSetIntegerValue(hNodeMap,
		"BinningVertical",
		maxBinningVertical);

	if (err != AC_ERR_SUCCESS)
		return err;

	err = acNodeMapSetIntegerValue(hNodeMap,
		"BinningHorizontal",
		maxBinningHorizontal);

	if (err != AC_ERR_SUCCESS)
		return err;

	printf("%sSet binning mode to sum\n", TAB1);
	err = acNodeMapSetStringValue(hNodeMap,
		"BinningVerticalMode",
		BINTYPE);

	if (err != AC_ERR_SUCCESS)
		return err;

	err = acNodeMapSetStringValue(hNodeMap,
		"BinningHorizontalMode",
		BINTYPE);

	if (err != AC_ERR_SUCCESS)
		return err;

	// start stream

	err = acDeviceStartStream(hDevice);
	if (err != AC_ERR_SUCCESS)
		return err;

	// get image
	printf("%sAcquire images\n", TAB1);

	acBuffer hBuffer = NULL;

	err = acDeviceGetBuffer(hDevice, TIMEOUT, &hBuffer);
	if (err != AC_ERR_SUCCESS)
		return err;


	// requeue image buffer

	err = acDeviceRequeueBuffer(hDevice, hBuffer);
	if (err != AC_ERR_SUCCESS)
		return err;

	// stop stream
	err = acDeviceStopStream(hDevice);
	if (err != AC_ERR_SUCCESS)
		return err;

	// restore original values

	err = acNodeMapSetStringValue(hNodeMap,
		"BinningSelector",
		initialBinningSelector);

	if (err != AC_ERR_SUCCESS)
		return err;

	err = acNodeMapSetStringValue(hNodeMap,
		"BinningVerticalMode",
		initialBinningVerticalMode);

	if (err != AC_ERR_SUCCESS)
		return err;

	err = acNodeMapSetStringValue(hNodeMap,
		"BinningHorizontalMode",
		initialBinningHorizontalMode);

	if (err != AC_ERR_SUCCESS)
		return err;

	err = acNodeMapSetIntegerValue(hNodeMap,
		"BinningVertical",
		initialBinningVertical);

	if (err != AC_ERR_SUCCESS)
		return err;

	err = acNodeMapSetIntegerValue(hNodeMap,
		"BinningHorizontal",
		initialBinningHorizontal);

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
	printf("C_Acquisition_SensorBinning\n");
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

		// Initial check if sensor binning is supported or not.
		//    If entry is non-null, then it is probably supported. There is a
		//    secondary check that must occur in the example.
		acNodeMap hNodeMap;
		acNode hNodeBinningSelector;
		acNode entry;
		bool check;

		err = acDeviceGetNodeMap(hDevice, &hNodeMap);
		CHECK_RETURN;

		err = acNodeMapGetNode(hNodeMap, "BinningSelector", &hNodeBinningSelector);
		CHECK_RETURN;

		err = acEnumerationGetEntryByName(hNodeBinningSelector, "Sensor", &entry);
		CHECK_RETURN;

		if (entry == 0) {
			printf("\nError. Sensor binning is not supported: not available under BinningSelector\n");
			printf("\n\nPress enter to complete\n");
			getchar();
			return -1;
		}
		err = acIsReadable(entry, &check);
		CHECK_RETURN;

		if (!check) {
			printf("\nError. Sensor binning is not supported: not available under BinningSelector\n");
			printf("\n\nPress enter to complete\n");
			getchar();
			return -1;
		}



		// run example
		printf("Commence example\n\n");
		err = AcquireImages(hDevice);
		CHECK_RETURN;

		// clean up example
		printf("%sClean Up Arena\n", TAB1);
		err = acSystemDestroyDevice(hSystem, hDevice);
		CHECK_RETURN;
		err = acCloseSystem(hSystem);
		CHECK_RETURN;

		printf("\nExample complete\n");
	}

	printf("Press enter to complete\n");
	while (getchar() != '\n') {};
	getchar();
	return 0;
}