/***************************************************************************************
 ***                                                                                 ***
 ***  Copyright (c) 2023, Lucid Vision Labs, Inc.                                    ***
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
#include "SaveApi.h"

#define TAB1 "  "
#define TAB2 "    "

// Acquisition: Compressed Image Handling
//    This example demonstrates how to acquire and process compressed image data
//    from the camera using the Arena SDK. The example includes steps to configure the
//    camera, acquire a compressed image, process the image to decompress it, and save
//    both the raw input and processed images.

// =-=-=-=-=-=-=-=-=-
// =-=- SETTINGS =-=-
// =-=-=-=-=-=-=-=-=-

#define TIMEOUT 2000

#define RAW_FILE_NAME "Images/Cpp_Acquisition_CompressedImageHandling/CompressedImage.raw"
#define PNG_FILE_NAME "Images/Cpp_Acquisition_CompressedImageHandling/DecompressedImage.png"

// =-=-=-=-=-=-=-=-=-
// =-=- EXAMPLE -=-=-
// =-=-=-=-=-=-=-=-=-

// helper function to save file of type ".raw" to disk
void saveRAWInputImage(Arena::ICompressedImage* pCompressedImage)
{
	std::cout << TAB2 << "Save input image to ";

	uint8_t* data = const_cast<uint8_t*>(pCompressedImage->GetData());

	Save::ImageWriter::SaveRawData(
		RAW_FILE_NAME,
		data,
		pCompressedImage->GetSizeFilled()
	);

	std::cout << RAW_FILE_NAME << "\n";
}

// helper function to decompress image and save as ".png" to disk
void processAndSaveDecompressedImage(Arena::ICompressedImage* pCompressedImage)
{ 
	std::cout << TAB2 << "Converting to Mono8" << std::endl;

	Arena::IImage* pDecompressedImage = Arena::ImageFactory::DecompressImage(pCompressedImage);

	// print out the sizes for comparison
	const size_t decompressedImageSize = pDecompressedImage->GetSizeFilled();
	std::cout << TAB2 << "Mono8 Image Size: " << decompressedImageSize << " bytes" << std::endl;

	// save the processed image
	std::cout << TAB2 << "Save decompressed QOI image to ";

	std::string filename = PNG_FILE_NAME;

	Save::ImageParams params(
		pDecompressedImage->GetWidth(), 
		pDecompressedImage->GetHeight(), 
		pDecompressedImage->GetBitsPerPixel()
	);

	Save::ImageWriter writer(params, filename.c_str());
	writer << pDecompressedImage->GetData();

	std::cout << PNG_FILE_NAME << "\n";

	Arena::ImageFactory::Destroy(pDecompressedImage);
}

// Demonstrates acquisition and processing of compressed image data.
// (1) Configures the camera to use a compressed pixel format
// (2) Acquires a compressed input image
// (3) Processes and saves the raw input image to decompress it
// (4) Save the processed image
void AcquireAndProcessCompressedImage(Arena::IDevice* pDevice)
{
	GenApi::INode* pNode = pDevice->GetNodeMap()->GetNode("PixelFormat");

	GenApi::CEnumerationPtr pEnumeration = pNode;

	// retrieve the list of entries in the enumeration
	GenApi::NodeList_t entries;
	pEnumeration->GetEntries(entries);

	// iterate through the entries and check for "QOI_Mono8"
	bool found = false;
	for (auto& entry : entries)
	{
		GenApi::CEnumEntryPtr pEntry = entry;
		if (GenApi::IsAvailable(pEntry) && pEntry->GetSymbolic() == "QOI_Mono8")
		{
			found = true;
			break;
		}
	}

	if (!found)
	{
		std::cout << TAB1 << "QOI_Mono8 is not available in the PixelFormat enumeration for this camera." << std::endl;
		return;
	}

	// get node values that will be changed in order to return
	// their values at the end of the example
	GenICam::gcstring pixelFormatInitial = Arena::GetNodeValue<GenICam::gcstring>(pDevice->GetNodeMap(), "PixelFormat");

	// set pixel format
	std::cout << TAB1 << "Set pixel format to 'QOI_Mono8'\n";

	Arena::SetNodeValue<GenICam::gcstring>(pDevice->GetNodeMap(), "PixelFormat", "QOI_Mono8");

	// enable stream auto negotiate packet size
	Arena::SetNodeValue<bool>(pDevice->GetTLStreamNodeMap(), "StreamAutoNegotiatePacketSize", true);

	// enable stream packet resend
	Arena::SetNodeValue<bool>(pDevice->GetTLStreamNodeMap(), "StreamPacketResendEnable", true);

	// start stream
	std::cout << TAB1 << "Start stream\n";

	pDevice->StartStream();

	std::cout << TAB2 << "Get one image\n";

	Arena::ICompressedImage* pCompressedImage = pDevice->GetCompressedImage(TIMEOUT);

	// get the compressed image size
	const size_t compressedImageSize = pCompressedImage->GetSizeFilled();
	std::cout << TAB2 << "QOI_Mono8 Compressed Image Size: " << compressedImageSize << " bytes" << std::endl;

	saveRAWInputImage(pCompressedImage);

	processAndSaveDecompressedImage(pCompressedImage);

	// re-queue the image buffer
	pDevice->RequeueBuffer(pCompressedImage);

	// stop stream
	std::cout << TAB1 << "Stop stream\n";

	pDevice->StopStream();

	// return nodes to their initial values
	Arena::SetNodeValue<GenICam::gcstring>(pDevice->GetNodeMap(), "PixelFormat", pixelFormatInitial);
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

	std::cout << "Cpp_Acquisition_CompressedImageHandling\n";

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
		AcquireAndProcessCompressedImage(pDevice);
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
	std::getchar();

	if (exceptionThrown)
		return -1;
	else
		return 0;
}
