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

#include "IBuffer.h"

namespace Arena
{
	class IChunkData;

	/**
	 * @class ICompressedImage
	 *
	 * An interface to compressed images.
	 *
	 * The compressed image interface helps read and interpret compressed data. It inherits
	 * from the buffer interface (Arena::IBuffer).
	 *
	 * \code{.cpp}
	 * 	// retrieving a buffer, checking for and casting to compressed image
	 * 	{
	 * 		Arena::IBuffer* pBuffer = pDevice->GetBuffer(100);
	 * 		
	 * 		if (pBuffer->IsCompressedImage())
	 * 		{
	 * 			Arena::ICompressedImage* pCompressedImage = pBuffer->AsCompressedImage();
	 * 			// ...
	 * 		}
	 * 	}
	 * \endcode
	 * 
	 * In order to receive compressed data, certain pixel formats must be used,
	 * such as QOI_Mono8 or QOI_BayerRG8. Ensure camera supports a compressed
	 * image pixel format and is selected as the active pixel format.
	 *
	 * \code{.cpp}
	 * 	// enabling compressed pixel formats
	 * 	{
	 * 		GenApi::CEnumerationPtr pPixelFormat = pNodeMap->GetNode("PixelFormat");
	 *		pPixelFormat->FromString("QOI_Mono8");
	 * 	}
	 * \endcode
	 *
	 * Along with the functionality of its parent class (Arena::IBuffer), compressed
	 * images provide access to additional information particular to images. This
	 * includes:
	 *  - Offsets (Arena::ICompressedImage::GetOffsetX, 
	 *    Arena::ICompressedImage::GetOffsetY)
	 *  - Padding (Arena::ICompressedImage::GetPaddingX, 
	 *    Arena::ICompressedImage::GetPaddingY)
	 *  - Pixel information (Arena::ICompressedImage::GetPixelFormat,
	 *    Arena::ICompressedImage::GetPixelEndianness)
	 *  - Timestamps (Arena::ICompressedImage::GetTimestamp,
	 *    Arena::ICompressedImage::GetTimestampNs)
	 * 
	 * Width and height are not included in the functionality as the compressed image
	 * is simply blob data. 
	 * 
	 * In order to decompress the image, utilize the image factory (Arena::ImageFactory)
	 * and pass the compressed image into the decompress function.
	 * 
	 * \code{.cpp}
	 * 	// decompressing compressed images
	 * 	{
	 * 		Arena::ICompressedImage* pCompressedImage = pBuffer->AsCompressedImage();
	 *		Arena::IImage* pImage = Arena::ImageFactory::Decompress(pCompressedImage);
	 * 	}
	 * \endcode
	 *
	 * @warning 
	 *  - Should be requeued; same as other buffers.
	 *  - Properties are lazily instantiated from the acquisition engine.
	 *
	 * @see 
	 *  - Arena::IDevice
	 *  - Arena::IBuffer
	 *  - Arena::IImage
	 *  - Arena::ICompressedImage
	 */
	class ARENA_API ICompressedImage : public IBuffer
	{
	public:

		/**
		 * @fn virtual uint64_t GetPixelFormat()
		 *
		 * @return 
		 *  - Type: uint64_t
		 *  - Represents: enum PfncFormat
		 *  - Pixel format of the image
		 *
		 * <B> GetPixelFormat </B> gets the pixel format (PfncFormat) of the
		 * compressed image, as defined by the PFNC (Pixel Format Naming Convention). 
		 * Images are self-describing, so the device does not need to be queried to get
		 * this information.
		 *
		 * Compressed images are retrieved from a device (Arena::IDevice). If the
		 * image was retrieved from a device, the pixel format is populated by the 
		 * acquisition engine payload leader. The device itself is not queried as
		 * this data is present in the image data.
		 *
		 * Pixel format values are determined by the PFNC (Pixel Format Naming
		 * Convention) specification. The PFNC assigns a name and number to each
		 * pixel format, helping to standardize pixel formats. The number of bits
		 * per pixel can be found in each integer at bytes 5 and 6 (mask
		 * 0x00FF0000). The pixel format can be determined by the integer using
		 * the GetPixelFormatName function provided by the PFNC.
		 *
		 * @warning 
		 *  - Causes undefined behaviour if buffer requeued.
		 *  - Properties lazily instantiated if buffer retrieved from device.
		 *  - May throw GenICam::GenericException or other derived exception.
		 *
		 * @see 
		 *  - Arena::IDevice
		 */
		virtual uint64_t GetPixelFormat() = 0;

		/**
		 * @fn virtual uint64_t GetTimestamp()
		 *
		 * @return 
		 *  - Type: uint64_t
		 *  - Unit: nanoseconds
		 *  - Timestamp of the image in nanoseconds
		 *
		 * <B> GetTimestamp </B> gets the timestamp of the compressed image in
		 * nanoseconds. Images are self-describing, so the device does not need to
		 * be queried to get this information.
		 *
		 * Compressed images are retrieved from a device (Arena::IDevice). If the 
		 * image was retrieved from a device, the timestamp is populated by the
		 * acquisition engine payload leader. The device itself is not queried
		 * as this data is present in the image data.
		 *
		 * This is the same as the nanosecond timestamp call
		 * (Arena::ICompressedImage::GetTimestampNs).
		 *
		 * @warning 
		 *  - Causes undefined behaviour if buffer requeued.
		 *  - Properties lazily instantiated if buffer retrieved from device.
		 *  - May throw GenICam::GenericException or other derived exception.
		 *
		 * @see 
		 *  - Arena::IDevice
		 *  - Arena::ICompressedImage::GetTimestampNs
		 */
		virtual uint64_t GetTimestamp() = 0;

		/**
		 * @fn virtual uint64_t GetTimestampNs()
		 *
		 * @return 
		 *  - Type: uint64_t
		 *  - Unit: nanoseconds
		 *  - Timestamp of the image in nanoseconds
		 *
		 * <B> GetTimestampNs </B> gets the timestamp of the compressed image in
		 * nanoseconds. Images are self-describing, so the device does not need to
		 * be queried to get this information.
		 *
		 * Compressed images are retrieved from a device (Arena::IDevice). If the 
		 * image was retrieved from a device, the timestamp is populated by the
		 * acquisition engine payload leader. The device itself is not queried
		 * as this data is present in the image data.
		 *
		 * This is the same as the general timestamp call
		 * (Arena::ICompressedImage::GetTimestamp).
		 *
		 * @warning 
		 *  - Causes undefined behaviour if buffer requeued.
		 *  - Properties lazily instantiated if buffer retrieved from device.
		 *  - May throw GenICam::GenericException or other derived exception.
		 *
		 * @see 
		 *  - Arena::IDevice
		 *  - Arena::ICompressedImage::GetTimestamp
		 */
		virtual uint64_t GetTimestampNs() = 0;

		/**
		 * @fn virtual const uint8_t* GetData()
		 *
		 * @return 
		 *  - Type: const uint8_t*
		 *  - Pointer to the beginning of the payload data
		 *
		 * <B> GetData </B> retrieves a pointer to the buffer's payload data.
		 * This data may include compressed image data (along with its own header), 
		 * chunk data, or both.
		 *
		 * To check the type of data returned, compressed image
		 * (Arena::IBuffer::HasImageData) and chunk data
		 * (Arena::IBuffer::HasChunkData) can be checked for specifically, or the
		 * payload type (Arena::EBufferPayloadType) can be retrieved.
		 *
		 * The returned data only includes payload data, not transport layer
		 * protocol leaders, which is handled internally. The pointer can be used
		 * in conjunction with size getters (Arena::IBuffer::GetSizeFilled) to
		 * read, process, and pass the compressed data around.
		 *
		 * @warning 
		 *  - Causes undefined behavior if buffer requeued.
		 *  - Properties lazily instantiated if buffer retrieved from device.
		 *  - May throw GenICam::GenericException or other derived exception.
		 *
		 * @see 
		 *  - Arena::IBuffer::HasImageData
		 *  - Arena::IBuffer::HasChunkData
		 *  - Arena::IBuffer::GetSizeFilled
		 *  - Arena::EBufferPayloadType
		 */
		virtual const uint8_t* GetData() = 0;

		/**
		 * @fn virtual size_t GetSizeFilled()
		 *
		 * @return 
		 *  - Type: size_t
		 *  - Unit: bytes
		 *  - Size of the compressed data of the payload
		 *
		 * A getter for the size of the compressed data
		 *
		 * <B> GetSizeFilled </B> retrieves the size of the data of a buffer,
		 * excluding transport layer protocol leaders. It takes no inputs and
		 * returns the size of the data as output.
		 *
		 * The return value of <B> GetSizeFilled </B> should always be the same
		 * as the return value of <B> GetSizeOfBuffer </B>
		 * (Arena::IBuffer::GetSizeOfBuffer), but not because they are one and the
		 * same. <B> GetSizeFilled </B> returns the size of the data whereas <B>
		 * GetSizeOfBuffer </B> returns the size of the buffer, which is set
		 * according to the expected payload size ('PayloadSize').
		 *
		 * @warning 
		 *  - Causes undefined behaviour if buffer requeued.
		 *  - Properties lazily instantiated if buffer retrieved from device.
		 *  - May throw GenICam::GenericException or other derived exception.
		 *
		 * @see 
		 *  - Arena::IBuffer::GetSizeOfBuffer
		 */
		virtual size_t GetSizeFilled() = 0;

		/**
		 * @fn virtual size_t GetPayloadSize()
		 *
		 * @return 
		 *  - Type: size_t
		 *  - Unit: bytes
		 *  - Size of the intended payload
		 *
		 * <B> GetPayloadSize </B> retrieves the intended size of the payload.
		 * This is similar to the retrieved payload size
		 * (Arena::IBuffer::GetSizeFilled), but different in that missed data is
		 * included. This returns the same as the SFNC feature by the same name
		 * ('PayloadSize').
		 *
		 * @warning 
		 *  - Causes undefined behavior if buffer requeued.
		 *
		 * @see 
		 *  - Arena::IBuffer::GetSizeFilled
		 */
		virtual size_t GetPayloadSize() = 0;

		/**
		 * @fn virtual size_t GetSizeOfBuffer()
		 *
		 * @return 
		 *  - Type: size_t
		 *  - Unit: bytes
		 *  - Size of the buffer
		 *
		 * <B> GetSizeOfBuffer </B> retrieves the size of the buffer.
		 *
		 * The size filled is often same as the size of the buffer
		 * (Arena::IBuffer::GetSizeOfBuffer), but not because they are one and the
		 * same. <B> GetSizeFilled </B> returns the number of bytes received
		 * whereas <B> GetSizeOfBuffer </B> returns the size of the buffer, which
		 * can either be allocated by the user or calculated by Arena
		 * (Arena::IDevice::GetNodeMap, 'PayloadSize').
		 *
		 * The payload size is calculated at the beginning of the stream and
		 * cannot be recalculated until the stream has stopped. Because of this,
		 * features that can affect payload size ('Width', 'Height',
		 * 'PixelFormat') become unwritable when the stream has started.
		 *
		 * @warning 
		 *  - Causes undefined behaviour if buffer requeued.
		 *  - Properties lazily instantiated if buffer retrieved from device.
		 *  - May throw GenICam::GenericException or other derived exception.
		 *
		 * @see 
		 *  - Arena::IBuffer::GetSizeOfBuffer
		 *  - Arena::IDevice::GetNodeMap
		 */
		virtual size_t GetSizeOfBuffer() = 0;

		/**
		 * @fn virtual uint64_t GetFrameId()
		 *
		 * @return 
		 *  - Type: uint64_t
		 *  - Frame ID
		 *
		 * <B> GetFrameId </B> returns the frame ID, a sequential identifier for
		 * buffers.
		 *
		 * Frame IDs start at '1' and continue until 2^64-1
		 * (64-bit), at which point they roll over back to '1'. The frame ID should
		 * never be '0'.
		 *
		 * @warning 
		 *  - Causes undefined behaviour if buffer requeued.
		 *  - Properties lazily instantiated if buffer retrieved from device.
		 *  - May throw GenICam::GenericException or other derived exception.
		 */
		virtual uint64_t GetFrameId() = 0;

		/**
		 * @fn virtual size_t GetPayloadType()
		 *
		 * @return 
		 *  - Type: size_t
		 *  - Represents: enum Arena::EBufferPayloadType
		 *  - Type of payload data
		 *
		 * <B> GetPayloadType </B> returns a buffer's payload type
		 * (Arena::EBufferPayloadType), as defined in the GigE Vision specification.
		 *
		 * The payload type indicates how to interpret the data stored in the
		 * buffer.
		 * (Arena::IBuffer::GetData). LUCID devices may provide three ways to
		 * interpret the data:
		 *  - As an image (Arena::EBufferPayloadType::BufferPayloadTypeImage)
		 *  - As an image with chunk data appended to the end
		 *    (Arena::EBufferPayloadType::BufferPayloadTypeImageExtended) 
		 *  - As chunk data, which may or may not include image data as a chunk
		 *  - (Arena::EBufferPayloadType::BufferPayloadTypeChunkData)
		 *
		 * @warning 
		 *  - Causes undefined behaviour if buffer requeued.
		 *  - Properties lazily instantiated if buffer retrieved from device.
		 *  - May throw GenICam::GenericException or other derived exception.
		 *
		 * @see 
		 *  - Arena::IBuffer::GetData
		 *  - Arena::EBufferPayloadType
		 */
		virtual size_t GetPayloadType() = 0;

		/**
		 * @fn virtual bool HasImageData()
		 *
		 * @return 
		 *  - Type: bool
		 *  - True if the payload has image data
		 *  - False if the payload has image data packaged as chunk
		 *  - Otherwise, false
		 *
		 * <B> HasImageData </B> returns whether or not a buffer's payload may be
		 * interpreted as image data.
		 *
		 * <B> HasImageData </B> returns true if the payload type is:
		 *  - Arena::EBufferPayloadType::BufferPayloadTypeImage
		 *  - Arena::EBufferPayloadType::BufferPayloadTypeImageExtendedChunk
		 *
		 * It returns false if the payload type is:
		 *  - Arena::EBufferPayloadType::BufferPayloadTypeChunkData
		 *
		 * @warning 
		 *  - Causes undefined behaviour if buffer requeued.
		 *  - Properties lazily instantiated if buffer retrieved from device.
		 *  - May throw GenICam::GenericException or other derived exception.
		 *
		 * @see 
		 *  - Arena::EBufferPayloadType
		 */
		virtual bool HasImageData() = 0;

		/**
		 * @fn virtual bool HasChunkData()
		 *
		 * @return 
		 *  - True if the payload has chunk data
		 *  - Otherwise, false
		 *
		 * <B> HasChunkData </B> returns whether or not a buffer's payload may be
		 * interpreted as chunk data. Calling <B> HasChunkData </B> from chunk
		 * data returns true.
		 *
		 * <B> HasChunkData </B> returns true if the payload type is:
		 *  - Arena::EBufferPayloadType::BufferPayloadTypeChunkData
		 *  - Arena::EBufferPayloadType::BufferPayloadTypeImageExtendedChunk
		 *
		 * It returns false if the payload type is:
		 *  - Arena::EBufferPayloadType::BufferPayloadTypeImage
		 *
		 * @warning 
		 *  - Causes undefined behaviour if buffer requeued.
		 *  - Properties lazily instantiated if buffer retrieved from device.
		 *  - May throw GenICam::GenericException or other derived exception.
		 *
		 * @see 
		 *  - Arena::EBufferPayloadType
		 */
		virtual bool HasChunkData() = 0;

		/**
		 * @fn virtual IChunkData* AsChunkData()
		 *
		 * @return 
		 *  - Type: Arena::IChunkData*
		 *  - Pointer to the original object cast to chunk data
		 *  - Null on failure
		 *
		 * <B> AsChunkData </B> casts the buffer to a chunk data
		 * (Arena::IChunkData). This is only possible if the payload contains
		 * chunk data.
		 *
		 * @warning 
		 *  - Causes undefined behavior if buffer requeued.
		 *
		 * @see 
		 *  - Arena::IChunkData
		 */
		virtual IChunkData* AsChunkData() = 0;

		/**
		 * @fn virtual bool IsIncomplete()
		 *
		 * @return 
		 *  - Type: bool
		 *  - True if the data is incomplete
		 *  - Otherwise, false
		 *
		 * <B> IsIncomplete </B> returns whether or not a buffer's payload data
		 * is complete.
		 *
		 * Error handling may be required if the data is
		 * incomplete. An incomplete image signifies that the data size
		 * (Arena::IBuffer::GetSizeFilled) does not match the expected data size
		 * ('PayloadSize'). This is either due to missed packets or a small buffer.
		 *
		 * The number of missed packets may be discovered through the stream node
		 * map
		 * (Arena::IDevice::GetTLStreamNodeMap). The missed packet count feature
		 * ('StreamMissedPacketCount') is a cumulative count of all missed packets,
		 * and does not necessarily reflect the number of missed packets for any
		 * given buffer.
		 *
		 * A buffer may be missing data if the buffer to hold the data is too
		 * small. This happens when the size of the buffer
		 * (Arena::IBuffer::GetSizeOfBuffer) does not match the expected data
		 * size ('PayloadSize'). This function will also return true when
		 * checking whether the data is larger than the buffer
		 * (Arena::IBuffer::DataLargerThanBuffer).
		 *
		 * @warning 
		 *  - Causes undefined behaviour if buffer requeued.
		 *  - Properties lazily instantiated if buffer retrieved from device.
		 *  - May throw GenICam::GenericException or other derived exception.
		 *
		 * @see 
		 *  - Arena::IBuffer::GetSizeFilled
		 *  - Arena::IDevice::GetTLStreamNodeMap
		 *  - Arena::IBuffer::GetSizeOfBuffer
		 *  - Arena::IBuffer::DataLargerThanBuffer
		 */
		virtual bool IsIncomplete() = 0;

		/**
		 * @fn virtual bool DataLargerThanBuffer()
		 *
		 * @return 
		 *  - Type: bool
		 *  - True if the payload is larger than the buffer
		 *  - Otherwise, false
		 *
		 * <B> DataLargerThanBuffer </B> returns whether or not a buffer's
		 * payload data is too large for the buffer.
		 *
		 * A buffer may be missing data if the buffer to hold the data is too
		 * small. This happens when the size of the buffer
		 * (Arena::IBuffer::GetSizeOfBuffer) does not match the expected data
		 * size ('PayloadSize'). This function will also return true when
		 * checking whether the data is larger than the buffer.
		 *
		 * @warning 
		 *  - Causes undefined behaviour if buffer requeued.
		 *  - Properties lazily instantiated if buffer retrieved from device.
		 *  - May throw GenICam::GenericException or other derived exception.
		 *
		 * @see 
		 *  - Arena::IBuffer::GetSizeOfBuffer
		 */
		virtual bool DataLargerThanBuffer() = 0;

		/**
		 * @fn virtual bool VerifyCRC()
		 *
		 * @return 
		 *  - Type: bool
		 *  - True if the calculated CRC value equals the one sent from the
		 *    device
		 *  - Otherwise, false
		 *
		 * <B> VerifyCRC </B> calculates the CRC of a buffer's data and verifies
		 * it against the CRC value sent from the device. This helps verify that
		 * no data has been changed or missed during a transmission. This
		 * function calls a global helper function to calculate the CRC
		 * (Arena::CalculateCRC32).
		 *
		 * A CRC is performed by running a set of calculations on a dataset both
		 * before and after a transmission. The two calculated values are then
		 * compared for equality. If the values are the same, then the
		 * transmission is deemed successful; if different, then something in the
		 * transmission went wrong.
		 *
		 * A device can be set to send a CRC value by enabling its chunk data
		 * setting.
		 *
		 * \code{.cpp}
		 * 	// Enable chunk data and the CRC chunk
		 * 	{
		 * 		GenApi::INodeMap* pNodeMap = pDevice->GetNodeMap();
		 * 		
		 * 		GenApi::CBooleanPtr pChunkModeActive = pNodeMap->GetNode("ChunkModeActive");
		 * 		pChunkModeActive->SetValue(true);
		 * 		
		 * 		GenApi::CEnumerationPtr pChunkSelector = pNodeMap->GetNode("ChunkSelector");
		 * 		GenApi::CEnumEntryPtr pCRC = pChunkSelector->GetEntryByname("CRC");
		 * 		pChunkSelector->SetIntValue(pCRC->GetValue());
		 * 		
		 * 		GenApi::CBooleanPtr pChunkEnable = pNodeMap->GetNode("ChunkEnable");
		 * 		pChunkEnable->SetValue(true);
		 * 	}
		 * \endcode
		 *
		 * The data can then be checked by verifying the CRC.
		 *
		 * \code{.cpp}
		 * 	// Verifying a buffer's data
		 * 	{
		 * 		Arena::IBuffer* pBuffer = pDevice->GetBuffer(timeout);
		 * 		if (!pBuffer->VerifyCRC())
		 * 		{
		 * 			// data not complete
		 * 		}
		 * 	}
		 * \endcode
		 *
		 * @warning 
		 *  - May throw GenICam::GenericException or other derived exception.
		 *  - Throws an exception if chunk data disabled or not present, or if CRC chunk
		 *    disabled.
		 *
		 * @see 
		 *  - Arena::CalculateCRC
		 *  - Arena::IBuffer
		 */
		virtual bool VerifyCRC() = 0;

		/**
		 * @fn virtual ~IChunkData()
		 *
		 * A destructor
		 */
		virtual ~ICompressedImage(){};

	protected:
		ICompressedImage(){};
		virtual ICompressedImage* AsCompressedImage() = 0;
	};
} // namespace Arena