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
#include "SaveCApi.h"
#include <inttypes.h> // defines macros for printf functions
#include <stdbool.h>  // defines boolean type and values
#include <stdlib.h>
#include <string.h>

#if (!defined _WIN32 && !defined _WIN64)
#define scanf_s scanf
#endif

// Multithreading is implemented differently for Windows and Linux
// systems, so headers, functions and macros are defined according to the
// operating system being used.
#ifdef _WIN32
#include "ArenaCThreadWindows.h"
#elif defined linux
#include "ArenaCThreadLinux.h"
#endif

#define TAB1 "  "
#define TAB2 "    "

// Acquiring and Saving Images on Separate Threads: Introduction
//    Saving images can sometimes create a bottleneck in the image 
//    acquisition pipeline. By separating saving onto a separate thread,
//    this bottleneck can be avoided. 
//    This example is programmed as a simple producer-consumer problem.


// =-=-=-=-=-=-=-=-=-
// =-=- SETTINGS =-=-
// =-=-=-=-=-=-=-=-=-

// Image timeout in milliseconds
#define IMAGE_TIMEOUT 5000

// number of images to grab
#define NUM_IMAGES 25

// maximum buffer length
#define MAX_BUF 1024

// file name
#define FILE_NAME "Images/C_Acquisition_MultithreadedAcquisitionAndSave/image"
#define FILE_TYPE ".png"

// pixel format
#define PIXEL_FORMAT PFNC_BGR8 // BGR8

bool isCompleted = false;

// queue implementation
typedef struct node {
	acBuffer val;
	struct node *next;
} node_t;

void enqueue(node_t **head, acBuffer val) {
	node_t *new_node = (node_t *)malloc(sizeof(node_t));
	if (!new_node) return;

	new_node->val = val;
	new_node->next = *head;
	*head = new_node;
}

acBuffer dequeue(node_t **head) {
	node_t *current, *prev = NULL;
	acBuffer retval = NULL;

	if (*head == NULL) return NULL;

	current = *head;
	while (current->next != NULL) {
		prev = current;
		current = current->next;
	}

	retval = current->val;
	free(current);

	if (prev)
		prev->next = NULL;
	else
		*head = NULL;

	return retval;
}

bool isEmpty(node_t **head)
{
	return (*head == NULL);
}

node_t *head = NULL;

THREAD_LOCK_VARIABLE(lock);
THREAD_CONDITION_VARIABLE(cv);

// =-=-=-=-=-=-=-=-=-
// =-=- HELPERS =-=-=-
// =-=-=-=-=-=-=-=-=-

AC_ERROR exitThreads(AC_ERROR err)
{
	acThreadConditionVariableWake(&cv);
	return err;
}

SC_ERROR exitSCThreads(SC_ERROR err)
{
	acThreadConditionVariableWake(&cv);
	return err;
}
// gets node value
// (1) gets node
// (2) checks access mode
// (3) gets value
AC_ERROR GetNodeValue(acNodeMap hNodeMap, const char* nodeName, char* pValue, size_t* pLen)
{
	AC_ERROR err = AC_ERR_SUCCESS;

	// get node
	acNode hNode = NULL;
	AC_ACCESS_MODE accessMode = 0;

	err = acNodeMapGetNodeAndAccessMode(hNodeMap, nodeName, &hNode, &accessMode);
	if (err != AC_ERR_SUCCESS)
		return err;

	// check access mode
	if (accessMode != AC_ACCESS_MODE_RO && accessMode != AC_ACCESS_MODE_RW)
		return AC_ERR_ERROR;

	// get value
	err = acValueToString(hNode, pValue, pLen);
	return err;
}

// sets node value
// (1) gets node
// (2) checks access mode
// (3) gets value
AC_ERROR SetNodeValue(acNodeMap hNodeMap, const char* nodeName, const char* pValue)
{
	AC_ERROR err = AC_ERR_SUCCESS;

	// get node
	acNode hNode = NULL;
	AC_ACCESS_MODE accessMode = 0;

	err = acNodeMapGetNodeAndAccessMode(hNodeMap, nodeName, &hNode, &accessMode);
	if (err != AC_ERR_SUCCESS)
		return err;

	// check access mode
	if (accessMode != AC_ACCESS_MODE_WO && accessMode != AC_ACCESS_MODE_RW)
		return AC_ERR_ERROR;

	// get value
	err = acValueFromString(hNode, pValue);
	return err;
}

// =-=-=-=-=-=-=-=-=-
// =-=- EXAMPLE -=-=-
// =-=-=-=-=-=-=-=-=-

// demonstration: Acquire Images (Producer)
// (1) Call the main thread on Acquire Images (producer)
// (2) Lock the thread when it reaches the critical section, push image in the queue
// (3) Unlock the thread, and notify the consumer
// (4) Repeat for the number of images
THREAD_FUNCTION_SIGNATURE(AcquireImages)
{
	AC_ERROR err = AC_ERR_SUCCESS;
	acDevice hDevice = (acDevice*)lpParam;

	// get node map
	acNodeMap hNodeMap = NULL;

	err = acDeviceGetNodeMap(hDevice, &hNodeMap);
	if (err != AC_ERR_SUCCESS)
		exitThreads(err);

	// get node values that will be changed in order to return their values at
	// the end of the example
	char pAcquisitionModeInitial[MAX_BUF];
	size_t len = MAX_BUF;

	err = GetNodeValue(hNodeMap, "AcquisitionMode", pAcquisitionModeInitial, &len);
	if (err != AC_ERR_SUCCESS)
		exitThreads(err);

	// set acquisiton mode to continuous
	err = SetNodeValue(
		hNodeMap,
		"AcquisitionMode",
		"Continuous");

	if (err != AC_ERR_SUCCESS)
		exitThreads(err);

	// get stream node map
	acNodeMap hTLStreamNodeMap = NULL;
	err = acDeviceGetTLStreamNodeMap(hDevice, &hTLStreamNodeMap);
	if (err != AC_ERR_SUCCESS)
		exitThreads(err);

	// set buffer handling mode
	err = SetNodeValue(
		hTLStreamNodeMap,
		"StreamBufferHandlingMode",
		"NewestOnly");
	if (err != AC_ERR_SUCCESS)
		exitThreads(err);

	// enable stream auto negotiate packet size
	err = acNodeMapSetBooleanValue(hTLStreamNodeMap, "StreamAutoNegotiatePacketSize", true);
	if (err != AC_ERR_SUCCESS)
		exitThreads(err);

	// enable stream packet resend
	err = acNodeMapSetBooleanValue(hTLStreamNodeMap, "StreamPacketResendEnable", true);
	if (err != AC_ERR_SUCCESS)
		exitThreads(err);

	// start stream
	printf("\n%sStart stream", TAB1);

	err = acDeviceStartStream(hDevice);
	if (err != AC_ERR_SUCCESS)
		exitThreads(err);

	// get images
	printf("\n%sGetting %d images", TAB2, NUM_IMAGES);

	int i = 0;
	for (i = 0; i < NUM_IMAGES; i++)
	{
		// get image
		printf("\n%sGet image %d", TAB1 TAB2, i);
		acBuffer hBuffer = NULL;

		err = acDeviceGetBuffer(hDevice, IMAGE_TIMEOUT, &hBuffer);
		if (err != AC_ERR_SUCCESS)
			exitThreads(err);

		// program threads do not have access to the device, thus copying the acquired images
		// and pushing them into the queue is required
		acBuffer hCopyBuffer = NULL;
		err = acImageFactoryCopy(hBuffer, &hCopyBuffer);
		if (err != AC_ERR_SUCCESS)
			exitThreads(err);

		// critical section
		{
			// lock the thread
			acThreadLock(&lock);

			// push to the shared queue
			enqueue(&head, hCopyBuffer);

			if (i == NUM_IMAGES - 1) {
				isCompleted = true;
			}

			// unlock the thread as soon as the work with shared variable is completed
			acThreadUnlock(&lock);
		}
		// notify SaveImage (consumer)
		acThreadConditionVariableWake(&cv);

		// requeue image buffer
		printf("\n%sRequeue Buffer", TAB1 TAB2);

		err = acDeviceRequeueBuffer(hDevice, hBuffer);
		if (err != AC_ERR_SUCCESS)
			exitThreads(err);
	}
	// Stop stream
	printf("\n%sStop stream", TAB1);

	err = acDeviceStopStream(hDevice);
	if (err != AC_ERR_SUCCESS)
		exitThreads(err);

	// return nodes to their initial values
	err = SetNodeValue(
		hNodeMap,
		"AcquisitionMode",
		pAcquisitionModeInitial);

	THREAD_RETURN(err);
}


// demonstration: Save Images (Consumer)
// (1) Wait for the signal from producer, and lock the thread
// (2) Once the lock is acquired and if the queue is not empty, dequeue the image
// (3) Once dequeued, unlock the thread 
// (4) Save the image outside the critical section
// (5) Repeat for the number of images
THREAD_FUNCTION_SIGNATURE(SaveImage)
{
	// AC_ERROR and SC_ERROR values are equivalent
	AC_ERROR acErr = AC_ERR_SUCCESS;
	SC_ERROR saveErr = SC_ERR_SUCCESS;

	bool isComplete = false;
	int i = 0;

	while (true)
	{
		acBuffer hCopyBuffer = NULL;

		// if the queue is empty, wait for acquire images (producer) to notify
		acThreadLock(&lock);
		if (isEmpty(&head))
		{
			acThreadConditionVariableSleep(&cv, &lock);
		}
		acThreadUnlock(&lock);

		// critical section
		{
			// lock the thread
			acThreadLock(&lock);

			//// if queue is not empty, dequeue
			if (!isEmpty(&head))
			{
				// dequeue from the shared queue
				hCopyBuffer = dequeue(&head);

			}
			if (isEmpty(&head) && isCompleted) {
				isComplete = isCompleted;
			}

			// unlock the thread as soon as the work with shared variable is completed
			acThreadUnlock(&lock);
		}

		printf("\n%sConvert image %d to %s", TAB2 TAB2, i, (PIXEL_FORMAT == PFNC_BGR8 ? "BGR8" : "RGB8"));
		acBuffer hConverted = NULL;

		acErr = acImageFactoryConvert(hCopyBuffer, PIXEL_FORMAT, &hConverted);
		if (acErr != AC_ERR_SUCCESS)
			exitThreads(acErr);

		// prepare image parameters

		// get width
		size_t width = 0;

		acErr = acImageGetWidth(hConverted, &width);
		if (acErr != AC_ERR_SUCCESS)
			exitThreads(acErr);

		// get height
		size_t height = 0;

		acErr = acImageGetHeight(hConverted, &height);
		if (acErr != AC_ERR_SUCCESS)
			exitThreads(acErr);

		// get bits per pixel
		size_t bpp = 0;

		acErr = acImageGetBitsPerPixel(hConverted, &bpp);
		if (acErr != AC_ERR_SUCCESS)
			exitThreads(acErr);

		// Prepare image writer
		printf("\n%sPrepare image %d writer", TAB2 TAB2, i);
		saveWriter hWriter = NULL;

		saveErr = saveWriterCreate(width, height, bpp, &hWriter);
		if (saveErr != SC_ERR_SUCCESS)
			exitSCThreads(saveErr);

		// creating the filename 
		char filename[MAX_BUF];
		snprintf(filename, MAX_BUF, FILE_NAME "%d" FILE_TYPE, i);

		saveErr = saveWriterSetFileNamePattern(hWriter, filename);
		if (saveErr != SC_ERR_SUCCESS)
			exitSCThreads(saveErr);

		// get image
		uint8_t* pData = NULL;

		acErr = acImageGetData(hConverted, &pData);
		if (acErr != AC_ERR_SUCCESS)
			exitThreads(acErr);

		// save image
		saveErr = saveWriterSave(hWriter, pData);
		if (saveErr != SC_ERR_SUCCESS)
			exitSCThreads(saveErr);

		printf("\n%sSaved image %d", TAB1 TAB2, i);

		// destroy image writer
		saveErr = saveWriterDestroy(hWriter);
		if (saveErr != SC_ERR_SUCCESS)
			exitSCThreads(saveErr);

		// destroy converted image
		acErr = acImageFactoryDestroy(hCopyBuffer);
		if (acErr != AC_ERR_SUCCESS)
			exitThreads(acErr);

		acErr = acImageFactoryDestroy(hConverted);
		if (acErr != AC_ERR_SUCCESS)
			exitThreads(acErr);

		i++;

		if (isComplete) {
			break;
		}
	}
	THREAD_RETURN(saveErr);
}

AC_ERROR Threads(acDevice hDevice)
{
	AC_ERROR err = AC_ERR_SUCCESS;

	// initializing the lock and condition variable
	acThreadLockInitialize(&lock);
	acThreadConditionVariableInitialize(&cv);

	// declaring the threads
	THREAD_ID(hAcquireThread);
	THREAD_ID(hSaveThread);

	// creating the threads
	acThreadCreate(AcquireImages, hDevice, &hAcquireThread);
	acThreadCreate(SaveImage, hDevice, &hSaveThread);

	// destroying the threads in the order they were created
	acThreadDestroy(hAcquireThread);
	acThreadDestroy(hSaveThread);

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
	printf("C_Acquisition_MultithreadedAcquisitionAndSave\n");
	AC_ERROR err = AC_ERR_SUCCESS;

	// prepare example
	acSystem hSystem = NULL;
	err = acOpenSystem(&hSystem);
	CHECK_RETURN;
	err = acSystemUpdateDevices(hSystem, 100);
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
	printf("Commence example\n");
	err = Threads(hDevice);
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
