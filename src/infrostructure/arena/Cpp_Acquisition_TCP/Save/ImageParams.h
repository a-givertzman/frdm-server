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

namespace Save
{
	/**
	 * @class ImageParams
	 *
	 * A container to hold and calculate image parameters.
	 */
	class SAVE_API ImageParams
	{
	public:
		/**
		 * @fn ImageParams()
		 *
		 * An empty constructor. Initializes width, height, and bits per pixel to
		 * 0.
		 */
		ImageParams();

		/**
		 * @fn ImageParams(size_t width, size_t height, size_t bitsPerPixel, bool topToBottom = true)
		 *
		 * @param width
		 *  - Type: size_t
		 *  - Image width
		 *
		 * @param height
		 *  - Type: size_t
		 *  - Image height
		 *
		 * @param bitsPerPixel
		 *  - Type: size_t
		 *  - Number of bits per pixel
		 *
		 * @param topToBottom
		 *  - Type: bool
		 *  - Default: true
		 *  - If true, image data is read/written top to bottom
		 *  - Otherwise, bottom to top
		 *  - Only applicable when saving JPEG and BMP
		 * 
		 * @param numVertices
		 *  - Type: size_t
		 *  - Default: zero (not applicable)
		 *  - Number of vertices in image
		 *  - Only applicable when saving XYTP images
		 *
		 * A constructor.
		 */
		ImageParams(size_t width, size_t height, size_t bitsPerPixel, bool topToBottom = true, size_t numVertices = 0);

		/**
		 * @fn ImageParams(const ImageParams& params)
		 *
		 * @param params
		 *  - Type: const Save::ImageParams&
		 *  - Image parameters to copy
		 *
		 * A copy constructor.
		 */
		ImageParams(const ImageParams& params);

		/**
		 * @fn const ImageParams& operator=(ImageParams params)
		 *
		 * @param params
		 *  - Type: Save::ImageParams
		 *  - Image parameters to copy
		 *
		 * @return 
		 *  - Type: const Save::ImageParams&
		 *  - Copied image parameters
		 *
		 * A copy assignment operator.
		 */
		const ImageParams& operator=(ImageParams params);

		/**
		 * @fn ~ImageParams()
		 *
		 * A destructor.
		 */
		virtual ~ImageParams();

		/**
		 * @fn virtual void SetWidth(size_t width)
		 *
		 * @param width
		 *  - Type: size_t
		 *  - Image width
		 *
		 * @return 
		 *  - none
		 *
		 * <B> SetWidth </B> sets the width parameter.
		 */
		virtual void SetWidth(size_t width);

		/**
		 * @fn virtual void SetHeight(size_t height)
		 *
		 * @param height
		 *  - Type: size_t
		 *  - Image height
		 *
		 * @return 
		 *  - none
		 *
		 * <B> SetHeight </B> sets the height parameter.
		 */
		virtual void SetHeight(size_t height);

		/**
		 * @fn virtual void SetBitsPerPixel(size_t bpp)
		 *
		 * @param bpp
		 *  - Type: size_t
		 *  - Number of bits per pixel
		 *
		 * @return 
		 *  - none
		 *
		 * <B> SetBitsPerPixel </B> sets the bits per pixel parameter.
		 */
		virtual void SetBitsPerPixel(size_t bpp);

		/**
		 * @fn virtual void SetTopToBottom(bool topToBottom)
		 *
		 * @param topToBottom
		 *  - Type: bool
		 *  - If true, image data is read/written top to bottom
		 *  - Otherwise, bottom to top
		 *  - Only applicable when saving JPEG and BMP
		 *
		 * @return 
		 *  - none
		 *
		 * <B> SetTopToBottom </B> sets whether the image is read/written from
		 * top to bottom or bottom to top.
		 */
		virtual void SetTopToBottom(bool topToBottom);

		/**
		 * @fn virtual void SetNumVertices(size_t numVertices)
		 *
		 * @param numVertices
		 *  - Type: size_t
		 *  - Number of vertices in the bytes. If true, image data is read/written top to bottom
		 *  - Only applicable for XYTP images
		 *
		 * @return 
		 *  - none
		 *
		 * <B> SetNumVertices </B> sets the number of valid vertices in the image.
		 * The number of vertices must be set correctly to the number of valid vertices in the image.
		 * The number of vertices can be calculated with sizeFilled/bytes per pixel.
		 */
		virtual void SetNumVertices(size_t numVertices);

		/**
		 * @fn virtual size_t GetWidth() const
		 *
		 * @return 
		 *  - Type: size_t
		 *  - Image width
		 *
		 * <B> GetWidth </B> gets the width parameter.
		 */
		virtual size_t GetWidth() const;

		/**
		 * @fn virtual size_t GetHeight() const
		 *
		 * @return 
		 *  - Type: size_t
		 *  - Image height
		 *
		 * <B> GetHeight </B> gets the height parameter.
		 */
		virtual size_t GetHeight() const;

		/**
		 * @fn virtual size_t GetBitsPerPixel() const
		 *
		 * @return 
		 *  - Type: size_t
		 *  - Number of bits per pixel
		 *
		 * <B> GetBitsPerPixel </B> gets the bits per pixel parameter.
		 */
		virtual size_t GetBitsPerPixel() const;

		/**
		 * @fn virtual bool GetTopToBottom() const
		 *
		 * @return 
		 *  - Type: bool
		 *  - If true, image data is read/written top to bottom
		 *  - Otherwise, bottom to top
		 *  - Only applicable when saving JPEG and BMP
		 *
		 * <B> GetTopToBottom </B> gets whether the image is read/written from
		 * top to bottom or bottom to top.
		 */
		virtual bool GetTopToBottom() const;

		/**
		 * @fn virtual size_t GetSize() const
		 *
		 * @return 
		 *  - Type: size_t
		 *  - Image size
		 *
		 * <B> GetSize </B> calculates the size of the image. The size is
		 * calculated by multiplying the width, height, and bytes per pixel (bits
		 * per pixel divided by 8) and adding an extra byte if a remainder of
		 * bits remains.
		 */
		virtual size_t GetSize() const;

		/**
		 * @fn virtual size_t GetStride() const
		 *
		 * @return 
		 *  - Type: size_t
		 *  - Image stride/pitch
		 *
		 * <B> GetStride </B> calculates the stride of an image. The stride is
		 * calculated by multiplying the width by the number of bytes per pixel
		 * (bits per pixel divided by 8).
		 */
		virtual size_t GetStride() const;

		/**
		 * @fn virtual size_t GetNumVertices() const
		 *
		 * @return 
		 *  - Type: size_t
		 *  - Number of vertices in the image
		 *  - Only applicable for XYTP images
		 * 
		 * <B> GetNumVertices </B> returns the number of vertices previously set by SetNumVertices()
		 */
		virtual size_t GetNumVertices() const;

	private:
		void* m_pInternal;
	};
} // namespace Save
