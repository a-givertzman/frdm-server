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
#include <vector>
#include <thread>
#include <iomanip> // For setw, setfill


#if defined(__linux__)
#include <unistd.h>
#endif

#define TAB1 "  "
#define TAB2 "    "

// Callback: Image Callbacks
//    This example demonstrates configuring an image callback for a device. Once
//    a callback is registered and the device is streaming, the user-implemented
//    OnImage() function will be called. OnImage will receive a pointer to the
//    image and a pointer to optional user-specified callback data. OnImage
//    will display the frame id and timestamp of the image before returning.

// =-=-=-=-=-=-=-=-=-
// =-=- SETTINGS =-=-
// =-=-=-=-=-=-=-=-=-

// number of seconds to stream for
#define STREAM_TIME_SEC 5

#if defined(_WIN32)
#define portable_sleep(x) Sleep(x * 1000)
#elif defined(__linux__)
#define portable_sleep(x) sleep(x)
#endif

// =-=-=-=-=-=-=-=-=-
// =-=- EXAMPLE -=-=-
// =-=-=-=-=-=-=-=-=-

// To use callbacks, derive a callback object from Arena::IImageCallback
//    The Arena library will call the user-implemented OnImage() when an image
//    has been received.
class ImageCallback : public Arena::IImageCallback
{
public:
	ImageCallback(std::string serialNumber) :
		m_serialNumber(serialNumber), m_imageCounter(0){};

	~ImageCallback()
	{
	}

	void OnImage(Arena::IImage* pImage)
	{
		static uint64_t lastTimestamp = 0;

		uint64_t currTimestamp = pImage->GetTimestamp();

		double diffMilliseconds = 0.0;

		if (lastTimestamp != 0)
		{
			diffMilliseconds = (currTimestamp - lastTimestamp) / 1000000.0;
		}

		std::cout << "Serial: [" << m_serialNumber << "], Image#: [" << std::setw(4) << std::setfill('0') << ++m_imageCounter << "], FrameId: [" << std::setw(4) << std::setfill('0') << pImage->GetFrameId() << "], TimeStamp: [" << currTimestamp << "], Diff: [" << diffMilliseconds << " ms]\n";

		lastTimestamp = currTimestamp;
	}

private:
	std::string m_serialNumber;
	uint64_t m_imageCounter;
};

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

	std::cout << "Cpp_Callback_ImageCallbacks";

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

		std::cout << "Using camera with serial number: " << deviceInfos[0].SerialNumber() << std::endl;

		// allocate the image callback handler object
		ImageCallback* pCallbackHandler = new ImageCallback(std::string(deviceInfos[0].SerialNumber()));

		// register the callback handler to the device
		pDevice->RegisterImageCallback(pCallbackHandler);

		// enable stream auto negotiate packet size
		Arena::SetNodeValue<bool>(pDevice->GetTLStreamNodeMap(), "StreamAutoNegotiatePacketSize", true);

		// enable stream packet resend
		Arena::SetNodeValue<bool>(pDevice->GetTLStreamNodeMap(), "StreamPacketResendEnable", true);

		pDevice->StartStream();
		portable_sleep(STREAM_TIME_SEC);
		pDevice->StopStream();

		// deregister the callback handler
		pDevice->DeregisterImageCallback(pCallbackHandler);

		// free the callback handler object
		delete pCallbackHandler;

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
