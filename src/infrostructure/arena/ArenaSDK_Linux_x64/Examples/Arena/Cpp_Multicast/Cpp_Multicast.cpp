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
#include <chrono>

#define TAB1 "  "
#define TAB2 "    "

// Multicast
//    This example demonstrates multicasting from the master's perspective.
//    Multicasting allows for the streaming of images and events to multiple
//    destinations. Multicasting requires nearly the same steps for both masters
//    and listeners. The only difference, as seen below, is that device features
//    can only be set by the master.

// =-=-=-=-=-=-=-=-=-
// =-=- SETTINGS =-=-
// =-=-=-=-=-=-=-=-=-

// image timeout
#define TIMEOUT 2000

// Length of time to grab images (sec)
//    Note that the listener must be started while the master is still streaming,
//    and that the listener will not receive any more images once the master
//    stops streaming.
#define NUM_SECONDS 20

// =-=-=-=-=-=-=-=-=-
// =-=- EXAMPLE -=-=-
// =-=-=-=-=-=-=-=-=-
// (1) enable multicast
// (2) prepare settings on master, not on listener
// (3) stream regularly
void AcquireImages(Arena::IDevice* pDevice)
{
	// get node values that will be changed in order to return their values at
	// the end of the example
	GenICam::gcstring acquisitionModeInitial = Arena::GetNodeValue<GenICam::gcstring>(pDevice->GetNodeMap(), "AcquisitionMode");

	// Enable multicast
	//    Multicast must be enabled on both the master and listener. A small
	//    number of transport layer features will remain writable even though a
	//    device's access mode might be read-only.
	std::cout << TAB1 << "Enable multicast\n";

	Arena::SetNodeValue<bool>(
		pDevice->GetTLStreamNodeMap(),
		"StreamMulticastEnable",
		true);

	// Prepare settings on master, not on listener
	//    Device features must be set on the master rather than the listener.
	//    This is because the listener is opened with a read-only access mode.
	GenICam::gcstring deviceAccessStatus = Arena::GetNodeValue<GenICam::gcstring>(
		pDevice->GetTLDeviceNodeMap(),
		"DeviceAccessStatus");

	// master
	if (deviceAccessStatus == "ReadWrite")
	{
		std::cout << TAB1 << "Host streaming as 'master'\n";

		// set acquisition mode
		std::cout << TAB2 << "Set acquisition mode to 'Continuous'\n";

		Arena::SetNodeValue<GenICam::gcstring>(
			pDevice->GetNodeMap(),
			"AcquisitionMode",
			"Continuous");

		// enable stream auto negotiate packet size
		Arena::SetNodeValue<bool>(pDevice->GetTLStreamNodeMap(), "StreamAutoNegotiatePacketSize", true);

		// enable stream packet resend
		Arena::SetNodeValue<bool>(pDevice->GetTLStreamNodeMap(), "StreamPacketResendEnable", true);
	}

	// listener
	else
	{
		std::cout << TAB1 << "Host streaming as 'listener'\n";
	}	

	// start stream
	std::cout << TAB1 << "Start stream\n";

	pDevice->StartStream();

	// define image count to detect if all images are not received
	int imageCount = 0;
	int unreceivedImageCount = 0;

	// get images
	std::cout << TAB1 << "Getting images for " << NUM_SECONDS << " seconds\n";

	Arena::IImage* pImage = NULL;

	// define start and latest time for timed image acquisition
	auto startTime = std::chrono::steady_clock::now();
	auto latestTime = std::chrono::steady_clock::now();

	while (std::chrono::duration_cast<std::chrono::seconds>(latestTime - startTime).count() < NUM_SECONDS)
	{
		// update time
		latestTime = std::chrono::steady_clock::now();		

		// get image
		imageCount++;
		try
		{
			pImage = pDevice->GetImage(TIMEOUT);
		}
		catch (GenICam::TimeoutException&)
		{
			std::cout << TAB2 << "No image received\n";
			unreceivedImageCount++;
			continue;
		}

		// Print identifying information
		//    Using the frame ID and timestamp allows for the comparison of
		//    images between multiple hosts.
		std::cout << TAB2 << "Image retrieved";

		uint64_t frameId = pImage->GetFrameId();
		uint64_t timestampNs = pImage->GetTimestampNs();

		std::cout << " (frame ID " << frameId << "; timestamp (ns): " << timestampNs << ")";

		// requeue buffer
		std::cout << " and requeue\n";
		pDevice->RequeueBuffer(pImage);
	}

	if (unreceivedImageCount == imageCount)
	{
		std::cout << "\nNo images were received, this can be caused by firewall or VPN settings\n";
		std::cout << "Please add the application to firewall exception\n\n";
	}
	// stop stream
	std::cout << TAB1 << "Stop stream\n";

	pDevice->StopStream();

	// return node to its initial value
	if (deviceAccessStatus == "ReadWrite")
	{
		Arena::SetNodeValue<GenICam::gcstring>(pDevice->GetNodeMap(), "AcquisitionMode", acquisitionModeInitial);
	}
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

	std::cout << "Cpp_Multicast";

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
