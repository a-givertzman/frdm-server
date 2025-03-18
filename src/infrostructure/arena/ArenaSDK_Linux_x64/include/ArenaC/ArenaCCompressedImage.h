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
 * @fn AC_ERROR AC_API acCompressedImageGetPixelFormat(acBuffer hBuffer, uint64_t* pPixelFormat)
 *
 * @param hBuffer
 *  - Type: acBuffer
 *  - [In] parameter
 *  - An image
 *
 * @param pPixelFormat
 *  - Type: uint64_t*
 *  - [Out] parameter
 *  - Pixel format of the compressed image
 *
 * @return 
 *  - Type: AC_ERROR
 *  - Error code for the function
 *  - Returns AC_ERR_SUCCESS (0) on success
 *
 * <B> acCompressedImageGetPixelFormat </B> gets the pixel format (PfncFormat) of the
 * compressed image, as defined by the PFNC (Pixel Format Naming Convention). Compressed
 * images are self-describing, so the device does not need to be queried to get this
 * information.
 */
AC_ERROR AC_API acCompressedImageGetPixelFormat(acBuffer hBuffer, uint64_t* pPixelFormat);

/**
 * @fn AC_ERROR AC_API acCompressedImageGetTimestamp(acBuffer hBuffer, uint64_t* pTimestamp)
 *
 * @param hBuffer
 *  - Type: acBuffer
 *  - [In] parameter
 *  - An image
 *
 * @param pTimestamp
 *  - Type: uint64_t*
 *  - Unit: nanoseconds
 *  - [Out] parameter
 *  - Timestamp of the compressed image in nanoseconds
 *
 * @return 
 *  - Type: AC_ERROR
 *  - Error code for the function
 *  - Returns AC_ERR_SUCCESS (0) on success
 *
 * <B> acCompressedImageGetTimestamp </B> gets the timestamp of the compressed image in nanoseconds.
 * Compressed images are self-describing, so the device does not need to be queried to get
 * this information.
 */
AC_ERROR AC_API acCompressedImageGetTimestamp(acBuffer hBuffer, uint64_t* pTimestamp);

/**
 * @fn AC_ERROR AC_API acCompressedImageGetTimestampNs(acBuffer hBuffer, uint64_t* pTimestampNs)
 *
 * @param hBuffer
 *  - Type: acBuffer
 *  - [In] parameter
 *  - An image
 *
 * @param pTimestampNs
 *  - Type: uint64_t*
 *  - Unit: nanoseconds
 *  - [Out] parameter
 *  - Timestamp of the compressed image in nanoseconds
 *
 * @return 
 *  - Type: AC_ERROR
 *  - Error code for the function
 *  - Returns AC_ERR_SUCCESS (0) on success
 *
 * <B> acCompressedImageGetTimestampNs </B> gets the timestamp of the compressed image in nanoseconds.
 * Compressed images are self-describing, so the device does not need to be queried to get
 * this information.
 */
AC_ERROR AC_API acCompressedImageGetTimestampNs(acBuffer hBuffer, uint64_t* pTimestampNs);

/**
 * @fn AC_ERROR AC_API acCompressedImageGetData(acBuffer hBuffer, uint8_t** ppData)
 *
 * @param hBuffer
 *  - Type: acBuffer
 *  - [In] parameter
 *  - An image
 *
 * @param ppData
 *  - Type: uint8_t**
 *  - [Out] parameter
 *  - Pointer to the payload data
 *
 * @return 
 *  - Type: AC_ERROR
 *  - Error code for the function
 *  - Returns AC_ERR_SUCCESS (0) on success
 *
 * <B> acCompressedImageGetData </B> returns a pointer to the beginning of the compressed image's
 * payload data. The payload may include chunk data.
 */
AC_ERROR AC_API acCompressedImageGetData(acBuffer hBuffer, uint8_t** ppData);

#ifdef __cplusplus
} // extern "C"
#endif
