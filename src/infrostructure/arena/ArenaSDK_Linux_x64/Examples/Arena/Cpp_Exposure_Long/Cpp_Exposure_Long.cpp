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

#include <math.h>       /* ceil */

#define TAB1 "  "
#define TAB2 "    "

// Long Exposure: Introduction
//    This example depicts code that increases the maximum exposure time. By
//    default, LUCID cameras are prioritized to achieve maximum frame rate.
//    However, due to the high frame rate configuration, the exposure time will
//    be limited as it is a dependant value. If the frame rate is 30 FPS, the
//    maximum allowable exposure would be 1/30 = 0.0333 seconds = 33.3
//    milliseconds. So, decreasing the frame rate is necessary to increase
//    the exposure time.

// =-=-=-=-=-=-=-=-=-
// =-=- SETTINGS =-=-
// =-=-=-=-=-=-=-=-=-

// number of images to grab
#define NUM_IMAGES 1

// =-=-=-=-=-=-=-=-=-
// =-=- EXAMPLE -=-=-
// =-=-=-=-=-=-=-=-=-

// demonstrates long exposure
// (1) Set Acquisition Frame Rate Enable to true
// (2) Decrease Acquisition Frame Rate
// (3) Set Exposure Auto to OFF
// (4) Increase Exposure Time
void ConfigureExposureMaximum(Arena::IDevice* pDevice) 
{
	// get node values that will be changed in order to return their values at
	// the end of the example
	GenICam::gcstring exposureAutoInitial = Arena::GetNodeValue<GenICam::gcstring>(pDevice->GetNodeMap(), "ExposureAuto");
	double exposureTimeInitial = Arena::GetNodeValue<double>(pDevice->GetNodeMap(), "ExposureTime");
	bool acquisitionFrameRateEnableInitial = Arena::GetNodeValue<bool>(pDevice->GetNodeMap(), "AcquisitionFrameRateEnable");
	double acquisitionFrameRateInitial = Arena::GetNodeValue<double>(pDevice->GetNodeMap(), "AcquisitionFrameRate");

	// set Acquisition Frame Rate Enable to true; this is required to change the
	// Acquisition Frame Rate
	Arena::SetNodeValue<bool>(
		pDevice->GetNodeMap(), 
		"AcquisitionFrameRateEnable", 
		true);


	// get Acquisition Frame Rate node
	GenApi::CFloatPtr pAcquisitionFrameRate = pDevice->GetNodeMap()->GetNode("AcquisitionFrameRate");

	// for the maximum exposure, the Acquisition Frame Rate is set to the lowest
	// value allowed by the camera.
	double newAcquisitionFramerate = pAcquisitionFrameRate->GetMin();

	Arena::SetNodeValue<double>(
		pDevice->GetNodeMap(), 
		"AcquisitionFrameRate", 
		newAcquisitionFramerate);

	// Disable automatic exposure
	//    Disable automatic exposure before setting an exposure time. Automatic
	//    exposure controls whether the exposure time is set manually or
	//    automatically by the device. Setting automatic exposure to 'Off' stops
	//    the device from automatically updating the exposure time while
	//    streaming.
	std::cout << TAB1 << "Disable Auto Exposure" << std::endl;

	Arena::SetNodeValue<GenICam::gcstring>(
		pDevice->GetNodeMap(),
		"ExposureAuto",
		"Off");

	// Get exposure time node
	//    In order to get the maximum and minimum values for exposure time, get the
	//    exposure time node. Failed attempts to get a node return null, so check
	//    that the node exists. Because we expect to set its value, check
	//    that the exposure time node is writable.
	GenApi::CFloatPtr pExposureTime = pDevice->GetNodeMap()->GetNode("ExposureTime");
	if (!pExposureTime)
	{
		throw GenICam::GenericException("ExposureTime node not found", __FILE__, __LINE__);
	}

	if (!GenApi::IsWritable(pExposureTime))
	{
		throw GenICam::GenericException("ExposureTime node not writable", __FILE__, __LINE__);
	}

	// set the exposure time to the maximum
	double exposureTime = pExposureTime->GetMax();

	std::cout << TAB1 << "Minimizing Acquisition Frame Rate and Maximizing Exposure Time" << std::endl;

	pExposureTime->SetValue(exposureTime);

	std::cout << TAB2 << "Changing Acquisiton Frame Rate from " << acquisitionFrameRateInitial << " to " << pAcquisitionFrameRate->GetValue() << std::endl;
	std::cout << TAB2 << "Changing Exposure Time from " << exposureTimeInitial << " to " << pExposureTime->GetValue() << " milliseconds" << std::endl;

	// enable stream auto negotiate packet size
	Arena::SetNodeValue<bool>(pDevice->GetTLStreamNodeMap(), "StreamAutoNegotiatePacketSize", true);

	// enable stream packet resend
	Arena::SetNodeValue<bool>(pDevice->GetTLStreamNodeMap(), "StreamPacketResendEnable", true);

	std::cout << std::endl << TAB1 << "Getting Single Long Exposure Image\n";

	pDevice->StartStream();

	for (int i = 0; i < NUM_IMAGES; i++)
	{
		//Note: Time should always be set greater than exposure time. 
		//    Best Practice: Set time to 3 times the exposure time. If the image
		//    is fetched with time to spare, the program does not wait the entire
		//    time duration.

		uint64_t timeout = (uint64_t) ceil(3 * exposureTime);
		Arena::IImage* pImage = pDevice->GetImage(timeout);
		
		std::cout << TAB2 << "Long Exposure Image Retirevied\n";
		pDevice->RequeueBuffer(pImage);
	}

	pDevice->StopStream();

	// return nodes to their initial values
	
	Arena::SetNodeValue<double>(pDevice->GetNodeMap(), "AcquisitionFrameRate", acquisitionFrameRateInitial);
	Arena::SetNodeValue<double>(pDevice->GetNodeMap(), "ExposureTime", exposureTimeInitial);
	Arena::SetNodeValue<GenICam::gcstring>(pDevice->GetNodeMap(), "ExposureAuto", exposureAutoInitial);
	Arena::SetNodeValue<bool>(pDevice->GetNodeMap(), "AcquisitionFrameRateEnable", acquisitionFrameRateEnableInitial);
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

	std::cout << "Cpp_Exposure_Long\n";
	std::cout << "Image retrieval will take over 10 seconds without feedback -- proceed? ('y' to continue) ";
	char continueExample = 'a';
	std::cin >> continueExample;
	
	if (continueExample == 'y') 
	{
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

			// run example
			std::cout << "Commence example\n\n";
			ConfigureExposureMaximum(pDevice);
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
	}

	std::cout << "Press enter to complete\n";

	while (std::cin.get() != '\n')
		continue;
	std::cin.ignore();
	std::getchar();

	if (exceptionThrown)
		return -1;
	else
		return 0;
}
