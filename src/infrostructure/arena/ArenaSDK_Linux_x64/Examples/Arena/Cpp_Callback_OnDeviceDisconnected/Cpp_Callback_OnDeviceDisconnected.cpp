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

// Callback OnDeviceDisconnected: Introduction
//    This example demonstrates how to register a callback to get notified when a
//    device has disconnected. At first this example will enumerate devices then
//    if there is any device found it will regsiter an OnDeviceDisconnected
//    callback for first discovered device.  Next the program will wait until a
//    user inputs an exit command.  While this example waits for input feel free
//    to disconnect the device.  When it is disconnected, the
//    OnDeviceDisconnected callback will be triggered and it will print out the
//    device that was removed.

// =-=-=-=-=-=-=-=-=-
// =-=- EXAMPLE -=-=-
// =-=-=-=-=-=-=-=-=-

class MyOnDeviceDisconnectCb : public Arena::IDisconnectCallback
{
public:
	MyOnDeviceDisconnectCb() {};
	virtual ~MyOnDeviceDisconnectCb() {};

	void OnDeviceDisconnected(Arena::IDevice* pDevice)
	{
		//This is the callback that will be triggered when a device is disconnected
		//    lets pring out the device Id
		auto nodemap = pDevice->GetTLDeviceNodeMap();
		auto serial = Arena::GetNodeValue<GenICam::gcstring>(nodemap, "DeviceSerialNumber");

		std::cout << "Device with Serial: [" << serial << "] was disconnected." << std::endl;

		std::cout << "\nPress any key to continue" << std::endl;
	}
};

// demonstrates disconnect callbacks
// (1) registers OnDeviceDisconnect callback
// (2) Triggeres the OnDeviceDisconnect callback
// (2) prints information from disconnected device
// (4) deregisters OnDeviceDisconnect callback
void RegisterOnDeviceDisconnect(Arena::ISystem* pSystem, Arena::IDevice* pDevice)
{
	//Instantiate the callback you want to be called when a deice is disconnected
	MyOnDeviceDisconnectCb myCb;

	//Register the callback with the system
	pSystem->RegisterDeviceDisconnectCallback(pDevice, &myCb);

	std::cout << "Waiting for user to disconnect a device or press enter to continue\n";
	std::getchar();

	std::cout << "Check if device is connected:\n";

	bool isConnected = pDevice->IsConnected();

	if (isConnected == false)
	{
		std::cout << "Device is disconnected\n";
	}
	else
	{
		std::cout << "Device is connected\n";
	}

	// make sure we unregister the callbakcs before they go out of scope

	// delete individual disconnect callback
	// pSystem->DeregisterDeviceDisconnectCallback(&myCb);

	// delete all disconnect callbacks
	pSystem->DeregisterAllDeviceDisconnectCallbacks();
}

Arena::DeviceInfo SelectDevice(std::vector<Arena::DeviceInfo>& deviceInfos)
{
    if (deviceInfos.size() == 1)
    {
        std::cout  << "\n" << TAB1 << "Only one device detected: "  << deviceInfos[0].ModelName() << TAB1 << deviceInfos[0].SerialNumber() << TAB1 << deviceInfos[0].IpAddressStr() << ".\n";
        std::cout  << TAB1 << "Automatically selecting this device.\n";
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
			while (std::cin.get() != '\n');
			std::cout << TAB1 << "Invalid input. Please enter a number.\n";			
		}
		else if (selection <= 0 || selection > deviceInfos.size())
        {
            std::cout << TAB1 << "Invalid device selected. Please select a device in the range (1-" << deviceInfos.size() << ").\n";
        }
		
	} while (selection <= 0 || selection > deviceInfos.size());
    
    return deviceInfos[selection - 1];
}

// =-=-=-=-=-=-=-=-=-
// =- PREPARATION -=-
// =- & CLEAN UP =-=-
// =-=-=-=-=-=-=-=-=-

int main()
{
	// flag to track when an exception has been thrown
	bool exceptionThrown = false;

	std::cout << "Cpp_Callback_OnDeviceDisconnected";

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
		RegisterOnDeviceDisconnect(pSystem, pDevice);
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
