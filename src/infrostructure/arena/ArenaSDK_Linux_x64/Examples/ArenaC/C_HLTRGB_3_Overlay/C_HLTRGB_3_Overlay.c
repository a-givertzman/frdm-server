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
#include <stdbool.h>  // defines boolean type and values
#include <stddef.h>

#include "ArenaCApi.h"
#include "SaveCApi.h"
#include <inttypes.h> // defines macros for printf functions
#include <stdio.h>
#include <string.h>   // defines string type and values

#include <opencv/cv.h>
#include <opencv/cxcore.h>
#include <opencv2/highgui/highgui_c.h>

 /*
 * This code requires OpenCV version 2.4.13
 *
 * Ensure that you have OpenCV 2.4.13 installed and properly configured in your development environment.
 * This specific version is necessary for compatibility with the functions and features used in this code.
 *
 * Visit the [OpenCV Releases](https://opencv.org/releases/) page to find a list of all OpenCV versions available for download.
 *
 * For a direct download of OpenCV 2.4.13, click [here](https://sourceforge.net/projects/opencvlibrary/files/opencv-win/2.4.13/opencv-2.4.13.4-vc14.exe/download). 
 * This link will redirect you to SourceForge, a trusted platform where the official OpenCV binaries are hosted.
 */


#define TAB1 "  "
#define TAB2 "    "

 // Helios RGB: Overlay
 //    This example is part 3 of a 3-part example on color overlay over 3D images.
 //    With the system calibrated, we can now remove the calibration target from
 //    the scene and grab new images with the Helios and Triton cameras, using the
 //    calibration result to find the RGB color for each 3D point measured with
 //    the Helios. Based on the output of solvePnP we can project the 3D points
 //    measured by the Helios onto the RGB camera image using the OpenCV function
 //    projectPoints. Grab a Helios image with the GetHeliosImage()
 //    function(output: xyz_mm) and a Triton RGB image with the
 //    GetTritionRGBImage() function(output: triton_rgb). The following code shows
 //    how to project the Helios xyz points onto the Triton image, giving a(row,
 //    col) position for each 3D point. We can sample the Triton image at
 //    that(row, col) position to find the 3D point's RGB value.

 // =-=-=-=-=-=-=-=-=-
 // =-=- SETTINGS =-=-
 // =-=-=-=-=-=-=-=-=-

 // image timeout
#define TIMEOUT 2000

 // orientation values file name
#define FILE_NAME_IN "orientation.yml"

 // file name
#define FILE_NAME_OUT "Images\\C_HLTRGB_3_Overlay.ply"

// maximum buffer length
#define MAX_BUF 1024

// =-=-=-=-=-=-=-=-=-
// =-=- HELPERS -=-=-
// =-=-=-=-=-=-=-=-=-

// helper function
AC_ERROR GetImageHLT(acDevice hDeviceHLT, acBuffer* hBufferHLT, CvMat** xyz_mm, size_t* width, size_t* height, double* xyz_scale_mm, double* x_offset_mm, double* y_offset_mm, double* z_offset_mm, size_t* bppHLT)
{
	AC_ERROR err = AC_ERR_SUCCESS;

	acNodeMap hNodeMapHLT= NULL;
	err = acDeviceGetNodeMap(hDeviceHLT, &hNodeMapHLT);
	if (err != AC_ERR_SUCCESS)
		return err;

	// Read the scale factor and offsets to convert from unsigned 16-bit values 
	//    in the Coord3D_ABCY16 pixel format to coordinates in mm
	err = acNodeMapGetFloatValue(hNodeMapHLT, "Scan3dCoordinateScale", xyz_scale_mm);
	if (err != AC_ERR_SUCCESS) 
		return err;
	err = acNodeMapSetStringValue(hNodeMapHLT, "Scan3dCoordinateSelector", "CoordinateA");
	if (err != AC_ERR_SUCCESS)
		return err;

	err = acNodeMapGetFloatValue(hNodeMapHLT, "Scan3dCoordinateOffset", x_offset_mm);
	if (err != AC_ERR_SUCCESS)
		return err;
	err = acNodeMapSetStringValue(hNodeMapHLT, "Scan3dCoordinateSelector", "CoordinateB");
	if (err != AC_ERR_SUCCESS)
		return err;

	err = acNodeMapGetFloatValue(hNodeMapHLT, "Scan3dCoordinateOffset", y_offset_mm);
	if (err != AC_ERR_SUCCESS)
		return err;
	err = acNodeMapSetStringValue(hNodeMapHLT, "Scan3dCoordinateSelector", "CoordinateC");
	if (err != AC_ERR_SUCCESS)
		return err;

	err = acNodeMapGetFloatValue(hNodeMapHLT, "Scan3dCoordinateOffset", z_offset_mm);
	if (err != AC_ERR_SUCCESS)
		return err;

	// start stream
	err = acDeviceStartStream(hDeviceHLT);
	if (err != AC_ERR_SUCCESS)
		return err;

	// get image
	acBuffer hBuffer;
	err = acDeviceGetBuffer(hDeviceHLT, TIMEOUT, &hBuffer);
	if (err != AC_ERR_SUCCESS)
		return err;

	err = acImageFactoryCopy(hBuffer, hBufferHLT);
	if (err != AC_ERR_SUCCESS)
		return err;

	err = acImageGetWidth(hBuffer, width);
	if (err != AC_ERR_SUCCESS)
		return err;

	err = acImageGetHeight(hBuffer, height);
	if (err != AC_ERR_SUCCESS)
		return err;

	err = acImageGetBitsPerPixel(hBuffer, bppHLT);
	if (err != AC_ERR_SUCCESS)
		return err;

	*xyz_mm = cvCreateMat((int)*height, (int)*width, CV_32FC3);

	uint8_t* input_data = NULL;
	err = acImageGetData(hBuffer, &input_data);
	if (err != AC_ERR_SUCCESS)
		return err;

	for (unsigned int ir = 0; ir < *height; ++ir) 
	{
		for (unsigned int ic = 0; ic < *width; ++ic) 
		{
			// Combine two uint8_t values to create each 16-bit value for X, Y, Z
			ushort x_u16 = ((ushort)input_data[0] << 8) | input_data[1];
			ushort y_u16 = ((ushort)input_data[2] << 8) | input_data[3];
			ushort z_u16 = ((ushort)input_data[4] << 8) | input_data[5];

			// Calculate and set the float XYZ values
			float* xyz_ptr = (float*)((*xyz_mm)->data.ptr + (*xyz_mm)->step * ir + ic * 3 * sizeof(float));
			xyz_ptr[0] = (float)(x_u16 * *xyz_scale_mm + *x_offset_mm); // X-coordinate
			xyz_ptr[1] = (float)(y_u16 * *xyz_scale_mm + *y_offset_mm); // Y-coordinate
			xyz_ptr[2] = (float)(z_u16 * *xyz_scale_mm + *z_offset_mm); // Z-coordinate

			input_data += 6; // Move to the next set of XYZ values
		}
	}
	
	// clean up
	input_data = NULL;
	
	err = acDeviceRequeueBuffer(hDeviceHLT, hBuffer) | acDeviceStopStream(hDeviceHLT);
	if (err != AC_ERR_SUCCESS)
		return err;

	return err;
}

AC_ERROR GetImageTRI(acDevice hDeviceTRI, acBuffer* hBufferTRI, CvMat** triton_image)
{
	AC_ERROR err = AC_ERR_SUCCESS;

	acNodeMap hNodeMapTRI = NULL;
	err = acDeviceGetNodeMap(hDeviceTRI, &hNodeMapTRI);
	if (err != AC_ERR_SUCCESS)
		return err;

	err = acNodeMapSetEnumerationValue(hNodeMapTRI, "PixelFormat", "RGB8");
	if (err != AC_ERR_SUCCESS)
		return err;

	// start stream
	err = acDeviceStartStream(hDeviceTRI);
	if (err != AC_ERR_SUCCESS)
		return err;

	// get image
	acBuffer hBuffer;
	err = acDeviceGetBuffer(hDeviceTRI, TIMEOUT, &hBuffer);
	if (err != AC_ERR_SUCCESS)
		return err;

	err = acImageFactoryCopy(hBuffer, hBufferTRI);
	if (err != AC_ERR_SUCCESS)
		return err;

	size_t triWidth = 0;
	size_t triHeight = 0;
	err = acImageGetWidth(hBuffer, &triWidth);
	if (err != AC_ERR_SUCCESS)
		return err;

	err = acImageGetHeight(hBuffer, &triHeight);
	if (err != AC_ERR_SUCCESS)
		return err;

	*triton_image = cvCreateMat((int)triHeight, (int)triWidth, CV_8UC3);

	uint8_t* pData = NULL;
	err = acImageGetData(hBuffer, &pData);
	if (err != AC_ERR_SUCCESS)
		return err;

	memcpy((*triton_image)->data.ptr, pData, triHeight * triWidth * 3);

	// clean up
	pData = NULL;
	err = acDeviceRequeueBuffer(hDeviceTRI, hBuffer) | acDeviceStopStream(hDeviceTRI);
	if (err != AC_ERR_SUCCESS)
		return err;

	return err;
}


// =-=-=-=-=-=-=-=-=-
// =-=- EXAMPLE -=-=-
// =-=-=-=-=-=-=-=-=-

AC_ERROR OverlayColorOnto3DAndSave(acDevice hDeviceTRI, acDevice hDeviceHLT)
{
	AC_ERROR err = AC_ERR_SUCCESS;

	acNodeMap hNodeMapTRI = NULL;
	err = acDeviceGetNodeMap(hDeviceTRI, &hNodeMapTRI);
	if (err != AC_ERR_SUCCESS)
		return err;

	acNodeMap hNodeMapHLT = NULL;
	err = acDeviceGetNodeMap(hDeviceHLT, &hNodeMapHLT);
	if (err != AC_ERR_SUCCESS)
		return err;

	// get node values that will be changed in order to return their values at
	// the end of the example
	char pPixelFormatInitialTRI[MAX_BUF];
	size_t pPixelFormatBufLenTRI = MAX_BUF;
	err = acNodeMapGetStringValue(hNodeMapTRI, "PixelFormat", pPixelFormatInitialTRI, &pPixelFormatBufLenTRI);
	if (err != AC_ERR_SUCCESS)
		return err;

	char pPixelFormatInitialHLT[MAX_BUF];
	size_t pPixelFormatBufLenHLT = MAX_BUF;
	err = acNodeMapGetStringValue(hNodeMapHLT, "PixelFormat", pPixelFormatInitialHLT, &pPixelFormatBufLenHLT);
	if (err != AC_ERR_SUCCESS)
		return err;

	// Read in camera matrix, distance coefficients, and rotation and translation vectors

	CvFileStorage* fs = cvOpenFileStorage(FILE_NAME_IN, NULL, CV_STORAGE_READ, NULL);
	CvMat* cameraMatrix;
	CvMat* distCoeffs;
	CvMat* rotationVector;
	CvMat* translationVector;

	CvMat* tempMatrix = (CvMat*)cvReadByName(fs, NULL, "cameraMatrix", NULL);
	if (tempMatrix != NULL) {
		cameraMatrix = cvCreateMat(tempMatrix->rows, tempMatrix->cols, tempMatrix->type);
		cvCopy(tempMatrix, cameraMatrix, NULL);
		cvReleaseMat(&tempMatrix);
	}
	cvReleaseMat(&tempMatrix);

	tempMatrix = (CvMat*)cvReadByName(fs, NULL, "distCoeffs", NULL);
	if (tempMatrix != NULL) {
		distCoeffs = cvCreateMat(tempMatrix->rows, tempMatrix->cols, tempMatrix->type);
		cvCopy(tempMatrix, distCoeffs, NULL);
		cvReleaseMat(&tempMatrix);
	}

	tempMatrix = (CvMat*)cvReadByName(fs, NULL, "rotationVector", NULL);
	if (tempMatrix != NULL) {
		rotationVector = cvCreateMat(tempMatrix->rows, tempMatrix->cols, tempMatrix->type);
		cvCopy(tempMatrix, rotationVector, NULL);
		cvReleaseMat(&tempMatrix);
	}

	tempMatrix = (CvMat*)cvReadByName(fs, NULL, "translationVector", NULL);
	if (tempMatrix != NULL) {
		translationVector = cvCreateMat(tempMatrix->rows, tempMatrix->cols, tempMatrix->type);
		cvCopy(tempMatrix, translationVector, NULL);
		cvReleaseMat(&tempMatrix);
	}

	cvReleaseFileStorage(&fs);

	// Get an image from Helios2
	printf("%sGet and prepare HLT image\n", TAB1);

	acBuffer hBufferHLT = NULL;
	CvMat* imageMatrixHLTXYZ = NULL;
	size_t width = 0;
	size_t height = 0;
	double scale;
	double offsetX, offsetY, offsetZ;
	size_t bppHLT = 0;

	err = GetImageHLT(hDeviceHLT, &hBufferHLT, &imageMatrixHLTXYZ, &width, &height, &scale, &offsetX, &offsetY, &offsetZ, &bppHLT);
	if (err != AC_ERR_SUCCESS)
		return err;

	char filename[256];
	snprintf(filename, sizeof(filename), "%sXYZ.jpg", FILE_NAME_OUT);										  
	cvSaveImage(filename, imageMatrixHLTXYZ, 0);

	// Get an image from Triton
	printf("%sGet and prepare TRI image\n", TAB1);

	acBuffer hBufferTRI = NULL;
	CvMat* imageMatrixRGB = NULL;
	err = GetImageTRI(hDeviceTRI, &hBufferTRI, &imageMatrixRGB);
	if (err != AC_ERR_SUCCESS)
		return err;

	char filenameTRI[256];
	snprintf(filenameTRI, sizeof(filenameTRI), "%sRGB.jpg", FILE_NAME_OUT);
	cvSaveImage(filenameTRI, imageMatrixRGB, 0);

	// Overlay RGB color data onto 3D XYZ points
	printf("%sOverlay the RGB color data onto the 3D XYZ points\n", TAB1);

	// reshape image matrix
	printf("%sReshape XYZ matrix\n", TAB2);

	int size = imageMatrixHLTXYZ->rows * imageMatrixHLTXYZ->cols;
	
	CvMat xyzPointsHeader, *xyzPoints;
	xyzPoints = cvReshape(imageMatrixHLTXYZ, &xyzPointsHeader, 3, size);

	// project points
	printf("%sProject points\n", TAB2);

	CvMat* projectedPointsTRI = cvCreateMat(size, 2, CV_32FC1);

	cvProjectPoints2(
		xyzPoints,
		rotationVector,
		translationVector,
		cameraMatrix,
		distCoeffs,
		projectedPointsTRI,
		NULL, NULL, NULL, NULL, NULL, 0
	);
	
	// loop through projected points to access RGB data at those points
	printf("%sGet values at projected points\n", TAB2);

	uint8_t* pColorData = (uint8_t*)malloc(width * height * 3 * sizeof(uint8_t));

	for (size_t i = 0; i < width * height; i++)
	{
		unsigned int colTRI = (unsigned int)round(CV_MAT_ELEM(*projectedPointsTRI, float, i, 0));
		unsigned int rowTRI = (unsigned int)round(CV_MAT_ELEM(*projectedPointsTRI, float, i, 1));

		// only handle appropriate points
		if (rowTRI < 0 || colTRI < 0 || rowTRI >= (unsigned int)imageMatrixRGB->rows || colTRI >= (unsigned int)imageMatrixRGB->cols)
			continue;

		// Access corresponding XYZ and RGB data
		uchar* pixel = (uchar*)(imageMatrixRGB->data.ptr + rowTRI * imageMatrixRGB->step + colTRI * 3);
		uchar R = pixel[2];
		uchar G = pixel[1];
		uchar B = pixel[0];

		// Grab RGB data to save colored .ply
		pColorData[i * 3 + 0] = B;
		pColorData[i * 3 + 1] = G;
		pColorData[i * 3 + 2] = R;
	}

	// Save result
	printf("%sSave image to %s\n", TAB1, FILE_NAME_OUT);

	// prepare to save
	saveWriter hWriter = NULL;
	err = saveWriterCreate(width, height, bppHLT, &hWriter);
	if (err != AC_ERR_SUCCESS)
		return err;

	err = saveWriterSetFileNamePattern(hWriter, FILE_NAME_OUT);
	if (err != AC_ERR_SUCCESS)
		return err;

	// save .ply with color data
	bool filterPoints = true;
	bool isSignedPixelFormat = false;

	savePlyParams params = {
		filterPoints,
		isSignedPixelFormat,
		(float)scale,
		(float)offsetX,
		(float)offsetY,
		(float)offsetZ
	};
	
	err = saveWriterSetPlyAndConfigExtended(hWriter, params);
	if (err != AC_ERR_SUCCESS)
		return err;

	uint8_t* pDataHLT = NULL;
	err = acImageGetData(hBufferHLT, &pDataHLT); 
	if (err != AC_ERR_SUCCESS)
		return err;

	err = saveWriterSaveWithColor(hWriter, pDataHLT, pColorData, true);
	if (err != AC_ERR_SUCCESS)
		return err;

	// return nodes to their initial values
	err = acNodeMapSetStringValue(hNodeMapTRI, "PixelFormat", pPixelFormatInitialTRI);
	if (err != AC_ERR_SUCCESS)
		return err;

	err = acNodeMapSetStringValue(hNodeMapHLT, "PixelFormat", pPixelFormatInitialHLT);
	if (err != AC_ERR_SUCCESS)
		return err;

	// clean up
	err = saveWriterDestroy(hWriter);
	if (err != AC_ERR_SUCCESS)
		return err;

	free(pColorData);
	err = acImageFactoryDestroy(hBufferHLT);
	if (err != AC_ERR_SUCCESS)
		return err;
	err = acImageFactoryDestroy(hBufferTRI);
	if (err != AC_ERR_SUCCESS)
		return err;

	cvReleaseMat(&projectedPointsTRI);

	if (imageMatrixHLTXYZ != NULL) {
		cvReleaseMat(&imageMatrixHLTXYZ);
	}
	if (imageMatrixRGB != NULL) {
		cvReleaseMat(&imageMatrixRGB);
	}

	cvReleaseMat(&cameraMatrix);
	cvReleaseMat(&distCoeffs);
	cvReleaseMat(&rotationVector);
	cvReleaseMat(&translationVector);

	return err;
}

bool IsApplicableDeviceTriton(char* pModelName)
{
	return (strstr(pModelName, "TRI") != NULL && strstr(pModelName, "-C") != NULL);
}

bool IsApplicableDeviceHelios2(char* pModelName)
{
	return (strstr(pModelName, "HLT") != NULL || strstr(pModelName, "HT") != NULL);
}

int fopen_portable(const char *pFilename, const char *pMode, FILE** ppFile)
{
#if (defined _WIN32 || defined _WIN64)
	return fopen_s(ppFile, pFilename, pMode);
#else
	*ppFile = fopen(pFilename, pMode);
	return 0;
#endif
}
	
// =-=-=-=-=-=-=-=-=-
// =- PREPARATION -=-
// =- & CLEAN UP =-=-
// =-=-=-=-=-=-=-=-=-

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
	printf("C_HLTRGB_3_Overlay\n");
	
	// Open the file
	FILE* ifile = NULL;
	int error;
	error = fopen_portable(FILE_NAME_IN, "r", &ifile);
	if (error != 0 || ifile == NULL)
	{
		printf("File '%s' not found\nPlease run example 'C_HLTRGB_1_Calibration' and 'C_HLTRGB_2_Orientation' prior to this one\nPress enter to complete\n", FILE_NAME_IN);
		getchar();
		return -1;
	}

	AC_ERROR err = AC_ERR_SUCCESS;

	// prepare example
	acSystem hSystem = NULL;
	err = acOpenSystem(&hSystem);
	CHECK_RETURN;
	err = acSystemUpdateDevices(hSystem, 100);
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
	acDevice hDeviceTRI = NULL;
	acDevice hDeviceHLT = NULL;

	size_t i = 0;
	for (i = 0; i < numDevices; i++)
	{
		// get and display model name
		char pBuf[MAX_BUF];
		size_t len = MAX_BUF;

		err = acSystemGetDeviceModel(hSystem, i, pBuf, &len);
		if (err != AC_ERR_SUCCESS)
			CHECK_RETURN;

		if (!hDeviceTRI && IsApplicableDeviceTriton(pBuf))
		{
			err = acSystemCreateDevice(hSystem, i, &hDeviceTRI);
			CHECK_RETURN;

			acNodeMap hTLStreamNodeMapTRI = NULL;
			err = acDeviceGetTLStreamNodeMap(hDeviceTRI, &hTLStreamNodeMapTRI);
			CHECK_RETURN;

			// enable stream auto negotiate packet size
			err = acNodeMapSetBooleanValue(hTLStreamNodeMapTRI, "StreamAutoNegotiatePacketSize", true);
			CHECK_RETURN;

			// enable stream packet resend
			err = acNodeMapSetBooleanValue(hTLStreamNodeMapTRI, "StreamPacketResendEnable", true);
			CHECK_RETURN;
		}
		else if (IsApplicableDeviceTriton(pBuf))
		{
			fprintf(stderr, "Too many Triton device connected\n");
			exit(EXIT_FAILURE);
		}
		else if (!hDeviceHLT && IsApplicableDeviceHelios2(pBuf))
		{
			err = acSystemCreateDevice(hSystem, i, &hDeviceHLT);
			CHECK_RETURN;

			acNodeMap hTLStreamNodeMapHLT = NULL;
			err = acDeviceGetTLStreamNodeMap(hDeviceHLT, &hTLStreamNodeMapHLT);
			CHECK_RETURN;

			// enable stream auto negotiate packet size
			err = acNodeMapSetBooleanValue(hTLStreamNodeMapHLT, "StreamAutoNegotiatePacketSize", true);
			CHECK_RETURN;

			// enable stream packet resend
			err = acNodeMapSetBooleanValue(hTLStreamNodeMapHLT, "StreamPacketResendEnable", true);
			CHECK_RETURN;
		}
		else if (IsApplicableDeviceHelios2(pBuf))
		{
			fprintf(stderr, "Too many Helios2 device connected\n");
			exit(EXIT_FAILURE);
		}
	}

	if (!hDeviceTRI)
	{
		fprintf(stderr, "No applicable Triton devices\n");
		exit(EXIT_FAILURE);
	}
	if (!hDeviceHLT)
	{
		fprintf(stderr, "No applicable Helios2 devices\n");
		exit(EXIT_FAILURE);
	}

	// run example
	if (hDeviceTRI && hDeviceHLT)
	{
		printf("Commence example\n\n");
		err = OverlayColorOnto3DAndSave(hDeviceTRI, hDeviceHLT);
		CHECK_RETURN;
		printf("\nExample complete\n");
	}

	// clean up example
	if (hDeviceTRI)
	{
		err = acSystemDestroyDevice(hSystem, hDeviceTRI);
		CHECK_RETURN;
	}
	if (hDeviceHLT)
	{
		err = acSystemDestroyDevice(hSystem, hDeviceHLT);
		CHECK_RETURN;
	}

	err = acCloseSystem(hSystem);
	CHECK_RETURN;

	fclose(ifile);

	printf("Press enter to complete\n");
	while (getchar() != '\n') {};
	getchar();
	return -1;
}
