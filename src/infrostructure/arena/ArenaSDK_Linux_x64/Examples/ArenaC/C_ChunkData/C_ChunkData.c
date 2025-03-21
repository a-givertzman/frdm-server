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
#include "ArenaCApi.h"
#include <inttypes.h> // defines macros for printf functions
#include <stdbool.h>  // defines boolean type and values

#if (!defined _WIN32 && !defined _WIN64)
#define scanf_s scanf
#endif

#define TAB1 "  "
#define TAB2 "    "
#define TAB3 "      "

// Chunk Data: CRC Validation
//    This example demonstrates the use of chunk data to verify data through a
//    Cyclical Redundancy Check (or CRC for short). CRCs are meant to check the
//    validity of sent data. It is performed by doing a series of calculations on
//    the raw data before and after it is sent. If the resultant integer values
//    match, then it is safe to assume the integrity of the data.

// =-=-=-=-=-=-=-=-=-
// =-=- SETTINGS =-=-
// =-=-=-=-=-=-=-=-=-

// system timeout
#define SYSTEM_TIMEOUT 100

// image timeout
#define IMAGE_TIMEOUT 2000

// number of images to grab
#define NUM_IMAGES 5

// maximum buffer length
#define MAX_BUF 1024

// =-=-=-=-=-=-=-=-=-
// =-=- EXAMPLE -=-=-
// =-=-=-=-=-=-=-=-=-

// demonstrates chunk data
// (1) activates chunk mode
// (2) enables exposure and gain chunks
// (3) starts the stream and gets images
// (4) retrieves exposure and gain chunk data from image
// (5) requeues buffers and stops the stream
AC_ERROR ConfigureAndRetrieveChunkData(acDevice hDevice)
{
	AC_ERROR err = AC_ERR_SUCCESS;

	// get node map
	acNodeMap hNodeMap = NULL;

	err = acDeviceGetNodeMap(hDevice, &hNodeMap);
	if (err != AC_ERR_SUCCESS)
		return err;

	// get initial node values that will be changed in order to return their
	// values at the end of the example get chunk mode active node
	acNode hChunkModeActiveNode = NULL;
	bool8_t chunkModeActiveInitial = false;

	err = acNodeMapGetNode(hNodeMap, "ChunkModeActive", &hChunkModeActiveNode);
	if (err != AC_ERR_SUCCESS)
		return err;

	err = acBooleanGetValue(hChunkModeActiveNode, &chunkModeActiveInitial);
	if (err != AC_ERR_SUCCESS)
		return err;

	// get chunk enable node
	acNode hChunkEnableNode = NULL;
	bool8_t chunkModeEnableInitial = false;

	err = acNodeMapGetNode(hNodeMap, "ChunkEnable", &hChunkEnableNode);
	if (err != AC_ERR_SUCCESS)
		return err;

	err = acBooleanGetValue(hChunkEnableNode, &chunkModeEnableInitial);
	if (err != AC_ERR_SUCCESS)
		return err;

	// Activate chunk mode
	//    Activate chunk mode before enabling chunks; otherwise, ChunkSelector
	//    and ChunkEnable nodes will be unavailable.
	printf("%sActivate chunk mode\n", TAB1);

	err = acBooleanSetValue(hChunkModeActiveNode, true);
	if (err != AC_ERR_SUCCESS)
		return err;

	// Enable exposure and gain chunks
	//    Enable desired chunks before starting the stream. Some chunks provide
	//    information already in an image, while others do not. This example looks at
	//    exposure and gain, which are two pieces of information unavailable from an
	//    image without chunk data.
	printf("%sEnable exposure and gain chunks\n", TAB1);

	// set exposure time
	err = acNodeMapSetEnumerationValue(hNodeMap, "ChunkSelector", "ExposureTime");
	if (err != AC_ERR_SUCCESS)
		return err;

	err = acBooleanSetValue(hChunkEnableNode, true);
	if (err != AC_ERR_SUCCESS)
		return err;

	// set gain
	err = acNodeMapSetEnumerationValue(hNodeMap, "ChunkSelector", "Gain");
	if (err != AC_ERR_SUCCESS)
		return err;

	err = acBooleanSetValue(hChunkEnableNode, true);
	if (err != AC_ERR_SUCCESS)
		return err;

	// get stream node map
	acNodeMap hTLStreamNodeMap = NULL;

	err = acDeviceGetTLStreamNodeMap(hDevice, &hTLStreamNodeMap);
	if (err != AC_ERR_SUCCESS)
		return err;

	// Enable stream auto negotiate packet size
	//    Setting the stream packet size is done before starting the stream.
	//    Setting the stream to automatically negotiate packet size instructs the
	//    camera to receive the largest packet size that the system will allow.
	//    This generally increases frame rate and results in fewer interrupts per
	//    image, thereby reducing CPU load on the host system. Ethernet settings
	//    may also be manually changed to allow for a larger packet size.
	printf("%sEnable stream to auto negotiate packet size\n", TAB1);

	err = acNodeMapSetBooleanValue(hTLStreamNodeMap, "StreamAutoNegotiatePacketSize", true);
	if (err != AC_ERR_SUCCESS)
		return err;

	// Enable stream packet resend
	//    Enable stream packet resend before starting the stream. Images are sent
	//    from the camera to the host in packets using UDP protocol, which
	//    includes a header image number, packet number, and timestamp
	//    information. If a packet is missed while receiving an image, a packet
	//    resend is requested and this information is used to retrieve and
	//    redeliver the missing packet in the correct order.
	printf("%sEnable stream packet resend\n", TAB1);

	err = acNodeMapSetBooleanValue(hTLStreamNodeMap, "StreamPacketResendEnable", true);
	if (err != AC_ERR_SUCCESS)
		return err;

	// Start stream and grab images
	//    Start streaming images with chunk data. Image data will now be appended
	//    with exposure and gain data because they were enabled earlier.
	err = acDeviceStartStream(hDevice);
	if (err != AC_ERR_SUCCESS)
		return err;

	printf("%sRetrieve images and chunks\n", TAB1);
	for (int i = 0; i < NUM_IMAGES; i++)
	{
		// grab images
		printf("%s Image %i", TAB2, i);
		acBuffer hBuffer = NULL;

		err = acDeviceGetBuffer(hDevice, IMAGE_TIMEOUT, &hBuffer);
		if (err != AC_ERR_SUCCESS)
			return err;

		// Check for completeness
		//    If an image is incomplete, it could be the case that the chunks
		//    hold garbage data. If incomplete, chunks can still be
		//    retrieved but should be checked against null to verify the data
		//    exists.
		bool8_t isIncomplete = false;

		err = acBufferIsIncomplete(hBuffer, &isIncomplete);
		if (err != AC_ERR_SUCCESS)
			return err;
		if (isIncomplete)
		{
			printf(" (incomplete)");
			printf("\n\nError: Payload data incomplete. Please review network \nconfigurations, increase "
				"packet size, increase inter-packet \ndelay and/or reduce image size, then retry example\n");
			return err;
		}

		// Get exposure and gain chunks
		//    Chunks work the same way as nodes; they have a node type,
		//    additional information, and return null if they don't exist or
		//    cannot be found. For example, the exposure time chunk can access a
		//    maximum, minimum, display name, and unit, just like the exposure
		//    time node. get exposure chunk
		acNode hChunkExposureNode = NULL;
		double exposureValue = 0;

		err = acChunkDataGetChunk(hBuffer, "ChunkExposureTime", &hChunkExposureNode);
		if (err != AC_ERR_SUCCESS)
			return err;

		err = acFloatGetValue(hChunkExposureNode, &exposureValue);
		if (err != AC_ERR_SUCCESS)
			return err;

		// get gain chunk
		acNode hChunkGainNode = NULL;
		double gainValue = 0;

		err = acChunkDataGetChunk(hBuffer, "ChunkGain", &hChunkGainNode);
		if (err != AC_ERR_SUCCESS)
			return err;

		err = acFloatGetValue(hChunkGainNode, &gainValue);
		if (err != AC_ERR_SUCCESS)
			return err;
		printf("%sexposure = %.1f, gain = %.1f\n", TAB1, exposureValue, gainValue);

		// requeue buffers
		err = acDeviceRequeueBuffer(hDevice, hBuffer);
		if (err != AC_ERR_SUCCESS)
			return err;
	}

	// stop stream
	err = acDeviceStopStream(hDevice);
	if (err != AC_ERR_SUCCESS)
		return err;

	// return nodes to their initial values
	err = acBooleanSetValue(hChunkModeActiveNode, chunkModeActiveInitial);
	if (err != AC_ERR_SUCCESS)
		return err;

	err = acBooleanSetValue(hChunkEnableNode, chunkModeEnableInitial);
	if (err != AC_ERR_SUCCESS)
		return err;

	return err;
}

// =-=-=-=-=-=-=-=-=-
// =- PREPARATION -=-
// =- & CLEAN UP =-=-
// =-=-=-=-=-=-=-=-=-

AC_ERROR SelectDevice(acSystem hSystem, size_t* pNumDevices, size_t* pSelection)
{
	AC_ERROR err = AC_ERR_SUCCESS;

	if (*pNumDevices == 1)
	{
		printf(TAB1 "Only one device detected, automatically selecting this device.\n");
		*pSelection = 0;
		return AC_ERR_SUCCESS;
	}

	printf(TAB1 "Select device:\n");
	for (size_t i = 0; i < *pNumDevices; i++)
	{
		// get device model
		char pDeviceModel[MAX_BUF];
		size_t pDeviceModelLen = MAX_BUF;
		err = acSystemGetDeviceModel(hSystem, i, pDeviceModel, &pDeviceModelLen);
		if (err != AC_ERR_SUCCESS)
			return err;

		// get device serial
		char pDeviceSerial[MAX_BUF];
		size_t pDeviceSerialLen = MAX_BUF;
		err = acSystemGetDeviceSerial(hSystem, i, pDeviceSerial, &pDeviceSerialLen);
		if (err != AC_ERR_SUCCESS)
			return err;

		// get device IP address
		char pIpAddressStr[MAX_BUF];
		size_t pIpAddressStrBufLen = MAX_BUF;
		err = acSystemGetDeviceIpAddressStr(hSystem, i, pIpAddressStr, &pIpAddressStrBufLen);
		if (err != AC_ERR_SUCCESS)
			return err;

		printf(TAB2 "%zu. %s%s%s%s%s\n", i + 1, pDeviceModel, TAB1, pDeviceSerial, TAB1, pIpAddressStr);
	}

	do
	{
		printf(TAB1 "Make selection (1-%zu): ", *pNumDevices);

		if (scanf_s("%zu", pSelection) != 1)
		{
			while (getchar() != '\n')
				;
			printf(TAB1 "Invalid input. Please enter a number.\n");
			continue;
		}

		if (*pSelection <= 0 || *pSelection > *pNumDevices)
		{
			printf(TAB1 "Invalid device selected. Please select a device in the range (1-%zu).\n", *pNumDevices);
		}
	} while (*pSelection <= 0 || *pSelection > *pNumDevices);

	*pSelection -= 1;
	return AC_ERR_SUCCESS;
}

// error buffer length
#define ERR_BUF 512

#define CHECK_RETURN                                  \
	if (err != AC_ERR_SUCCESS)                        \
	{                                                 \
		char pMessageBuf[ERR_BUF];                    \
		size_t pBufLen = ERR_BUF;                     \
		acGetLastErrorMessage(pMessageBuf, &pBufLen); \
		printf("\nError: %s", pMessageBuf);           \
		printf("\n\nPress enter to complete\n");      \
		getchar();                                    \
		return -1;                                    \
	}

int main()
{
	printf("C_ChunkData\n");
	AC_ERROR err = AC_ERR_SUCCESS;

	// prepare example
	acSystem hSystem = NULL;
	err = acOpenSystem(&hSystem);
	CHECK_RETURN;
	err = acSystemUpdateDevices(hSystem, SYSTEM_TIMEOUT);
	CHECK_RETURN;
	size_t numDevices = 0;
	err = acSystemGetNumDevices(hSystem, &numDevices);
	CHECK_RETURN;
	if (numDevices == 0)
	{
		printf("\nNo camera connected\nPress enter to complete\n");
		getchar();
		return -1;
	}
	acDevice hDevice = NULL;
	size_t selection = 0;
	err = SelectDevice(hSystem, &numDevices, &selection);
	CHECK_RETURN;
	err = acSystemCreateDevice(hSystem, selection, &hDevice);
	CHECK_RETURN;

	// run example
	printf("Commence example\n\n");
	err = ConfigureAndRetrieveChunkData(hDevice);
	CHECK_RETURN;
	printf("\nExample complete\n");

	// clean up example
	err = acSystemDestroyDevice(hSystem, hDevice);
	CHECK_RETURN;
	err = acCloseSystem(hSystem);
	CHECK_RETURN;

	printf("Press enter to complete\n");
	while (getchar() != '\n') {};
	getchar();
	return -1;
}
