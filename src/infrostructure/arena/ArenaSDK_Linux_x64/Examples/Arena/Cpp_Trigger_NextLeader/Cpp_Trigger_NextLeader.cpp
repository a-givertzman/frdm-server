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
#include "GenTL.h"

#ifdef __linux__
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wunused-but-set-variable"
#endif

#include "GenICam.h"

#ifdef __linux__
#pragma GCC diagnostic pop
#endif

#include "ArenaApi.h"

#define TAB1 "  "
#define TAB2 "    "
#define TAB3 "      "

// Trigger: WaitForNextLeader
//    WaitForNextLeader feature uses a first packet of every incoming image to
//    inform users that the camera is done integrating. This is an approximation
//    of what the Exposure End event does, but it simplifies the process because
//    we don't need to start a whole new event channel, and reuses data that has
//    to be transmitted already for the purpose of delivering the image to the
//    user.

// =-=-=-=-=-=-=-=-=-
// =-=- SETTINGS =-=-
// =-=-=-=-=-=-=-=-=-

// image timeout
#define TIMEOUT_MILLISEC 2000

// number of images to capture
#define NUMBER_IMAGES_TO_CAPTURE 10

// Wait for the next leader for each triggered image or every 3rd image.
//    In some cases, the user might need to reset WaitForNextleader device state
#define WAIT_FOR_LEADER_EVERY_3RD_IMAGE false

// =-=-=-=-=-=-=-=-=-
// =-=- EXAMPLE -=-=-
// =-=-=-=-=-=-=-=-=-

// demonstrates trigger configuration and WaitForNextleader use
// (1) sets trigger mode, source, and selector
// (2) starts stream
// (3) waits until trigger is armed
// (4) triggers image
// (5) wait for next leader
// (6) gets image
// (7) requeues buffer
// (8) stops stream
void ConfigureTriggerAndAcquireImage(Arena::IDevice* pDevice)
{
	// Get node values that will be changed in order to return their values at the end of the example
	GenICam::gcstring triggerSelectorInitial = Arena::GetNodeValue<GenICam::gcstring>(pDevice->GetNodeMap(), "TriggerSelector");
	GenICam::gcstring triggerModeInitial = Arena::GetNodeValue<GenICam::gcstring>(pDevice->GetNodeMap(), "TriggerMode");
	GenICam::gcstring triggerSourceInitial = Arena::GetNodeValue<GenICam::gcstring>(pDevice->GetNodeMap(), "TriggerSource");

	// set trigger selector
	std::cout << TAB1 << "Set trigger selector to FrameStart\n";

	Arena::SetNodeValue<GenICam::gcstring>(
		pDevice->GetNodeMap(),
		"TriggerSelector",
		"FrameStart");

	// set trigger mode
	std::cout << TAB1 << "Enable trigger mode\n";

	Arena::SetNodeValue<GenICam::gcstring>(
		pDevice->GetNodeMap(),
		"TriggerMode",
		"On");

	// set trigger source
	std::cout << TAB1 << "Set trigger source to Software\n";

	Arena::SetNodeValue<GenICam::gcstring>(
		pDevice->GetNodeMap(),
		"TriggerSource",
		"Software");

	// enable stream auto negotiate packet size
	Arena::SetNodeValue<bool>(
		pDevice->GetTLStreamNodeMap(),
		"StreamAutoNegotiatePacketSize",
		true);

	// enable stream packet resend
	Arena::SetNodeValue<bool>(
		pDevice->GetTLStreamNodeMap(),
		"StreamPacketResendEnable",
		true);

	// start stream
	std::cout << TAB1 << "Start stream\n\n";
	pDevice->StartStream();

	for (uint32_t i = 1; i <= NUMBER_IMAGES_TO_CAPTURE; i++)
	{
		// Trigger Armed
		//    Continually check until trigger is armed. Once the trigger is
		//    armed, it is ready to be executed.
		std::cout << TAB2 << "Wait until trigger is armed\n";
		bool triggerArmed = false;

		do
		{
			triggerArmed = Arena::GetNodeValue<bool>(pDevice->GetNodeMap(), "TriggerArmed");
		} while (triggerArmed == false);

		// Trigger an image
		//    Trigger an image manually, since trigger mode is enabled. This
		//    triggers the camera to acquire a single image. A buffer is then
		//    filled and moved to the output queue, where it will wait to be
		//    retrieved.
		std::cout << TAB2 << "Trigger image " << i << std::endl;

		// this waits for the next leader for each triggered image
		if (WAIT_FOR_LEADER_EVERY_3RD_IMAGE == false)
		{
			Arena::ExecuteNode(
				pDevice->GetNodeMap(),
				"TriggerSoftware");

			// Wait for next leader
			//    This will return when the leader for the next image arrives at
			//    the host if it arrives before the timeout. Otherwise, it will
			//    throw a timeout exception
			std::cout << TAB2 << "Wait for leader to arrive" << std::endl;
			pDevice->WaitForNextLeader(TIMEOUT_MILLISEC);

			std::cout << TAB2 << "Leader has arrived for image " << i << std::endl;
		}
		// This waits for the leader of every 3rd image.
		//    Since Wait is not called for the other images, we call Reset to
		//    clear the current wait state before continuing. If we do not do a
		//    Reset, then the next Wait would return immediately for the last
		//    leader.
		else
		{
			if (i % 3 == 0)
			{
				std::cout << TAB2 << "Resetting WaitForNextLeader state" << std::endl;
				pDevice->ResetWaitForNextLeader();
			}

			Arena::ExecuteNode(
				pDevice->GetNodeMap(),
				"TriggerSoftware");

			if (i % 3 == 0)
			{
				std::cout << TAB2 << "Wait for leader to arrive" << std::endl;
				pDevice->WaitForNextLeader(TIMEOUT_MILLISEC);
				std::cout << TAB2 << "Leader has arrived for image " << i << std::endl;
			}
		}

		// Get image
		//    Once an image has been triggered, it can be retrieved. If no image
		//    has been triggered, trying to retrieve an image will hang for the
		//    duration of the timeout and then throw an exception.
		std::cout << TAB2 << "Get image";

		Arena::IImage* pImage = pDevice->GetImage(TIMEOUT_MILLISEC);

		std::cout << " (" << pImage->GetWidth() << "x" << pImage->GetHeight() << ")\n";

		// requeue buffer
		std::cout << TAB2 << "Requeue buffer\n\n";
		pDevice->RequeueBuffer(pImage);
	}

	// stop the stream
	std::cout << TAB1 << "Stop stream\n";
	pDevice->StopStream();

	// return nodes to their initial values
	Arena::SetNodeValue<GenICam::gcstring>(pDevice->GetNodeMap(), "TriggerSource", triggerSourceInitial);
	Arena::SetNodeValue<GenICam::gcstring>(pDevice->GetNodeMap(), "TriggerMode", triggerModeInitial);
	Arena::SetNodeValue<GenICam::gcstring>(pDevice->GetNodeMap(), "TriggerSelector", triggerSelectorInitial);
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

	std::cout << "Cpp_Trigger_NextLeader";

	try
	{
		// prepare example
		Arena::ISystem* pSystem = Arena::OpenSystem();
		pSystem->UpdateDevices(1000);
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
		ConfigureTriggerAndAcquireImage(pDevice);
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
