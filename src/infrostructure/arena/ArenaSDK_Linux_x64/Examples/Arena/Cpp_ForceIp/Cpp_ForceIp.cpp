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
#include <algorithm> // for find_if


#define TAB1 "  "
#define TAB2 "    "

// Force IP
//    This example demonstrates how to force network settings. It does this by
//    adding 1 to the final octet of the IP address. It leaves the subnet mask
//    and default gateway unchanged, although the same method is used to change these
//    as well.

// =-=-=-=-=-=-=-=-=-
// =-=- SETTINGS =-=-
// =-=-=-=-=-=-=-=-=-

// update timeout
#define TIMEOUT 100

// =-=-=-=-=-=-=-=-=-
// =-=- EXAMPLE -=-=-
// =-=-=-=-=-=-=-=-=-

Arena::DeviceInfo SelectDevice(std::vector<Arena::DeviceInfo>& deviceInfos);

// demonstrates forcing a new IP address
// (1) discovers devices and gets information
// (2) prepares IP address by adding 1 to the last octet
// (3) forces new IP address
// (4) discovers devices and gets information again
void ForceNetworkSettings(Arena::ISystem* pSystem)
{
	// discover devices
	std::cout << TAB1 << "Discover devices\n";

	pSystem->UpdateDevices(TIMEOUT);
	std::vector<Arena::DeviceInfo> deviceInfos = pSystem->GetDevices();

	// Get device information
	//    Forcing the IP address requires a device's MAC address to specify the
	//    device. This example grabs the IP address, subnet mask, and default
	//    gateway as well to display changes and return the device to its
	//    original IP address.
	std::cout << TAB1 << "Get device information\n";

	Arena::DeviceInfo selectedDeviceInfo = SelectDevice(deviceInfos);

	const uint64_t macAddress = selectedDeviceInfo.MacAddress();
	const uint32_t ipAddress = selectedDeviceInfo.IpAddress();
	const uint32_t subnetMask = selectedDeviceInfo.SubnetMask();
	const uint32_t defaultGateway = selectedDeviceInfo.DefaultGateway();

	std::cout << TAB2 << "MAC " << selectedDeviceInfo.MacAddressStr() << "\n";
	std::cout << TAB2 << "IP " << selectedDeviceInfo.IpAddressStr() << "\n";
	std::cout << TAB2 << "Subnet " << selectedDeviceInfo.SubnetMaskStr() << "\n";
	std::cout << TAB2 << "Gateway " << selectedDeviceInfo.DefaultGatewayStr() << "\n";

	// Add 1 to IP address
	//    The new IP address takes the current IP address and adds 1 to the final
	//    octet. If the final octet is 254, the final octet is set to 1 (to avoid
	//    0 and 255).
	uint32_t ipAddressToSet;

	if ((ipAddress & 0x000000FF) == 0x000000FE)
	{
		ipAddressToSet = selectedDeviceInfo.IpAddress() & 0xFFFFFF01;
	}
	else
	{
		ipAddressToSet = selectedDeviceInfo.IpAddress() + 0x00000001;
	}

	std::cout << TAB1 << "Prepare new IP address " << (ipAddressToSet >> 24 & 0xFF) << "." << (ipAddressToSet >> 16 & 0xFF) << "." << (ipAddressToSet >> 8 & 0xFF) << "." << (ipAddressToSet & 0xFF) << "\n";

	// Force network settings
	//    Forcing the IP uses the MAC address to specify a device and forces the
	//    IP address, subnet mask, and default gateway. In this case, the IP
	//    address is being changed while the subnet mask and default gateway
	//    remain the same.
	std::cout << TAB1 << "Force network settings\n";

	pSystem->ForceIp(macAddress, ipAddressToSet, subnetMask, defaultGateway);

	// Discover devices again
	//    Once a device has been forced, it needs to be rediscovered with its new
	//    network settings. This is especially important if moving on to
	//    configuration and acquisition.
	std::cout << TAB1 << "Discover devices again\n";

	pSystem->UpdateDevices(TIMEOUT);
	deviceInfos = pSystem->GetDevices();

	// Get device information again
	//    Notice that the MAC address, subnet mask, and default gateway all
	//    retain their initial values while the last octet of the IP address
	//    has been incremented by 1.
	std::cout << TAB1 << "Get device information again\n";

	auto it = std::find_if(
		deviceInfos.begin(),
		deviceInfos.end(),
		[&macAddress](Arena::DeviceInfo deviceInfo) {
			return deviceInfo.MacAddress() == macAddress;
		});

	std::cout << TAB2 << "MAC " << (*it).MacAddressStr() << "\n";
	std::cout << TAB2 << "IP " << (*it).IpAddressStr() << "\n";
	std::cout << TAB2 << "Subnet " << (*it).SubnetMaskStr() << "\n";
	std::cout << TAB2 << "Gateway " << (*it).DefaultGatewayStr() << "\n";

	// return IP address to its initial value
	pSystem->ForceIp(macAddress, ipAddress, subnetMask, defaultGateway);
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

	std::cout << "Cpp_ForceIp\n";

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

		// run example
		std::cout << "Commence example\n\n";
		ForceNetworkSettings(pSystem);
		std::cout << "\nExample complete\n";

		// clean up example
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
