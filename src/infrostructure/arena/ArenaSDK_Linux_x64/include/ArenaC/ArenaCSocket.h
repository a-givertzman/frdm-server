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
 * @fn AC_ERROR AC_API acSocketCreate(acSocket* phSocket)
 *
 * @param phSocket
 *  - Type: acSocket*
 *  - [Out] parameter
 *  - The socket object
 *
 * @return 
 *  - Type: AC_ERROR
 *  - Error code for the function
 *  - Returns AC_ERR_SUCCESS (0) on success
 *
 * <B> acCreateSocket </B> retrieves the socket object (acSocket). 
 *
 * @see 
 *  - acSocket
 */
AC_ERROR AC_API acSocketCreate(acSocket* phSocket);

/**
 * @fn AC_ERROR AC_API acSocketDestroy(acSocket hSocket)
 *
 * @param hSocket
 *  - Type: acSocket
 *  - [In] parameter
 *  - The socket object
 *
 * @return 
 *  - Type: AC_ERROR
 *  - Error code for the function
 *  - Returns AC_ERR_SUCCESS (0) on success
 *
 * <B> acDestroySocket </B> cleans up the socket (acSocket).
 *
 * @see 
 *  - acSocket
 */
AC_ERROR AC_API acSocketDestroy(acSocket hSocket);

/**
* @fn AC_ERROR AC_API acSocketOpenSender(acSocket hSocket);
*
* @param hSocket
*  - Type: acSocket
*  - [In] parameter
*  - The system object
*
* @return
*  - Type: AC_ERROR
*  - Error code for the function
*  - Returns AC_ERR_SUCCESS (0) on success
*
* <B> acSocketOpenSender </B>  initializes the send socket.
*
*/
AC_ERROR AC_API acSocketOpenSender(acSocket hSocket);

/**
 * @fn AC_ERROR AC_API acSocketCloseSender(acSocket hSocket);
 *
 * @param hSocket
 *  - Type: acSocket
 *  - [In] parameter
 *  - The socket object
 *
 * @return 
 *  - Type: AC_ERROR
 *  - Error code for the function
 *  - Returns AC_ERR_SUCCESS (0) on success
 *
 * <B> acSocketCloseSender </B> shuts down the send functionality of the 
 * socket and releases any associated resources.
 */
AC_ERROR AC_API acSocketCloseSender(acSocket hSocket);

/**
 * @fn AC_ERROR AC_API acSocketAddDestination(acSocket hSocket, unsigned short port);
 *
 * @param hSocket
 *  - Type: acSocket
 *  - [In] parameter
 *  - The socket object
 * 
 * @param port
 *  - Type: unsigned short
 *  - [In] parameter
 *  - The destination port number
 *
 * @return 
 *  - Type: AC_ERROR
 *  - Error code for the function
 *  - Returns AC_ERR_SUCCESS (0) on success
 *
 * <B> acSocketAddDestination </B> configures the socket to send data to a specified network port.
 */
AC_ERROR AC_API acSocketAddDestination(acSocket hSocket, unsigned short port);

/**
 * @fn AC_ERROR AC_API acSocketSendMessage(acSocket hSocket, const char* pMsg);
 *
 * @param hSocket
 *  - Type: acSocket
 *  - [In] parameter
 *  - The socket object
 * 
 * @param pMsg
 *  - Type: const char*
 *  - [In] parameter
 *  - The message to be sent
 *
 * @return 
 *  - Type: AC_ERROR
 *  - Error code for the function
 *  - Returns AC_ERR_SUCCESS (0) on success
 *
 * <B> acSendMessage </B> transmits a text message over the socket.
 */
AC_ERROR AC_API acSocketSendMessage(acSocket hSocket, const char* pMsg);

/**
 * @fn AC_ERROR AC_API acSocketSendImage(acSocket hSocket, acBuffer hBuffer);
 *
 * @param hSocket
 *  - Type: acSocket
 *  - [In] parameter
 *  - The socket object
 * 
 * @param hBuffer
 *  - Type: acBuffer
 *  - [In] parameter
 *  - The buffer containing the image to be sent
 *
 * @return 
 *  - Type: AC_ERROR
 *  - Error code for the function
 *  - Returns AC_ERR_SUCCESS (0) on success
 *
 * <B> acSendImage </B> sends an image data buffer over the socket.
 */
AC_ERROR AC_API acSocketSendImage(acSocket hSocket, acBuffer hBuffer);

/**
 * @fn AC_ERROR AC_API acSocketOpenListener(acSocket hSocket, unsigned short port);
 *
 * @param hSocket
 *  - Type: acSocket
 *  - [In] parameter
 *  - The socket object
 * 
 * @param port
 *  - Type: unsigned short
 *  - [In] parameter
 *  - The port number to listen on
 *
 * @return 
 *  - Type: AC_ERROR
 *  - Error code for the function
 *  - Returns AC_ERR_SUCCESS (0) on success
 *
 * <B> acSocketOpenListener </B> sets up the socket to receive incoming connections 
 * on a specified port.
 */
AC_ERROR AC_API acSocketOpenListener(acSocket hSocket, unsigned short port);

/**
 * @fn AC_ERROR AC_API acSocketCloseListener(acSocket hSocket);
 *
 * @param hSocket
 *  - Type: acSocket
 *  - [In] parameter
 *  - The socket object
 *
 * @return 
 *  - Type: AC_ERROR
 *  - Error code for the function
 *  - Returns AC_ERR_SUCCESS (0) on success
 *
 * <B> acSocketCloseListener </B> stops the socket from listening for 
 * incoming connections and releases any associated resources.
 */
AC_ERROR AC_API acSocketCloseListener(acSocket hSocket);

/**
 * @fn AC_ERROR AC_API acSocketReceiveMessage(acSocket hSocket, char* pMessageBuf, size_t* pBufLen);
 *
 * @param hSocket
 *  - Type: acSocket
 *  - [In] parameter
 *  - The socket object
 * 
 * @param pMessageBuf
 *  - Type: char*
 *  - [Out] parameter
 *  - Buffer to store the received message
 * 
 * @param pBufLen
 *  - Type: size_t*
 *  - [In/Out] parameter
 *  - On input, specifies the size of the buffer. On output, returns the length of the received message.
 *
 * @return 
 *  - Type: AC_ERROR
 *  - Error code for the function
 *  - Returns AC_ERR_SUCCESS (0) on success
 *
 * <B> acSocketReceiveMessage </B> retrieves a text message that was sent to the socket.
 */
AC_ERROR AC_API acSocketReceiveMessage(acSocket hSocket, char* pMessageBuf, size_t* pBufLen);

/**
 * @fn AC_ERROR AC_API acSocketReceiveSingleImage(acSocket hSocket, acBuffer* phBuffer);
 *
 * @param hSocket
 *  - Type: acSocket
 *  - [In] parameter
 *  - The socket object
 * 
 * @param phBuffer
 *  - Type: acBuffer*
 *  - [Out] parameter
 *  - Pointer to a buffer where the received image will be stored
 *
 * @return 
 *  - Type: AC_ERROR
 *  - Error code for the function
 *  - Returns AC_ERR_SUCCESS (0) on success
 *
 * <B> acSocketReceiveSingleImage </B> retrieves a single image sent to the socket and stores it in the specified buffer.
 */

AC_ERROR AC_API acSocketReceiveImage(acSocket hSocket, acBuffer* phBuffer);

#ifdef __cplusplus
} // extern "C"
#endif
