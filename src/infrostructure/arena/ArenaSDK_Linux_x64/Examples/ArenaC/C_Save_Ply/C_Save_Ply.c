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
#if (!defined _WIN32 && !defined _WIN64)
#define scanf_s scanf
#endif

#define TAB1 "  "
#define TAB2 "    "

// Save: Introduction
//    This example introduces the basic save capabilities of the save library. It
//    shows the construction of an image parameters object and an image writer,
//    and saves a single image.

// =-=-=-=-=-=-=-=-=-
// =-=- SETTINGS =-=-
// =-=-=-=-=-=-=-=-=-

// File name
//    The relative path and file name to save to. After running the example, an
//    image should exist at the location specified. The image writer chooses the
//    file format by the image's extension. Aside from PNG (.png), images can be
//    saved as JPEG (.jpg), TIFF (.tiff), BMP (.bmp), and raw (.raw) files. For
//    3D file formats the PLY (.ply) extension can be used.
#define FILE_NAME "Images/C_Save_Ply/C_Save_Ply.ply"

// timeout for detecting camera devices (in milliseconds).
#define SYSTEM_TIMEOUT 100

// maximum buffer length
#define MAX_BUF 1024

// introduce variable which will be passed to saveWriterSetPlyAndConfigExtended()
// trough savePlyParams
bool isSignedPixelFormat = false;

// =-=-=-=-=-=-=-=-=-
// =-=- EXAMPLE -=-=-
// =-=-=-=-=-=-=-=-=-

bool ValidateDevice(acDevice hDevice)
{
	acNodeMap hNodeMap = NULL;
	AC_ERROR err1 = acDeviceGetNodeMap(hDevice, &hNodeMap);
	AC_ERROR err2 = acDeviceGetNodeMap(hDevice, &hNodeMap);

	// Validate if the Scan3dCoordinateSelector node exists. If it doesn't exist, the camera being
	// used to run the example is likely not a Helios
	char checkpScan3dCoordinateSelector[MAX_BUF];
	size_t checkpScan3dCoordinateSelectorBufLen = MAX_BUF;
	err1 = acNodeMapGetEnumerationValue(hNodeMap, "Scan3dCoordinateSelector", checkpScan3dCoordinateSelector, &checkpScan3dCoordinateSelectorBufLen);

	// Validate if the Scan3dCoordinateOffset node exists. If it doesn't exist, it is likely that the Helios
	// has old firmware
	double checkOffset = 0.0;
	err2 = acNodeMapGetFloatValue(hNodeMap, "Scan3dCoordinateOffset", &checkOffset);

	if (err1 != AC_ERR_SUCCESS)
	{
		printf("%sScan3dCoordinateSelector node is not found. Please make sure that a Helios device is used for the example.\n\n", TAB1);
		return false;
	}
	else if (err2 != AC_ERR_SUCCESS)

	{
		printf("%sScan3dCoordinateOffset node is not found. Please update Helios firmware.\n\n", TAB1);
		return false;
	}
	else
	{
		return true;
	}
}

// demonstrates saving an image
// (1) converts image to a displayable pixel format
// (2) prepares image parameters
// (3) prepares image writer
// (4) saves image
// (5) destroys converted image
AC_ERROR SaveImage(acBuffer hBuffer, const char* filename)
{
	// AC_ERROR and SC_ERROR values are equivalent
	AC_ERROR acErr = AC_ERR_SUCCESS;
	SC_ERROR saveErr = SC_ERR_SUCCESS;

	// Prepare image parameters
	//    An image's width, height, and bits per pixel are required to save to
	//    disk.
	printf("%sPrepare image parameters\n", TAB1);

	// get width
	size_t width = 0;

	acErr = acImageGetWidth(hBuffer, &width);
	if (acErr != AC_ERR_SUCCESS)
		return acErr;

	// get height
	size_t height = 0;

	acErr = acImageGetHeight(hBuffer, &height);
	if (acErr != AC_ERR_SUCCESS)
		return acErr;

	// get bits per pixel
	size_t bpp = 0;

	acErr = acImageGetBitsPerPixel(hBuffer, &bpp);
	if (acErr != AC_ERR_SUCCESS)
		return acErr;

	// Prepare image writer
	//    The image writer requires the image's parameters, a specified file name
	//    or pattern, and the image data to save. Providing these should result
	//    in a successfully saved file on the disk. Because an image's parameters
	//    and file name pattern may repeat, they can be passed into the image
	//    writer's constructor.
	printf("%sPrepare image writer\n", TAB1);
	saveWriter hWriter = NULL;

	saveErr = saveWriterCreate(width, height, bpp, &hWriter);
	if (saveErr != SC_ERR_SUCCESS)
		return saveErr;

	saveErr = saveWriterSetFileNamePattern(hWriter, filename);
	if (saveErr != SC_ERR_SUCCESS)
		return saveErr;

	// parameters for saveWriterSetPlyAndConfigExtended
	savePlyParams params = {
		true, // filterPoints default
		isSignedPixelFormat,
		0.25f, // scale default
		0.0f,  // offsetA default
		0.0f,  // offsetB default
		0.0f   // offsetC default
	};

	saveErr = saveWriterSetPlyAndConfigExtended(hWriter, params);
	if (saveErr != AC_ERR_SUCCESS)
		return saveErr;

	// Save image
	//    Get and save the image. Notice that pointers to the beginning of the
	//    image data stay valid for the life cycle of the object, and do not
	//    require memory allocation or object destruction.
	printf("%sSave image\n", TAB1);

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
	{
		return saveErr;
	}

	return SC_ERR_SUCCESS;
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
	printf("C_Save_Ply\n");
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
	acDevice hDevice = NULL;
	size_t selection = 0;
	err = SelectDevice(hSystem, &numDevices, &selection);
	CHECK_RETURN;
	err = acSystemCreateDevice(hSystem, selection, &hDevice);
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

	// start stream
	err = acDeviceStartStream(hDevice);
	CHECK_RETURN;

	// get buffer
	acBuffer hBuffer = NULL;
	err = acDeviceGetBuffer(hDevice, 2000, &hBuffer);
	CHECK_RETURN;

	acNodeMap hNodeMap = NULL;
	err = acDeviceGetNodeMap(hDevice, &hNodeMap);
	if (err != AC_ERR_SUCCESS)
		return err;

	// get bits per pixel
	uint64_t pixelFormat = 0;
	err = acImageGetPixelFormat(hBuffer, &pixelFormat);
	CHECK_RETURN;

	if (pixelFormat == LUCID_Coord3D_ABCY16s || pixelFormat == LUCID_Coord3D_ABC16s)
	{
		isSignedPixelFormat = true;
	}

	bool isDeviceValid = ValidateDevice(hDevice);

	if (isDeviceValid == true)

	{
		if (pixelFormat == LUCID_Coord3D_ABCY16 || pixelFormat == PFNC_Coord3D_ABC16 || pixelFormat == LUCID_Coord3D_ABCY16s || pixelFormat == LUCID_Coord3D_ABC16s)
		{
			// run example
			printf("Commence example\n\n");
			err = SaveImage(hBuffer, FILE_NAME);
			CHECK_RETURN;
			printf("\nExample complete\n");
		}
		else
		{
			printf("This example requires camera to be in a 3D image format like Coord3D_ABC16, Coord3D_ABCY16, Coord3D_ABC16s or Coord3D_ABCY16s\n\n");
		}
	}

	// clean up example
	err = acDeviceRequeueBuffer(hDevice, hBuffer);
	CHECK_RETURN;
	err = acDeviceStopStream(hDevice);
	CHECK_RETURN;
	err = acSystemDestroyDevice(hSystem, hDevice);
	CHECK_RETURN;
	err = acCloseSystem(hSystem);
	CHECK_RETURN;

	printf("Press enter to complete\n");
	while (getchar() != '\n') {};
	getchar();
	return 0;
}
