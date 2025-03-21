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
#include <string.h>
#include <stdbool.h>

#if (!defined _WIN32 && !defined _WIN64)
#define scanf_s scanf
#endif

#define TAB1 "  "
#define TAB2 "    "

// Helios: Heat Map
//    This example demonstrates saving an RGB heatmap of a 3D image. It captures
//    a 3D image, interprets the ABCY data to retrieve the distance value for
//    each pixel and then converts this data into a BGR and an RGB buffer. The
//    BGR buffer is used to create a jpg heatmap image and the RGB buffer is used
//    to color the ply image.

// =-=-=-=-=-=-=-=-=-
// =-=- SETTINGS =-=-
// =-=-=-=-=-=-=-=-=-

// file name
#define PLY_FILE_NAME "Images/C_Helios_HeatMap.ply"
#define JPG_FILE_NAME "Images/C_Helios_HeatMap.jpg"

// pixel format
#define PIXEL_FORMAT PFNC_BGR8 // BGR8

// image timeout
#define IMAGE_TIMEOUT 2000

// device timeout
#define DEVICE_TIMEOUT 100

// maximum buffer length
#define MAX_BUF 1024

// =-=-=-=-=-=-=-=-=-
// =-=- EXAMPLE -=-=-
// =-=-=-=-=-=-=-=-=-


// demonstrates saving a bgr heatmap image
// (1) gets image
// (2) interprets ABCY data to get z coordinate
// (3) creates a buffer for BGR and RGB colorings using z data
// (4) creates jpg heatmap image using BGR buffer
// (5) colors ply image using RGB buffer
// (6) saves jpg and ply image
AC_ERROR AcquireImageAndInterpretData(acDevice hDevice)
{
	AC_ERROR err = AC_ERR_SUCCESS;

	acNodeMap hNodeMap = NULL;
	err = acDeviceGetNodeMap(hDevice, &hNodeMap);
	if (err != AC_ERR_SUCCESS)
			return err;	

	// Validate if the Scan3dCoordinateSelector node exists. If it doesn't exist, the camera being
	// used to run the example is likely not a Helios
	char checkpScan3dCoordinateSelector[MAX_BUF];
	size_t checkpScan3dCoordinateSelectorBufLen = MAX_BUF;
	err = acNodeMapGetEnumerationValue(hNodeMap, "Scan3dCoordinateSelector", checkpScan3dCoordinateSelector, &checkpScan3dCoordinateSelectorBufLen);
	if (err != AC_ERR_SUCCESS) {
		printf("%sScan3dCoordinateSelector node is not found. Please make sure that a Helios device is used for the example.\n\n", TAB1);
		return 0;
	}

	// Validate if the Scan3dCoordinateOffset node exists. If it doesn't exist, it is likely that the Helios
	// has old firmware
	double checkOffset = 0.0;
	err = acNodeMapGetFloatValue(hNodeMap, "Scan3dCoordinateOffset", &checkOffset);
	if (err != AC_ERR_SUCCESS) {
		printf("%sScan3dCoordinateOffset node is not found. Please update Helios firmware.\n\n", TAB1);
		return 0;
	}

	// check if Helios2 camera used for the example
	bool isHelios2 = false;
	char deviceModelName[MAX_BUF];
	size_t deviceModelNameBufLen = MAX_BUF;
	err = acNodeMapGetEnumerationValue(hNodeMap, "DeviceModelName", deviceModelName, &deviceModelNameBufLen);
	if (strstr(deviceModelName, "HLT") != NULL || strstr(deviceModelName, "HT") != NULL)
	{
		isHelios2 = true;
	}


	// get node values that will be changed in order to return their values at
	// the end of the example
	char pPixelFormatInitial[MAX_BUF];
	size_t pPixelFormatBufLen = MAX_BUF;
	char pScan3dModeInitial[MAX_BUF];
	size_t pScan3dModeBufLen = MAX_BUF;	
	
	err = acNodeMapGetStringValue(hNodeMap, "PixelFormat", pPixelFormatInitial, &pPixelFormatBufLen) | 
		acNodeMapGetStringValue(hNodeMap, "Scan3dOperatingMode", pScan3dModeInitial, &pScan3dModeBufLen);
	if (err != AC_ERR_SUCCESS)
		return err;

	// set pixel format
	printf("%sSet Coord3D_ABCY16 to pixel format\n", TAB1);

	err = acNodeMapSetStringValue(hNodeMap, "PixelFormat", "Coord3D_ABCY16");
	if (err != AC_ERR_SUCCESS)
		return err;

	// set operating mode distance

	if (isHelios2) 
	{
		printf("%sSet 3D operating mode to Distance3000mm\n", TAB1);
		err = acNodeMapSetStringValue(hNodeMap, "Scan3dOperatingMode", "Distance3000mmSingleFreq");
		if (err != AC_ERR_SUCCESS)
			return err;
	}
	else 
	{
		printf("%sSet 3D operating mode to Distance1500mm\n", TAB1);
		err = acNodeMapSetStringValue(hNodeMap, "Scan3dOperatingMode", "Distance1500mm");
		if (err != AC_ERR_SUCCESS)
			return err;
	}

	// get the z coordinate scale in order to convert z values to mm
	printf("%sGet z coordinate scale\n\n", TAB1);

	err = acNodeMapSetStringValue(hNodeMap, "Scan3dCoordinateSelector", "CoordinateC");
	if (err != AC_ERR_SUCCESS)
		return err;

	double scale = 0.0;
	err = acNodeMapGetFloatValue(hNodeMap, "Scan3dCoordinateScale", &scale);
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
	err = acDeviceStartStream(hDevice);
	if (err != AC_ERR_SUCCESS)
		return err;

	// retrieve image
	printf("%sAcquire image\n", TAB2);
	acBuffer hBuffer = NULL;

	err = acDeviceGetBuffer(hDevice, IMAGE_TIMEOUT, &hBuffer);
	if (err != AC_ERR_SUCCESS)
		return err;

	// prepare info from input buffer
	size_t width = 0;
	size_t height = 0;
	size_t srcBpp = 0;
	uint64_t srcPixelFormat = 0;
	uint8_t* pInput = NULL;

	err = acImageGetWidth(hBuffer, &width) |
		acImageGetHeight(hBuffer, &height) |
		acImageGetBitsPerPixel(hBuffer, &srcBpp) |
		acImageGetPixelFormat(hBuffer, &srcPixelFormat) |
		acImageGetData(hBuffer, &pInput);
	if (err != AC_ERR_SUCCESS)
		return err;

	size_t size = width * height;
	size_t srcPixelSize = srcBpp / 8;

	// prepare memory output buffer
	size_t dstBpp = 0;

	err = acGetBitsPerPixel(PIXEL_FORMAT, &dstBpp);
	if (err != AC_ERR_SUCCESS)
		return err;

	size_t dstPixelSize = dstBpp / 8;
	size_t dstDataSize = width * height * dstBpp / 8;
	uint8_t* pOutput = (uint8_t*)malloc(dstDataSize);
	memset(pOutput, 0, dstDataSize);

	// Prepare coloring buffer for ply image
	//    Saving ply with color takes RGB coloring compared to the BGR coloring
	//    the jpg image uses, therefore we need a separate buffer for this data.
	uint8_t* pColoring = (uint8_t*)malloc(dstDataSize);
	memset(pColoring, 0, dstDataSize);
	uint8_t* pColor = pColoring;

	// manually convert to BGR image

	const uint8_t* pIn = pInput;
	uint8_t* pOut = pOutput;

	const double RGBmin = 0;
	const double RGBmax = 255;

	double redColorBorder;
	double yellowColorBorder;
	double greenColorBorder;
	double cyanColorBorder;
	double blueColorBorder;

	if (isHelios2)
	{
		redColorBorder = 0;      // = 0 // start
		yellowColorBorder = 750; // = Scan3dOperatingMode / 4 // 1st border
		greenColorBorder = 1500; // = (Scan3dOperatingMode / 4) * 2 // 2nd border
		cyanColorBorder = 2250;  // = (Scan3dOperatingMode / 4) * 3 // 3rd border
		blueColorBorder = 3000;  //  = Scan3dOperatingMode  // finish - maximum distance
	}
	else
	{
		redColorBorder = 0;
		yellowColorBorder = 375;
		greenColorBorder = 750;
		cyanColorBorder = 1125;
		blueColorBorder = 1500;
	}

	// iterate through each pixel and assign a color to it according to a
	// distance
	for (size_t i = 0; i < size; i++)
	{
		// Isolate the z data
		//    The first channel is the x coordinate, the second channel is the y
		//    coordinate, the third channel is the z coordinate (which is what we
		//    will use to determine the colouring), and the fourth channel is
		//    confidence.
		uint16_t z = *(uint16_t*)((pIn + 4));

		// Convert z to millimeters
		//    The z data converts at a specified ratio to mm, so by multiplying
		//    it by the Scan3dCoordinateScale for CoordinateC, we can
		//    convert it to millimeters and can then compare it to the maximum distance of
		//    1500mm (in this case, 3000mm for Helios2).
		z = (int16_t)((double)(z)*scale);

		double coordinateColorBlue = 0.0;
		double coordinateColorGreen = 0.0;
		double coordinateColorRed = 0.0;

		// colors between red and yellow
		if ((z >= redColorBorder) && (z <= yellowColorBorder))
		{
			double yellowColorPercentage = z / yellowColorBorder;

			coordinateColorBlue = RGBmin;
			coordinateColorGreen = RGBmax * yellowColorPercentage;
			coordinateColorRed = RGBmax;
		}

		// colors between yellow and green
		else if ((z > yellowColorBorder) && (z <= greenColorBorder))
		{
			double greenColorPercentage = (z - yellowColorBorder) / yellowColorBorder;

			coordinateColorBlue = RGBmin;
			coordinateColorGreen = RGBmax;
			coordinateColorRed = RGBmax - RGBmax * greenColorPercentage;
		}

		// colors between green and cyan
		else if ((z > greenColorBorder) && (z <= cyanColorBorder))
		{
			double cyanColorPercentage = (z - greenColorBorder) / yellowColorBorder;

			coordinateColorBlue = RGBmax * cyanColorPercentage;
			coordinateColorGreen = RGBmax;
			coordinateColorRed = RGBmin;
		}

		// colors between cyan and blue
		else if ((z > cyanColorBorder) && (z <= blueColorBorder))
		{
			double blueColorPercentage = (z - cyanColorBorder) / yellowColorBorder;

			coordinateColorBlue = RGBmax;
			coordinateColorGreen = RGBmax - RGBmax * blueColorPercentage;
			coordinateColorRed = RGBmin;

		}
		else
		{
			coordinateColorBlue = RGBmin;
			coordinateColorGreen = RGBmin;
			coordinateColorRed = RGBmin;
		}

		// set pixel format values and move to next pixel
		*pOut = (uint8_t)(coordinateColorBlue);
		*(pOut + 1) = (uint8_t)(coordinateColorGreen);
		*(pOut + 2) = (uint8_t)(coordinateColorRed);

		pIn += srcPixelSize;
		pOut += dstPixelSize;

		// set RGB pixel coloring for ply
		*pColor = (uint8_t)(coordinateColorRed);
		*(pColor + 1) = (uint8_t)(coordinateColorGreen);
		*(pColor + 2) = (uint8_t)(coordinateColorBlue);
		pColor += dstPixelSize;
	}

	printf("%sCreate BGR heatmap using z data from 3D image\n", TAB2);

	// cast width and height to size_t for writer creation
	size_t saveWidth = (size_t)width;
	size_t saveHeight = (size_t)height;
	saveWriter hJpegWriter = NULL;
	err = saveWriterCreate(saveWidth, saveHeight, dstBpp, &hJpegWriter);
	if (err != AC_ERR_SUCCESS)
		return err;

	// save image as jpg
	printf("%sSave heatmap image as jpg to ", TAB2);

	// create image from buffer and save
	acBuffer hJpegCreate = NULL;
	uint8_t* hJpegImageData = NULL;
	const char* pJpegFileNamePattern = JPG_FILE_NAME;

	err = acImageFactoryCreate(pOutput, dstDataSize, saveWidth, saveHeight, PIXEL_FORMAT, &hJpegCreate);
	if (err != AC_ERR_SUCCESS)
		return err;

	err = acImageGetData(hJpegCreate, &hJpegImageData);
	if (err != AC_ERR_SUCCESS)
		return err;

	err = saveWriterSetFileNamePattern(hJpegWriter, pJpegFileNamePattern);
	if (err != AC_ERR_SUCCESS)
		return err;

	err = saveWriterSave(hJpegWriter, hJpegImageData);
	if (err != AC_ERR_SUCCESS)
		return err;

	char jpegFileName[MAX_BUF];
	size_t jpegFileLen = MAX_BUF;

	err = saveWriterGetLastFileName(hJpegWriter, jpegFileName, &jpegFileLen);
	if (err != AC_ERR_SUCCESS)
		return err;

	printf("%s\n", jpegFileName);

	// save image as ply
	printf("%sSave 3D image as ply to ", TAB2);
	saveWriter hPlyWriter = NULL;

	err = saveWriterCreate(saveWidth, saveHeight, srcBpp, &hPlyWriter);
	if (err != AC_ERR_SUCCESS)
		return err;
	uint8_t* hPlyImageData = NULL;
	const char* pPlyFileNamePattern = PLY_FILE_NAME;

	err = acImageGetData(hBuffer, &hPlyImageData);
	if (err != AC_ERR_SUCCESS)
		return err;

	err = saveWriterSetFileNamePattern(hPlyWriter, pPlyFileNamePattern);
	if (err != AC_ERR_SUCCESS)
		return err;

	// parameters for saveWriterSetPlyAndConfigExtended
	savePlyParams params = 
	{
					true,			// filterPoints default
					false,			// isSignedPixelFormat = false; the example use Coord3D_ABCY16
					(float)scale,	// scale is cast to float since saveWriterSetPlyAndConfigExtended will expect scale as float
					0.0f,			// offsetA default
					0.0f,			// offsetB default
					0.0f			// offsetC default
	};

	err = saveWriterSetPlyAndConfigExtended(hPlyWriter, params);
	if (err != AC_ERR_SUCCESS)
		return err;

	err = saveWriterSaveWithColor(hPlyWriter, hPlyImageData, pColoring, true);
	if (err != AC_ERR_SUCCESS)
		return err;

	char plyFileName[MAX_BUF];
	size_t plyFileLen = MAX_BUF;

	err = saveWriterGetLastFileName(hPlyWriter, plyFileName, &plyFileLen);
	if (err != AC_ERR_SUCCESS)
		return err;

	printf("%s\n\n", plyFileName);

	// clean up
	free(pOutput);
	free(pColoring);

	err = acImageFactoryDestroy(hJpegCreate) |
		acDeviceRequeueBuffer(hDevice, hBuffer) |
		acDeviceStopStream(hDevice) |
		saveWriterDestroy(hJpegWriter) | saveWriterDestroy(hPlyWriter);
	if (err != AC_ERR_SUCCESS)
		return err;

	// return nodes to their initial values
	err = acNodeMapSetStringValue(hNodeMap, "Scan3dOperatingMode", pScan3dModeInitial) |
		acNodeMapSetStringValue(hNodeMap, "PixelFormat", pPixelFormatInitial);
	if (err != AC_ERR_SUCCESS)
		return err;
	printf("%sNodes were set back to initial values\n", TAB1);

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
	printf("C_Helios_HeatMap\n");
	AC_ERROR err = AC_ERR_SUCCESS;

	// prepare example
	acSystem hSystem = NULL;
	err = acOpenSystem(&hSystem);
	CHECK_RETURN;
	err = acSystemUpdateDevices(hSystem, DEVICE_TIMEOUT);
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
	acDevice hDevice = NULL;
	size_t selection = 0;
	err = SelectDevice(hSystem, &numDevices, &selection);
	CHECK_RETURN;
	err = acSystemCreateDevice(hSystem, selection, &hDevice);
	CHECK_RETURN;

	printf("Commence example\n\n");

	// run example
	err = AcquireImageAndInterpretData(hDevice);

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
	return 0;
}
