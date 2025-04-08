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

// Simple Acquisition
//    This example demonstrates the most basic code path of acquiring an image
//    using Arena SDK. This includes device enumeration, image acquisition, and
//    clean up.

// =-=-=-=-=-=-=-=-=-
// =-=- SETTINGS =-=-
// =-=-=-=-=-=-=-=-=-

// timeout for updating the device list
#define UPDATE_TIMEOUT 100

// timeout for grabbing an image
#define IMAGE_TIMEOUT 2000

// =-=-=-=-=-=-=-=-=-
// =-=- EXAMPLE -=-=-
// =-=-=-=-=-=-=-=-=-

Arena::DeviceInfo SelectDevice(std::vector<Arena::DeviceInfo>& deviceInfos);

// demonstrates simplest route to acquiring an image
// (1) enumerates device
// (2) acquires image
// (3) cleans up

void EnumerateDeviceAndAcquireImage()
{
	// Enumerate device
	//    Starting Arena just requires opening the system. From there, update and
	//    grab the device list, and create the device. Notice that failing to
	//    update the device list will return an empty list, even if devices are
	//    connected.
	std::cout << "Enumerate device";

	Arena::ISystem* pSystem = Arena::OpenSystem();
	pSystem->UpdateDevices(100);
	std::vector<Arena::DeviceInfo> deviceInfos = pSystem->GetDevices();
	
	if (deviceInfos.size() > 0)
	{
		Arena::DeviceInfo selectedDeviceInfo = SelectDevice(deviceInfos);

		Arena::IDevice* pDevice = pSystem->CreateDevice(selectedDeviceInfo);

		// enable stream auto negotiate packet size
		Arena::SetNodeValue<bool>(pDevice->GetTLStreamNodeMap(), "StreamAutoNegotiatePacketSize", true);

		// enable stream packet resend
		Arena::SetNodeValue<bool>(pDevice->GetTLStreamNodeMap(), "StreamPacketResendEnable", true);

		// Acquire image
		//    Once a device is created, it is only a single call to acquire an
		//    image. The timeout must be larger than the exposure time.
		std::cout << TAB1 << "Acquire image\n";

		pDevice->StartStream();
		Arena::IImage* pImage = pDevice->GetImage(IMAGE_TIMEOUT);

		// Clean up
		//    Clean up each of the 3 objects in reverse order: image, device, and
		//    system. The list of devices is a standard vector, so it cleans
		//    itself up at the end of scope.
		std::cout << TAB1 << "Clean up Arena\n";

		pDevice->RequeueBuffer(pImage);
		pDevice->StopStream();
		pSystem->DestroyDevice(pDevice);
	}

	Arena::CloseSystem(pSystem);
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

	std::cout << "Cpp_SimpleAcquisition\n";

	try
	{
		// run example
		std::cout << "Commence example\n\n";
		EnumerateDeviceAndAcquireImage();
		std::cout << "\nExample complete\n";
	}
	catch (GenICam::GenericException& ge)
	{
		std::cout << "\nGenICam exception thrown: " << ge.what() << "\n";
		exceptionThrown = true;
	}
	catch (std::exception& ex)
	{
		std::cout << "Standard exception thrown: " << ex.what() << "\n";
		exceptionThrown = true;
	}
	catch (...)
	{
		std::cout << "Unexpected exception thrown\n";
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
