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
#include <cmath>

#define TAB1 "  "

// IP Config Manual
//    This example sets persistent IP on the camera. 5 parts:
//    1) Persistent IP address to 169.254.3.2
//    2) Subnet mask to 255.255.0.0
//    3) Enables persistent IP
//    4) Disables DHCP
//    5) Disables ARP conflict detection

// =-=-=-=-=-=-=-=-=-
// =-=- SETTINGS =-=-
// =-=-=-=-=-=-=-=-=-

// timeout for updating the device list
#define UPDATE_TIMEOUT 100


// =-=-=-=-=-=-=-=-=-
// =-=- EXAMPLE -=-=-
// =-=-=-=-=-=-=-=-=-
Arena::DeviceInfo SelectDevice(std::vector<Arena::DeviceInfo>& deviceInfos);

void EnumerateDeviceAndSetIpConfig(Arena::ISystem* pSystem)
{
	// enumerate device
	std::cout << "Enumerate device\n";

	pSystem = Arena::OpenSystem();
	pSystem->UpdateDevices(100);
	std::vector<Arena::DeviceInfo> deviceInfos = pSystem->GetDevices();

	if (deviceInfos.size() > 0)
	{
		Arena::DeviceInfo selectedDeviceInfo = SelectDevice(deviceInfos);
		Arena::IDevice* pDevice = pSystem->CreateDevice(selectedDeviceInfo);

		// Calculate IP addresses as integer
		//    Takes each octet and shifts them individually. Although IPv4
		//    addresses are 32 bits long, we must use 64-bit integers.
		int64_t address = (int64_t)(169 * std::pow(2, 24) + 254 * std::pow(2, 16) + 3 * std::pow(2, 8) + 2);
		int64_t subnet_mask = (int64_t)(255 * std::pow(2, 24) + 255 * std::pow(2, 16) + 0 * std::pow(2, 8) + 0);

		// Set IP configurations
		//    Set a specific IPv4 address, subnet mask, disable DHCP, enable
		//    persistent IP, and disable ARP conflict resolution
		std::cout << TAB1 << "Set persistent IP address" << std::endl;
		Arena::SetNodeValue<int64_t>(pDevice->GetNodeMap(),
			"GevPersistentIPAddress",
			address);

		std::cout << TAB1 << "Set persistent subnet mask" << std::endl;
		Arena::SetNodeValue<int64_t>(pDevice->GetNodeMap(),
			"GevPersistentSubnetMask",
			subnet_mask);

		std::cout << TAB1 << "Enabling persistent IP" << std::endl;
		Arena::SetNodeValue<bool>(pDevice->GetNodeMap(),
			"GevCurrentIPConfigurationPersistentIP",
			true);

		std::cout << TAB1 << "Disabling DHCP" << std::endl;
		Arena::SetNodeValue<bool>(pDevice->GetNodeMap(),
			"GevCurrentIPConfigurationDHCP",
			false);

		std::cout << TAB1 << "Disabling ARP conflict detection" << std::endl;
		Arena::SetNodeValue<bool>(pDevice->GetNodeMap(),
			"GevPersistentARPConflictDetectionEnable",
			false);

		pSystem->DestroyDevice(pDevice);
	}

	Arena::CloseSystem(pSystem);
	pSystem = nullptr;

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

	std::cout << "Cpp IP Config Manual\n";
	std::cout << "Example may overwrite device settings saved -- proceed? ('y' to continue) ";
	char continueExample = 'a';
	std::cin >> continueExample;

	if (continueExample == 'y')
	{
		Arena::ISystem* pSystem = nullptr;

		try
		{
			// run example
			std::cout << "Commence example\n\n";
			EnumerateDeviceAndSetIpConfig(pSystem);
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

		if (pSystem)
		{
			Arena::CloseSystem(pSystem);
		}
	}

	std::cout << "Press enter to complete\n";
	std::cin.ignore();
	std::getchar();

	if (exceptionThrown)
		return -1;
	else
		return 0;
}
