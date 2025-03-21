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


#if defined _WIN32
#include <windows.h>
#elif defined linux
#include <unistd.h>
#endif

#if defined (_WIN32)
#define portable_sleep(x) Sleep(x * 1000)
#elif defined (__linux__)
#define portable_sleep(x) sleep(x)
#endif


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

// Helios RGB: TritonCalibration
//    This example is part 1 of a 3-part example on color overlay over 3D images.
//    Before the data between the two cameras can be combined,
//    we must first calibrate the lens on the Triton color camera to find its
//    optical center and focal length (intrinsics), and lens distortion
//    coefficients (pinhole model). We can achieve this by printing a target with
//    a checkerboard pattern or you can download our calibration target here
//    (15kB, PDF, 8.5 x 11 in)
//    https:arenasdk.s3-us-west-2.amazonaws.com/LUCID_target_whiteCircles.pdf
//    Before calibrating the Triton camera you must focus its lens. Place the
//    target at your application's working distance and focus the Triton's
//    lens so that the calibration target is in focus. Calibrating the Triton
//    camera requires grabbing several images of the calibration chart at
//    different positions within the camera's field of view. At least 3 images
//    are required but 4 to 8 images are typically used to get a better - quality
//    calibration.

// =-=-=-=-=-=-=-=-=-
// =-=- SETTINGS =-=-
// =-=-=-=-=-=-=-=-=-

// image timeout
#define TIMEOUT 200

// number of calibration points to compare
#define NUM_IMAGES 10

// calibration values file name
#define FILE_NAME "tritoncalibration.yml"

// time to sleep between images (in seconds)
#define SLEEP_MS 1

// maximum buffer length
#define MAX_BUF 1024

// =-=-=-=-=-=-=-=-=-
// =-=- EXAMPLE -=-=-
// =-=-=-=-=-=-=-=-=-

typedef enum
{
	NOT_EXISTING,
	CHESSBOARD,
	CIRCLES_GRID,
	ASYMMETRIC_CIRCLES_GRID
} Pattern;

typedef enum
{
	INVALID,
	CAMERA,
	VIDEO_FILE,
	IMAGE_LIST
} InputType;

typedef struct
{
	CvSize boardSize; // The size of the board -> Number of items by width and height
	Pattern calibrationPattern; // One of the Chessboard, circles, or asymmetric circle pattern
	float squareSize; // The size of a square in your defined unit (point, millimeter,etc).
	int nrFrames; // The number of frames to use from the input for calibration
	float aspectRatio; // The aspect ratio
	int delay; // In case of a video input
	bool writePoints; // Write detected feature points
	bool writeExtrinsics; // Write extrinsic parameters
	bool calibZeroTangentDist; // Assume zero tangential distortion
	bool calibFixPrincipalPoint; // Fix the principal point at the center
	bool flipVertical; // Flip the captured images around the horizontal axis
	char* outputFileName; // The name of the file where to write
	bool showUndistorsed; // Show undistorted images after calibration
	char* input; // The input ->
	bool useFisheye; // use fisheye camera model for calibration
	bool fixK1; // fix K1 distortion coefficient
	bool fixK2; // fix K2 distortion coefficient
	bool fixK3; // fix K3 distortion coefficient
	bool fixK4; // fix K4 distortion coefficient
	bool fixK5; // fix K5 distortion coefficient

	int cameraID;
	char** imageList; 
	size_t atImageList;
	CvCapture* inputCapture;
	InputType inputType;
	bool goodInput;
	int flag;
	char* patternToUse; 
} Settings;

bool findCalibrationPoints(CvMat* image_in_orig, CvSeq* grid_centers)
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
		scaling = (float) ((double)image_in_orig->rows / scaled_nrows);

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
		
		// Image Preprocessing:
		// 1. Apply Gaussian smoothing to reduce image noise and improve edge detection.
		//    We use a 9x9 kernel for this operation.
		// 2. Threshold the image to isolate target features. In the current configuration,
		//    pixel intensities below 80 are set to 255 (white) and those above 80 are set to 0 (black),
		//    effectively inverting the binary image.
		//
		// Note: The chosen threshold of 80 may need adjustment based on the specific imaging conditions
		// and desired features. It's recommended to visualize the thresholded image during initial tests
		// to fine-tune this value for optimal results.
		//
		cvSmooth(grayImage, grayImage, CV_GAUSSIAN, 9, 9, 0, 0);
		cvThreshold(grayImage, grayImage, 80, 255, CV_THRESH_BINARY_INV);

		// Uncomment the visualization block below to display the thresholded image for inspection.
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
	
	return is_found;
}

void calcBoardCornerPositions(CvSize boardSize, float squareSize, CvSeq* corners)
{
	cvClearSeq(corners);

	for (int i = 0; i < boardSize.height; ++i)
	{
		for (int j = 0; j < boardSize.width; ++j)
		{
			CvPoint3D32f point;
			point.x = j * squareSize;
			point.y = i * squareSize;
			point.z = 0;
			cvSeqPush(corners, &point);
		}
	}
}

void resizeObjectPointsToMatchImagePoints(CvSeq* objectPoints, CvSeq* imagePoints)
{
	CvMemStorage* storage = objectPoints->storage;

	CvSeq* firstInnerSeq = (objectPoints->total > 0) ? *(CvSeq**)cvGetSeqElem(objectPoints, 0) : NULL;

	int sizeDifference = imagePoints->total - objectPoints->total;

	if (sizeDifference > 0)
	{ 
		for (int i = 0; i < sizeDifference; i++)
		{
			if (firstInnerSeq)
			{
				CvSeq* newInnerSeq = cvCloneSeq(firstInnerSeq, storage);
				cvSeqPush(objectPoints, &newInnerSeq);
			}
			else
			{
				CvSeq* newEmptySeq = cvCreateSeq(0, sizeof(CvSeq), sizeof(CvPoint3D32f), storage);
				cvSeqPush(objectPoints, &newEmptySeq);
			}
		}
	}
	else if (sizeDifference < 0)
	{ 
		for (int i = 0; i < -sizeDifference; i++)
		{
			cvSeqPop(objectPoints, NULL); 
		}
	}
}

int checkRange(CvMat* mat)
{
	int i, j;
	for (i = 0; i < mat->rows; i++)
	{
		for (j = 0; j < mat->cols; j++)
		{
			float value = (float)cvGetReal2D(mat, i, j);
			if (isnan(value) || isinf(value))
			{
				return 0; // False, because NaN or Inf value found
			}
		}
	}
	return 1;
}

double computeReprojectionErrors(CvSeq* objectPoints, CvSeq* imagePoints, CvMat* rvecs, CvMat* tvecs, CvMat* cameraMatrix, CvMat* distCoeffs, float* perViewErrors)
{
	CvMat* imagePoints2 = cvCreateMat(20, 1, CV_32FC2);

	int totalPoints = 0;
	double totalErr = 0;
	double err = 0;
	
	int currentSize = NUM_IMAGES;
	int desiredSize = objectPoints->total;
	float* temp = (float*)realloc(perViewErrors, desiredSize * sizeof(float));
	
	if (currentSize < desiredSize)
	{
		if (temp != NULL)
		{
			for (int i = currentSize; i < desiredSize; i++)
			{
				temp[i] = 0.0f;
			}
			perViewErrors = temp;
		}
		else
		{
			free(perViewErrors);
			exit(-1);
		}
		
	}
	else if (currentSize > desiredSize)
	{
		if (temp != NULL)
		{
			perViewErrors = temp;
		}
		else
		{
			free(perViewErrors);
			exit(-1); 
		}
	}
	
	for (int i = 0; i < objectPoints->total; ++i)
	{
		CvSeq* currentObjectPoints = *((CvSeq**)cvGetSeqElem(objectPoints, i));
		CvMat* matFromSeq = cvCreateMat(currentObjectPoints->total, 3, CV_32FC1); 
		cvCvtSeqToArray(currentObjectPoints, matFromSeq->data.ptr, CV_WHOLE_SEQ);

		CvSeq* currentImagePoints = *((CvSeq**)cvGetSeqElem(imagePoints, i));

		CvMat tvec, rvec;
		cvGetRow(tvecs, &tvec, i);
		cvGetRow(rvecs, &rvec, i);
		
		cvProjectPoints2(matFromSeq, &rvec, &tvec, cameraMatrix, distCoeffs, imagePoints2, NULL, NULL, NULL, NULL, NULL, 0);

		err = cvNorm(currentImagePoints, imagePoints2, CV_L2, NULL);

		int n = currentObjectPoints->total;
		perViewErrors[i] = (float)sqrt(err * err / n);
		totalErr += err * err;
		totalPoints += n;

		cvReleaseMat(&matFromSeq);
	}

	double result = sqrt(totalErr / totalPoints);

	cvReleaseMat(&imagePoints2);

	return result;
}

bool calculate(Settings s, CvSize imageSize, CvMat** cameraMatrix, CvMat** distCoeffs, CvSeq* imagePoints, CvMat** rvecs, CvMat** tvecs, float* reprojErrs, double* totalAvgErr)
{
	// ! [fixed_aspect]
	*cameraMatrix = cvCreateMat(3, 3, CV_64F);
	cvSetIdentity(*cameraMatrix, cvRealScalar(1));
	if (s.flag & CV_CALIB_FIX_ASPECT_RATIO)
		cvmSet(*cameraMatrix, 0, 0, s.aspectRatio);

	
	*distCoeffs = cvCreateMat(8, 1, CV_64F);
	
	cvSetZero(*distCoeffs);

	// Using CvSeq to represent the list of points and CvMemStorage for memory storage
	CvMemStorage* storage = cvCreateMemStorage(0);
	CvSeq* objectPoints = cvCreateSeq(CV_32FC3, sizeof(CvSeq), sizeof(CvPoint3D32f), storage);

	// Add an empty list (for now). This simulates pushing an empty vector into objectPoints.
	CvSeq* corners = cvCreateSeq(CV_32FC3, sizeof(CvSeq), sizeof(CvPoint3D32f), storage);
	cvSeqPush(objectPoints, &corners);

	s.boardSize.height = 4;
	s.boardSize.width = 5;
	s.squareSize = 50;

	calcBoardCornerPositions(s.boardSize, s.squareSize, corners);

	resizeObjectPointsToMatchImagePoints(objectPoints, imagePoints);
	
	// find intrinsic and extrinsic camera parameters
	int numImages = objectPoints->total; // 10
	CvSeq* firstObjectPoints = *((CvSeq**)cvGetSeqElem(objectPoints, 0));
	int numCornersPerImage = firstObjectPoints->total; // 20

	CvMat* objectPointsMat = cvCreateMat(numImages * numCornersPerImage, 3, CV_32FC1);
	CvMat* imagePointsMat = cvCreateMat(numImages * numCornersPerImage, 2, CV_32FC1);
	CvMat* pointCountsMat = cvCreateMat(numImages, 1, CV_32SC1);

	for (int i = 0; i < numImages; i++)
	{
		CvSeq* objectSeq = *((CvSeq**)cvGetSeqElem(objectPoints, i));
		CvSeq* imageSeq = *((CvSeq**)cvGetSeqElem(imagePoints, i));

		for (int j = 0; j < numCornersPerImage; j++)
		{
			CvPoint3D32f* objPt = (CvPoint3D32f*)cvGetSeqElem(objectSeq, j);
			CvPoint2D32f* imgPt = (CvPoint2D32f*)cvGetSeqElem(imageSeq, j);

			CV_MAT_ELEM(*objectPointsMat, float, i* numCornersPerImage + j, 0) = objPt->x;
			CV_MAT_ELEM(*objectPointsMat, float, i* numCornersPerImage + j, 1) = objPt->y;
			CV_MAT_ELEM(*objectPointsMat, float, i* numCornersPerImage + j, 2) = objPt->z;

			CV_MAT_ELEM(*imagePointsMat, float, i* numCornersPerImage + j, 0) = imgPt->x;
			CV_MAT_ELEM(*imagePointsMat, float, i* numCornersPerImage + j, 1) = imgPt->y;
		}

		CV_MAT_ELEM(*pointCountsMat, int, i, 0) = numCornersPerImage;
	}

	CvTermCriteria term_crit = cvTermCriteria(CV_TERMCRIT_EPS | CV_TERMCRIT_ITER, 30, DBL_EPSILON);

	*rvecs = cvCreateMat(numImages, 3, CV_32F);
	*tvecs = cvCreateMat(numImages, 3, CV_32F);

	cvCalibrateCamera2(objectPointsMat, imagePointsMat, pointCountsMat, imageSize, *cameraMatrix, *distCoeffs, *rvecs, *tvecs, 0, term_crit);

	bool checkDist = checkRange(*distCoeffs);

	bool checkCam = checkRange(*cameraMatrix);
	
	bool ok = checkCam && checkDist;
	
	*totalAvgErr = computeReprojectionErrors(objectPoints, imagePoints, *rvecs, *tvecs, *cameraMatrix, *distCoeffs, reprojErrs);

	cvReleaseMat(&pointCountsMat);
	cvReleaseMat(&imagePointsMat);
	cvReleaseMat(&objectPointsMat);

	cvReleaseMemStorage(&storage);

	return ok;
}

AC_ERROR CalculateAndSaveCalibrationValues(acDevice hDevice)
{
	AC_ERROR err = AC_ERR_SUCCESS;

	// get node map
	acNodeMap hNodeMap = NULL;
	err = acDeviceGetNodeMap(hDevice, &hNodeMap);
	if (err != AC_ERR_SUCCESS)
		return err;

	// get AcquisitionMode values that will be changed in order to return their values at
	// the end of the example
	char pAcquisitionModeInitial[MAX_BUF];
	size_t pAcquisitionModeBufLen = MAX_BUF;

	err = acNodeMapGetStringValue(hNodeMap, "AcquisitionMode", pAcquisitionModeInitial, &pAcquisitionModeBufLen);
	if (err != AC_ERR_SUCCESS)
		return err;

	// get pixelFormat values that will be changed in order to return their values at
	// the end of the example
	char pixelFormatInitial[MAX_BUF];
	size_t pixelFormatBufLen = MAX_BUF;

	err = acNodeMapGetEnumerationValue(hNodeMap, "PixelFormat", pixelFormatInitial, &pixelFormatBufLen);
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

	// set pixel format
	printf("%sSet pixel format to 'Mono8'\n", TAB1);

	err = acNodeMapSetEnumerationValue(hNodeMap, "PixelFormat", "Mono8");
	if (err != AC_ERR_SUCCESS)
		return err;

	// set acquisition mode
	printf("%sSet acquisition mode to 'Continuous'\n", TAB1);

	err = acNodeMapSetStringValue(hNodeMap, "AcquisitionMode", "Continuous");
	if (err != AC_ERR_SUCCESS)
		return err;

	// set buffer handling mode
	printf("%sSet buffer handling mode to 'NewestOnly'\n", TAB1);
	err = acNodeMapSetStringValue(hTLStreamNodeMap, "StreamBufferHandlingMode", "NewestOnly");
	if (err != AC_ERR_SUCCESS)
		return err;

	// start stream
	err = acDeviceStartStream(hDevice);
	if (err != AC_ERR_SUCCESS)
		return err;

	// get sets of calibration points
	printf("%sGetting %d sets of calibration points\n", TAB1, NUM_IMAGES);
	printf("%sMove the calibration target around the frame for best results\n", TAB1);

	// CvSize patternSize = cvSize(5, 4);
	CvMemStorage* storage = cvCreateMemStorage(0);
	CvSeq* calibrationPoints = cvCreateSeq(0, sizeof(CvSeq), sizeof(CvSeq*), storage); 
	CvSize imageSize;
	size_t attempts = 0;
	size_t images = 0;
	size_t gridCentersFound = 0;
	size_t successes = 0;

	while (successes < NUM_IMAGES)
	{
		acBuffer hBuffer = NULL;
		
		// get image
		attempts++;
		err = acDeviceGetBuffer(hDevice, TIMEOUT, &hBuffer);
		images++;

		if (err == AC_ERR_TIMEOUT)
		{
			printf("%sIncomplete image", TAB2);
		}
		else if (err == AC_ERR_SUCCESS)
		{
			size_t w = 0;
			size_t h = 0;
			uint8_t* imageData = NULL;
			err = acImageGetWidth(hBuffer, &w) |
				  acImageGetHeight(hBuffer, &h) |
				  acImageGetData(hBuffer, &imageData);
			if (err != AC_ERR_SUCCESS)
				return err;
			imageSize.height = (int) h;
			imageSize.width = (int) w;

			// copy data into OpenCV matrix
			CvMat* imageMatrix = cvCreateMat(imageSize.height, imageSize.width, CV_8UC1);
			memset(imageMatrix->data.ptr, 0, w * h);
			memcpy(imageMatrix->data.ptr, imageData, w * h);

			err = acDeviceRequeueBuffer(hDevice, hBuffer);
			if (err != AC_ERR_SUCCESS)
				return err;

			// find calibration circles
			CvMemStorage* gridStorage = cvCreateMemStorage(0);
			CvSeq* gridCenters = cvCreateSeq(CV_32FC2, sizeof(CvSeq), sizeof(CvPoint2D32f), gridStorage);
			findCalibrationPoints(imageMatrix, gridCenters);
			gridCentersFound = gridCenters->total; 
			if (gridCentersFound == 20)
			{
				CvMemStorage* calibrationStorage = cvCreateMemStorage(0);
				CvSeq* gridCentersCopy = cvCloneSeq(gridCenters, calibrationStorage);
				cvSeqPush(calibrationPoints, &gridCentersCopy);
				successes++;
				printf("%sCalibration image has been successful.\n", TAB2);
			} 
			else 
			{
				printf("%sFound %zu circles. Please adjust the calibration target!\n", TAB2, gridCentersFound);
			}

			cvReleaseMemStorage(&gridStorage);
		}
		else
		{
			// on other errors, ignore and retry
		}

		printf("%s%zu attempts, %zu images, %zu circles found, %zu calibration points\n", TAB3, attempts, images, gridCentersFound, successes);
		printf("%sPlease move the dot-pattern to a new position and press Enter to continue.\r", TAB3);
		getchar();

		// sleep between images
		portable_sleep(SLEEP_MS);
	}

	// calculate camera matrix and distance coefficients
	printf("\n%sCalculate camera matrix and distance coefficients\n", TAB1);

	CvMat* cameraMatrix = cvCreateMat(3, 3, CV_64F);
	CvMat* distCoeffs = cvCreateMat(14, 1, CV_64F);
	Settings s;
	s.nrFrames = NUM_IMAGES;
	s.inputType = IMAGE_LIST;

	CvMat* rvecs = cvCreateMat(NUM_IMAGES, 3, CV_32F);
	CvMat* tvecs = cvCreateMat(NUM_IMAGES, 3, CV_32F);

	float* reprojErrs = (float*)malloc(NUM_IMAGES * sizeof(float));

	double totalAvgErr = 0;

	s.calibrationPattern = CIRCLES_GRID;
	s.flag = CV_CALIB_RATIONAL_MODEL;

	bool calculationSucceeded = calculate(s, imageSize, &cameraMatrix, &distCoeffs, calibrationPoints, &rvecs, &tvecs, reprojErrs, &totalAvgErr);

	printf("%sCalibration %s\n", TAB1, calculationSucceeded ? "succeeded" : "failed");
	printf("%sCalculated reprojection error is %f\n", TAB1, totalAvgErr);

	// save calibration information
	printf("%sSave camera matrix and distance coefficients to file '%s'\n", TAB1, FILE_NAME);

	CvFileStorage* fs = cvOpenFileStorage(FILE_NAME, NULL, CV_STORAGE_WRITE, NULL);
	if (fs) {
		cvWrite(fs, "cameraMatrix", cameraMatrix, cvAttrList(0, 0));
		cvWrite(fs, "distCoeffs", distCoeffs, cvAttrList(0, 0));
		cvReleaseFileStorage(&fs);
	}

	// Cleanup
	int m = calibrationPoints->total;
	for (int i = 0; i < m; i++)
	{
		CvSeq* innerSeq = *(CvSeq**)cvGetSeqElem(calibrationPoints, i);
		if (innerSeq && innerSeq->storage)
		{
			cvReleaseMemStorage(&(innerSeq->storage));
		}
	}
	cvClearSeq(calibrationPoints);

	// Release the outer sequence
	cvReleaseMemStorage(&storage);

	cvReleaseMat(&cameraMatrix);
	cvReleaseMat(&distCoeffs);
	cvReleaseMat(&rvecs);
	cvReleaseMat(&tvecs);

	memset(reprojErrs, 0, NUM_IMAGES * sizeof(float));
	reprojErrs = NULL;
	
	// stop stream
	err = acDeviceStopStream(hDevice);
	if (err != AC_ERR_SUCCESS)
		return err;

	// return nodes to their initial values
	err = acNodeMapSetEnumerationValue(hNodeMap, "PixelFormat", pixelFormatInitial);
	if (err != AC_ERR_SUCCESS)
		return err;

	err = acNodeMapSetStringValue(hNodeMap, "AcquisitionMode", pAcquisitionModeInitial);
	if (err != AC_ERR_SUCCESS)
		return err;

	return err;
}

int IsApplicableDevice(char* pBuf)
{
	if (strstr(pBuf, "TRI") != NULL && strstr(pBuf, "-C") != NULL)
		return 1;
	else
		return 0;
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
	printf("C_HLTRGB_1_Calibration\n");
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
	acDevice hDevice = NULL;
	size_t i = 0;

	for (i = 0; i < numDevices; i++)
	{
		// get and display model name
		char pBuf[MAX_BUF];
		size_t len = MAX_BUF;

		err = acSystemGetDeviceModel(hSystem, i, pBuf, &len);
		if (err != AC_ERR_SUCCESS)
			return err;
		
		if (IsApplicableDevice(pBuf))
		{
			err = acSystemCreateDevice(hSystem, i, &hDevice);
			CHECK_RETURN;
		}
	}

	if (hDevice)
	{
		// run example
		printf("Commence example\n\n");
		err = CalculateAndSaveCalibrationValues(hDevice);
		CHECK_RETURN;
		printf("\nExample complete\n");

		// clean up example
		err = acSystemDestroyDevice(hSystem, hDevice);
		CHECK_RETURN;
	}

	err = acCloseSystem(hSystem);
	CHECK_RETURN;

	printf("Press enter to complete\n");
	while (getchar() != '\n') {};
	getchar();
	return -1;
}
