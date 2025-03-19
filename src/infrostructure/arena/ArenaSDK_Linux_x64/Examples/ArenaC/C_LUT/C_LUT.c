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
#include "SaveCApi.h"
#include <stdbool.h> // defines boolean type and values
#include <string.h>	 // defines strcmp functions

#if (!defined _WIN32 && !defined _WIN64)
#define scanf_s scanf
#endif

#define TAB1 "  "
#define TAB2 "    "

// Lookup Tables: Introduction
//    This example introduces lookup tables (LUT), which are used to
//    transform image data into a desired output format. LUTs give an output
//    value for each of a range of index values. This example enables a lookup
//    table node to invert the intensity of a single image. This is done by
//    accessing the LUT index node and setting the LUT node values to the newly
//    calculated pixel intensity value. It takes some time to update each pixel
//    with the new value. The example then saves the new image by saving to the
//    image writer.

// =-=-=-=-=-=-=-=-=-
// =-=- SETTINGS =-=-
// =-=-=-=-=-=-=-=-=-

// slope for calculating the new intensity value to set
#define SLOPE -1

// file name
#define FILE_NAME "Images/C_LUT/image.png"

// timeout for detecting camera devices (in milliseconds).
#define SYSTEM_TIMEOUT 100

// image timeout
#define IMAGE_TIMEOUT 2000

// maximum buffer length
#define MAX_BUF 1024

// =-=-=-=-=-=-=-=-=-
// =-=- EXAMPLE -=-=-
// =-=-=-=-=-=-=-=-=-

// demonstrates using the lookup table to invert intensity
// (1) enables lookup table
// (2) substitutes each pixel with its inverted value
// (3) retrieves image
// (4) cleans up image
AC_ERROR InvertIntensity(acDevice hDevice)
{
	// AC_ERROR and SC_ERROR values are equivalent
	AC_ERROR acErr = AC_ERR_SUCCESS;
	SC_ERROR saveErr = SC_ERR_SUCCESS;

	// get node map
	acNodeMap hNodeMap = NULL;

	acErr = acDeviceGetNodeMap(hDevice, &hNodeMap);
	if (acErr != AC_ERR_SUCCESS)
		return acErr;

	// get initial value and enable lookup table
	printf("%sEnable lookup table\n", TAB1);
	acNode hLUTEnableNode = NULL;
	bool8_t lutEnableInitial = false;

	acErr = acNodeMapGetNode(hNodeMap, "LUTEnable", &hLUTEnableNode);
	if (acErr != AC_ERR_SUCCESS)
		return acErr;

	acErr = acBooleanGetValue(hLUTEnableNode, &lutEnableInitial);
	if (acErr != AC_ERR_SUCCESS)
		return acErr;

	acErr = acBooleanSetValue(hLUTEnableNode, true);
	if (acErr != AC_ERR_SUCCESS)
		return acErr;

	// Invert values
	printf("%sInvert values\n", TAB1);

	// get nodes for LUT index and value
	acNode hLUTIndexNode = NULL;
	acNode hLUTValueNode = NULL;

	acErr = acNodeMapGetNode(hNodeMap, "LUTIndex", &hLUTIndexNode);
	if (acErr != AC_ERR_SUCCESS)
		return acErr;

	acErr = acNodeMapGetNode(hNodeMap, "LUTValue", &hLUTValueNode);
	if (acErr != AC_ERR_SUCCESS)
		return acErr;

	// get maximum LUT index
	int64_t maximumLUTIndex = 0;

	acErr = acIntegerGetMax(hLUTIndexNode, &maximumLUTIndex);
	if (acErr != AC_ERR_SUCCESS)
		return acErr;

	for (int64_t index = 0; index <= maximumLUTIndex; index++)
	{
		// select pixel value
		acErr = acIntegerSetValue(hLUTIndexNode, index);
		if (acErr != AC_ERR_SUCCESS)
			return acErr;

		// set substitution value
		int64_t value = (SLOPE * index) + maximumLUTIndex;

		acErr = acIntegerSetValue(hLUTValueNode, value);
		if (acErr != AC_ERR_SUCCESS)
			return acErr;
		if (index % 1024 == 0)
			printf("%s", TAB2);
		if (index % 256 == 255)
			printf(".");
		if (index % 1024 == 1023)
			printf("\n");
	}

	// get stream node map
	acNodeMap hTLStreamNodeMap = NULL;

	acErr = acDeviceGetTLStreamNodeMap(hDevice, &hTLStreamNodeMap);
	if (acErr != AC_ERR_SUCCESS)
		return acErr;

	// enable stream auto negotiate packet size
	acErr = acNodeMapSetBooleanValue(hTLStreamNodeMap, "StreamAutoNegotiatePacketSize", true);
	if (acErr != AC_ERR_SUCCESS)
		return acErr;

	// enable stream packet resend
	acErr = acNodeMapSetBooleanValue(hTLStreamNodeMap, "StreamPacketResendEnable", true);
	if (acErr != AC_ERR_SUCCESS)
		return acErr;

	// start stream
	acErr = acDeviceStartStream(hDevice);
	if (acErr != AC_ERR_SUCCESS)
		return acErr;

	// get image
	acBuffer hBuffer = NULL;

	acErr = acDeviceGetBuffer(hDevice, IMAGE_TIMEOUT, &hBuffer);
	if (acErr != AC_ERR_SUCCESS)
		return acErr;

	// get width, height, and bits per pixel needed for image writer
	size_t width = 0;
	size_t height = 0;
	size_t bpp = 0;

	acImageGetWidth(hBuffer, &width);
	if (acErr != AC_ERR_SUCCESS)
		return acErr;

	acImageGetHeight(hBuffer, &height);
	if (acErr != AC_ERR_SUCCESS)
		return acErr;

	acErr = acImageGetBitsPerPixel(hBuffer, &bpp);
	if (acErr != AC_ERR_SUCCESS)
		return acErr;

	// prepare image writer
	saveWriter hWriter = NULL;

	saveErr = saveWriterCreate(width, height, bpp, &hWriter);
	if (saveErr != SC_ERR_SUCCESS)
		return saveErr;

	saveErr = saveWriterSetFileNamePattern(hWriter, FILE_NAME);
	if (saveErr != SC_ERR_SUCCESS)
		return saveErr;

	// get image
	uint8_t* pData = NULL;

	acErr = acImageGetData(hBuffer, &pData);
	if (acErr != AC_ERR_SUCCESS)
		return acErr;

	// save image
	saveErr = saveWriterSave(hWriter, pData);
	if (saveErr != SC_ERR_SUCCESS)
		return saveErr;

	// destroy image writer
	saveErr = saveWriterDestroy(hWriter);
	if (saveErr != SC_ERR_SUCCESS)
		return saveErr;

	// requeue buffer
	acErr = acDeviceRequeueBuffer(hDevice, hBuffer);
	if (acErr != AC_ERR_SUCCESS)
		return acErr;

	// stop stream
	acErr = acDeviceStopStream(hDevice);
	if (acErr != AC_ERR_SUCCESS)
		return acErr;

	// return nodes to their initial values
	acErr = acBooleanSetValue(hLUTEnableNode, lutEnableInitial);
	if (acErr != AC_ERR_SUCCESS)
		return acErr;

	return acErr;
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
	printf("C_LUT\n");
	AC_ERROR err = AC_ERR_SUCCESS;

	// user prompt for possible device settings overwrite
	printf("Example may change device settings -- proceed?  ('y' to continue) ");
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
		err = InvertIntensity(hDevice);
		CHECK_RETURN;
		printf("\nExample complete\n");

		// clean up example
		err = acSystemDestroyDevice(hSystem, hDevice);
		CHECK_RETURN;
		err = acCloseSystem(hSystem);
		CHECK_RETURN;
	}

	printf("Press enter to complete\n");
	while (getchar() != '\n') {};
	getchar();
	return -1;
}
