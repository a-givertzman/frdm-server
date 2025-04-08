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

// Save: Raw
//    This example introduces saving raw image data in the saving library. It
//    shows the construction of an image parameters object and an image writer,
//    sets writer to raw and saves a single RAW image.

// =-=-=-=-=-=-=-=-=-
// =-=- SETTINGS =-=-
// =-=-=-=-=-=-=-=-=-

// file name
#define FILE_NAME "Images/C_Save/image.raw"

// pixel format
#define PIXEL_FORMAT PFNC_BGR8 // BGR8

// timeout for detecting camera devices (in milliseconds).
#define SYSTEM_TIMEOUT 100

// maximum buffer length
#define MAX_BUF 1024

// =-=-=-=-=-=-=-=-=-
// =-=- EXAMPLE -=-=-
// =-=-=-=-=-=-=-=-=-

// demonstrates saving an image
// (1) converts image to a displayable pixel format
// (2) prepares image parameters
// (3) prepares image writer
// (4) sets image writer to RAW
// (5) saves image
// (6) destroys converted image
AC_ERROR SaveImage(acBuffer hBuffer, const char* filename)
{
	// AC_ERROR and SC_ERROR values are equivalent
	AC_ERROR acErr = AC_ERR_SUCCESS;
	SC_ERROR saveErr = SC_ERR_SUCCESS;

	// convert image
	printf("%sConvert image to %s\n", TAB1, (PIXEL_FORMAT == PFNC_BGR8 ? "BGR8" : "RGB8"));
	acBuffer hConverted = NULL;

	acErr = acImageFactoryConvert(hBuffer, PIXEL_FORMAT, &hConverted);
	if (acErr != AC_ERR_SUCCESS)
		return acErr;

	// prepare image parameters
	printf("%sPrepare image parameters\n", TAB1);

	// get width
	size_t width = 0;

	acErr = acImageGetWidth(hConverted, &width);
	if (acErr != AC_ERR_SUCCESS)
		return acErr;

	// get height
	size_t height = 0;

	acErr = acImageGetHeight(hConverted, &height);
	if (acErr != AC_ERR_SUCCESS)
		return acErr;

	// get bits per pixel
	size_t bpp = 0;

	acErr = acImageGetBitsPerPixel(hConverted, &bpp);
	if (acErr != AC_ERR_SUCCESS)
		return acErr;

	// prepare image writer
	printf("%sPrepare image writer\n", TAB1);
	saveWriter hWriter = NULL;

	saveErr = saveWriterCreate(width, height, bpp, &hWriter);
	if (saveErr != SC_ERR_SUCCESS)
		return saveErr;

	saveErr = saveWriterSetFileNamePattern(hWriter, filename);
	if (saveErr != SC_ERR_SUCCESS)
		return saveErr;

	// Set image writer to RAW
	//   Set the output file format of the image writer to RAW.
	//   The writer saves the image file as raw file even without
	//	 the extension in the file name.
	printf("%sSet image writer to RAW\n", TAB1);
	saveErr = saveWriterSetRaw(hWriter);
	if (saveErr != SC_ERR_SUCCESS)
		return saveErr;

	// save image
	printf("%sSave image\n", TAB1);

	// get image
	uint8_t* pData = NULL;

	acErr = acImageGetData(hConverted, &pData);
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
	acErr = acImageFactoryDestroy(hConverted);
	if (acErr != AC_ERR_SUCCESS)
		return acErr;

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
	printf("C_Save_Raw\n");
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

	// Enable stream auto negotiate packet size
	//    Setting the stream packet size is done before starting the stream.
	//    Setting the stream to automatically negotiate packet size instructs the
	//    camera to receive the largest packet size that the system will allow.
	//    This generally increases frame rate and results in fewer interrupts per
	//    image, thereby reducing CPU load on the host system. Ethernet settings
	//    may also be manually changed to allow for a larger packet size.
	err = acNodeMapSetBooleanValue(hTLStreamNodeMap, "StreamAutoNegotiatePacketSize", true);
	if (err != AC_ERR_SUCCESS)
		return err;

	// Enable stream packet resend
	//    Enable stream packet resend before starting the stream. Images are sent
	//    from the camera to the host in packets using UDP protocol, which
	//    includes a header image number, packet number, and timestamp
	//    information. If a packet is missed while receiving an image, a packet
	//    resend is requested and this information is used to retrieve and
	//    redeliver the missing packet in the correct order.
	err = acNodeMapSetBooleanValue(hTLStreamNodeMap, "StreamPacketResendEnable", true);
	if (err != AC_ERR_SUCCESS)
		return err;

	// start stream
	err = acDeviceStartStream(hDevice);
	CHECK_RETURN;

	acBuffer hBuffer = NULL;
	err = acDeviceGetBuffer(hDevice, 2000, &hBuffer);
	CHECK_RETURN;

	// get bits per pixel
	uint64_t pixelFormat = 0;
	err = acImageGetPixelFormat(hBuffer, &pixelFormat);
	CHECK_RETURN;

	// run example
	printf("Commence example\n\n");
	err = SaveImage(hBuffer, FILE_NAME);
	CHECK_RETURN;
	printf("\nExample complete\n");

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
