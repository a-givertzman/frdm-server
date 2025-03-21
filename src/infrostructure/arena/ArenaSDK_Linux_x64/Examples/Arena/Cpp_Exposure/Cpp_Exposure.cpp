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

#define TAB1 "  "
#define TAB2 "    "

// Exposure: Introduction
//    This example introduces the exposure feature. An image's exposure time
//    refers to the amount of time that a device's sensor is exposed to a scene
//    before the data is collected. The exposure can be handled automatically or
//    manually.

// =-=-=-=-=-=-=-=-=-
// =-=- SETTINGS =-=-
// =-=-=-=-=-=-=-=-=-

// Exposure time
//    Time that the sensor will be exposed when grabbing an image (in
//    microseconds). Exposure time depends on the application, but generally the
//    less available the light, the higher the exposure time.
#define EXPOSURE_TIME 4000.0

// Image timeout
//    Timeout for grabbing images (in milliseconds). Make the timeout longer 
//    than the exposure time to avoid timeout exceptions.
#define TIMEOUT 2000

// number of images to grab
#define NUM_IMAGES 25

// =-=-=-=-=-=-=-=-=-
// =-=- EXAMPLE -=-=-
// =-=-=-=-=-=-=-=-=-

// demonstrates basic exposure configuration
// (1) disables automatic exposure
// (2) gets exposure node
// (3) ensures exposure above min/below max
// (4) sets exposure
// (5) acquires images
void ConfigureExposureAndAcquireImages(Arena::IDevice* pDevice)
{
	// get node values that will be changed in order to return their values at
	// the end of the example
	GenICam::gcstring exposureAutoInitial = Arena::GetNodeValue<GenICam::gcstring>(pDevice->GetNodeMap(), "ExposureAuto");
	double exposureTimeInitial = Arena::GetNodeValue<double>(pDevice->GetNodeMap(), "ExposureTime");

	// Disable automatic exposure
	//    Disable automatic exposure before setting an exposure time. Automatic
	//    exposure controls whether the exposure time is set manually or
	//    automatically by the device. Setting automatic exposure to 'Off' stops
	//    the device from automatically updating the exposure time while
	//    streaming.
	std::cout << TAB1 << "Disable automatic exposure\n";

	Arena::SetNodeValue<GenICam::gcstring>(
		pDevice->GetNodeMap(),
		"ExposureAuto",
		"Off");

	// Get exposure time node
	//    In order to get the exposure time maximum and minimum values, get the
	//    exposure time node. Failed attempts to get a node return null, so check
	//    that the node exists. Because we expect to set its value, check
	//    that the exposure time node is writable.
	std::cout << TAB1 << "Get exposure time node\n";

	GenApi::CFloatPtr pExposureTime = pDevice->GetNodeMap()->GetNode("ExposureTime");

	if (!pExposureTime)
	{
		throw GenICam::GenericException("ExposureTime node not found", __FILE__, __LINE__);
	}

	if (!GenApi::IsWritable(pExposureTime))
	{
		throw GenICam::GenericException("ExposureTime node not writable", __FILE__, __LINE__);
	}

	// Set exposure time
	//    Before setting the exposure time, check that the new exposure time is not 
	//    outside of the exposure time's acceptable range. If the value is above the 
	//    maximum or below the minimum, update the value to be within range. Lastly, 
	//    set the new exposure time.
	double exposureTime = EXPOSURE_TIME;

	if (exposureTime < pExposureTime->GetMin())
	{
		exposureTime = pExposureTime->GetMin();
	}

	if (exposureTime > pExposureTime->GetMax())
	{
		exposureTime = pExposureTime->GetMax();
	}

	std::cout << TAB1 << "Set exposure time to " << exposureTime << " " << pExposureTime->GetUnit() << "\n";

	pExposureTime->SetValue(exposureTime);

	// Enable stream auto negotiate packet size
	//    Setting the stream packet size is done before starting the stream.
	//    Setting the stream to automatically negotiate packet size instructs the
	//    camera to receive the largest packet size that the system will allow.
	//    This generally increases frame rate and results in fewer interrupts per
	//    image, thereby reducing CPU load on the host system. Ethernet settings
	//    may also be manually changed to allow for a larger packet size.
	std::cout << TAB1 << "Enable stream to auto negotiate packet size\n";
	Arena::SetNodeValue<bool>(pDevice->GetTLStreamNodeMap(), "StreamAutoNegotiatePacketSize", true);

	// Enable stream packet resend
	//    Enable stream packet resend before starting the stream. Images are sent
	//    from the camera to the host in packets using UDP protocol, which
	//    includes a header image number, packet number, and timestamp
	//    information. If a packet is missed while receiving an image, a packet
	//    resend is requested and this information is used to retrieve and
	//    redeliver the missing packet in the correct order.
	std::cout << TAB1 << "Enable stream packet resend\n";
	Arena::SetNodeValue<bool>(pDevice->GetTLStreamNodeMap(), "StreamPacketResendEnable", true);

	// Grab images with new exposure time
	//    When getting images, ensure the timeout is longer than the exposure
	//    time to avoid receiving a timeout exception. The timeout is in
	//    milliseconds while the exposure time is in microseconds, so a timeout
	//    of 2000 is quite a bit longer than an exposure time of 4000.
	std::cout << TAB1 << "Getting " << NUM_IMAGES << " images\n";

	pDevice->StartStream();
	for (int i = 0; i < NUM_IMAGES; i++)
	{
		Arena::IImage* pImage = pDevice->GetImage(TIMEOUT);
		std::cout << TAB2 << "Image " << i << " (timestamp " << pImage->GetTimestampNs() << " ns)\n";
		pDevice->RequeueBuffer(pImage);
	}
	pDevice->StopStream();

	// return nodes to their initial values
	Arena::SetNodeValue<double>(pDevice->GetNodeMap(), "ExposureTime", exposureTimeInitial);
	Arena::SetNodeValue<GenICam::gcstring>(pDevice->GetNodeMap(), "ExposureAuto", exposureAutoInitial);
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

	std::cout << "Cpp_Exposure";

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
		ConfigureExposureAndAcquireImages(pDevice);
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
