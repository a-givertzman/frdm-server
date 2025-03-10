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
// #include "SaveApi.h"

#define TAB1 "  "
#define TAB2 "    "

// Acquisition: TCP
//    Some 10G higher bandwidth LUCID cameras support TCP streaming.The TCP protocol
//    implements a reliable connection-based stream at the hardware level,
//    eliminating the need for a software-based packet resend mechanism.

// =-=-=-=-=-=-=-=-=-
// =-=- SETTINGS =-=-
// =-=-=-=-=-=-=-=-=-

// image timeout
#define TIMEOUT 2000

// number of images to grab
#define NUM_IMAGES 25

// pixel format
#define PIXEL_FORMAT BGR8

// =-=-=-=-=-=-=-=-=-
// =-=- EXAMPLE -=-=-
// =-=-=-=-=-=-=-=-=-

void PrintInfo(Arena::IDevice* pDevice)
{
	int64_t width = Arena::GetNodeValue<int64_t>(pDevice->GetNodeMap(), "Width");
	int64_t height = Arena::GetNodeValue<int64_t>(pDevice->GetNodeMap(), "Height");

	std::cout << TAB1 << "Image (w,h) = (" << width << "," << height << ") " << std::endl;
}

// demonstrates acquisition
// (1) sets acquisition mode
// (2) sets buffer handling mode
// (3) set transport stream protocol to TCP
// (4) starts the stream
// (5) gets a number of images
// (6) prints information from images
// (7) requeues buffers
// (8) stops the stream
void AcquireImages(Arena::IDevice* pDevice)
{
	PrintInfo(pDevice);
	// get node values that will be changed in order to return their values at
	// the end of the example
	GenICam::gcstring acquisitionModeInitial = Arena::GetNodeValue<GenICam::gcstring>(pDevice->GetNodeMap(), "AcquisitionMode");

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

	// The TransportStreamProtocol node can tell the camera to use the TCP datastream engine. When
	//    set to TCP - Arena will switch to using the TCP datastream engine. 
	//    There is no further necessary configuration, though to achieve maximum throughput 
	//    users may want to set the "DeviceLinkThroughputReserve" to 0 and 
	//    also set the stream channel packet delay "GevSCPD" to 0.

	if (GenApi::IsImplemented(pDevice->GetNodeMap()->GetNode("TransportStreamProtocol")))
	{
		// get node value
		GenICam::gcstring transportStreamProtocolInitial = Arena::GetNodeValue<GenICam::gcstring>(pDevice->GetNodeMap(), "TransportStreamProtocol");

		std::cout << TAB1 << "Set Transport Stream Protocol to TCP\n";

		Arena::SetNodeValue<GenICam::gcstring>(
			pDevice->GetNodeMap(),
			"TransportStreamProtocol",
			"TCP");

		// start stream
		std::cout << TAB1 << "Start stream\n";

		pDevice->StartStream();

		// get images
		std::cout << TAB1 << "Getting " << NUM_IMAGES << " images\n";

		for (int i = 0; i < NUM_IMAGES; i++)
		{
			// get image
			std::cout << TAB2 << "Get image " << i;

			Arena::IImage* pImage = pDevice->GetImage(TIMEOUT);

			// get image information
			size_t size = pImage->GetSizeFilled();
			size_t width = pImage->GetWidth();
			size_t height = pImage->GetHeight();
			GenICam::gcstring pixelFormat = GetPixelFormatName(static_cast<PfncFormat>(pImage->GetPixelFormat()));
			uint64_t timestampNs = pImage->GetTimestampNs();

			std::cout << " (" << size << " bytes; " << width << "x" << height << "; " << pixelFormat << "; timestamp (ns): " << timestampNs << ")";

			// requeue image buffer
			std::cout << " and requeue\n";

			pDevice->RequeueBuffer(pImage);
		}

		// stop stream
		std::cout << TAB1 << "Stop stream\n";

		pDevice->StopStream();

		// return node to its initial value
		Arena::SetNodeValue<GenICam::gcstring>(pDevice->GetNodeMap(), "TransportStreamProtocol", transportStreamProtocolInitial);

	} else {

		std::cout << TAB1 << "\nConnected camera does not support TCP stream\n";
	}

	// return node to its initial value
	Arena::SetNodeValue<GenICam::gcstring>(pDevice->GetNodeMap(), "AcquisitionMode", acquisitionModeInitial);
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

	std::cout << TAB1 << "\nSelect device:\n";
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

	std::cout << "Cpp_Acquisition_TCP";

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
		AcquireImages(pDevice);
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
