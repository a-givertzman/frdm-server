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
#pragma once

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @fn AC_ERROR AC_API acDeviceStartStream(acDevice hDevice)
 *
 * @param hDevice
 *  - Type: acDevice
 *  - [In] parameter
 *  - A device
 *
 * @return 
 *  - Type: AC_ERROR
 *  - Error code for the function
 *  - Returns AC_ERR_SUCCESS (0) on success
 *
 * <B> acDeviceStartStream </B> causes the device to begin streaming image/chunk
 * data (acBuffer). It must be called before image or chunk data buffers are
 * retrieved (acDeviceGetBuffer). The stream must be stopped
 * (acDeviceStopStream) when no longer needed.
 *
 * @see 
 *  - acBuffer
 *  - acDeviceGetBuffer
 *  - acDeviceStopStream
 */
AC_ERROR AC_API acDeviceStartStream(acDevice hDevice);

// todo check this function doc
/**
 * @fn AC_ERROR AC_API acDeviceStartStreamNumBuffersAndFlags(acDevice hDevice, size_t numBuffers)
 *
 * @param hDevice
 *  - Type: acDevice
 *  - [In] parameter
 *  - A device
 *
 * @param numBuffers
 *  - Type: size_t
 *  - [In] parameter
 *  - Number of internal buffers to use in the acquisition engine
 *
 * @return 
 *  - Type: AC_ERROR
 *  - Error code for the function
 *  - Returns AC_ERR_SUCCESS (0) on success
 *
 * <B> acDeviceStartStreamNumBuffersAndFlags </B> causes the device to begin
 * streaming image/chunk data (acBuffer) with given numBuffers and a streaming
 * standard. It must be called before image or chunk data buffers are retrieved
 * (acDeviceGetBuffer). The stream must be stopped (acDeviceStopStream) when no
 * longer needed.
 *
 * @see 
 *  - acBuffer
 *  - acDeviceGetBuffer
 *  - acDeviceStopStream
 */
AC_ERROR AC_API acDeviceStartStreamNumBuffersAndFlags(acDevice hDevice, size_t numBuffers);

/**
 * @fn AC_ERROR AC_API acDeviceStopStream(acDevice hDevice)
 *
 * @param hDevice
 *  - Type: acDevice
 *  - [In] parameter
 *  - A device
 *
 * @return 
 *  - Type: AC_ERROR
 *  - Error code for the function
 *  - Returns AC_ERR_SUCCESS (0) on success
 *
 * <B> acDeviceStopStream </B> stops the device from streaming image/chunk data
 * (acBuffer) and cleans up the stream. The stream must be stopped when
 * streaming is no longer needed.
 *
 * @see 
 *  - acBuffer
 */
AC_ERROR AC_API acDeviceStopStream(acDevice hDevice);

/**
 * @fn AC_ERROR AC_API acDeviceGetBuffer(acDevice hDevice, uint64_t timeout, acBuffer* phBuffer)
 *
 * @param hDevice
 *  - Type: acDevice
 *  - [In] parameter
 *  - A device
 *
 * @param timeout
 *  - Type: uint64_t
 *  - [In] parameter
 *  - Maximum time to wait for a buffer
 *
 * @param phBuffer
 *  - Type: acBuffer*
 *  - [Out] parameter
 *  - Next buffer in the output queue
 *
 * @return 
 *  - Type: AC_ERROR
 *  - Error code for the function
 *  - Returns AC_ERR_SUCCESS (0) on success
 *
 * <B> acDeviceGetBuffer </B> retrieves a buffer (acBuffer) from the device. It
 * must be called after the stream has started (acDeviceStartStream) and before
 * the stream has stopped (acDeviceStopStream). Retrieved buffers must be
 * requeued (acDeviceRequeueBuffer).
 *
 * @see 
 *  - acBuffer
 *  - acDeviceStartStream
 *  - acDeviceStopStream
 *  - acDeviceRequeueBuffer
 */
AC_ERROR AC_API acDeviceGetBuffer(acDevice hDevice, uint64_t timeout, acBuffer* phBuffer);

/**
 * @fn AC_ERROR AC_API acDeviceRequeueBuffer(acDevice hDevice, acBuffer pBuffer)
 *
 * @param hDevice
 *  - Type: acDevice
 *  - [In] parameter
 *  - A device
 *
 * @param pBuffer
 *  - Type: acBuffer
 *  - [In] parameter
 *  - Buffer to requeue
 *
 * @return 
 *  - Type: AC_ERROR
 *  - Error code for the function
 *  - Returns AC_ERR_SUCCESS (0) on success
 *
 * <B> acDeviceRequeueBuffer </B> relinquishes control of a buffer (acBuffer)
 * back to Arena. It must be called after a buffer has been retrieved
 * (acDeviceGetBuffer).
 *
 * @see 
 *  - acBuffer
 *  - acDeviceGetBuffer
 */
AC_ERROR AC_API acDeviceRequeueBuffer(acDevice hDevice, acBuffer pBuffer);

/**
 * @fn AC_ERROR AC_API acDeviceInitializeEvents(acDevice hDevice)
 *
 * @param hDevice
 *  - Type: acDevice
 *  - [In] parameter
 *  - A device
 *
 * @return 
 *  - Type: AC_ERROR
 *  - Error code for the function
 *  - Returns AC_ERR_SUCCESS (0) on success
 *
 * <B> acDeviceInitializeEvents </B> causes the underlying events engine to start
 * listening for events. It must be called before waiting on events
 * (acDeviceWaitOnEvent). The event infrastructure must be turned off
 * (acDeviceDeinitializeEvents) when no longer needed.
 *
 * @see 
 *  - acDeviceWaitOnEvent
 *  - acDeviceDeinitializeEvents
 */
AC_ERROR AC_API acDeviceInitializeEvents(acDevice hDevice);

/**
 * @fn AC_ERROR AC_API acDeviceDeinitializeEvents(acDevice hDevice)
 *
 * @param hDevice
 *  - Type: acDevice
 *  - [In] parameter
 *  - A device
 *
 * @return 
 *  - Type: AC_ERROR
 *  - Error code for the function
 *  - Returns AC_ERR_SUCCESS (0) on success
 *
 * <B> acDeviceDeinitializeEvents </B> stops the underlying events engine from listening
 * for messages, shutting it down and cleaning it up. It should be called only
 * after the events infrastructure has been initialized
 * (acDeviceInitializeEvents) and after all events have been processed
 * (acDeviceWaitOnEvent).
 *
 * @see 
 *  - acDeviceInitializeEvents
 *  - acDeviceWaitOnEvent
 */
AC_ERROR AC_API acDeviceDeinitializeEvents(acDevice hDevice);

/**
 * @fn AC_ERROR AC_API acDeviceWaitOnEvent(acDevice hDevice, int64_t timeout)
 *
 * @param hDevice
 *  - Type: acDevice
 *  - [In] parameter
 *  - A device
 *
 * @param timeout
 *  - Type: uint64_t
 *  - [In] parameter
 *  - Maximum time to wait for an event
 *
 * @return 
 *  - Type: AC_ERROR
 *  - Error code for the function
 *  - Returns AC_ERR_SUCCESS (0) on success
 *
 * <B> acDeviceWaitOnEvent </B> waits for an event to occur in order to process its data.
 * It must be called after the event infrastructure has been initialized
 * (acDeviceInitializeEvents) and before it is deinitialized
 * (acDeviceDeinitializeEvents). This will also trigger callbacks registered to
 * event nodes.
 *
 * @see 
 *  - acDeviceInitializeEvents
 *  - acDeviceDeinitializeEvents
 */
AC_ERROR AC_API acDeviceWaitOnEvent(acDevice hDevice, int64_t timeout);

/**
* @fn AC_ERROR AC_API acDeviceIsConnected(acDevice hDevice, bool8_t* pIsConnected)
*
* @param hDevice
*  - Type: acDevice
*  - [In] parameter
*  - A device
*
* @param pIsConnected
*  - Type: bool8_t*
*  - [Out] parameter
*  - True if the device is connected
*  - Otherwise, false
*
* @return
*  - Type: AC_ERROR
*  - Error code for the function
*  - Returns AC_ERR_SUCCESS (0) on success
*
* <B> acDeviceIsConnected </B> returns true if a device has been opened and
* maintains a valid communication socket. The device is opened when
* acSystemCreateDevice is called. If the connection to the device
* is lost this will return false.
*
* More specifically, for GigE devices, this flag is set to false when
* the Arena is not able to refresh the heartbeat on the device. If an
* operation times out more than 3 times, the device will be flagged as
* not connected.
*
* @see
*  - acSystemCreateDevice
*/
AC_ERROR AC_API acDeviceIsConnected(acDevice hDevice, bool8_t* pIsConnected);

/**
 * @fn AC_ERROR AC_API acDeviceGetNodeMap(acDevice hDevice, acNodeMap* phNodeMap)
 *
 * @param hDevice
 *  - Type: acDevice
 *  - [In] parameter
 *  - A device
 *
 * @param phNodeMap
 *  - Type: acNodeMap*
 *  - [Out] parameter
 *  - Main node map for the device
 *
 * @return 
 *  - Type: AC_ERROR
 *  - Error code for the function
 *  - Returns AC_ERR_SUCCESS (0) on success
 *
 * <B> acDeviceGetNodeMap </B> retrieves the already initialized main node map
 * (acNodeMap), used to access a device's complete feature set of nodes
 * (acNode).
 *
 * @see 
 *  - acNodeMap
 *  - acNode
 */
AC_ERROR AC_API acDeviceGetNodeMap(acDevice hDevice, acNodeMap* phNodeMap);

/**
 * @fn AC_ERROR AC_API acDeviceGetTLDeviceNodeMap(acDevice hDevice, acNodeMap* phNodeMap)
 *
 * @param hDevice
 *  - Type: acDevice
 *  - [In] parameter
 *  - A device
 *
 * @param phNodeMap
 *  - Type: acNodeMap*
 *  - [Out] parameter
 *  - GenTL node map for the device
 *
 * @return 
 *  - Type: AC_ERROR
 *  - Error code for the function
 *  - Returns AC_ERR_SUCCESS (0) on success
 *
 * <B> acDeviceGetTLDeviceNodeMap </B> retrieves the already initialized GenTL
 * device node map (acNodeMap), used to access a subset of cached device related
 * nodes (acNode).
 *
 * @see 
 *  - acNodeMap
 *  - acNode
 */
AC_ERROR AC_API acDeviceGetTLDeviceNodeMap(acDevice hDevice, acNodeMap* phNodeMap);

/**
 * @fn AC_ERROR AC_API acDeviceGetTLStreamNodeMap(acDevice hDevice, acNodeMap* phNodeMap)
 *
 * @param hDevice
 *  - Type: acDevice
 *  - [In] parameter
 *  - A device
 *
 * @param phNodeMap
 *  - Type: acNodeMap*
 *  - [Out] parameter
 *  - GenTL node map for the stream
 *
 * @return 
 *  - Type: AC_ERROR
 *  - Error code for the function
 *  - Returns AC_ERR_SUCCESS (0) on success
 *
 * <B> acDeviceGetTLStreamNodeMap </B> retrieves the already initialized GenTL
 * stream node map (acNodeMap), used to access stream-related nodes (acNode).
 *
 * @see 
 *  - acNodeMap
 *  - acNode
 */
AC_ERROR AC_API acDeviceGetTLStreamNodeMap(acDevice hDevice, acNodeMap* phNodeMap);

/**
 * @fn AC_ERROR AC_API acDeviceGetTLInterfaceNodeMap(acDevice hDevice, acNodeMap* phNodeMap)
 *
 * @param hDevice
 *  - Type: acDevice
 *  - [In] parameter
 *  - A device
 *
 * @param phNodeMap
 *  - Type: acNodeMap*
 *  - [Out] parameter
 *  - GenTL node map for the interface
 *
 * @return 
 *  - Type: AC_ERROR
 *  - Error code for the function
 *  - Returns AC_ERR_SUCCESS (0) on success
 *
 * <B> acDeviceGetTLInterfaceNodeMap </B> retrieves the already initialized GenTL
 * interface node map (acNodeMap), used to access interface related nodes
 * (acNode).
 *
 * @see 
 *  - acNodeMap
 *  - acNode
 */
AC_ERROR AC_API acDeviceGetTLInterfaceNodeMap(acDevice hDevice, acNodeMap* phNodeMap);

/**
* @fn AC_ERROR AC_API acDeviceRegisterImageCallback(acDevice hDevice, acCallback* phCallback, acImageCallbackFunction callbackFn, void* pParam);
*
* @param hDevice
 *  - Type: acDevice
 *  - [In] parameter
 *  - A device
*
* @param phCallback
 *  - Type: acCallback*
 *  - [Out] parameter
 *  - Handle to registered acCallback
*
* @param callbackFn
 *  - Type: acImageCallbackFunction
 *  - [In] parameter
 *  - The image callback function to register
*
* @param pParam
 *  - Type: void*
 *  - [In] parameter
 *  - Pointer to optional user-specified callback data
*
* @return
 *  - Type: AC_ERROR
 *  - Error code for the function
 *  - Returns AC_ERR_SUCCESS (0) on success
*
 * <B> acDeviceRegisterImageCallback </B> registers the specified image callback
 * function for the device. The optional user-specified data will be passed to
 * the image callback function.
*
* @see
 *  - acDeviceDeregisterImageCallback
*/
AC_ERROR AC_API acDeviceRegisterImageCallback(acDevice hDevice, acCallback* phCallback, acImageCallbackFunction callbackFn, void* pParam);

/**
* @fn AC_ERROR AC_API acDeviceDeregisterImageCallback(acDevice hDevice, acCallback* phCallback);
*
* @param hDevice
 *  - Type: acDevice
 *  - [In] parameter
 *  - A device
*
* @param phCallback
 *  - Type: acCallback*
 *  - [In] parameter
 *  - Handle to registered acCallback
*
* @return
 *  - Type: AC_ERROR
 *  - Error code for the function
 *  - Returns AC_ERR_SUCCESS (0) on success
*
 * <B> acDeviceDeregisterImageCallback </B> deregisters the specified image
 * callback for the device.
*
* @see
 *  - acDeviceRegisterImageCallback
*/
AC_ERROR AC_API acDeviceDeregisterImageCallback(acDevice hDevice, acCallback* phCallback);

/**
* @fn AC_ERROR AC_API acDeviceWaitForNextLeader(acDevice hDevice, int64_t timeout)
*
* @param hDevice
*  - Type: acDevice
*  - [In] parameter
*  - A device
*
* @param timeout
*  - Type: uint64_t
*  - [In] parameter
*  - Maximum time to wait for next leader 
*
* @return
*  - Type: AC_ERROR
*  - Error code for the function
*  - Returns AC_ERR_SUCCESS (0) on success
*
* <B> acDeviceWaitForNextLeader </B> will wait until the leader for the next image
* has arrived. It must be called after the stream has started
* (acDeviceStartStream) and before the stream has stopped
* (acDeviceStopStream).
*
* This function can be used to determine when the host has received the
* leader for the next image. Note that if the time that the camera has
* finished the exposure for the next image is desired, it is
* recommended to use the GenICam ExposureEnd Event instead.
*
* @see
*  - acDeviceResetWaitForNextLeader
*  - acDeviceStartStream
*  - acDeviceStopStream
*/
AC_ERROR AC_API acDeviceWaitForNextLeader(acDevice hDevice, int64_t timeout);

/**
* @fn AC_ERROR AC_API acDeviceResetWaitForNextLeader(acDevice hDevice)
*
* @param hDevice
*  - Type: acDevice
*  - [In] parameter
*  - A device
*
* @return
*  - Type: AC_ERROR
*  - Error code for the function
*  - Returns AC_ERR_SUCCESS (0) on success
*
* <B> acDeviceResetWaitForNextLeader </B> clears any pending flag for a received leader event.
*
* @see
*  - acDeviceWaitForNextLeader
*  - acDeviceStartStream
*  - acDeviceStopStream
*/
AC_ERROR AC_API acDeviceResetWaitForNextLeader(acDevice hDevice);

#ifdef __cplusplus
} // extern "C"
#endif
