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
#include <stdbool.h>  // defines boolean type and values
#include <inttypes.h> // defines macros for printf functions

#if (!defined _WIN32 && !defined _WIN64)
#define scanf_s scanf
#endif

// Multithreading is implemented differently for Windows and Linux
//    systems, so headers, functions and macros are defined according to the
//    operating system being used.
#ifdef _WIN32
#include "ArenaCThreadWindows.h"
#elif defined linux
#include "ArenaCThreadLinux.h"
#endif

#define TAB1 "  "
#define TAB2 "    "
#define TAB3 "      "
#define TAB4 "        "

// Enumeration: Handling Disconnections
//    This example demonstrates a multi-threaded approach to handling device
//    disconnections. It spawns two threads, each with a different
//    responsibility. First, the acquisition thread is responsible for acquiring images when the
//    device is connected. Second, the enumeration thread handles
//    disconnections by reconnecting the device and notifying the acquisition
//    thread.

// =-=-=-=-=-=-=-=-=-
// =-=- SETTINGS =-=-
// =-=-=-=-=-=-=-=-=-

// image timeout
#define IMAGE_TIMEOUT 2000

// update timeout
#define SYSTEM_TIMEOUT 100

// maximum number of images
#define MAX_IMAGES 500

// maximum buffer length
#define MAX_BUF 1024

// Global settings
//    This example uses global scope for variables and information shared between
//    threads. This is not a best practice but is done this way for the sake of
//    simplicity.

THREAD_LOCK_VARIABLE(g_deviceMutex)

THREAD_CONDITION_VARIABLE(g_deviceConnected)

THREAD_CONDITION_VARIABLE(g_deviceDisconnected)

// arenaC
acDevice g_pDevice = NULL;

// other
static bool g_isRunning = false;

// =-=-=-=-=-=-=-=-=-
// =-=- HELPER =-=-=-
// =-=-=-=-=-=-=-=-=-

// return error
// (1) exit current thread
// (2) notify other thread to exit
// (3) return error
AC_ERROR exitThreads(AC_ERROR err)
{
	g_isRunning = false;
	acThreadConditionVariableWake(&g_deviceConnected);
	acThreadConditionVariableWake(&g_deviceDisconnected);
	return err;
}

// =-=-=-=-=-=-=-=-=-
// =-=- EXAMPLE -=-=-
// =-=-=-=-=-=-=-=-=-

// reconnects a device when disconnected
// (1) on disconnection, activate
// (2) search for device
// (3) reconnect device
// (4) notify all on reconnection
// (5) notify all on exit

THREAD_FUNCTION_SIGNATURE(EnumerationThread)
{
	AC_ERROR err = AC_ERR_SUCCESS;
	acSystem hSystem = (acSystem*)lpParam;

	// grab subnet upon entering thread
	uint32_t subnetMask = 0;

	err = acSystemGetDeviceSubnetMask(hSystem, 0, &subnetMask);
	if (err != AC_ERR_SUCCESS)
		exitThreads(err);

	while (g_isRunning)
	{
		// wait to see if a device is disconnected
		acThreadLock(&g_deviceMutex);
		acThreadConditionVariableWait(&g_deviceDisconnected, &g_deviceMutex);

		while(g_isRunning  && !g_pDevice)
		{

			// Search for device
			//    When the device has been disconnected, this thread waits for it
			//    to reconnect, constantly updating and scanning the device list
			//    for the lost device.
			err = acSystemUpdateDevices(hSystem, SYSTEM_TIMEOUT);
			if (err == AC_ERR_SUCCESS)
			{
				size_t numDevices = 0;
				err = acSystemGetNumDevices(hSystem, &numDevices);
				if (err == AC_ERR_SUCCESS && numDevices > 0)
				{

					//TODO: need to check if the device that disconnected is in the list of devices

					err = acSystemCreateDevice(hSystem, 0, &g_pDevice);
					if (err == AC_ERR_SUCCESS)
					{
						// Recreate device and notify other thread
						//    Once the device has been found, recreate it and
						//    notify the acquisition thread that it can stop
						//    waiting and continue acquiring images.
						printf("%sDevice reconnected\n", TAB4);

						// Ensure appropriate network settings
						//    Check that the device is on the same subnet after
						//    reconnecting the camera. If the camera/adapter are on a non
						//    169.254.*.* subnet but not using a persistent IP or DHCP,
						//    the camera will automatically be assigned an LLA and pick a
						//    169.254.*.* IP/subnet after reconnecting, causing the
						//    example to exit. There are several ways to fix this issue:
						//    (1) by setting a static IP to the Ethernet port, (2)
						//    forcing an IP address whenever the device is updated (see
						//    C_ForceIP), (3) running ArenaConfig to configure
						//    the adapter settings, or (4) setting up a
						//    persistent IP on the camera using IPConfigUtility
						uint32_t subnetMaskReconnect = 0;

						err = acSystemGetDeviceSubnetMask(hSystem, 0, &subnetMaskReconnect);
						if (err != AC_ERR_SUCCESS)
							exitThreads(err);

						if (subnetMask != subnetMaskReconnect)
						{

							printf("\n%sError: Subnet has changed upon reconnecting\n", TAB2);
							printf("%sSubnet at example start:   %" PRIu32 "\n", TAB3, subnetMask);
							printf("%sSubnet after reconnection: %" PRIu32 "\n", TAB3, subnetMaskReconnect);
							printf("\n%sPress enter to exit example\n", TAB1);

							// exit thread and notify other thread to exit
							exitThreads(err);
						}

						acThreadConditionVariableWake(&g_deviceConnected);
					}
				}
			}
			acThreadUnlock(&g_deviceMutex);
		}
	}

	// Notify other thread on exit
	//    If the device is disconnected at the time of exit, the other thread
	//    will be waiting for reconnection. Sending this notification allows the
	//    other thread to stop waiting and continue to exit.
	printf("%sNotify other thread on exit\n", TAB3);

	acThreadConditionVariableWake(&g_deviceConnected);

	THREAD_RETURN(err);
}

// acquires images while device is connected
// (1) starts stream
// (2) retrieves images
// (3) catches disconnections, destroying device appropriately
// (4) waits for reconnection
// (5) restarts stream, continuing to retrieve images
// (6) stops stream
THREAD_FUNCTION_SIGNATURE(AcquisitionThread)
{
	AC_ERROR err = AC_ERR_SUCCESS;
	acSystem hSystem = (acSystem*)lpParam;

	// get stream node map
	acNodeMap hTLStreamNodeMap = NULL;

	err = acDeviceGetTLStreamNodeMap(g_pDevice, &hTLStreamNodeMap);
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
	int numImages = 0;
	err = acDeviceStartStream(g_pDevice);
	if (err != AC_ERR_SUCCESS)
		exitThreads(err);

	// Get images while connected
	//    While the device is connected and streaming, grab images. This example
	//    just counts the images and requeues their buffers; however, using the
	//    image factory to copy or convert the images for display or processing
	//    might be a useful addition.
	while (g_isRunning && numImages < MAX_IMAGES)
	{
		// check that exit condition has not been called while inside the loop
		if (!g_pDevice)
		{
			acThreadLock(&g_deviceMutex);

			// wake up enumeration thread
			acThreadConditionVariableWake(&g_deviceDisconnected);

			
			
			// Wait for reconnection if necessary
			//    When the device has been disconnected, the acquisition thread
			//    waits on the enumeration thread to reconnect the device. This
			//    is done through a conditional variable. This conditional
			//    variable has also been set up to stop waiting if the
			//    application has been terminated.
			acThreadConditionVariableWait(&g_deviceConnected, &g_deviceMutex);

			if (g_pDevice)
			{
				// now if there is new device restart the stream
				err = acDeviceStartStream(g_pDevice);
				if (err != AC_ERR_SUCCESS)
					exitThreads(err);
			}

			acThreadUnlock(&g_deviceMutex);
		}


		printf("\r%sGet image %i", TAB3, numImages);
		fflush(stdout);
		acBuffer hBuffer = NULL;

		err = acDeviceGetBuffer(g_pDevice, IMAGE_TIMEOUT, &hBuffer);
		if (err != AC_ERR_SUCCESS)
		{
			// Catch disconnection
			//    Disconnections will most likely show themselves as read/write
			//    timeouts. This is caused as the host attempts to signal the
			//    device, but the device doesn't respond, timing out.
			printf("\n%sDevice disconnected\n", TAB4);

			err = acSystemDestroyDevice(hSystem, g_pDevice);
			if (err != AC_ERR_SUCCESS)
				exitThreads(err);

			// Lock access across threads
			//    Use a lock to protect access to shared resources, ensuring that
			//    simultaneous writes/reads across threads don't clobber one
			//    another.
			acThreadLock(&g_deviceMutex);
			g_pDevice = NULL;
			acThreadUnlock(&g_deviceMutex);
		}
		else
		{
			// increment image count and requeue buffer
			numImages = numImages + 1;

			err = acDeviceRequeueBuffer(g_pDevice, hBuffer);
			if (err != AC_ERR_SUCCESS)
				exitThreads(err);

			if (numImages >= MAX_IMAGES)
			{
				printf("\n%sAcquisition completed, press enter to continue\n", TAB3);
				g_isRunning = false;
			}
		}
	};

	// stop stream
	if (g_pDevice)
	{
		err = acDeviceStopStream(g_pDevice);
		if (err != AC_ERR_SUCCESS)
			exitThreads(err);
	}

	// wake up enumeration thread so it can close too
	acThreadConditionVariableWake(&g_deviceDisconnected);

	THREAD_RETURN(err);
}

// run example
// (1) create acquisition and enumeration threads
// (2) stop acquisition on key press
// (3) wait for threads to complete and close handles
AC_ERROR RunExample(acSystem hSystem)
{
	AC_ERROR err = AC_ERR_SUCCESS;

	// initialize lock and condition variable
	acThreadLockInitialize(&g_deviceMutex);
	acThreadConditionVariableInitialize(&g_deviceConnected);

	// Start acquisition and enumeration threads and wait for key
	//    Spawn a thread for acquisition and a thread for enumeration. Keeping
	//    this kind of work off the main thread allows for multiple things
	//    to happen simultaneously in more complex applications.
	printf("%sStart acquisition and enumeration threads\n", TAB1);
	g_isRunning = true;

	THREAD_ID(hChildAcquisitionThread);
	THREAD_ID(hChildEnumerationThread);

	acThreadCreate(AcquisitionThread, hSystem, &hChildAcquisitionThread);
	acThreadCreate(EnumerationThread, hSystem, &hChildEnumerationThread);

	// Stop acquisition on key press
	//    Because the main thread pushed all the work into the worker threads, it
	//    is free to wait for user input to end the application.
	printf("%sPress enter to stop acquisition\n", TAB2);
	getchar();

	// Wait for threads to complete and close handles
	//    When user input is received, the application must send a signal to the
	//    worker threads to indicate that this has happened. This is accomplished with a
	//    global boolean. The application should then wait for each thread to
	//    complete before continuing its shutdown.
	g_isRunning = false;

	acThreadDestroy(hChildEnumerationThread);
	acThreadDestroy(hChildAcquisitionThread);

	// deinitialize condition variable and thread lock and destroying all threads
	acThreadLockDeinitialize(&g_deviceMutex);
	acThreadConditionVariableDeinitialize(&g_deviceConnected);

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
	printf("C_Enumeration_HandlingDisconnections\n");
	printf("Please manually disconnect and reconnect device as device acquires images\n");
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
		printf("Waiting for a device...\n");
		while (true)
		{
			err = acSystemUpdateDevices(hSystem, SYSTEM_TIMEOUT);
			err = acSystemGetNumDevices(hSystem, &numDevices);
			if (numDevices > 0)
				break;
		}
	}
	size_t selection = 0;
	err = SelectDevice(hSystem, &numDevices, &selection);
	CHECK_RETURN;
	err = acSystemCreateDevice(hSystem, selection, &g_pDevice);
	CHECK_RETURN;

	// run example
	printf("Commence example\n\n");
	err = RunExample(hSystem);
	CHECK_RETURN;
	printf("\nExample complete\n");

	// clean up example
	if (g_pDevice)
	{
		err = acSystemDestroyDevice(hSystem, g_pDevice);
		CHECK_RETURN;
	}
	err = acCloseSystem(hSystem);
	CHECK_RETURN;

	printf("Press enter to complete\n");
	while (getchar() != '\n') {};
	getchar();
	return -1;
}
