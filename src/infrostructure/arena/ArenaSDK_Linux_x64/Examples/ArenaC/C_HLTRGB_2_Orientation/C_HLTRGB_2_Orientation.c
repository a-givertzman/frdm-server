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
#define TAB3 "      "

 // Helios RGB: Orientation
 //    This example is part 2 of a 3-part example on color overlay over 3D images.
 //    Color data can be overlaid over 3D images by reading the 
 //    3D points ABC (XYZ) from the Helios and projecting them onto
 //    the Triton color (RGB) camera directly. This requires first solving for the
 //    orientation of the Helios coordinate system relative to the Triton's
 //    native coordinate space (rotation and translation wise). This step can be
 //    achieved by using the open function solvePnP(). Solving for orientation of
 //    the Helios relative to the Triton requires a single image of the
 //    calibration target from each camera. Place the calibration target near the
 //    center of both cameras field of view and at an appropriate distance from
 //    the cameras. Make sure the calibration target is placed at the same
 //    distance you will be imaging in your application. Make sure not to move the
 //    calibration target or cameras in between grabbing the Helios image and
 //    grabbing the Triton image.

// =-=-=-=-=-=-=-=-=-
// =-=- SETTINGS =-=-
// =-=-=-=-=-=-=-=-=-

// image timeout
#define TIMEOUT 2000

// calibration values file name
#define FILE_NAME_IN "tritoncalibration.yml"

// orientation values file name
#define FILE_NAME_OUT "orientation.yml"

// maximum buffer length
#define MAX_BUF 1024

// Please set to 1 for more verbose debugging printouts
#define VERBOSE_PRINTOUT 0

// =-=-=-=-=-=-=-=-=-
// =-=- HELPERS -=-=-
// =-=-=-=-=-=-=-=-=-

// helper function
AC_ERROR GetImageHLT(acDevice hDeviceHLT, CvMat** intensity_image, CvMat** xyz_mm)
{
	AC_ERROR err = AC_ERR_SUCCESS;

	acNodeMap hNodeMapHLT= NULL;
	err = acDeviceGetNodeMap(hDeviceHLT, &hNodeMapHLT);
	if (err != AC_ERR_SUCCESS)
		return err;

	err = acNodeMapSetEnumerationValue(hNodeMapHLT, "PixelFormat", "Coord3D_ABCY16");
	if (err != AC_ERR_SUCCESS)
		return err;

	// get stream node map
	acNodeMap hTLStreamNodeMap = NULL;
	err = acDeviceGetTLStreamNodeMap(hDeviceHLT, &hTLStreamNodeMap);
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

	// Read the scale factor and offsets to convert from unsigned 16-bit values 
	//    in the Coord3D_ABCY16 pixel format to coordinates in mm
	double xyz_scale_mm = 0.0;
	err = acNodeMapGetFloatValue(hNodeMapHLT, "Scan3dCoordinateScale", &xyz_scale_mm);
	if (err != AC_ERR_SUCCESS) 
		return err;
	err = acNodeMapSetStringValue(hNodeMapHLT, "Scan3dCoordinateSelector", "CoordinateA");
	if (err != AC_ERR_SUCCESS)
		return err;

	double x_offset_mm = 0.0;
	err = acNodeMapGetFloatValue(hNodeMapHLT, "Scan3dCoordinateOffset", &x_offset_mm);
	if (err != AC_ERR_SUCCESS)
		return err;
	err = acNodeMapSetStringValue(hNodeMapHLT, "Scan3dCoordinateSelector", "CoordinateB");
	if (err != AC_ERR_SUCCESS)
		return err;

	double y_offset_mm = 0.0;
	err = acNodeMapGetFloatValue(hNodeMapHLT, "Scan3dCoordinateOffset", &y_offset_mm);
	if (err != AC_ERR_SUCCESS)
		return err;
	err = acNodeMapSetStringValue(hNodeMapHLT, "Scan3dCoordinateSelector", "CoordinateC");
	if (err != AC_ERR_SUCCESS)
		return err;

	double z_offset_mm = 0.0;
	err = acNodeMapGetFloatValue(hNodeMapHLT, "Scan3dCoordinateOffset", &z_offset_mm);
	if (err != AC_ERR_SUCCESS)
		return err;

	// start stream
	err = acDeviceStartStream(hDeviceHLT);
	if (err != AC_ERR_SUCCESS)
		return err;

	// get image
	acBuffer hBuffer = NULL;
	err = acDeviceGetBuffer(hDeviceHLT, TIMEOUT, &hBuffer);
	if (err != AC_ERR_SUCCESS)
		return err;

	size_t width = 0;
	size_t height = 0;
	err = acImageGetWidth(hBuffer, &width);
	if (err != AC_ERR_SUCCESS)
		return err;

	err = acImageGetHeight(hBuffer, &height);
	if (err != AC_ERR_SUCCESS)
		return err;

	*xyz_mm = cvCreateMat((int)height, (int)width, CV_32FC3);
	*intensity_image = cvCreateMat((int)height, (int)width, CV_16UC1);

	uint8_t* input_data = NULL;
	err = acImageGetData(hBuffer, &input_data);
	if (err != AC_ERR_SUCCESS)
		return err;

	for (unsigned int ir = 0; ir < height; ++ir)
	{
		for (unsigned int ic = 0; ic < width; ++ic)
		{
			// Get unsigned 16 bit values for X,Y,Z coordinates
			ushort x_u16 = (input_data[0] << 8) | input_data[1];
			ushort y_u16 = (input_data[2] << 8) | input_data[3];
			ushort z_u16 = (input_data[4] << 8) | input_data[5];

			// Convert 16-bit X,Y,Z to float values in mm and store them in xyz_mm
			float* pXyzMm = ((float*)((*xyz_mm)->data.ptr + (*xyz_mm)->step * ir));
			pXyzMm[ic * 3] = (float)(x_u16 * xyz_scale_mm + x_offset_mm);
			pXyzMm[ic * 3 + 1] = (float)(y_u16 * xyz_scale_mm + y_offset_mm);
			pXyzMm[ic * 3 + 2] = (float)(z_u16 * xyz_scale_mm + z_offset_mm);

			ushort intensity = (input_data[6] << 8) | input_data[7];
			((ushort*)((*intensity_image)->data.ptr + (*intensity_image)->step * ir))[ic] = intensity;

			input_data += 8;
		}
	}

	// clean up
	input_data = NULL;
	
	err = acDeviceRequeueBuffer(hDeviceHLT, hBuffer) | acDeviceStopStream(hDeviceHLT);
	if (err != AC_ERR_SUCCESS)
		return err;

	return err;
}

AC_ERROR GetImageTRI(acDevice hDeviceTRI, CvMat** triton_image)
{
	AC_ERROR err = AC_ERR_SUCCESS;

	acNodeMap hNodeMapTRI = NULL;
	err = acDeviceGetNodeMap(hDeviceTRI, &hNodeMapTRI);
	if (err != AC_ERR_SUCCESS)
		return err;

	err = acNodeMapSetEnumerationValue(hNodeMapTRI, "PixelFormat", "RGB8");
	if (err != AC_ERR_SUCCESS)
		return err;

	// get stream node map
	acNodeMap hTLStreamNodeMap = NULL;
	err = acDeviceGetTLStreamNodeMap(hDeviceTRI, &hTLStreamNodeMap);
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
	err = acDeviceStartStream(hDeviceTRI);
	if (err != AC_ERR_SUCCESS)
		return err;

	// get image
	acBuffer hBuffer = NULL;
	err = acDeviceGetBuffer(hDeviceTRI, TIMEOUT, &hBuffer);
	if (err != AC_ERR_SUCCESS)
		return err;

	// convert Triton image to mono for dot finding
	acBuffer pConvert = NULL;
	err = acImageFactoryConvert(hBuffer, PFNC_Mono8, &pConvert);
	err = acDeviceGetBuffer(hDeviceTRI, TIMEOUT, &hBuffer);
	if (err != AC_ERR_SUCCESS)
		return err;

	size_t width = 0;
	size_t height = 0;
	err = acImageGetWidth(hBuffer, &width);
	if (err != AC_ERR_SUCCESS)
		return err;

	err = acImageGetHeight(hBuffer, &height);
	if (err != AC_ERR_SUCCESS)
		return err;

	*triton_image = cvCreateMat((int)height, (int)width, CV_8UC1);

	uint8_t* convert_data = NULL;
	err = acImageGetData(pConvert, &convert_data);
	if (err != AC_ERR_SUCCESS)
		return err;

	memcpy((*triton_image)->data.ptr, convert_data, height * width);

	// clean up
	convert_data = NULL;
	err = acImageFactoryDestroy(pConvert);
	if (err != AC_ERR_SUCCESS)
		return err;

	err = acDeviceRequeueBuffer(hDeviceTRI, hBuffer) | acDeviceStopStream(hDeviceTRI);
	if (err != AC_ERR_SUCCESS)
		return err;

	return err;
}

void findCalibrationPointsHLT(CvMat* image_in, CvSeq* grid_centers)
{
	// Ensure the input image is 8 - bit
	int channels = CV_MAT_CN(image_in->type);
	IplImage* image_8bit = cvCreateImage(cvGetSize(image_in), IPL_DEPTH_8U, channels);
	double minVal, maxVal;
	cvMinMaxLoc(image_in, &minVal, &maxVal, NULL, NULL, NULL);
	cvConvertScale(image_in, image_8bit, 255.0 / maxVal, -minVal * 255.0 / maxVal);

	// Simple blob detection (Note: This is not equivalent to SimpleBlobDetector in C++)
	CvMemStorage* storage = cvCreateMemStorage(0);
	IplImage* tempImage = cvCloneImage(image_8bit);

	// To adjust threshold, uncomment the visualization block below before processing the image.
	/*cvNamedWindow("Thresholded", CV_WINDOW_AUTOSIZE);
	cvShowImage("Thresholded", tempImage);
	cvWaitKey(0);
	cvDestroyWindow("Thresholded");*/

	// Image Processing
	cvSmooth(tempImage, tempImage, CV_GAUSSIAN,
		9, 9, // Adjust these kernel size values (9x9) to control the blurring effect.
		0, 0);

	cvThreshold(tempImage, tempImage,
		80,  // Adjust this threshold value (80) to control the binary segmentation sensitivity.
		255, CV_THRESH_BINARY);

	// To adjust threshold, uncomment the visualization block below to display the thresholded image for inspection.
	/*cvNamedWindow("Thresholded", CV_WINDOW_AUTOSIZE);
	cvShowImage("Thresholded", tempImage);
	cvWaitKey(0);
	cvDestroyWindow("Thresholded");*/

	CvSeq* contours;
	cvFindContours(tempImage, storage, &contours, sizeof(CvContour), CV_RETR_LIST, CV_CHAIN_APPROX_SIMPLE, cvPoint(0, 0));

	while (contours) {
		double area = cvContourArea(contours, CV_WHOLE_SEQ, 0);
		CvRect rect = cvBoundingRect(contours, 0);
		double aspect_ratio = (double)rect.width / rect.height;

		if (area > 10 && area < 1000 && aspect_ratio > 0.9 && aspect_ratio < 1.1) {
			CvRect rect = cvBoundingRect(contours, 0);
			CvPoint2D32f center = cvPoint2D32f(rect.x + rect.width / 2.0f, rect.y + rect.height / 2.0f);
			cvSeqPush(grid_centers, &center);
		}
		contours = contours->h_next;
	}
	
	cvReleaseImage(&tempImage);
	cvReleaseMemStorage(&storage);
	cvReleaseImage(&image_8bit);

#if defined(VERBOSE_PRINTOUT)
	printf("%sNumber of HLT Circles found: %d'\n", TAB1, grid_centers->total);
#endif
}

void findCalibrationPointsTRI(CvMat* image_in_orig, CvSeq* grid_centers)
{
	float scaling = 1.0;

	int channels = CV_MAT_CN(image_in_orig->type);
	IplImage* image_in = cvCreateImageHeader(cvSize(image_in_orig->cols, image_in_orig->rows), IPL_DEPTH_8U, channels);
	cvGetImageRawData(image_in_orig, (uchar**)(&(image_in->imageData)), NULL, NULL);

	bool is_found = false;
	double scaled_nrows = 2400.0;
	while (!is_found && scaled_nrows >= 100)
	{
		scaled_nrows /= 2.0;
		scaling = (float)((double)image_in_orig->rows / scaled_nrows);

		IplImage* resizedImage = cvCreateImage(cvSize((int)((double)image_in_orig->cols / scaling), (int)((double)image_in_orig->rows / scaling)), image_in->depth, image_in->nChannels);
		cvResize(image_in_orig, resizedImage, CV_INTER_LINEAR);

		IplImage* grayImage = cvCreateImage(cvGetSize(resizedImage), 8, 1);
		if (resizedImage->nChannels == 3)
		{
			cvCvtColor(resizedImage, grayImage, CV_BGR2GRAY);
		}
		else
		{
			cvCopy(resizedImage, grayImage, NULL);
		}

		// To adjust threshold, uncomment the visualization block below before processing the image.
		/*cvNamedWindow("Thresholded", CV_WINDOW_AUTOSIZE);
		cvShowImage("Thresholded", grayImage);
		cvWaitKey(0);
		cvDestroyWindow("Thresholded");*/

		cvSmooth(grayImage, grayImage, CV_GAUSSIAN, 
			9, 9, // Adjust these kernel size values (9x9) to control the blurring effect.
			0, 0);
		cvThreshold(grayImage, grayImage, 
			100, // Adjust this threshold value (100) to control the binary segmentation sensitivity.
			255, CV_THRESH_BINARY_INV);

		// To adjust threshold, uncomment the visualization block below to display the thresholded image for inspection.
		/*cvNamedWindow("Thresholded", CV_WINDOW_AUTOSIZE);
		cvShowImage("Thresholded", grayImage);
		cvWaitKey(0);
		cvDestroyWindow("Thresholded");*/

		CvMemStorage* storage = cvCreateMemStorage(0);
		CvSeq* contours = 0;
		cvFindContours(grayImage, storage, &contours, sizeof(CvContour), CV_RETR_LIST, CV_CHAIN_APPROX_SIMPLE, cvPoint(0, 0));

		// white circles in the calibration target
		// Filter the contours based on blob-like properties.
		CvSeq* current_contour = contours;
		while (current_contour != NULL)
		{
			// Filter by area, circularity etc.
			double area = cvContourArea(current_contour, CV_WHOLE_SEQ, 0);
			CvRect rect = cvBoundingRect(current_contour, 0);
			double aspect_ratio = (double)rect.width / rect.height;

			if (area > 500 && aspect_ratio > 0.9 && aspect_ratio < 1.1)
			{
				CvPoint2D32f center;
				float radius;
				cvMinEnclosingCircle(current_contour, &center, &radius);
				cvSeqPush(grid_centers, &center);
			}

			current_contour = current_contour->h_next;
		}

		is_found = (grid_centers->total != 0);

		cvReleaseImage(&grayImage);
		cvReleaseMemStorage(&storage);

		if (!is_found) {
			cvClearSeq(grid_centers);
		}

		cvReleaseImage(&resizedImage);
	}

	for (int i = 0; i < grid_centers->total; ++i)
	{
		CvPoint2D32f* pt = (CvPoint2D32f*)cvGetSeqElem(grid_centers, i);
		pt->x *= scaling;
		pt->y *= scaling;
	}

	cvReleaseImageHeader(&image_in);

#if defined(VERBOSE_PRINTOUT)
	printf("%sNumber of TRI Circles found: %d'\n", TAB1, grid_centers->total);
#endif
}

// =-=-=-=-=-=-=-=-=-
// =-=- EXAMPLE -=-=-
// =-=-=-=-=-=-=-=-=-

AC_ERROR CalculateAndSaveOrientationValues(acDevice hDeviceTRI, acDevice hDeviceHLT)
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

	// Read in camera matrix and distance coefficients
	printf("%sRead camera matrix and distance coefficients from file '%s'\n", TAB1, FILE_NAME_IN);

	CvFileStorage* fs = cvOpenFileStorage(FILE_NAME_IN, NULL, CV_STORAGE_READ, NULL);
	CvMat* cameraMatrix;
	CvMat* distCoeffs;

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

	cvReleaseFileStorage(&fs);

	// Get an image from Helios2
	printf("%sGet and prepare HLT image\n", TAB1);

	CvMat* imageMatrixHLTIntensity = NULL;
	CvMat* imageMatrixHLTXYZ = NULL;

	err = GetImageHLT(hDeviceHLT, &imageMatrixHLTIntensity, &imageMatrixHLTXYZ);
	if (err != AC_ERR_SUCCESS)
		return err;

	// Get an image from Triton
	printf("%sGet and prepare TRI image\n", TAB1);

	CvMat* imageMatrixTRI = NULL;
	err = GetImageTRI(hDeviceTRI, &imageMatrixTRI);
	if (err != AC_ERR_SUCCESS)
		return err;

	// Calculate orientation values
	printf("%sCalculate orientation values\n", TAB1);

	CvMemStorage* gridStorageHLT = cvCreateMemStorage(0);
	CvSeq* gridCentersHLT = cvCreateSeq(CV_32FC2, sizeof(CvSeq), sizeof(CvPoint2D32f), gridStorageHLT);

	CvMemStorage* gridStorageTRI = cvCreateMemStorage(0);
	CvSeq* gridCentersTRI = cvCreateSeq(CV_32FC2, sizeof(CvSeq), sizeof(CvPoint2D32f), gridStorageTRI);


	// find HLT calibration points using HLT intensity image
	printf("%sFind points in HLT image\n", TAB2);

	size_t numTriesHLT = 0;

	while (gridCentersHLT->total != 20)
	{
		if (numTriesHLT <= 20)
		{
			cvReleaseMemStorage(&gridStorageHLT);

			gridStorageHLT = cvCreateMemStorage(0);
			CvSeq* gridCentersHLT = cvCreateSeq(CV_32FC2, sizeof(CvSeq), sizeof(CvPoint2D32f), gridStorageHLT);

			err = GetImageHLT(hDeviceHLT, &imageMatrixHLTIntensity, &imageMatrixHLTXYZ);
			if (err != AC_ERR_SUCCESS)
				return err;

			findCalibrationPointsHLT(imageMatrixHLTIntensity, gridCentersHLT);
			
			numTriesHLT++;
		}
		else
		{
			fprintf(stderr, "Unable to find points in HLT intensity image\n");
			exit(EXIT_FAILURE);
		}
		
	}

	// find TRI calibration points 
	printf("%sFind points in TRI image\n", TAB2);

	size_t numTriesTRI = 0;

	while (gridCentersTRI->total != 20)
	{
		if (numTriesTRI <= 20)
		{
			cvReleaseMemStorage(&gridStorageTRI);

			gridStorageTRI = cvCreateMemStorage(0);
			CvSeq* gridCentersTRI = cvCreateSeq(CV_32FC2, sizeof(CvSeq), sizeof(CvPoint2D32f), gridStorageTRI);

			err = GetImageTRI(hDeviceTRI, &imageMatrixTRI);
			if (err != AC_ERR_SUCCESS)
				return err;

			findCalibrationPointsTRI(imageMatrixTRI, gridCentersTRI);

			numTriesTRI++;
		}
		else
		{
			fprintf(stderr, "Unable to find points in TRI image\n");
			exit(EXIT_FAILURE);	
		}
	}

	// prepare for PnP
	printf("%sPrepare for PnP\n", TAB2);

	CvMat* targetPoints3Dmm = cvCreateMat(gridCentersHLT->total, 3, CV_32FC1);
	CvMat* targetPoints2DPixels = cvCreateMat(gridCentersTRI->total, 2, CV_32FC1);

	for (int i = 0; i < gridCentersHLT->total; ++i) {
		CvPoint2D32f ptHLT = *(CvPoint2D32f*)cvGetSeqElem(gridCentersHLT, i);
		CvPoint2D32f ptTRI = *(CvPoint2D32f*)cvGetSeqElem(gridCentersTRI, i);

		unsigned int c1 = (unsigned int)round(ptHLT.x);
		unsigned int r1 = (unsigned int)round(ptHLT.y);

		CvScalar xyz = cvGet2D(imageMatrixHLTXYZ, r1, c1);
		float x = (float)xyz.val[0];
		float y = (float)xyz.val[1];
		float z = (float)xyz.val[2];

		cvmSet(targetPoints3Dmm, i, 0, x);
		cvmSet(targetPoints3Dmm, i, 1, y);
		cvmSet(targetPoints3Dmm, i, 2, z);

		printf("%sPoint %d: [%.2f, %.2f, %.2f]\n", TAB3, i, x, y, z);

		cvmSet(targetPoints2DPixels, i, 0, ptTRI.x);
		cvmSet(targetPoints2DPixels, i, 1, ptTRI.y);
	}

	CvMat* rotationVector = cvCreateMat(3, 1, CV_32FC1);
	CvMat* translationVector = cvCreateMat(3, 1, CV_32FC1);

	cvFindExtrinsicCameraParams2(targetPoints3Dmm,
		targetPoints2DPixels,
		cameraMatrix,
		distCoeffs,
		rotationVector,
		translationVector,
		0);

	printf(TAB2 "Orientation processing completed\n");

	// Save orientation information
	printf(TAB1 "Save camera matrix, distance coefficients, and rotation and translation vectors to file '%s'\n", FILE_NAME_OUT);

	CvFileStorage* fs2 = cvOpenFileStorage(FILE_NAME_OUT, 0, CV_STORAGE_WRITE, NULL);
	if (fs2)
	{
		cvWrite(fs2, "cameraMatrix", cameraMatrix, cvAttrList(0, 0));
		cvWrite(fs2, "distCoeffs", distCoeffs, cvAttrList(0, 0));
		cvWrite(fs2, "rotationVector", rotationVector, cvAttrList(0, 0));
		cvWrite(fs2, "translationVector", translationVector, cvAttrList(0, 0));
		cvReleaseFileStorage(&fs2);
	}

	// return nodes to their initial values
	err = acNodeMapSetStringValue(hNodeMapTRI, "PixelFormat", pPixelFormatInitialTRI);
	if (err != AC_ERR_SUCCESS)
		return err;

	err = acNodeMapSetStringValue(hNodeMapHLT, "PixelFormat", pPixelFormatInitialHLT);
	if (err != AC_ERR_SUCCESS)
		return err;

	// clean up
	cvReleaseMat(&targetPoints3Dmm);
	cvReleaseMat(&targetPoints2DPixels);
	cvReleaseMat(&rotationVector);
	cvReleaseMat(&translationVector);
	
	cvReleaseMemStorage(&gridStorageHLT);
	cvReleaseMemStorage(&gridStorageTRI);

	if (imageMatrixHLTIntensity != NULL) {
		cvReleaseMat(&imageMatrixHLTIntensity);
	}
	if (imageMatrixHLTXYZ != NULL) {
		cvReleaseMat(&imageMatrixHLTXYZ);
	}
	if (imageMatrixTRI != NULL) {
		cvReleaseMat(&imageMatrixTRI);
	}

	cvReleaseMat(&cameraMatrix);
	cvReleaseMat(&distCoeffs);

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
	printf("C_HLTRGB_2_Orientation\n");
	
	// Open the file
	FILE* ifile = NULL;
	int error;
	error = fopen_portable(FILE_NAME_IN, "r", &ifile);
	if (error != 0 || ifile == NULL)
	{
		printf("File '%s' not found\nPlease run example 'C_HLTRGB_1_Calibration' prior to this one\nPress enter to complete\n", FILE_NAME_IN);
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
		err = CalculateAndSaveOrientationValues(hDeviceTRI, hDeviceHLT);
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
