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
#include "SaveApi.h"

#include <sys/stat.h> // for reading size of file
#include <chrono>

#define TAB1 "  "
#define TAB2 "    "

// Acquisition: Compressed Image Loading
//		This example demonstrates how to handle compressed image data, specifically
//		loading and processing from raw data files using the Arena SDK. The example
//		includes steps to configure the camera, acquire a compressed image, save
//		the raw file, load the raw file, decompress the data, and save the decompressed
//		image.

// =-=-=-=-=-=-=-=-=-
// =-=- SETTINGS =-=-
// =-=-=-=-=-=-=-=-=-

#define TIMEOUT 2000

#define RAW_FILE_PATH "Images/Cpp_Acquisition_CompressedImageLoading/CompressedImage"
#define PNG_FILE_PATH "Images/Cpp_Acquisition_CompressedImageLoading/DecompressedImage"

// =-=-=-=-=-=-=-=-=-
// =-=- EXAMPLE -=-=-
// =-=-=-=-=-=-=-=-=-

size_t readFileSize(std::string file)
{
	// read size of file
	struct stat stat_buf;
	int rc = stat(file.c_str(), &stat_buf);
	if (rc != 0)
	{
		throw GenICam::GenericException("Unable to read file", __FILE__, __LINE__);
	}
	return stat_buf.st_size;
}

void saveCompressedImage(Arena::ICompressedImage* pImage, int index)
{
	std::cout << TAB2 << "Save compressed input image data to ";

	// save buffer size
	size_t bufferSize = pImage->GetSizeFilled();

	// create temporary buffer
	uint8_t* pData = new uint8_t[bufferSize];

	// copy data to temporary buffer
	memcpy(pData, pImage->GetData(), bufferSize);

	// filepath
	std::string filepath = RAW_FILE_PATH + std::to_string(index) + ".raw";

	// save raw data to disk
	// note: if function doesn't work, make sure the directories are present
	Save::ImageWriter::SaveRawData(filepath.c_str(), pData, bufferSize);

	std::cout << filepath << "\n";

	// clean up
	delete[] pData;
	pData = NULL;
}

// Demonstrates acquisition and saving of compressed image data.
// (1) Configures the camera to use a compressed pixel format
// (2) Acquires a compressed input image
// (3) Saves the raw input image
void AquireAndSaveRawImage(Arena::IDevice* pDevice)
{
	std::cout << TAB1 << "Acquire and save raw image\n";

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
		throw GenICam::GenericException("QOI_Mono8 is not available in the PixelFormat enumeration for this camera.", __FILE__, __LINE__);
	}

	// get node values that will be changed in order to
	// return their values at the end of the example
	GenICam::gcstring pixelFormatInitial = Arena::GetNodeValue<GenICam::gcstring>(pDevice->GetNodeMap(), "PixelFormat");

	// set pixel format
	std::cout << TAB1 << "Set pixel format to 'QOI_Mono8'\n";
	Arena::SetNodeValue<GenICam::gcstring>(pDevice->GetNodeMap(), "PixelFormat", "QOI_Mono8");

	// enable stream auto negotiate packet size and stream packet resend
	Arena::SetNodeValue<bool>(pDevice->GetTLStreamNodeMap(), "StreamAutoNegotiatePacketSize", true);
	Arena::SetNodeValue<bool>(pDevice->GetTLStreamNodeMap(), "StreamPacketResendEnable", true);

	// start stream
	std::cout << TAB1 << "Start stream\n";
	pDevice->StartStream();

	// get ten compressed images
	std::cout << TAB1 << "Get ten compressed images\n";
	for (int i = 0; i < 10; i++)
	{
		// get one compressed image
		std::cout << TAB2 << "Get compressed image " << i << "\n";
		Arena::ICompressedImage* pCompressedImage = pDevice->GetCompressedImage(TIMEOUT);

		// print out the sizes for comparison
		const size_t compressedImageSize = pCompressedImage->GetSizeFilled();
		std::cout << TAB2 << "Compressed image " << i << " size : " << compressedImageSize << " bytes\n ";

		// save the raw image
		saveCompressedImage(pCompressedImage, i);

		// re-queue the image buffer
		pDevice->RequeueBuffer(pCompressedImage);
	}

	// stop stream
	std::cout << TAB1 << "Stop stream\n";

	pDevice->StopStream();

	// return nodes to their initial values
	Arena::SetNodeValue<GenICam::gcstring>(pDevice->GetNodeMap(), "PixelFormat", pixelFormatInitial);
}

// Demonstrates loading and processing of compressed image data.
// (1) Loads raw image
// (2) Decompresses raw image
// (3) Saves decompressed image into readable format
void LoadAndProcessRawImage(std::string inFile, std::string outFile)
{
	std::cout << "\n" << TAB1 << "Load and process 10 images\n";

	std::chrono::steady_clock::time_point begin = std::chrono::steady_clock::now();

	for (int i = 0; i < 10; i++)
	{
		// in file path
		std::string inFilePath = inFile + std::to_string(i) + ".raw";

		// read raw file size
		size_t size = readFileSize(inFilePath);

		// allocate memory to load raw data
		uint8_t* pIn = new uint8_t[size];
		memset(pIn, 0, size);

		// load raw data
		Save::ImageReader::LoadRawData(inFilePath.c_str(), pIn, size);

		// convert raw data into Arena::ICompressedImage*
		Arena::ICompressedImage* pCompressedImage = Arena::ImageFactory::CreateCompressedImage(pIn, size, QOI_Mono8);

		// decompress image to Mono8
		Arena::IImage* pDecompressedImage = Arena::ImageFactory::DecompressImage(pCompressedImage);

		// print out the sizes to compare to compressed image
		const size_t decompressedImageSize = pDecompressedImage->GetSizeFilled();
		std::cout << TAB2 << "Decompressed image " << i << " image size : " << decompressedImageSize << " bytes\n ";

		// save the processed image
		std::cout << TAB2 << "Save decompressed QOI image to ";

		// get image parameters
		Save::ImageParams params(
			pDecompressedImage->GetWidth(),
			pDecompressedImage->GetHeight(),
			pDecompressedImage->GetBitsPerPixel());

		// out file path
		std::string outFilePath = outFile + std::to_string(i) + ".png";

		Save::ImageWriter writer(params, outFilePath.c_str());
		writer << pDecompressedImage->GetData();

		std::cout << outFilePath << "\n";

		// destroy converted buffer to prevent memory loss
		delete[] pIn;
		pIn = NULL;
		Arena::ImageFactory::DestroyCompressedImage(pCompressedImage);
		Arena::ImageFactory::Destroy(pDecompressedImage);
	}

	std::chrono::steady_clock::time_point end = std::chrono::steady_clock::now();
	std::cout << "Time to decompress 10 images (sec) = " << (std::chrono::duration_cast<std::chrono::microseconds>(end - begin).count()) / 1000000.0 << std::endl;
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

	std::cout << "Cpp_Acquisition_CompressedImageLoading";

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
		AquireAndSaveRawImage(pDevice);
		LoadAndProcessRawImage(RAW_FILE_PATH, PNG_FILE_PATH);
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
