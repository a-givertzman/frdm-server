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

// Acquisition: Sensor Binning
//    This example demonstrates how to configure device settings to enable
//    binning at the sensor level, so that the sensor will combine rectangles of
//    pixels into larger "bins". This reduces image resolution, but
//    also reduces the amount of data sent to the software and networking
//    layers.

// =-=-=-=-=-=-=-=-=-
// =-=- SETTINGS =-=-
// =-=-=-=-=-=-=-=-=-

// time to wait to see if an image is available
#define TIMEOUT 2000

// number of images to grab
#define NUM_IMAGES 25

// This is the entry we will use for BinningVerticalMode and BinningHorizontalMode. 
//    Sum will result in a brighter image, compared to Average.
#define BINTYPE "Sum"

// =-=-=-=-=-=-=-=-=-
// =-=- EXAMPLE -=-=-
// =-=-=-=-=-=-=-=-=-

void MaximizeSensorBinningAndAcquireImages(Arena::IDevice* pDevice)
{
	// get node values that will be changed in order to return their values at
	// the end of the example
	GenICam::gcstring acquisitionModeInitial = Arena::GetNodeValue<GenICam::gcstring>(pDevice->GetNodeMap(), "AcquisitionMode");
	GenICam::gcstring binningSelectorInitial = Arena::GetNodeValue<GenICam::gcstring>(pDevice->GetNodeMap(), "BinningSelector");

	GenICam::gcstring binningVerticalModeInitial = Arena::GetNodeValue<GenICam::gcstring>(pDevice->GetNodeMap(), "BinningHorizontalMode");
	GenICam::gcstring binningHorizontalModeInitial = Arena::GetNodeValue<GenICam::gcstring>(pDevice->GetNodeMap(), "BinningHorizontalMode");

	int64_t binningVerticalInitial = Arena::GetNodeValue<int64_t>(pDevice->GetNodeMap(), "BinningVertical");
	int64_t binningHorizontalInitial = Arena::GetNodeValue<int64_t>(pDevice->GetNodeMap(), "BinningHorizontal");

	Arena::SetNodeValue<GenICam::gcstring>(
		pDevice->GetNodeMap(),
		"AcquisitionMode",
		"Continuous");


	std::cout << TAB1 << "Set binning mode to sensor" << std::endl;
	// Set binning mode
	//    Sets binning mode to sensor, so that processing is done before
	//    transport to software.
	Arena::SetNodeValue<GenICam::gcstring>(pDevice->GetNodeMap(),
		"BinningSelector",
		"Sensor");


	// Check if the nodes for the height and width of the bin are available
	//    For rare case where sensor binning is unsupported but still appears as
	//    an option. Must be done after setting BinningSelector to Sensor. It was
	//    probably just a bug in the firmware.
	if (!GenApi::IsAvailable(pDevice->GetNodeMap()->GetNode("BinningVertical")) || !GenApi::IsAvailable(pDevice->GetNodeMap()->GetNode("BinningVertical"))) {

		// restore initial values
		Arena::SetNodeValue<GenICam::gcstring>(pDevice->GetNodeMap(),
			"AcquisitionMode",
			acquisitionModeInitial);
		Arena::SetNodeValue<GenICam::gcstring>(pDevice->GetNodeMap(),
			"BinningSelector",
			binningSelectorInitial);
		std::cout << TAB1 << "Sensor binning not supported by device: BinningVertical or BinningHorizontal not available.\n";
		return;
	}

	std::cout << TAB1 << "Finding max binning height and width" << std::endl;

	// Find max for bin height & width.
	//    For maximum compression.
	GenApi::CIntegerPtr pBinningVerticalNode = pDevice->GetNodeMap()->GetNode("BinningVertical");
	GenApi::CIntegerPtr pBinningHorizontalNode = pDevice->GetNodeMap()->GetNode("BinningHorizontal");

	int64_t binHeight = pBinningVerticalNode->GetMax();
	int64_t binWidth = pBinningHorizontalNode->GetMax();

	// Set BinningHorizontal and BinningVertical to their maxes.
	//    This sets width and height of the bins: the number of pixels along each
	//    axis.
	std::cout << TAB1 << "Set binning horizontal and vertical to " << binWidth << " and " << binHeight << " respectively" << std::endl;

	Arena::SetNodeValue<int64_t>(pDevice->GetNodeMap(),
		"BinningVertical",
		binHeight);

	Arena::SetNodeValue<int64_t>(pDevice->GetNodeMap(),
		"BinningHorizontal",
		binWidth);

	// Sets binning mode
	//    Sets binning mode for horizontal and vertical axes. Generally, they're
	//    set to the same value.
	std::cout << TAB1 << "Set binning mode to " << BINTYPE << std::endl;
	Arena::SetNodeValue<GenICam::gcstring>(pDevice->GetNodeMap(),
		"BinningVerticalMode",
		BINTYPE);

	Arena::SetNodeValue<GenICam::gcstring>(pDevice->GetNodeMap(),
		"BinningHorizontalMode",
		BINTYPE);

	// set buffer handling mode
	Arena::SetNodeValue<GenICam::gcstring>(
		pDevice->GetTLStreamNodeMap(),
		"StreamBufferHandlingMode",
		"NewestOnly");

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

	// return nodes to their initial values
	Arena::SetNodeValue<GenICam::gcstring>(pDevice->GetNodeMap(), "AcquisitionMode", acquisitionModeInitial);
	Arena::SetNodeValue<GenICam::gcstring>(pDevice->GetNodeMap(), "BinningSelector", binningSelectorInitial);
	Arena::SetNodeValue<GenICam::gcstring>(pDevice->GetNodeMap(), "BinningVerticalMode", binningVerticalModeInitial);
	Arena::SetNodeValue<GenICam::gcstring>(pDevice->GetNodeMap(), "BinningHorizontalMode", binningHorizontalModeInitial);
	Arena::SetNodeValue<int64_t>(pDevice->GetNodeMap(), "BinningVertical", binningVerticalInitial);
	Arena::SetNodeValue<int64_t>(pDevice->GetNodeMap(), "BinningHorizontal", binningHorizontalInitial);
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

	std::cout << "Cpp_Acquisition_SensorBinning";

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

		// Initial check if sensor binning is supported.
		//    Entry may not be in XML file. Entry may be in the file but set to
		//    unreadable or unavailable. Note: there is a case where sensor
		//    binning is not supported but this test passes. However, we must set
		//    BinningSelector to Sensor before we can test for that.
		GenApi::CEnumerationPtr pBinningSelectorNode = pDevice->GetNodeMap()->GetNode("BinningSelector");
		GenApi::CEnumEntryPtr binningSensorEntry = pBinningSelectorNode->GetEntryByName("Sensor");
		if (binningSensorEntry == 0 || !GenApi::IsAvailable(binningSensorEntry)) {
			std::cout << "\nSensor binning not supported by device: not available from BinningSelector\n\nPress enter to complete\n";
			std::getchar();
			return 0;
		}

		// run example
		std::cout << "Commence example\n\n";
		MaximizeSensorBinningAndAcquireImages(pDevice);
		std::cout << "\nExample complete\n";

		// clean up example
		pSystem->DestroyDevice(pDevice);
		Arena::CloseSystem(pSystem);
	}
	catch (GenICam::GenericException& ge)
	{
		std::cout << "\nGenICam exception thrown: " << ge.what() << std::endl;
		exceptionThrown = true;
	}
	catch (std::exception& ex)
	{
		std::cout << "\nStandard exception thrown: " << ex.what() << std::endl;
		exceptionThrown = true;
	}
	catch (...)
	{
		std::cout << "\nUnexpected exception thrown" << std::endl;
		exceptionThrown = true;
	}

	std::cout << "Press enter to complete" << std::endl;
	std::cin.ignore();
	std::getchar();

	if (exceptionThrown)
		return -1;
	else
		return 0;
}
