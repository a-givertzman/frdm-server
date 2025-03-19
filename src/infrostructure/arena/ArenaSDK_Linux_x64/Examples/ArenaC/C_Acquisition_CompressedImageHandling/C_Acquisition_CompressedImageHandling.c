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
#include <inttypes.h> // defines macros for printf functions
#include <stdbool.h>  // defines boolean type and values
#include <string.h>

#if (!defined _WIN32 && !defined _WIN64)
#define scanf_s scanf
#endif

#define TAB1 "  "
#define TAB2 "    "

// Acquisition: Compressed Image Handling
//    This example demonstrates how to acquire and process compressed image data
//    from the camera using the Arena SDK. The example includes steps to configure the
//    camera, acquire a compressed image, process the image to decompress it, and save 
//    both the raw input and processed images.

// =-=-=-=-=-=-=-=-=-
// =-=- SETTINGS =-=-
// =-=-=-=-=-=-=-=-=-

// raw file name
#define RAW_FILE_NAME "Images/C_Acquisition_CompressedImageHandling/CompressedImage.raw"

// png file name
#define PNG_FILE_NAME "Images/C_Acquisition_CompressedImageHandling/DecompressedImage.png"

// timeout for getting image buffer
#define IMAGE_TIMEOUT 2000

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

// helper function to save file of type ".raw" to disk
AC_ERROR saveRAWInputImage(acBuffer hBuffer, const char* filename)
{
	// AC_ERROR and SC_ERROR values are equivalent
	AC_ERROR acErr = AC_ERR_SUCCESS;
	SC_ERROR saveErr = SC_ERR_SUCCESS;

	// get size
	size_t size = 0;
	acErr = acBufferGetSizeFilled(hBuffer, &size);
	if (acErr != AC_ERR_SUCCESS)
		return acErr;

	// save compressed image
	printf("%sSaving image to %s\n", TAB2, filename);

	// get compressed image
	uint8_t* pData = NULL;
	acErr = acCompressedImageGetData(hBuffer, &pData);
	if (acErr != AC_ERR_SUCCESS)
		return acErr;

	// save raw compressed image
	saveErr = saveWriterSaveRawData(filename, pData, size);
	if (saveErr != SC_ERR_SUCCESS)
		return saveErr;

	return acErr;
}

// helper function to decompress image and save as ".png" to disk
AC_ERROR processAndSaveDecompressedImage(acBuffer hBuffer, const char* filename)
{
	// AC_ERROR and SC_ERROR values are equivalent
	AC_ERROR acErr = AC_ERR_SUCCESS;
	SC_ERROR saveErr = SC_ERR_SUCCESS;

	// decompress image to Mono8
	printf("%sDecompress image to %s\n", TAB2, "Mono8");
	acBuffer hDecompressed = NULL;
	acErr = acImageFactoryDecompressImage(hBuffer, &hDecompressed);
	if (acErr != AC_ERR_SUCCESS)
		return acErr;

	// prepare image parameters
	// get width
	size_t width = 0;
	acErr = acImageGetWidth(hDecompressed, &width);
	if (acErr != AC_ERR_SUCCESS)
		return acErr;

	// get height
	size_t height = 0;
	acErr = acImageGetHeight(hDecompressed, &height);
	if (acErr != AC_ERR_SUCCESS)
		return acErr;

	// get bits per pixel
	size_t bpp = 0;
	acErr = acImageGetBitsPerPixel(hDecompressed, &bpp);
	if (acErr != AC_ERR_SUCCESS)
		return acErr;

	// get compresed image size
	size_t sizeFilled = 0;
	acErr = acBufferGetSizeFilled(hDecompressed, &sizeFilled);
	if (acErr != AC_ERR_SUCCESS)
		return acErr;

	printf("%sMono8 decompressed image size: %zu bytes\n", TAB2, sizeFilled);

	// prepare image writer
	saveWriter hWriter = NULL;
	saveErr = saveWriterCreate(width, height, bpp, &hWriter);
	if (saveErr != SC_ERR_SUCCESS)
		return saveErr;
	saveErr = saveWriterSetFileNamePattern(hWriter, filename);
	if (saveErr != SC_ERR_SUCCESS)
		return saveErr;

	// save image
	printf("%sSave image to %s\n", TAB2, filename);

	// get image
	uint8_t* pData = NULL;
	acErr = acImageGetData(hDecompressed, &pData);
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

	// destroy converted image
	acErr = acImageFactoryDestroy(hDecompressed);
	if (acErr != AC_ERR_SUCCESS)
		return acErr;

	return SC_ERR_SUCCESS;
}

// Demonstrates acquisition and processing of compressed image data.
// (1) Configures the camera to use a compressed pixel format
// (2) Acquires a compressed input image
// (3) Processes and saves the raw input image to decompress it
// (4) Save the processed image
AC_ERROR AcquireAndProcessCompressedImage(acDevice hDevice)
{
	AC_ERROR err = AC_ERR_SUCCESS;

	// get node map
	acNodeMap hNodeMap = NULL;

	err = acDeviceGetNodeMap(hDevice, &hNodeMap);
	if (err != AC_ERR_SUCCESS)
		return err;

	// get pixel format node
	acNode hNode = NULL;
	AC_ACCESS_MODE accessMode = 0;

	err = acNodeMapGetNodeAndAccessMode(hNodeMap, "PixelFormat", &hNode, &accessMode);
	if (err != AC_ERR_SUCCESS)
		return err;

	// check access mode
	if (accessMode != AC_ACCESS_MODE_RO && accessMode != AC_ACCESS_MODE_RW)
		return AC_ERR_ERROR;

	// retrieve entires and check for "QOI_Mono8"
	size_t numEntries = 0; // number of entries in pixel format

	err = acEnumerationGetNumEntries(hNode, &numEntries);
	if (err != AC_ERR_SUCCESS)
		return err;

	bool found = false;
	for (size_t index = 0; index < numEntries; index++)
	{
		// access entry by index
		char pSymbolicBuf[MAX_BUF];
		size_t symbolicBufLen = MAX_BUF;

		err = acEnumerationGetSymbolicByIndex(hNode, index, pSymbolicBuf, &symbolicBufLen);
		if (err != AC_ERR_SUCCESS)
			return err;

		if (strstr(pSymbolicBuf, "QOI_Mono8") != NULL) 
		{
			// current camera supports QOI_Mono8
			found = true;
			break;
		}
	}

	if (!found)
	{
		printf("%sQOI_Mono8 is not available in the PixelFormat enumeration for this camera.\n", TAB1);
		return AC_ERR_ERROR;
	}

	// get node values that will be changed in order to return their values at
	// the end of the example
	char pPixelFormatInitial[MAX_BUF];
	size_t len = MAX_BUF;

	err = GetNodeValue(hNodeMap, "PixelFormat", pPixelFormatInitial, &len);
	if (err != AC_ERR_SUCCESS)
		return err;

	// set pixel format to QOI_Mono8
	printf("%sSet pixel format to 'QOI_Mono8'\n", TAB1);

	err = SetNodeValue(
		hNodeMap,
		"PixelFormat",
		"QOI_Mono8");

	if (err != AC_ERR_SUCCESS)
		return err;

	// get stream node map
	acNodeMap hTLStreamNodeMap = NULL;

	err = acDeviceGetTLStreamNodeMap(hDevice, &hTLStreamNodeMap);
	if (err != AC_ERR_SUCCESS)
		return err;

	// enable stream auto negotiate packet size
	printf("%sEnable stream to auto negotiate packet size\n", TAB1);

	err = acNodeMapSetBooleanValue(hTLStreamNodeMap, "StreamAutoNegotiatePacketSize", true);
	if (err != AC_ERR_SUCCESS)
		return err;

	// enable stream packet resend
	printf("%sEnable stream packet resend\n", TAB1);

	err = acNodeMapSetBooleanValue(hTLStreamNodeMap, "StreamPacketResendEnable", true);
	if (err != AC_ERR_SUCCESS)
		return err;

	// start stream
	printf("%sStart stream\n", TAB1);

	err = acDeviceStartStream(hDevice);
	if (err != AC_ERR_SUCCESS)
		return err;

	// gets one image
	printf("%sGet compressed image\n", TAB2);
	acBuffer hBuffer = NULL;

	err = acDeviceGetBuffer(hDevice, IMAGE_TIMEOUT, &hBuffer);
	if (err != AC_ERR_SUCCESS)
		return err;

	// get compresed image size
	size_t sizeFilled = 0;

	err = acBufferGetSizeFilled(hBuffer, &sizeFilled);
	if (err != AC_ERR_SUCCESS)
		return err;

	printf("%sQOI_Mono8 compressed image size: %zu bytes\n", TAB2, sizeFilled);

	// saves RAW input image

	saveRAWInputImage(hBuffer, RAW_FILE_NAME);

	// decompress image and saves decompressed image

	processAndSaveDecompressedImage(hBuffer, PNG_FILE_NAME);

	// requeue image buffer
	printf("%sRequeue image buffer\n", TAB2);

	err = acDeviceRequeueBuffer(hDevice, hBuffer);
	if (err != AC_ERR_SUCCESS)
		return err;

	// stop stream
	printf("%sStop stream\n", TAB1);

	err = acDeviceStopStream(hDevice);
	if (err != AC_ERR_SUCCESS)
		return err;

	// return nodes to their initial values
	err = SetNodeValue(
		hNodeMap,
		"PixelFormat",
		pPixelFormatInitial);

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
	printf("C_Acquisition_CompressedImageHandling\n");
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
	err = AcquireAndProcessCompressedImage(hDevice);
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
