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

#include <algorithm> // for std::find_if

#define TAB1 "  "
#define TAB2 "    "
#define TAB3 "      "

// Enumeration: Introduction
//    This example introduces device enumeration. This includes opening and
//    closing the system, updating and retrieving the list of devices, searching
//    for devices, and creating and destroying a device.

// =-=-=-=-=-=-=-=-=-
// =-=- SETTINGS =-=-
// =-=-=-=-=-=-=-=-=-

// Update timeout
//    Timeout for updating the list of devices (in milliseconds). Because it is
//    unclear how many devices are expected, this timeout waits the entire
//    timeout, not returning early as devices are found.
#define SYSTEM_TIMEOUT 100

// size of input for IP Address
#define SIZE_IP 16

// =-=-=-=-=-=-=-=-=-
// =-=- EXAMPLE -=-=-
// =-=-=-=-=-=-=-=-=-

size_t SelectDevice(std::vector<Arena::DeviceInfo>& deviceInfos)
{
	if (deviceInfos.size() == 1)
	{
		std::cout << "\n"
				  << TAB1 << "Only one device detected: " << deviceInfos[0].ModelName() << TAB1 << deviceInfos[0].SerialNumber() << TAB1 << deviceInfos[0].IpAddressStr() << ".\n";
		std::cout << TAB1 << "Automatically selecting this device.\n";
		return 0;
	}

	std::cout << "\nSelect a device for serial to find:\n";
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
			while (std::cin.get() != '\n');
			std::cout << TAB1 << "Invalid input. Please enter a number.\n";
		}
		else if (selection <= 0 || selection > deviceInfos.size())
		{
			std::cout << TAB1 << "Invalid device selected. Please select a device in the range (1-" << deviceInfos.size() << ").\n";
		}

	} while (selection <= 0 || selection > deviceInfos.size());

	return selection - 1;
}

// enumerates device(s)
// (1) updates device list
// (2) gets device list
// (3) prints device information
// (4) checks connection for device
// (5) prints connection information
void EnumerateDevices()
{
	// Open system
	//    Open the system in order to update and retrieve the device list.
	//    Opening the system is the entry point to the rest of the Arena SDK.
	//    Only one system can be opened at a time.
	std::cout << TAB1 << "Open system\n";

	Arena::ISystem* pSystem = Arena::OpenSystem();

	// Update and retrieve the device list
	//    Update and retrieve the list of connected devices. Failing to update
	//    results in an empty list being retrieved, even if devices are
	//    connected. Getting the devices does not update or affect the list at
	//    all.
	std::cout << TAB1 << "Update and retrieve list of devices\n";

	pSystem->UpdateDevices(SYSTEM_TIMEOUT);
	std::vector<Arena::DeviceInfo> deviceInfos = pSystem->GetDevices();

	// get information on connected devices save serial number to demonstrate
	// search later in the example
	std::cout << TAB1 << "Get device information\n";
	GenICam::gcstring serialToFind = "00000";

	size_t selection = SelectDevice(deviceInfos);

	for (size_t i = 0; i < deviceInfos.size(); i++)
	{
		// save the serial number from selected camera
		if (i == selection)
		{
			serialToFind = deviceInfos[selection].SerialNumber();
		}

		// Display device information
		//    The list of devices is kept as a std::vector of device information
		//    objects. Device information objects provide access to devices'
		//    discovery information. This information is provided without having
		//    to create the device and includes information related to
		//    identification and network settings.
		std::cout << TAB2 << "Information for device " << i + 1;

		Arena::DeviceInfo deviceInfo = deviceInfos[i];

		GenICam::gcstring vendor = deviceInfo.VendorName();
		GenICam::gcstring model = deviceInfo.ModelName();
		GenICam::gcstring serial = deviceInfo.SerialNumber();
		GenICam::gcstring macStr = deviceInfo.MacAddressStr();
		GenICam::gcstring ipStr = deviceInfo.IpAddressStr();

		std::cout << " (" << vendor << "; " << model << "; serial " << serial << "; MAC " << macStr << "; IP " << ipStr << ")\n";
	}

	// Search for device
	//    Search for a specific device using the std library. Providing the
	//    device list as a vector allows for the use of all std functionality
	//    with regards to searching, sorting, and modifying the list.
	std::cout << TAB1 << "Search for device with serial " << serialToFind << "\n";

	auto it = std::find_if(
		deviceInfos.begin(),
		deviceInfos.end(),
		[&serialToFind](Arena::DeviceInfo deviceInfo) {
			return deviceInfo.SerialNumber() == serialToFind;
		});

	if (it != deviceInfos.end())
	{
		std::cout << TAB2 << "Device found\n";

		// Create device
		//    Create device in order to configure it and grab images. Creating a
		//    device requires a device information object to be passed in.
		//    Created objects need to be destroyed. A device can only be created
		//    once per process, and can only be opened with read-write access
		//    once.
		std::cout << TAB3 << "Create device\n";

		Arena::IDevice* pDevice = pSystem->CreateDevice(*it);

		// Destroy device
		//    Destroy the device before closing the system. Destroying devices cleans
		//    up allocated memory.
		std::cout << TAB3 << "Destroy device\n";

		pSystem->DestroyDevice(pDevice);
	}

	// Close system
	//    Close the system to end use of Arena SDK. Releasing the system cleans
	//    up allocated memory. Failing to close the system causes memory to leak.
	std::cout << TAB1 << "Close system\n";

	Arena::CloseSystem(pSystem);
}

// =-=-=-=-=-=-=-=-=-
// =- PREPARATION -=-
// =- & CLEAN UP =-=-
// =-=-=-=-=-=-=-=-=-

int main()
{
	// flag to track when an exception has been thrown
	bool exceptionThrown = false;

	std::cout << "Cpp_Enumeration\n";

	try
	{
		// run example
		std::cout << "Commence example\n\n";
		EnumerateDevices();
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
