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

#include <opencv2/opencv.hpp>
#include <opencv2/core/mat.hpp>
#include <opencv2/calib3d/calib3d.hpp>
#include <opencv2/imgproc/imgproc.hpp>
#include <opencv2/highgui/highgui.hpp>
#include <opencv2/core/version.hpp>

#include <iostream>
#include <fstream>
#include <sstream>

// Helios RGB: Orientation
//    This example is part 2 of a 3-part example on color overlay over 3D images.
//    Color data can be overlaid over 3D images by reading the 
//    3D points ABC (XYZ) from the Helios and projecting them onto
//    the Triton color (RGB) camera directly. This requires first solving for the
//    orientation of the Helios coordinate system relative to the Triton’s
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

// =-=-=-=-=-=-=-=-=-
// =-=- HELPERS -=-=-
// =-=-=-=-=-=-=-=-=-

// helper function
void getImageHLT(Arena::IDevice* pHeliosDevice, cv::Mat& intensity_image, cv::Mat& xyz_mm)
{
	Arena::SetNodeValue<GenICam::gcstring>(pHeliosDevice->GetNodeMap(), "PixelFormat", "Coord3D_ABCY16");

	// enable stream auto negotiate packet size
	Arena::SetNodeValue<bool>(
		pHeliosDevice->GetTLStreamNodeMap(),
		"StreamAutoNegotiatePacketSize",
		true);

	// enable stream packet resend
	Arena::SetNodeValue<bool>(
		pHeliosDevice->GetTLStreamNodeMap(),
		"StreamPacketResendEnable",
		true);

	// Read the scale factor and offsets to convert from unsigned 16-bit values 
	//    in the Coord3D_ABCY16 pixel format to coordinates in mm
	GenApi::INodeMap* node_map = pHeliosDevice->GetNodeMap();
	double xyz_scale_mm = Arena::GetNodeValue<double>(node_map, "Scan3dCoordinateScale");
	Arena::SetNodeValue<GenICam::gcstring>(node_map, "Scan3dCoordinateSelector", "CoordinateA");
	double x_offset_mm = Arena::GetNodeValue<double>(node_map, "Scan3dCoordinateOffset");
	Arena::SetNodeValue<GenICam::gcstring>(node_map, "Scan3dCoordinateSelector", "CoordinateB");
	double y_offset_mm = Arena::GetNodeValue<double>(node_map, "Scan3dCoordinateOffset");
	Arena::SetNodeValue<GenICam::gcstring>(node_map, "Scan3dCoordinateSelector", "CoordinateC");
	double z_offset_mm = Arena::GetNodeValue<double>(node_map, "Scan3dCoordinateOffset");

	pHeliosDevice->StartStream();
	Arena::IImage* image = pHeliosDevice->GetImage(TIMEOUT);

	size_t height, width;
	height = image->GetHeight();
	width = image->GetWidth();

	xyz_mm = cv::Mat((int)height, (int)width, CV_32FC3);
	intensity_image = cv::Mat((int)height, (int)width, CV_16UC1);

	const uint16_t* input_data;
	input_data = (uint16_t*)image->GetData();

	for (unsigned int ir = 0; ir < height; ++ir)
	{
		for (unsigned int ic = 0; ic < width; ++ic)
		{
			// Get unsigned 16 bit values for X,Y,Z coordinates
			ushort x_u16 = input_data[0];
			ushort y_u16 = input_data[1];
			ushort z_u16 = input_data[2];

			// Convert 16-bit X,Y,Z to float values in mm
			xyz_mm.at<cv::Vec3f>(ir, ic)[0] = (float)(x_u16 * xyz_scale_mm + x_offset_mm);
			xyz_mm.at<cv::Vec3f>(ir, ic)[1] = (float)(y_u16 * xyz_scale_mm + y_offset_mm);
			xyz_mm.at<cv::Vec3f>(ir, ic)[2] = (float)(z_u16 * xyz_scale_mm + z_offset_mm);

			intensity_image.at<ushort>(ir, ic) = input_data[3]; // // Intensity value

			input_data += 4;
		}
	}

	// clean up
	input_data = NULL;
	delete input_data;

	pHeliosDevice->RequeueBuffer(image);
	pHeliosDevice->StopStream();
}

// helper function
void getImageTRI(Arena::IDevice* pDeviceTriton, cv::Mat& triton_image)
{
	Arena::SetNodeValue<GenICam::gcstring>(pDeviceTriton->GetNodeMap(), "PixelFormat", "RGB8");

	// enable stream auto negotiate packet size
	Arena::SetNodeValue<bool>(
		pDeviceTriton->GetTLStreamNodeMap(),
		"StreamAutoNegotiatePacketSize",
		true);

	// enable stream packet resend
	Arena::SetNodeValue<bool>(
		pDeviceTriton->GetTLStreamNodeMap(),
		"StreamPacketResendEnable",
		true);

	pDeviceTriton->StartStream();
	Arena::IImage* pImage = pDeviceTriton->GetImage(TIMEOUT);

	// convert Triton image to mono for dot finding
	Arena::IImage* pConvert = Arena::ImageFactory::Convert(pImage, Mono8);
	size_t w = pImage->GetWidth();
	size_t h = pImage->GetHeight();
	cv::Size patternsize(5, 4); //
	triton_image = cv::Mat((int)h, (int)w, CV_8UC1);
	memcpy(triton_image.data, pConvert->GetData(), h * w);

	// clean up
	Arena::ImageFactory::Destroy(pConvert);
	pDeviceTriton->RequeueBuffer(pImage);
	pDeviceTriton->StopStream();	
}

// helper function
void findCalibrationPointsHLT(const cv::Mat& image_in, std::vector<cv::Point2f>& grid_centers)
{
	cv::SimpleBlobDetector::Params bright_params;
	bright_params.filterByColor = true;
	bright_params.blobColor = 255; // white circles in the calibration target
	bright_params.thresholdStep = 2;
	bright_params.minArea = 10.0;  // Min/max area can be adjusted based on size of dots in image
	bright_params.maxArea = 1000.0;

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

	// Find max value in input image
	double min_value, max_value;
	cv::minMaxIdx(image_in, &min_value, &max_value);

	// Scale image to 8-bit, using full 8-bit range
	cv::Mat image_8bit;
	image_in.convertTo(image_8bit, CV_8U, 255.0 / max_value);

	/*bool is_found = */cv::findCirclesGrid(image_8bit, pattern_size, grid_centers, cv::CALIB_CB_SYMMETRIC_GRID, blob_detector);
}

// helper function
bool findCalibrationPointsTRI(const cv::Mat& image_in_orig, std::vector<cv::Point2f>& grid_centers)
{
	float scaling = 1.0;
	cv::Mat image_in = image_in_orig;

	/*ArenaView*/
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

	bool is_found = findCirclesGrid(image_in, pattern_size, grid_centers, 1, blob_detector);

	double scaled_nrows = 2400.0;
	while (!is_found && scaled_nrows >= 100)
	{
		scaled_nrows /= 2.0;
		scaling = static_cast<float>((double)image_in_orig.rows / scaled_nrows);
		cv::resize(image_in_orig, image_in, cv::Size(static_cast<int>((double)image_in_orig.cols / scaling), static_cast<int>((double)image_in_orig.rows / scaling)));

		is_found = findCirclesGrid(image_in, pattern_size, grid_centers, 1, blob_detector);
		std::cout << "Found " << grid_centers.size() << "circle centers.\n";
	}

	// Scale back the grid centers
	for (unsigned int i = 0; i < grid_centers.size(); ++i)
	{
		grid_centers[i].x = grid_centers[i].x * scaling;
		grid_centers[i].y = grid_centers[i].y * scaling;
	}
	return is_found;
}

// =-=-=-=-=-=-=-=-=-
// =-=- EXAMPLE -=-=-
// =-=-=-=-=-=-=-=-=-

void CalculateAndSaveOrientationValues(Arena::IDevice* pDeviceTRI, Arena::IDevice* pDeviceHLT)
{
	// get node values that will be changed in order to return their values at
	// the end of the example
	GenICam::gcstring pixelFormatInitialTRI = Arena::GetNodeValue<GenICam::gcstring>(pDeviceTRI->GetNodeMap(), "PixelFormat");
	GenICam::gcstring pixelFormatInitialHLT = Arena::GetNodeValue<GenICam::gcstring>(pDeviceHLT->GetNodeMap(), "PixelFormat");

	// Read in camera matrix and distance coefficients
	std::cout << TAB1 << "Read camera matrix and distance coefficients from file '" << FILE_NAME_IN << "'\n";

	cv::FileStorage fs(FILE_NAME_IN, cv::FileStorage::READ);
	cv::Mat cameraMatrix;
	cv::Mat distCoeffs;
	
	fs["cameraMatrix"] >> cameraMatrix;
	fs["distCoeffs"] >> distCoeffs;
	
	fs.release();

	// Get an image from Helios2
	std::cout << TAB1 << "Get and prepare HLT image\n";

	cv::Mat imageMatrixHLTIntensity;
	cv::Mat imageMatrixHLTXYZ;

	getImageHLT(pDeviceHLT, imageMatrixHLTIntensity, imageMatrixHLTXYZ);

	// Get an image from Triton
	std::cout << TAB1 << "Get and prepare TRI image\n";

	cv::Mat imageMatrixTRI;

	getImageTRI(pDeviceTRI, imageMatrixTRI);

	// Calculate orientation values
	std::cout << TAB1 << "Calculate orientation values\n";

	std::vector<cv::Point2f> gridCentersHLT;
	std::vector<cv::Point2f> gridCentersTRI;
	
	// find HLT calibration points using HLT intensity image
	std::cout << TAB2 << "Find points in HLT image\n";

	findCalibrationPointsHLT(imageMatrixHLTIntensity, gridCentersHLT);

	if (gridCentersHLT.size() != 20)
		throw std::logic_error("Unable to find points in HLT intensity image\n");

	// find TRI calibration points
	std::cout << TAB2 << "Find points in TRI image\n";
	
	findCalibrationPointsTRI(imageMatrixTRI, gridCentersTRI);
	
	if (gridCentersTRI.size() != 20)
		throw std::logic_error("Unable to find points in TRI image\n");
	
	// prepare for PnP
	std::cout << TAB2 << "Prepare for PnP\n";
	
	std::vector<cv::Point3f> targetPoints3Dmm;
	std::vector<cv::Point2f> targetPoints3DPixels;
	std::vector<cv::Point2f> targetPoints2DPixels;
	
	for (int i = 0; i < static_cast<int>(gridCentersTRI.size()); i++)
	{
		unsigned int c1 = (unsigned int)round(gridCentersHLT[i].x);
		unsigned int r1 = (unsigned int)round(gridCentersHLT[i].y);
		//unsigned int c2 = (unsigned int)round(gridCentersTRI[i].x);
		//unsigned int r2 = (unsigned int)round(gridCentersTRI[i].y);

		float x = imageMatrixHLTXYZ.at<cv::Vec3f>(r1, c1)[0];
		float y = imageMatrixHLTXYZ.at<cv::Vec3f>(r1, c1)[1];
		float z = imageMatrixHLTXYZ.at<cv::Vec3f>(r1, c1)[2];

		cv::Point3f pt(x, y, z);
		std::cout << TAB3 << "Point " << i << ": " << pt << "\n";

		targetPoints3Dmm.push_back(pt);
		targetPoints3DPixels.push_back(gridCentersHLT[i]);
		targetPoints2DPixels.push_back(gridCentersTRI[i]);
	}

	cv::Mat rotationVector;
	cv::Mat translationVector;

	bool orientationSucceeded = cv::solvePnP(targetPoints3Dmm,
		targetPoints2DPixels,
		cameraMatrix,
		distCoeffs,
		rotationVector,
		translationVector);

	std::cout << TAB2 << "Orientation " << (orientationSucceeded ? "succeeded" : "failed") << "\n";
	
	// Save orientation information
	std::cout << TAB1 << "Save camera matrix, distance coefficients, and rotation and translation vectors to file '" << FILE_NAME_OUT << "'\n";

	cv::FileStorage fs2(FILE_NAME_OUT, cv::FileStorage::WRITE);

	fs2 << "cameraMatrix" << cameraMatrix;
	fs2 << "distCoeffs" << distCoeffs;
	fs2 << "rotationVector" << rotationVector;
	fs2 << "translationVector" << translationVector;

	fs2.release();
	
	// return nodes to their initial values
	Arena::SetNodeValue<GenICam::gcstring>(pDeviceTRI->GetNodeMap(), "PixelFormat", pixelFormatInitialTRI);
	Arena::SetNodeValue<GenICam::gcstring>(pDeviceHLT->GetNodeMap(), "PixelFormat", pixelFormatInitialHLT);
}

// =-=-=-=-=-=-=-=-=-
// =- PREPARATION -=-
// =- & CLEAN UP =-=-
// =-=-=-=-=-=-=-=-=-

bool isApplicableDeviceTriton(Arena::DeviceInfo deviceInfo)
{
	// color Triton camera needed
	return ((deviceInfo.ModelName().find("TRI") != GenICam::gcstring::npos) && (deviceInfo.ModelName().find("-C") != GenICam::gcstring::npos));
}

bool isApplicableDeviceHelios2(Arena::DeviceInfo deviceInfo)
{
	return ((deviceInfo.ModelName().find("HLT") != GenICam::gcstring::npos) || (deviceInfo.ModelName().find("HT") != GenICam::gcstring::npos));
}

int main()
{
	// flag to track when an exception has been thrown
	bool exceptionThrown = false;

	std::cout << "Cpp_HLTRGB_2_Orientation\n";

	try
	{
		std::ifstream ifile;
		ifile.open(FILE_NAME_IN);
		if (!ifile)
		{
			std::cout << "File '" << FILE_NAME_IN << "' not found\nPlease run example 'Cpp_HLTRGB_1_Calibration' prior to this one\nPress enter to complete\n";
			std::getchar();
			return 0;
		}
		
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
		
		Arena::IDevice* pDeviceTRI = nullptr;
		Arena::IDevice* pDeviceHLT = nullptr;
		for (auto& deviceInfo : deviceInfos)
		{
			if (!pDeviceTRI && isApplicableDeviceTriton(deviceInfo))
			{
				pDeviceTRI = pSystem->CreateDevice(deviceInfo);
			}
			else if (isApplicableDeviceTriton(deviceInfo))
			{
				throw std::logic_error("Too many Triton devices connected");
			}
			else if (!pDeviceHLT && isApplicableDeviceHelios2(deviceInfo))
			{
				pDeviceHLT = pSystem->CreateDevice(deviceInfo);
			}
			else if (isApplicableDeviceHelios2(deviceInfo))
			{
				throw std::logic_error("Too many Helios2 devices connected");
			}
		}

		if (!pDeviceTRI)
			throw std::logic_error("No applicable Triton devices");

		if (!pDeviceHLT)
			throw std::logic_error("No applicable Helios2 devices");
		
		// run example
		if (pDeviceTRI && pDeviceHLT)
		{
			std::cout << "Commence example\n\n";
			CalculateAndSaveOrientationValues(pDeviceTRI, pDeviceHLT);
			std::cout << "\nExample complete\n";
		}

		// clean up example
		if (pDeviceTRI)
			pSystem->DestroyDevice(pDeviceTRI);
		if (pDeviceHLT)
			pSystem->DestroyDevice(pDeviceHLT);
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
