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
#include "ArenaApi.h"
#include "SaveApi.h"

#define TAB1 "  "
#define TAB2 "    "
#define TAB3 "      "

#include <iostream>
#include <fstream>
#include <thread>
#include <chrono>

#include <opencv2/opencv.hpp>
#include <opencv2/core/mat.hpp>
#include <opencv2/calib3d/calib3d.hpp>
#include <opencv2/imgproc/imgproc.hpp>
#include <opencv2/highgui/highgui.hpp>
#include <opencv2/core/version.hpp>

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

// time to sleep between images (in milliseconds)
#define SLEEP_MS 1000

// =-=-=-=-=-=-=-=-=-
// =-= HELPERS =-=-=-
// =-=-=-=-=-=-=-=-=-

// helper class
class Settings
{
public:
	Settings() :
		goodInput(false)
	{
	}

	enum Pattern
	{
		NOT_EXISTING,
		CHESSBOARD,
		CIRCLES_GRID,
		ASYMMETRIC_CIRCLES_GRID
	};
	enum InputType
	{
		INVALID,
		CAMERA,
		VIDEO_FILE,
		IMAGE_LIST
	};

	cv::Size boardSize; // The size of the board -> Number of items by width and height
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
	std::string outputFileName; // The name of the file where to write
	bool showUndistorsed; // Show undistorted images after calibration
	std::string input; // The input ->
	bool useFisheye = false; // use fisheye camera model for calibration
	bool fixK1; // fix K1 distortion coefficient
	bool fixK2; // fix K2 distortion coefficient
	bool fixK3; // fix K3 distortion coefficient
	bool fixK4; // fix K4 distortion coefficient
	bool fixK5; // fix K5 distortion coefficient

	int cameraID;
	std::vector<std::string> imageList;
	size_t atImageList;
	//cv::VideoCapture inputCapture;
	InputType inputType;
	bool goodInput;
	int flag;

private:
	std::string patternToUse;
};

// helper function
bool findCalibrationPoints(const cv::Mat& image_in_orig, std::vector<cv::Point2f>& grid_centers)
{
	float scaling = 1.0;
	cv::Mat image_in = image_in_orig;

	cv::SimpleBlobDetector::Params bright_params;
	bright_params.filterByColor = true;
	bright_params.blobColor = 255; // white circles in the calibration target
	bright_params.filterByCircularity = true;
	bright_params.minCircularity = 0.8f;

#if defined(CV_VERSION_EPOCH) && CV_VERSION_EPOCH == 2
	cv::Ptr<cv::SimpleBlobDetector> blob_detector(new cv::SimpleBlobDetector(bright_params));
#elif CV_VERSION_MAJOR >= 3
	cv::Ptr<cv::SimpleBlobDetector> blob_detector = cv::SimpleBlobDetector::create(bright_params);
#endif

	// pattern_size(num_cols, num_rows) num_cols: number of columns (number of
	// circles in a row) of the calibration target viewed by the camera num_rows:
	// number of rows (number of circles in a column) of the calibration target
	// viewed by the camera Specify according to the orientation of the
	// calibration target
	cv::Size pattern_size(5, 4);

	bool is_found = cv::findCirclesGrid(image_in, pattern_size, grid_centers, 1, blob_detector);

	double scaled_nrows = 2400.0;
	while (!is_found && scaled_nrows >= 100)
	{
		scaled_nrows /= 2.0;
		scaling = static_cast<float>((double)image_in_orig.rows / scaled_nrows);
		cv::resize(image_in_orig, image_in, cv::Size(static_cast<int>((double)image_in_orig.cols / scaling), static_cast<int>((double)image_in_orig.rows / scaling)));

		is_found = cv::findCirclesGrid(image_in, pattern_size, grid_centers, 1, blob_detector);
	}

	// Scale back the grid centers
	for (unsigned int i = 0; i < grid_centers.size(); ++i)
	{
		grid_centers[i].x = grid_centers[i].x * scaling;
		grid_centers[i].y = grid_centers[i].y * scaling;
	}

	return is_found;
}

// helper function
static double computeReprojectionErrors(
	const std::vector<std::vector<cv::Point3f>>& objectPoints,
	const std::vector<std::vector<cv::Point2f>>& imagePoints,
	const std::vector<cv::Mat>& rvecs,
	const std::vector<cv::Mat>& tvecs,
	const cv::Mat& cameraMatrix,
	const cv::Mat& distCoeffs,
	std::vector<float>& perViewErrors,
	bool fisheye)
{
	std::vector<cv::Point2f> imagePoints2;
	size_t totalPoints = 0;
	double totalErr = 0, err;
	perViewErrors.resize(objectPoints.size());

	for (size_t i = 0; i < objectPoints.size(); ++i)
	{
		if (fisheye)
		{
			cv::fisheye::projectPoints(objectPoints[i], imagePoints2, rvecs[i], tvecs[i], cameraMatrix, distCoeffs);
		}
		else
		{
			projectPoints(objectPoints[i], rvecs[i], tvecs[i], cameraMatrix, distCoeffs, imagePoints2);
		}
		err = norm(imagePoints[i], imagePoints2, cv::NORM_L2);

		size_t n = objectPoints[i].size();
		perViewErrors[i] = (float)std::sqrt(err * err / n);
		totalErr += err * err;
		totalPoints += n;
	}

	return std::sqrt(totalErr / totalPoints);
}

// helper function
static void calcBoardCornerPositions(cv::Size boardSize, float squareSize, std::vector<cv::Point3f>& corners, Settings::Pattern patternType)
{
	corners.clear();

	for (int i = 0; i < boardSize.height; ++i)
		for (int j = 0; j < boardSize.width; ++j)
			corners.push_back(cv::Point3f(j * squareSize, i * squareSize, 0));
}

// helper function
static bool calculate(Settings& s, cv::Size& imageSize, cv::Mat& cameraMatrix, cv::Mat& distCoeffs, std::vector<std::vector<cv::Point2f>> imagePoints, std::vector<cv::Mat>& rvecs, std::vector<cv::Mat>& tvecs, std::vector<float>& reprojErrs, double& totalAvgErr)
{
	//! [fixed_aspect]
	cameraMatrix = cv::Mat::eye(3, 3, CV_64F);
	if (s.flag & cv::CALIB_FIX_ASPECT_RATIO)
		cameraMatrix.at<double>(0, 0) = s.aspectRatio;
	
	//! [fixed_aspect]
	if (s.useFisheye)
	{
		distCoeffs = cv::Mat::zeros(4, 1, CV_64F);
	}
	else
	{
		distCoeffs = cv::Mat::zeros(8, 1, CV_64F);
	}

	std::vector<std::vector<cv::Point3f>> objectPoints;
	objectPoints.push_back(std::vector<cv::Point3f>());
	s.boardSize.width = 5;
	s.boardSize.height = 4;
	s.squareSize = 50;

	calcBoardCornerPositions(s.boardSize, s.squareSize, objectPoints[0], s.calibrationPattern);
	objectPoints.resize(imagePoints.size(), objectPoints[0]);

	// find intrinsic and extrinsic camera parameters
	//double rms = 0.0f;

	if (s.useFisheye)
	{
		cv::Mat _rvecs, _tvecs;
		/*rms = */cv::fisheye::calibrate(objectPoints, imagePoints, imageSize, cameraMatrix, distCoeffs, _rvecs, _tvecs, s.flag);

		rvecs.reserve(_rvecs.rows);
		tvecs.reserve(_tvecs.rows);
		for (int i = 0; i < int(objectPoints.size()); i++)
		{
			rvecs.push_back(_rvecs.row(i));
			tvecs.push_back(_tvecs.row(i));
		}
	}
	else
	{
		/*rms = */calibrateCamera(objectPoints, imagePoints, imageSize, cameraMatrix, distCoeffs, rvecs, tvecs, s.flag);
	}

	bool ok = checkRange(cameraMatrix) && checkRange(distCoeffs);

	totalAvgErr = computeReprojectionErrors(objectPoints, imagePoints, rvecs, tvecs, cameraMatrix, distCoeffs, reprojErrs, s.useFisheye);

	return ok;
}

// =-=-=-=-=-=-=-=-=-
// =-=- EXAMPLE -=-=-
// =-=-=-=-=-=-=-=-=-

// calculates and saves calibration values
void CalculateAndSaveCalibrationValues(Arena::IDevice* pDevice)
{
	// get node values that will be changed in order to return their values at
	// the end of the example
	GenICam::gcstring acquisitionModeInitial = Arena::GetNodeValue<GenICam::gcstring>(pDevice->GetNodeMap(), "AcquisitionMode");
	GenICam::gcstring pixelFormatInitial = Arena::GetNodeValue<GenICam::gcstring>(pDevice->GetNodeMap(), "PixelFormat");

	// enable stream auto negotiate packet size
	Arena::SetNodeValue<bool>(
		pDevice->GetTLStreamNodeMap(),
		"StreamAutoNegotiatePacketSize",
		true);

	// enable stream packet resend
	Arena::SetNodeValue<bool>(
		pDevice->GetTLStreamNodeMap(),
		"StreamPacketResendEnable",
		true);

	// set pixel format
	std::cout << TAB1 << "Set pixel format to 'Mono8'\n";

	Arena::SetNodeValue<GenICam::gcstring>(
		pDevice->GetNodeMap(),
		"PixelFormat",
		"Mono8");

	// set acquisition mode
		std::cout << TAB1 << "Set acquisition mode to 'Continuous'\n";

	Arena::SetNodeValue<GenICam::gcstring>(
		pDevice->GetNodeMap(),
		"AcquisitionMode",
		"Continuous");

	// set buffer handling mode
	std::cout << TAB1 << "Set buffer handling mode to 'NewestOnly'\n";
	Arena::SetNodeValue<GenICam::gcstring>(
		pDevice->GetTLStreamNodeMap(),
		"StreamBufferHandlingMode",
		"NewestOnly");

	// start stream
	pDevice->StartStream();

	// get sets of calibration points
	std::cout << TAB1 << "Getting " << NUM_IMAGES << " sets of calibration points\n";
	std::cout << TAB1 << "Move the calibration target around the frame for best results\n";
	
	const cv::Size patternSize(5, 4);
	std::vector<std::vector<cv::Point2f>> calibrationPoints;
	cv::Size imageSize;	
	size_t attempts = 0;
	size_t images = 0;
	size_t gridCentersFound = 0;
	size_t successes = 0;

	while (successes < NUM_IMAGES)
	{
		
		try
		{
			// get image
			attempts++;
			Arena::IImage* pImage = pDevice->GetImage(TIMEOUT);
			images++;
			if (pImage->IsIncomplete())
				throw std::runtime_error("Incomplete image");

			size_t w = pImage->GetWidth();
			size_t h = pImage->GetHeight();
			imageSize.width = static_cast<int>(w);
			imageSize.height = static_cast<int>(h);

			// copy data into an OpenCV matrix
			cv::Mat imageMatrix = cv::Mat(imageSize.height, imageSize.width, CV_8UC1);
			memset(imageMatrix.data, 0, w * h);
			memcpy(imageMatrix.data, pImage->GetData(), w * h);

			pDevice->RequeueBuffer(pImage);
			
			// find calibration circles
			std::vector<cv::Point2f> gridCenters;
			findCalibrationPoints(imageMatrix, gridCenters);
			gridCentersFound = gridCenters.size();

			if (gridCentersFound == 20)
			{
				calibrationPoints.push_back(gridCenters);
				successes++;
				std::cout << TAB2 << "Calibration image has been successful.\n";	
			}
			else
			{
				std::cout << TAB2 << "Found " << gridCentersFound << " circles. Please adjust the calibration target! \n";	
			}
		}
		catch (...)
		{
			// on failure, ignore and retry
		}
	
		std::cout << TAB3 << attempts << " attempts, " << images << " images, " << gridCentersFound << " circles found, " << successes << " calibration points\n";
		
		std::cout << TAB3 << "Please move the dot-pattern to a new position and press Enter to continue.\r";
		std::cin.get();
		// sleep between images
		std::this_thread::sleep_for(std::chrono::milliseconds(SLEEP_MS));		
	};

	// calculate camera matrix and distance coefficients
	std::cout << "\n" << TAB1 << "Calculate camera matrix and distance coefficients\n";
	
	cv::Mat cameraMatrix;
	cv::Mat distCoeffs;

	Settings s;
	s.nrFrames = NUM_IMAGES;
	s.inputType = Settings::IMAGE_LIST;
	std::vector<cv::Mat> rvecs, tvecs;
	std::vector<float> reprojErrs;
	double totalAvgErr = 0;

	s.calibrationPattern = Settings::CIRCLES_GRID;
	s.flag = cv::CALIB_RATIONAL_MODEL;
	bool calculationSucceeded = calculate(s, imageSize, cameraMatrix, distCoeffs, calibrationPoints, rvecs, tvecs, reprojErrs, totalAvgErr);
	
	std::cout << TAB2 << "Calibration " << (calculationSucceeded ? "succeeded" : "failed") << "\n";
	std::cout << TAB2 << "Calculated reprojection error is " << totalAvgErr << "\n";
	
	// save calibration information
	std::cout << TAB1 << "Save camera matrix and distance coefficients to file '" << FILE_NAME << "'\n";

	cv::FileStorage fs(FILE_NAME, cv::FileStorage::WRITE);
	fs << "cameraMatrix" << cameraMatrix;
	fs << "distCoeffs" << distCoeffs;
	fs.release();

	// stop stream
	pDevice->StopStream();

	// return nodes to their initial values
	Arena::SetNodeValue<GenICam::gcstring>(pDevice->GetNodeMap(), "PixelFormat", pixelFormatInitial);
	Arena::SetNodeValue<GenICam::gcstring>(pDevice->GetNodeMap(), "AcquisitionMode", acquisitionModeInitial);
}

// =-=-=-=-=-=-=-=-=-
// =- PREPARATION -=-
// =- & CLEAN UP =-=-
// =-=-=-=-=-=-=-=-=-

bool isApplicableDevice(Arena::DeviceInfo deviceInfo)
{
	// color Triton camera needed
	return ((deviceInfo.ModelName().find("TRI") != GenICam::gcstring::npos) && (deviceInfo.ModelName().find("-C") != GenICam::gcstring::npos));
}

int main()
{
	// flag to track when an exception has been thrown
	bool exceptionThrown = false;

	std::cout << "Cpp_HLTRGB_1_Calibration\n";

	try
	{
		// prepare example
		Arena::ISystem* pSystem = Arena::OpenSystem();
		pSystem->UpdateDevices(100);
		std::vector<Arena::DeviceInfo> deviceInfos = pSystem->GetDevices();
		if (deviceInfos.size() == 0)
		{
			std::cout << "\nNo camera connected\nPress enter to complete\n";
			std::getchar();
			return 0;
		}

		Arena::IDevice* pDevice = nullptr;
		for (auto& deviceInfo : deviceInfos)
		{
			if (isApplicableDevice(deviceInfo))
			{
				pDevice = pSystem->CreateDevice(deviceInfo);
				break;
			}
		}

		if (!pDevice)
			throw std::logic_error("No applicable device");

		// run example
		if (pDevice)
		{
			std::cout << "Commence example\n\n";
			CalculateAndSaveCalibrationValues(pDevice);
			std::cout << "\nExample complete\n";
		}

		// clean up example
		if (pDevice)
			pSystem->DestroyDevice(pDevice);
		Arena::CloseSystem(pSystem);
	}
	catch (GenICam::GenericException& ge)
	{
		std::cout << "\nGenICam exception thrown: " << ge.what() << "\n";
		exceptionThrown = true;
	}
	catch (std::exception& ex)
	{
		std::cout << "\nStandard exception thrown: " << ex.what() << "\n";
		exceptionThrown = true;
	}
	catch (...)
	{
		std::cout << "\nUnexpected exception thrown\n";
		exceptionThrown = true;
	}

	std::cout << "Press enter to complete\n";
	std::cin.ignore();
	std::getchar();

	if (exceptionThrown)
		return -1;
	else
		return 0;
}
