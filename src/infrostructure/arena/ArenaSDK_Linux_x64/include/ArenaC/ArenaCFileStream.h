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
 * @fn AC_ERROR AC_API acIDevFileStreamOpen(acIDevFileStream* phStream, acNodeMap hNodeMap, const char* pFileName)
 *
 * @param phStream
 *  - Type: acIDevFileStream*
 *  - [Out] parameter
 *  - Pointer to the input file stream handle
 *
 * @param hNodeMap
 *  - Type: acNodeMap
 *  - [In] parameter
 *  - Node map handle
 *
 * @param pFileName
 *  - Type: const char*
 *  - [In] parameter
 *  - File name to open
 *
 * @return
 *  - Type: AC_ERROR
 *  - Error code for the function
 *  - Returns AC_ERR_SUCCESS (0) on success
 *
 * <B> acIDevFileStreamOpen </B> opens an input file stream for reading from camera memory.
 */
AC_ERROR AC_API acIDevFileStreamOpen(acIDevFileStream* phStream, acNodeMap hNodeMap, const char* pFileName);

/**
 * @fn AC_ERROR AC_API acIDevFileStreamRead(acIDevFileStream hStream, char* pBuffer, size_t count)
 *
 * @param hStream
 *  - Type: acIDevFileStream
 *  - [In] parameter
 *  - Input file stream handle
 *
 * @param pBuffer
 *  - Type: char*
 *  - [Out] parameter
 *  - Buffer to read data into
 *
 * @param count
 *  - Type: size_t
 *  - [In] parameter
 *  - Number of bytes to read
 *
 * @return
 *  - Type: AC_ERROR
 *  - Error code for the function
 *  - Returns AC_ERR_SUCCESS (0) on success
 *
 * <B> acIDevFileStreamRead </B> reads data from the input file stream.
 */
AC_ERROR AC_API acIDevFileStreamRead(acIDevFileStream hStream, char* pBuffer, size_t count);

/**
 * @fn AC_ERROR AC_API acIDevFileStreamFail(acIDevFileStream hStream, bool8_t* pFail)
 *
 * @param hStream
 *  - Type: acIDevFileStream
 *  - [In] parameter
 *  - Input file stream handle
 *
 * @param pFail
 *  - Type: bool*
 *  - [Out] parameter
 *  - True if the stream is in a failed state, false otherwise
 *
 * @return
 *  - Type: AC_ERROR
 *  - Error code for the function
 *  - Returns AC_ERR_SUCCESS (0) on success
 *
 * <B> acIDevFileStreamFail </B> checks if the input file stream is in a failed state.
 */
AC_ERROR AC_API acIDevFileStreamFail(acIDevFileStream hStream, bool8_t* pFail);

/**
 * @fn AC_ERROR AC_API acIDevFileStreamClose(acIDevFileStream hStream)
 *
 * @param hStream
 *  - Type: acIDevFileStream
 *  - [In] parameter
 *  - Input file stream handle
 *
 * @return
 *  - Type: AC_ERROR
 *  - Error code for the function
 *  - Returns AC_ERR_SUCCESS (0) on success
 *
 * <B> acIDevFileStreamClose </B> closes the input file stream.
 */
AC_ERROR AC_API acIDevFileStreamClose(acIDevFileStream hStream);

/**
 * @fn AC_ERROR AC_API acODevFileStreamOpen(
 *     acODevFileStream* phStream,
 *     acNodeMap hNodeMap,
 *     const char* pFileName,
 *     AC_O_DEV_FILE_STREAM_OPEN_MODE openMode)
 *
 * @param phStream
 *  - Type: acODevFileStream*
 *  - [Out] parameter
 *  - Pointer to the output file stream handle
 *
 * @param hNodeMap
 *  - Type: acNodeMap
 *  - [In] parameter
 *  - Node map handle
 *
 * @param pFileName
 *  - Type: const char*
 *  - [In] parameter
 *  - Name of the file to open
 *
 * @param openMode
 *  - Type: AC_O_DEV_FILE_STREAM_OPEN_MODE
 *  - [In] parameter
 *  - Open mode flags (combination of AC_O_DEV_FILE_STREAM_OPEN_MODE_LIST enum values)
 *  - Use `AC_O_DEV_FILE_STREAM_OPEN_MODE_DEFAULT` for default mode
 *
 * @return
 *  - Type: AC_ERROR
 *  - Error code for the function
 *  - Returns `AC_ERR_SUCCESS` (0) on success
 *
 * <B> acODevFileStreamOpen </B> opens an output file stream for writing data to the camera's local storage.
 *
 */
AC_ERROR AC_API acODevFileStreamOpen(acODevFileStream* phStream, acNodeMap hNodeMap, const char* pFileName, 
	AC_O_DEV_FILE_STREAM_OPEN_MODE openMode);


/**
 * @fn AC_ERROR AC_API acODevFileStreamWrite(acODevFileStream hStream, const char* pBuffer, size_t count)
 *
 * @param hStream
 *  - Type: acODevFileStream
 *  - [In] parameter
 *  - Output file stream handle
 *
 * @param pBuffer
 *  - Type: const char*
 *  - [In] parameter
 *  - Buffer to write data from
 *
 * @param count
 *  - Type: size_t
 *  - [In] parameter
 *  - Number of bytes to write
 *
 * @return
 *  - Type: AC_ERROR
 *  - Error code for the function
 *  - Returns AC_ERR_SUCCESS (0) on success
 *
 * <B> acODevFileStreamWrite </B> writes data to the output file stream.
 */
AC_ERROR AC_API acODevFileStreamWrite(acODevFileStream hStream, const char* pBuffer, size_t count);

/**
 * @fn AC_ERROR AC_API acODevFileStreamFail(acODevFileStream hStream, bool8_t* pFail)
 *
 * @param hStream
 *  - Type: acODevFileStream
 *  - [In] parameter
 *  - Output file stream handle
 *
 * @param pFail
 *  - Type: bool*
 *  - [Out] parameter
 *  - True if the stream is in a failed state, false otherwise
 *
 * @return
 *  - Type: AC_ERROR
 *  - Error code for the function
 *  - Returns AC_ERR_SUCCESS (0) on success
 *
 * <B> acODevFileStreamFail </B> checks if the output file stream is in a failed state.
 */
AC_ERROR AC_API acODevFileStreamFail(acODevFileStream hStream, bool8_t* pFail);

/**
 * @fn AC_ERROR AC_API acODevFileStreamClose(acODevFileStream hStream)
 *
 * @param hStream
 *  - Type: acODevFileStream
 *  - [In] parameter
 *  - Output file stream handle
 *
 * @return
 *  - Type: AC_ERROR
 *  - Error code for the function
 *  - Returns AC_ERR_SUCCESS (0) on success
 *
 * <B> acODevFileStreamClose </B> closes the output file stream.
 */
AC_ERROR AC_API acODevFileStreamClose(acODevFileStream hStream);

#ifdef __cplusplus
} // extern "C"
#endif
