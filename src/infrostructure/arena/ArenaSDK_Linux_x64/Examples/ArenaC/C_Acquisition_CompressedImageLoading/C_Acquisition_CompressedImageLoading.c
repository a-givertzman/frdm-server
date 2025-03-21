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

#include <stdlib.h>
#include <inttypes.h> // defines macros for printf functions
#include <stdbool.h>  // defines boolean type and values
#include <time.h>	  // for timer
#include <string.h>

#if (!defined _WIN32 && !defined _WIN64)
#define scanf_s scanf
#include <sys/types.h>
#include <sys/stat.h> 
#endif

#define TAB1 "  "
#define TAB2 "    "

// Acquisition: Compressed Image Loading
//		This example demonstrates how to handle compressed image data, specifically
//		loading and processing from raw data files using the Arena SDK. The example
//		includes steps to configure the camera, acquire a compressed image, save
//		the raw file, load the raw file, decompress the data, and save the decompressed
//		image.

// =-=-=-=-=-=-=-=-=-
// =-=- SETTINGS =-=-
// =-=-=-=-=-=-=-=-=-

// raw file name
#define RAW_FILE_NAME "Images/C_Acquisition_CompressedImageLoading/CompressedImage"

// png file name
#define PNG_FILE_NAME "Images/C_Acquisition_CompressedImageLoading/DecompressedImage"

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
AC_ERROR GetStringNodeValue(acNodeMap hNodeMap, const char* nodeName, char* pValue, size_t* pLen)
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

// integer version of get node helper function
AC_ERROR GetIntegerNodeValue(acNodeMap hNodeMap, const char* nodeName, size_t* value)
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
	err = acIntegerGetValue(hNode, (int64_t*)value);
	return err;
}

// sets node value
// (1) gets node
// (2) checks access mode
// (3) gets value
AC_ERROR SetStringNodeValue(acNodeMap hNodeMap, const char* nodeName, const char* pValue)
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

// reads file size
AC_ERROR ReadFileSize(const char* file, size_t* size)
{
	struct stat stat_buf;
	int rc = stat(file, &stat_buf);
	if (rc != 0)
		return AC_ERR_ERROR;
	*size = stat_buf.st_size;
	return AC_ERR_SUCCESS;
}


// =-=-=-=-=-=-=-=-=-
// =-=- EXAMPLE -=-=-
// =-=-=-=-=-=-=-=-=-


AC_ERROR saveRAWInputImage(acBuffer hBuffer, const char* filename, int index)
{
	// AC_ERROR and SC_ERROR values are equivalent
	AC_ERROR acErr = AC_ERR_SUCCESS;
	SC_ERROR saveErr = SC_ERR_SUCCESS;

	printf("%sSave compressed image data to ", TAB2);

	// get size of compressed image
	size_t size = 0;
	acErr = acBufferGetSizeFilled(hBuffer, &size);
	if (acErr != AC_ERR_SUCCESS)
		return acErr;

	// get compressed image data
	uint8_t* compressedImageData = NULL;
	acErr = acCompressedImageGetData(hBuffer, &compressedImageData);
	if (acErr != AC_ERR_SUCCESS)
		return acErr;

	// allocate temporary buffer
	uint8_t* pData = (uint8_t*)malloc(size);

	// copy data to temporary buffer
	memcpy(pData, compressedImageData, size);

	// filename
	char file[MAX_BUF];
	snprintf(file, MAX_BUF, "%s%d%s", filename, index, ".raw");

	// save raw data to disk
	// if function doesn't work, make sure the directories are present
	saveErr = saveWriterSaveRawData(file, pData, size);
	if (saveErr != SC_ERR_SUCCESS)
		return saveErr;

	printf("%s\n", file);

	// clean up
	free(pData);

	return acErr;
}

// Demonstrates acquisition and saving of compressed image data.
// (1) Configures the camera to use a compressed pixel format
// (2) Acquires a compressed input image
// (3) Saves the raw input image
AC_ERROR AcquireAndSaveRawImage(acDevice hDevice)
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

	err = GetStringNodeValue(hNodeMap, "PixelFormat", pPixelFormatInitial, &len);
	if (err != AC_ERR_SUCCESS)
		return err;

	// set pixel format
	printf("%sSet pixel format to 'QOI_Mono8'\n", TAB1);

	err = SetStringNodeValue(
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

	// gets 10 compressed images
	printf("%sGet 10 compressed images\n", TAB1);
	acBuffer hBuffer = NULL;

	for (int i = 0; i < 10; i++)
	{
		// get one compressed image
		printf("%sGet compressed image %d\n", TAB2, i);
		err = acDeviceGetBuffer(hDevice, IMAGE_TIMEOUT, &hBuffer);
		if (err != AC_ERR_SUCCESS)
			return err;

		// get compresed image size
		size_t sizeFilled = 0;

		err = acBufferGetSizeFilled(hBuffer, &sizeFilled);
		if (err != AC_ERR_SUCCESS)
			return err;

		// print out size for comparison
		printf("%sCompressed image %d size: %zu bytes\n", TAB2, i, sizeFilled);

		// saves RAW input image
		err = saveRAWInputImage(hBuffer, RAW_FILE_NAME, i);
		if (err != AC_ERR_SUCCESS)
			return err;

		// requeue image buffer
		printf("%sRequeue image buffer\n", TAB2);

		err = acDeviceRequeueBuffer(hDevice, hBuffer);
		if (err != AC_ERR_SUCCESS)
			return err;
	}

	// stop stream
	printf("%sStop stream\n", TAB1);

	err = acDeviceStopStream(hDevice);
	if (err != AC_ERR_SUCCESS)
		return err;

	// return nodes to their initial values
	err = SetStringNodeValue(
		hNodeMap,
		"PixelFormat",
		pPixelFormatInitial);

	return err;
}

// Demonstrates loading and processing of compressed image data.
// (1) Loads raw image
// (2) Decompresses raw image
// (3) Saves decompressed image into readable format
AC_ERROR LoadAndProcessRawImage(const char* inFile, const char* outFile)
{
	AC_ERROR err = AC_ERR_SUCCESS;
	SC_ERROR saveErr = SC_ERR_SUCCESS;

	printf("\n%sLoad and process 10 images\n", TAB1);

	clock_t begin = clock();

	for (int i = 0; i < 10; i++)
	{
		// in filename
		char inFilename[MAX_BUF];
		snprintf(inFilename, MAX_BUF, "%s%d%s", inFile, i, ".raw");

		// read size of file
		size_t size;
		err = ReadFileSize(inFilename, &size);
		if (err != AC_ERR_SUCCESS)
			return err;

		// set source pixel format
		uint64_t srcPF = LUCID_QOI_Mono8;

		// allocate memory to load raw data
		uint8_t* pIn = (uint8_t*)malloc(size);
		memset(pIn, 0, size);

		// load raw data
		saveErr = saveReaderLoadRawData(inFilename, pIn, size);
		if (saveErr != AC_ERR_SUCCESS)
			return saveErr;

		// convert raw data into image
		acBuffer pCompressedImage = NULL;
		err = acImageFactoryCreateCompressedImage(pIn, size, srcPF, &pCompressedImage);
		if (err != AC_ERR_SUCCESS)
			return err;

		acBuffer pDecompressedImage = NULL;
		err = acImageFactoryDecompressImage(pCompressedImage, &pDecompressedImage);
		if (err != AC_ERR_SUCCESS)
			return err;

		// get compresed image size
		size_t decompressedSizeFilled = 0;

		err = acBufferGetSizeFilled(pDecompressedImage, &decompressedSizeFilled);
		if (err != AC_ERR_SUCCESS)
			return err;

		// print out size for comparison
		printf("%sDecompressed image %d size: %zu bytes\n", TAB2, i, decompressedSizeFilled);

		// saved the processed image
		printf("%sSave decompressed Mono8 image to ", TAB2);

		// prepare image parameters
		size_t width = 0;
		size_t height = 0;
		size_t bpp = 0;
		err = acImageGetWidth(pDecompressedImage, &width) |
			  acImageGetHeight(pDecompressedImage, &height) |
			  acImageGetBitsPerPixel(pDecompressedImage, &bpp);
		if (err != AC_ERR_SUCCESS)
			return err;

		// prepare image writer
		saveWriter hWriter = NULL;
		saveErr = saveWriterCreate(width, height, bpp, &hWriter);
		if (saveErr != SC_ERR_SUCCESS)
			return saveErr;

		// out filename
		char outFilename[MAX_BUF];
		snprintf(outFilename, MAX_BUF, "%s%d%s", outFile, i, ".png");

		saveErr = saveWriterSetFileNamePattern(hWriter, outFilename);
		if (saveErr != SC_ERR_SUCCESS)
			return saveErr;

		// get image data
		uint8_t* pData = NULL;
		err = acImageGetData(pDecompressedImage, &pData);
		if (err != AC_ERR_SUCCESS)
			return err;

		// save image
		saveErr = saveWriterSave(hWriter, pData);
		if (saveErr != SC_ERR_SUCCESS)
			return saveErr;

		printf("%s\n", outFilename);

		// destroy image writer
		saveErr = saveWriterDestroy(hWriter);
		if (saveErr != SC_ERR_SUCCESS)
			return saveErr;

		// clean up
		free(pIn);

		// destroy decompressed image and compressed image to prevent memory loss
		err = acImageFactoryDestroy(pDecompressedImage);
		if (err != AC_ERR_SUCCESS)
			return err;

		err = acImageFactoryDestroyCompressedImage(pCompressedImage);
		if (err != AC_ERR_SUCCESS)
			return err;
	}

	clock_t end = clock();
	double time_spent = (double)(end - begin) / CLOCKS_PER_SEC;
	printf("%sTime to decompress 10 images (sec) =  %f", TAB1, time_spent);

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
	printf("C_Acquisition_CompressedImageLoading\n");
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
	err = AcquireAndSaveRawImage(hDevice);
	CHECK_RETURN;
	err = LoadAndProcessRawImage(RAW_FILE_NAME, PNG_FILE_NAME);
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
