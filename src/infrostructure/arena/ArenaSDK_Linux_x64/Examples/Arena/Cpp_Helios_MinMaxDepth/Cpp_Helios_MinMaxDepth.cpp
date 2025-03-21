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

// Helios: Min/Max Depth
//    This example captures a 3D image and interprets the ABCY data into their
//    appropriate x, y and z coordinates and intensities. It converts this data
//    into millimeters and then displays this data for points with both the
//    largest and smallest values of z.

// =-=-=-=-=-=-=-=-=-
// =-=- SETTINGS =-=-
// =-=-=-=-=-=-=-=-=-

// file name
#define FILE_NAME "Images/Cpp_Helios_MinMaxDepth.ply"

// pixel format
#define PIXEL_FORMAT "Coord3D_ABCY16"

// image timeout
#define IMAGE_TIMEOUT 2000

// =-=-=-=-=-=-=-=-=-
// =-=- EXAMPLE -=-=-
// =-=-=-=-=-=-=-=-=-

// store x, y, z data in millimeters and intensity for a given point
struct PointData
{
	int16_t x;
	int16_t y;
	int16_t z;
	int16_t intensity;
};

// demonstrates acquiring 3D data for a specific point
// (1) gets image
// (2) interprets ABCY data to get x, y, z, and intensity
// (3) stores data for point with min and max z values
// (4) displays 3D data for min and max points
void AcquireImageAndInterpretData(Arena::IDevice* pDevice)
{
	GenApi::INodeMap* pNodeMap = pDevice->GetNodeMap();

	// Validate if the Scan3dCoordinateSelector node exists. If it doesn't exist, the camera being
	// used to run the example is likely not a Helios
	GenApi::CEnumerationPtr checkpCoordSelector = pNodeMap->GetNode("Scan3dCoordinateSelector");
	if (!checkpCoordSelector)
	{
		std::cout << TAB1 << "Scan3dCoordinateSelector node is not found. Please make sure that a Helios device is used for the example.\n";
		return;
	}

	// Validate if the Scan3dCoordinateOffset node exists. If it doesn't exist, it is likely that the Helios
	// has old firmware
	GenApi::CFloatPtr checkpCoord = pNodeMap->GetNode("Scan3dCoordinateOffset");
	if (!checkpCoord)
	{
		std::cout << TAB1 << "Scan3dCoordinateOffset node is not found. Please update Helios firmware.\n";
		return;
	}

	// check if Helios2 camera used for the example
	bool isHelios2 = false;
	GenICam::gcstring deviceModelName = Arena::GetNodeValue<GenICam::gcstring>(pDevice->GetNodeMap(), "DeviceModelName");
	std::string deviceModelName_tmp = deviceModelName.c_str();
	if (deviceModelName_tmp.rfind("HLT", 0) == 0 || deviceModelName_tmp.rfind("HT", 0) == 0)
	{
		isHelios2 = true;
	}


	// get node values that will be changed in order to return their values at
	// the end of the example
	GenICam::gcstring pixelFormatInitial = Arena::GetNodeValue<GenICam::gcstring>(pNodeMap, "PixelFormat");
	GenICam::gcstring operatingModeInitial = Arena::GetNodeValue<GenICam::gcstring>(pNodeMap, "Scan3dOperatingMode");

	// Set pixel format
	//    Warning: HLT003S-001 / Helios2 - has only Coord3D_ABCY16 in this case
	//    This example demonstrates data interpretation for both a signed or
	//    unsigned pixel format. Default PIXEL_FORMAT here is set to
	//    Coord3D_ABCY16 but this can be modified to be a signed pixel format by
	//    changing it to Coord3D_ABCY16s.
	std::cout << TAB1 << "Set " << PIXEL_FORMAT << " to pixel format\n";

	Arena::SetNodeValue<GenICam::gcstring>(pNodeMap, "PixelFormat", PIXEL_FORMAT);

	// set operating mode distance
	if (isHelios2)
	{
		std::cout << TAB1 << "Set 3D operating mode to Distance3000mm\n";
		Arena::SetNodeValue<GenICam::gcstring>(pNodeMap, "Scan3dOperatingMode", "Distance3000mmSingleFreq");
	}
	else
	{
		std::cout << TAB1 << "Set 3D operating mode to Distance1500mm\n";
		Arena::SetNodeValue<GenICam::gcstring>(pNodeMap, "Scan3dOperatingMode", "Distance1500mm");
	}

	// get the coordinate scale in order to convert x, y and z values to millimeters as
	// well as the offset for x and y to correctly adjust values when in an
	// unsigned pixel format
	std::cout << TAB1 << "Get xyz coordinate scales and offsets\n\n";

	Arena::SetNodeValue<GenICam::gcstring>(pNodeMap, "Scan3dCoordinateSelector", "CoordinateA");
	// getting scaleX as float by casting since SetPly() will expect it passed as
	// float
	float scaleX = static_cast<float>(Arena::GetNodeValue<double>(pNodeMap, "Scan3dCoordinateScale"));
	// getting offsetX as float by casting since SetPly() will expect it passed
	// as float
	float offsetX = static_cast<float>(Arena::GetNodeValue<double>(pNodeMap, "Scan3dCoordinateOffset"));
	Arena::SetNodeValue<GenICam::gcstring>(pNodeMap, "Scan3dCoordinateSelector", "CoordinateB");
	double scaleY = Arena::GetNodeValue<double>(pNodeMap, "Scan3dCoordinateScale");
	// getting offsetY as float by casting since SetPly() will expect it passed
	// as float
	float offsetY = static_cast<float>(Arena::GetNodeValue<double>(pNodeMap, "Scan3dCoordinateOffset"));
	Arena::SetNodeValue<GenICam::gcstring>(pNodeMap, "Scan3dCoordinateSelector", "CoordinateC");
	double scaleZ = Arena::GetNodeValue<double>(pNodeMap, "Scan3dCoordinateScale");

	// enable stream auto negotiate packet size
	Arena::SetNodeValue<bool>(pDevice->GetTLStreamNodeMap(), "StreamAutoNegotiatePacketSize", true);

	// enable stream packet resend
	Arena::SetNodeValue<bool>(pDevice->GetTLStreamNodeMap(), "StreamPacketResendEnable", true);

	// retrieve image
	std::cout << TAB2 << "Acquire image\n";

	pDevice->StartStream();
	Arena::IImage* pImage = pDevice->GetImage(IMAGE_TIMEOUT);

	// prepare info from input buffer
	size_t width = pImage->GetWidth();
	size_t height = pImage->GetHeight();
	size_t size = width * height;
	size_t srcBpp = pImage->GetBitsPerPixel();
	size_t srcPixelSize = srcBpp / 8;
	const uint8_t* pInput = pImage->GetData();
	const uint8_t* pIn = pInput;

	// minDepth z value is set to 32767 to guarantee closer points exist as this
	// is the largest value possible
	PointData minDepth = { 0, 0, 32767, 0 };
	PointData maxDepth = { 0, 0, 0, 0 };

	// find points with min and max z values
	std::cout << TAB2 << "Find points with min and max z values\n";

	// using strcmp to avoid conversion issue
	int compareResult_ABCY16s = strcmp(PIXEL_FORMAT, "Coord3D_ABCY16s"); // if they are equal compareResult_ABCY16s = 0
	int compareResult_ABCY16 = strcmp(PIXEL_FORMAT, "Coord3D_ABCY16");	 // if they are equal compareResult_ABCY16 = 0

	bool isSignedPixelFormat = false;

	// if PIXEL_FORMAT is equal to Coord3D_ABCY16s
	if (compareResult_ABCY16s == 0)
	{
		isSignedPixelFormat = true;

		for (size_t i = 0; i < size; i++)
		{
			// Extract point data to signed 16 bit integer
			//    The first channel is the x coordinate, the second channel is the y
			//    coordinate, the third channel is the z coordinate, and the
			//    fourth channel is intensity. We offset pIn by 2 for each
			//    channel because pIn is an 8 bit integer and we want to read it
			//    as a 16 bit integer.
			int16_t x = *reinterpret_cast<const int16_t*>(pIn);
			int16_t y = *reinterpret_cast<const int16_t*>((pIn + 2));
			int16_t z = *reinterpret_cast<const int16_t*>((pIn + 4));
			int16_t intensity = *reinterpret_cast<const int16_t*>((pIn + 6));

			// convert x, y and z values to millimeters using their coordinate scales
			x = int16_t(double(x) * scaleX);
			y = int16_t(double(y) * scaleY);
			z = int16_t(double(z) * scaleZ);

			if (z < minDepth.z && z > 0)
			{
				minDepth.x = x;
				minDepth.y = y;
				minDepth.z = z;
				minDepth.intensity = intensity;
			}
			else if (z > maxDepth.z)
			{
				maxDepth.x = x;
				maxDepth.y = y;
				maxDepth.z = z;
				maxDepth.intensity = intensity;
			}

			pIn += srcPixelSize;
		}

		// display data
		std::cout << TAB3 << "Minimum depth point found with z distance of " << minDepth.z
				<< "mm and intensity " << minDepth.intensity << " at coordinates (" << minDepth.x
				<< "mm, " << minDepth.y << "mm)" << std::endl;

		std::cout << TAB3 << "Maximum depth point found with z distance of " << maxDepth.z
				<< "mm and intensity " << maxDepth.intensity << " at coordinates (" << maxDepth.x
				<< "mm, " << maxDepth.y << "mm)" << std::endl;
	}
	// if PIXEL_FORMAT is equal to Coord3D_ABCY16
	else if (compareResult_ABCY16 == 0)
	{
		for (size_t i = 0; i < size; i++)
		{
			// Extract point data to signed 16 bit integer
			//    The first channel is the x coordinate, the second channel is the y
			//    coordinate, the third channel is the z coordinate and the
			//    fourth channel is intensity. We offset pIn by 2 for each
			//    channel because pIn is an 8 bit integer and we want to read it
			//    as a 16 bit integer.
			uint16_t x = *reinterpret_cast<const uint16_t*>(pIn);
			uint16_t y = *reinterpret_cast<const uint16_t*>((pIn + 2));
			uint16_t z = *reinterpret_cast<const uint16_t*>((pIn + 4));
			uint16_t intensity = *reinterpret_cast<const uint16_t*>((pIn + 6));

			// if z is less than max value, as invalid values get filtered to
			// 65535
			if (z < 65535)
			{
				// Convert x, y and z to millimeters
				//    Using each coordinates' appropriate scales, convert x, y
				//    and z values to mm. For the x and y coordinates in an
				//    unsigned pixel format, we must then add the offset to our
				//    converted values in order to get the correct position in
				//    millimeters.
				x = uint16_t(double(x) * scaleX + offsetX);
				y = uint16_t((double(y) * scaleY) + offsetY);
				z = uint16_t(double(z) * scaleZ);

				if (z < minDepth.z && z > 0)
				{
					minDepth.x = x;
					minDepth.y = y;
					minDepth.z = z;
					minDepth.intensity = intensity;
				}
				else if (z > maxDepth.z)
				{
					maxDepth.x = x;
					maxDepth.y = y;
					maxDepth.z = z;
					maxDepth.intensity = intensity;
				}
			}

			pIn += srcPixelSize;
		}

		// display data
		std::cout << TAB3 << "Minimum depth point found with z distance of " << minDepth.z
				<< "mm and intensity " << minDepth.intensity << " at coordinates (" << minDepth.x
				<< "mm, " << minDepth.y << "mm)" << std::endl;

		std::cout << TAB3 << "Maximum depth point found with z distance of " << maxDepth.z
				<< "mm and intensity " << maxDepth.intensity << " at coordinates (" << maxDepth.x
				<< "mm, " << maxDepth.y << "mm)" << std::endl;
	}
	else
	{
		std::cout << "This example requires the camera to be in either 3D image format Coord3D_ABCY16 or Coord3D_ABCY16s\n\n";
	}

	// prepare image parameters and writer
	Save::ImageParams params(
		pImage->GetWidth(),
		pImage->GetHeight(),
		pImage->GetBitsPerPixel());

	Save::ImageWriter writer(
		params,
		FILE_NAME);

	// set parameters for SetPly()
	bool filterPoints = true; // default
	float offsetZ = 0.0f;	  // default

	// set the output file format of the image writer to .ply
	writer.SetPly(".ply",
				filterPoints,
				isSignedPixelFormat,
				scaleX, // using scaleX as scale since all scales = 0.25f
				offsetX,
				offsetY,
				offsetZ);

	// save image
	writer << pImage->GetData();

	std::cout << TAB2 << "Save image to " << writer.GetLastFileName() << "\n\n";

	// clean up
	pInput = NULL;
	delete[] pInput;
	pIn = NULL;
	delete[] pIn;
	pDevice->RequeueBuffer(pImage);
	pDevice->StopStream();

	// return nodes to their initial values
	Arena::SetNodeValue<GenICam::gcstring>(pNodeMap, "Scan3dOperatingMode", operatingModeInitial);
	Arena::SetNodeValue<GenICam::gcstring>(pNodeMap, "PixelFormat", pixelFormatInitial);
	std::cout << TAB1 << "Nodes were set back to initial values\n";
}

// =-=-=-=-=-=-=-=-=-
// =- PREPARATION -=-
// =- & CLEAN UP =-=-
// =-=-=-=-=-=-=-=-=-

Arena::DeviceInfo SelectDevice(std::vector<Arena::DeviceInfo>& deviceInfos)
{
	if (deviceInfos.size() == 1)
	{
		std::cout << "\n"
				  << TAB1 << "Only one device detected: " << deviceInfos[0].ModelName() << TAB1 << deviceInfos[0].SerialNumber() << TAB1 << deviceInfos[0].IpAddressStr() << ".\n";
		std::cout << TAB1 << "Automatically selecting this device.\n";
		return deviceInfos[0];
	}

	std::cout << "\nSelect device:\n";
	for (size_t i = 0; i < deviceInfos.size(); i++)
	{
		std::cout << TAB1 << i + 1 << ". " << deviceInfos[i].ModelName() << TAB1 << deviceInfos[i].SerialNumber() << TAB1 << deviceInfos[i].IpAddressStr() << "\n";
	}
	size_t selection = 0;

	do
	{
		std::cout << TAB1 << "Make selection (1-" << deviceInfos.size() << "): ";
		std::cin >> selection;

		if (std::cin.fail())
		{
			std::cin.clear();
			while (std::cin.get() != '\n')
				;
			std::cout << TAB1 << "Invalid input. Please enter a number.\n";
		}
		else if (selection <= 0 || selection > deviceInfos.size())
		{
			std::cout << TAB1 << "Invalid device selected. Please select a device in the range (1-" << deviceInfos.size() << ").\n";
		}

	} while (selection <= 0 || selection > deviceInfos.size());

	return deviceInfos[selection - 1];
}

int main()
{
	// flag to track when an exception has been thrown
	bool exceptionThrown = false;

	std::cout << "Cpp_Helios_MinMaxDepth";

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
		Arena::DeviceInfo selectedDeviceInfo = SelectDevice(deviceInfos);
		Arena::IDevice* pDevice = pSystem->CreateDevice(selectedDeviceInfo);

		std::cout << "Commence example\n\n";

		// run example
		AcquireImageAndInterpretData(pDevice);

		std::cout << "\nExample complete\n";

		// clean up example
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
