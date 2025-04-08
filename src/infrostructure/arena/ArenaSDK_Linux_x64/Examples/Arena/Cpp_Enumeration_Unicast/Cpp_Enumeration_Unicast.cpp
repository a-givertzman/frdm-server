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
#define TAB3 "      "

// Enumeration Unicast: Introduction
//    This example introduces adding unicast device. This includes opening and
//    closing the system, updating and retrieving the list of devices, adding
//    unicast devices using the IP address for the device, and chekcing connections
//    of the devices

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

// enumerates device(s)
// (1) updates device list
// (2) gets device list
// (3) creates device
// (4) prints device information
// (5) checks connection for device
// (6) prints connection information
// (7) destroys device
void EnumerateDevices(Arena::ISystem* pSystem)
{
	pSystem->UpdateDevices(SYSTEM_TIMEOUT);
	std::vector<Arena::DeviceInfo> deviceInfos = pSystem->GetDevices();

	for (size_t i = 0; i < deviceInfos.size(); i++)
	{
		// display device information
		std::cout << TAB2 << "Information for device " << i;

		Arena::DeviceInfo deviceInfo = deviceInfos[i];

		GenICam::gcstring vendor = deviceInfo.VendorName();
		GenICam::gcstring model = deviceInfo.ModelName();
		GenICam::gcstring serial = deviceInfo.SerialNumber();
		GenICam::gcstring macStr = deviceInfo.MacAddressStr();
		GenICam::gcstring ipStr = deviceInfo.IpAddressStr();

		std::cout << " (" << vendor << "; " << model << "; serial " << serial << "; MAC " << macStr << "; IP " << ipStr << ")\n";

		// create device
		Arena::IDevice* pDevice = pSystem->CreateDevice(deviceInfos[i]);
		
		// display connection information
		std::cout << TAB2 << "Check connection" << ": ";
		bool isConnected = pDevice->IsConnected();

		if (isConnected)
		{
			std::cout << "Device " << i << " is connected";
			
			GenApi::CValuePtr pPixelFormat = pDevice->GetNodeMap()->GetNode("PixelFormat");
			GenICam::gcstring pixelFormat = pPixelFormat->ToString();
			GenApi::CValuePtr pFrameRate = pDevice->GetNodeMap()->GetNode("AcquisitionFrameRate");
			GenICam::gcstring frameRate = pFrameRate->ToString();
			
			std::cout << " (Pixel Format: " << pixelFormat << "; Frame Rate: " << frameRate << "fps" << ")\n\n";
		}
		else
		{
			std::cout << "Device " << i << " is not connected\n\n";
		}
		
		// destroy device
		pSystem->DestroyDevice(pDevice);
	}	
}

// adds unicast device(s)
// (1) opens system
// (2) enumerates devices before adding unicast device
// (3) takes IP address for device to be added from user
// (4) adds unicast discovery device(s)
// (5) enumerates devices after adding unicast device(s)
// (6) closes system
void AddUnicastDevice()
{
	// open system
	std::cout << TAB1 << "Open system\n";
	Arena::ISystem* pSystem = Arena::OpenSystem();

	// enumerate devices before adding unicast device(s) 
	std::cout << TAB1 << "Device list before adding unicast device(s)\n";
	EnumerateDevices(pSystem);

	// variable to store input ip address
	char ip[SIZE_IP];

	// stay in loop until exit
	while (true)
	{
		std::cout << "\n" << TAB2 << "Input IP for device to be added ('x' to exit)\n" << TAB2;
		std::cin.getline(ip, SIZE_IP);

		// exit manually on 'x'
		if (ip[0] == 'x')
		{
			std::cout << TAB2 << "Exit\n\n";
			break;
		}

		// Add a unicast discovery device
		//    registers an IP address for a device on a different subnet 
		//    than the host. Registered devices will be enumerated using 
		//    unicast discovery messages. The list of remote devices will 
		//    persist until they are removed using RemoveUnicastDiscoveryDevice() 
		//    or until the application terminates. Unicast discovery's will be 
		//    sent when UpdateDevices() is called.
		std::cout << TAB1 << "Add device with IP: " << ip << "\n";
		pSystem->AddUnicastDiscoveryDevice(ip);
			
		// reset input
		memset(ip, 0, SIZE_IP);	
	}
	// enumerate devices after adding unicast device(s)
	std::cout << TAB1 << "Device list after adding unicast device(s)\n";
	EnumerateDevices(pSystem);

	// close system
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

	std::cout << "Eunmeration_Unicast\n";

	try
	{
		// run example
		std::cout << "Commence example\n\n";
		AddUnicastDevice();
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
