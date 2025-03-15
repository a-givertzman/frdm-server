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
 * @fn AC_ERROR AC_API acImageFactoryCreate(uint8_t* pData, size_t dataSize, size_t width, size_t height, uint64_t pixelFormat, acBuffer* phDst)
 *
 * FactoryCreate(uint8_t pData, size_t dataSize, size_t width, size_t height,
 * uint64_t pixelFormat, acBuffer* phDst)
 *
 * @param pData
 *  - Type: uint8_t*
 *  - [Out] parameter
 *  - Pointer to the beginning of the payload data
 *
 * @param dataSize
 *  - Type: size_t
 *  - [In] parameter
 *  - Size of the data
 *
 * @param width
 *  - Type: size_t
 *  - [In] parameter
 *  - Width of the image to create
 *
 * @param height
 *  - Type: size_t
 *  - [In] parameter
 *  - Height of the image to create
 *
 * @param pixelFormat
 *  - Type: uint64_t
 *  - [In] parameter
 *  - Pixel format of the image to create
 *
 * @param phDst
 *  - Type: acBuffer*
 *  - [Out] parameter
 *  - Image created from the parameters
 *
 * @return 
 *  - Type: AC_ERROR
 *  - Error code for the function
 *  - Returns AC_ERR_SUCCESS (0) on success
 *
 * <B> acImageFactoryCreate </B> creates an image (acBuffer) from a minimal set
 * of parameters. Images created with the image factory must be destroyed
 * (acImageFactoryDestroy) when no longer needed.
 *
 * @see 
 *  - acBuffer
 *  - acImageFactoryDestroy
 */
AC_ERROR AC_API acImageFactoryCreate(uint8_t* pData, size_t dataSize, size_t width, size_t height, uint64_t pixelFormat, acBuffer* phDst);
AC_ERROR AC_API acImageFactoryCreateEmpty(size_t dataSize, size_t width, size_t height, uint64_t pixelFormat, acBuffer* phDst);

/**
* @fn AC_ERROR AC_API acImageFactoryShallow(uint8_t* pData, size_t dataSize, size_t width, size_t height, uint64_t pixelFormat, acBuffer* phDst);
 *
 * @param pData
 *  - Type: uint8_t*
 *  - [Out] parameter
 *  - Pointer to the beginning of the payload data
 *
 * @param dataSize
 *  - Type: size_t
 *  - [In] parameter
 *  - Size of the data
 *
 * @param width
 *  - Type: size_t
 *  - [In] parameter
 *  - Width of the image to create
 *
 * @param height
 *  - Type: size_t
 *  - [In] parameter
 *  - Height of the image to create
 *
 * @param pixelFormat
 *  - Type: uint64_t
 *  - [In] parameter
 *  - Pixel format of the image to create
 *
 * @param phDst
 *  - Type: acBuffer*
 *  - [Out] parameter
 *  - Image created from the parameters
 *
 * @return 
 *  - Type: AC_ERROR
 *  - Error code for the function
 *  - Returns AC_ERR_SUCCESS (0) on success
 *
 * @see 
 *  - acBuffer
 *  - acImageFactoryDestroy
*/
AC_ERROR AC_API acImageFactoryShallow(uint8_t* pData, size_t dataSize, size_t width, size_t height, uint64_t pixelFormat, acBuffer* phDst);

/**
 * @fn AC_ERROR AC_API acImageFactoryCreateCompressedImage(uint8_t* pData, size_t dataSize, uint64_t pixelFormat, acBuffer* phDst)
 *
 * @param pData
 *  - Type: uint8_t*
 *  - [Out] parameter
 *  - Pointer to the beginning of the payload data
 * 
 * @param dataSize
 *  - Type: size_t
 *  - [In] parameter
 *  - Size of the data
 *
 * @param pixelFormat
 *  - Type: uint64_t
 *  - [In] parameter
 *  - Pixel format of the compressed image to create
 *
 * @param phDst
 *  - Type: acBuffer*
 *  - [Out] parameter
 *  - Compressed image created from the parameters
 *
 * @return 
 *  - Type: AC_ERROR
 *  - Error code for the function
 *  - Returns AC_ERR_SUCCESS (0) on success
 *
 * <B> acImageFactoryCreateCompressedImage </B> creates a compressed image
 * (acBuffer) from a minimal set of parameters. Images created with the image
 * factory must be destroyed (acImageFactoryDestroy) when no longer needed.
 *
 * @see 
 *  - acBuffer
 *  - acImageFactoryDestroy
 */
AC_ERROR AC_API acImageFactoryCreateCompressedImage(uint8_t* pData, size_t dataSize, uint64_t pixelFormat, acBuffer* phDst);

/**
 * @fn AC_ERROR AC_API acImageFactoryCopy(acBuffer hSrc, acBuffer* phDst)
 *
 * @param hSrc
 *  - Type: acBuffer
 *  - [In] parameter
 *  - Image to copy
 *
 * @param phDst
 *  - Type: acBuffer*
 *  - [Out] parameter
 *  - Deep copy of the image
 *
 * @return 
 *  - Type: AC_ERROR
 *  - Error code for the function
 *  - Returns AC_ERR_SUCCESS (0) on success
 *
 * <B> acImageFactoryCopy </B> creates a deep copy of an image (acBuffer) from
 * another image. Images created with the image factory must be destroyed
 * (acImageFactoryDestroy) when no longer needed.
 *
 * @see 
 *  - acBuffer
 *  - acImageFactoryDestroy
 */
AC_ERROR AC_API acImageFactoryCopy(acBuffer hSrc, acBuffer* phDst);
AC_ERROR AC_API acImageFactoryCopyCompressedImage(acBuffer hSrc, acBuffer* phDst);

/**
 * @fn AC_ERROR AC_API acImageFactoryConvert(acBuffer hSrc, uint64_t pixelFormat, acBuffer* phDst)
 *
 * @param hSrc
 *  - Type: acBuffer
 *  - [In] parameter
 *  - Image to convert
 *
 * @param pixelFormat
 *  - Type: uint64_t
 *  - [In] parameter
 *  - Pixel format to convert to
 *
 * @param phDst
 *  - Type: acBuffer*
 *  - [Out] parameter
 *  - Convert image
 *
 * @return 
 *  - Type: AC_ERROR
 *  - Error code for the function
 *  - Returns AC_ERR_SUCCESS (0) on success
 *
 * <B> acImageFactoryConvert </B> converts an image (acBuffer) to the selected pixel
 * format. In doing so, it creates a completely new image, similar to a deep copy
 * but with a different pixel format. Images created with the image factory must
 * be destroyed (acImageFactoryDestroy) when no longer needed; otherwise, memory
 * will leak.
 *
 * @see 
 *  - acBuffer
 *  - acImageFactoryDestroy
 */
AC_ERROR AC_API acImageFactoryConvert(acBuffer hSrc, uint64_t pixelFormat, acBuffer* phDst);

/**
 * @fn AC_ERROR AC_API acImageFactoryDecompressImage(acBuffer hSrc, acBuffer* phDst)
 *
 * @param hSrc
 *  - Type: acBuffer
 *  - [In] parameter
 *  - Image to convert
 *
 * @param phDst
 *  - Type: acBuffer*
 *  - [Out] parameter
 *  - Decompressed image
 *
 * @return
 *  - Type: AC_ERROR
 *  - Error code for the function
 *  - Returns AC_ERR_SUCCESS (0) on success
 *
 * <B> acImageFactoryDecompressImage </B> decompresses a compressed image (acBuffer). 
 * In doing so, it creates a completely new image, similar to a deep copy
 * but with an uncompressed pixel format. Images created with the image factory must
 * be destroyed (acImageFactoryDestroy) when no longer needed; otherwise, memory
 * will leak.
 * 
 * @see
 *  - acBuffer
 */
AC_ERROR AC_API acImageFactoryDecompressImage(acBuffer hSrc, acBuffer* phDst);

/**
 * @fn AC_ERROR AC_API acImageFactoryConvertBayerAlgorithm(acBuffer hSrc, uint64_t pixelFormat, AC_BAYER_ALGORITHM bayerAlgo, acBuffer* phDst)
 *
 * @param hSrc
 *  - Type: acBuffer
 *  - [In] parameter
 *  - Image to convert
 *
 * @param pixelFormat
 *  - Type: uint64_t
 *  - [In] parameter
 *  - Pixel format to convert to
 *
 * @param bayerAlgo
 *  - Type: AC_BAYER_ALGORITHM
 *  - [In] parameter
 *  - Bayer conversion algorithm to use
 *  - Only applicable when converting from bayer
 *
 * @param phDst
 *  - Type: acBuffer*
 *  - [Out] parameter
 *  - Converted image
 *
 * @return 
 *  - Type: AC_ERROR
 *  - Error code for the function
 *  - Returns AC_ERR_SUCCESS (0) on success
 *
 * <B> acImageFactoryConvert </B> converts an image (acBuffer) to the selected pixel
 * format. In doing so, it creates a completely new image, similar to a deep copy
 * but with a different pixel format. Images created with the image factory must
 * be destroyed (acImageFactoryDestroy) when no longer needed; otherwise, memory
 * will leak.
 *
 * @see 
 *  - acBuffer
 *  - acImageFactoryDestroy
 */
AC_ERROR AC_API acImageFactoryConvertBayerAlgorithm(acBuffer hSrc, uint64_t pixelFormat, AC_BAYER_ALGORITHM bayerAlgo, acBuffer* phDst);

/**
 * @fn AC_ERROR AC_API acImageFactoryDestroy(acBuffer hBuffer)
 *
 * @param hBuffer
 *  - Type: acBuffer
 *  - [In] parameter
 *  - Image to destroy
 *  - Image must be from image factory
 *
 * @return 
 *  - Type: AC_ERROR
 *  - Error code for the function
 *  - Returns AC_ERR_SUCCESS (0) on success
 *
 * <B> acImageFactoryDestroy </B> cleans up an image (acBuffer) and deallocates
 * its memory. It must be called on any image created by the image factory
 * (acImageFactoryCreate, acImageFactoryCopy, acImageFactoryConvert, acImageFactoryDecompressImage).
 *
 * @see 
 *  - acBuffer
 *  - acImageFactoryCreate
 */
AC_ERROR AC_API acImageFactoryDestroy(acBuffer hBuffer);
AC_ERROR AC_API acImageFactoryDestroyCompressedImage(acBuffer hBuffer);

/**
* @fn AC_ERROR AC_API acImageFactorySelectBits(acBuffer hSrc, size_t numBits, int offset, acBuffer* phDst);
* 
* @param hSrc
*  - Type: acBuffer
*  - [In] parameter
*  - Image to select bits from
*
* @param numBits
*  - Type: size_t
*  - [In] parameter
*  - Number of Bits to select
*
* @param offset
*  - Type: int
*  - [In] parameter
*  - Offset the Bits to select
*
* @param phDst
*  - Type: acBuffer*
*  - [Out] parameter
*  - Copy of the image
*
* @return
*  - Type: AC_ERROR
*  - Error code for the function
*  - Returns AC_ERR_SUCCESS (0) on success
*
* <B> acImageFactorySelectBits </B> select bits from an image.
*
* @see
*  - acBuffer
*  - acImageFactoryCreate
*/
AC_ERROR AC_API acImageFactorySelectBits(acBuffer hSrc, size_t numBits, int offset, acBuffer* phDst);

/**
* @fn AC_ERROR AC_API acImageFactorySelectBitsAndScale(acBuffer hSrc, size_t numBits, double offset, acBuffer* phDst)
*
* @param hSrc
*  - Type: acBuffer
*  - [In] parameter
*  - Image to select bits and scale
*
* @param numBits
*  - Type: size_t
*  - [In] parameter
*  - Number of Bits to scale to
*
* @param offset
*  - Type: double
*  - [In] parameter
*  - Offset to scale to
*
* @param phDst
*  - Type: acBuffer*
*  - [Out] parameter
*  - Scaled copy of the image
*
* @return
*  - Type: AC_ERROR
*  - Error code for the function
*  - Returns AC_ERR_SUCCESS (0) on success
*
* <B> acImageFactorySelectBitsAndScale </B> scales image.
*
* @see
*  - acBuffer
*  - acImageFactoryCreate
*/
AC_ERROR AC_API acImageFactorySelectBitsAndScale(acBuffer hSrc, size_t numBits, double offset, acBuffer* phDst);

/**
* @fn AC_ERROR AC_API acImageFactoryProcessSoftwareLUT(acBuffer hSrc, uint8_t* pLUT, size_t len, acBuffer* phDst)
*
* @param hSrc
*  - Type: acBuffer
*  - [In] parameter
*  - Image to select bits and scale
*
* @param pLUT
*  - Type: uint8_t*
*  - [In] parameter
*  - Pointer to the beginning of the lookup table
*
* @param len
*  - Type: size_t
*  - [In] parameter
*  - Length of image buffer
*
* @param phDst
*  - Type: acBuffer*
*  - [Out] parameter
*  - Copy of the image with redefined of values
*
* @return
*  - Type: AC_ERROR
*  - Error code for the function
*  - Returns AC_ERR_SUCCESS (0) on success
*
* <B> acImageFactoryProcessSoftwareLUT </B> runs an image through a lookup table, allowing for a redefinition of values.
*
* @see
*  - acBuffer
*/
AC_ERROR AC_API acImageFactoryProcessSoftwareLUT(acBuffer hSrc, uint8_t* pLUT, size_t len, acBuffer* phDst);

/**
* @fn AC_ERROR AC_API acImageFactoryDeinterleaveChannels(acBuffer hSrc, acBuffer* phDst)
*
* @param hSrc
*  - Type: acBuffer
*  - [In] parameter
*  - Image to deinterleave channels
*
* @param phDst
*  - Type: acBuffer*
*  - [Out] parameter
*  - Copy of the planar image
*
* @return
*  - Type: AC_ERROR
*  - Error code for the function
*  - Returns AC_ERR_SUCCESS (0) on success
*
* <B> acImageFactoryDeinterleaveChannels </B> separates interleaved channels into a planar image.
*
* @see
*  - acBuffer
*/
AC_ERROR AC_API acImageFactoryDeinterleaveChannels(acBuffer hSrc, acBuffer* phDst);
AC_ERROR AC_API acImageFactoryDeinterleaveChannelsShallow(acBuffer hSrc, uint8_t* pData, size_t len, acBuffer* phDst);
AC_ERROR AC_API acImageFactoryDeinterleaveChannelsLen(acBuffer hSrc, size_t* len);

/**
* @fn AC_ERROR AC_API acImageFactorySplitChannels(acBuffer hSrc, acBuffer* phDst[], size_t* numBuffers)
*
* @param hSrc
*  - Type: acBuffer
*  - [In] parameter
*  - Image to split channels from
*
* @param phDst[]
*  - Type: acBuffer*
*  - [Out] parameter
*  - Pointer to a vector with splited image
*
* @param numBuffers
*  - Type: size_t*
*  - [In] parameter
*  - Number of buffers to split image
*
* @return
*  - Type: AC_ERROR
*  - Error code for the function
*  - Returns AC_ERR_SUCCESS (0) on success
*
* <B> acImageFactorySplitChannels </B> takes an interleaved image and separates the channels into multiple images.
*
* @see
*  - acBuffer
*/
AC_ERROR AC_API acImageFactorySplitChannels(acBuffer hSrc, acBuffer* phDst[], size_t* numBuffers);

#ifdef __cplusplus
} // extern "C"
#endif
