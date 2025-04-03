use sal_core::error::Error;
use sal_sync::services::entity::name::Name;
use crate::infrostructure::arena::ac_err::AcErr;
use super::{bindings::{acBuffer, acDevice}, image::Image, pixel_format::PixelFormat};

///
/// - Received image buffer from device,
/// - Returns an [Image]
/// - Decompress received from Arena SDK buffer if required
pub struct AcBuffer {
    name: Name,
    device: acDevice,
    pub pixel_format: PixelFormat,
    input: acBuffer,
    decompressed: acBuffer,
}
//
//
impl AcBuffer {
    ///
    /// Returns new instance of the Device Node Map of kind:
    /// - DeviceNodeMap
    /// - TLStreamNodeMap
    pub fn new(parent: impl Into<String>, device: acDevice, buffer: acBuffer, pixel_format: PixelFormat) -> Self {
        let name = Name::new(parent.into(), format!("AcBuffer"));
        Self {
            name,
            device,
            input: buffer,
            decompressed: std::ptr::null_mut(),
            pixel_format,
        }
    }
    ///
    /// Returns filled bytes
    fn len(&self, buffer: acBuffer) -> Result<usize, Error> {
        let error = Error::new(&self.name, "len");
        let mut bytes = 0;
        let err = AcErr::from(unsafe { super::bindings::acBufferGetSizeFilled(buffer, &mut bytes) });
        if err != AcErr::Success {
            return Err(error.pass(err.to_string()));
        };
        Ok(bytes)
    }
    ///
    /// Returns width of image buffer
    /// Images are self-describing, so the device does not need to be queried to get this information.
    fn width(&self, buffer: acBuffer) -> Result<usize, Error> {
        let error = Error::new(&self.name, "width");
        let mut width = 0;
        let err = AcErr::from(unsafe { super::bindings::acImageGetWidth(buffer, &mut width) });
        if err != AcErr::Success {
            return Err(error.pass(err.to_string()));
        };
        Ok(width)
    }
    ///
    /// Returns height of image buffer
    /// Images are self-describing, so the device does not need to be queried to get this information.
    fn height(&self, buffer: acBuffer) -> Result<usize, Error> {
        let error = Error::new(&self.name, "height");
        let mut height = 0;
        let err = AcErr::from(unsafe { super::bindings::acImageGetHeight(buffer, &mut height) });
        if err != AcErr::Success {
            return Err(error.pass(err.to_string()));
        };
        Ok(height)
    }
    ///
    /// Returns the timestamp of the image in nanoseconds.
    /// Images are self-describing, so the device does not need to be queried to get this information
    fn timestamp(&self, buffer: acBuffer) -> Result<usize, Error> {
        let error = Error::new(&self.name, "timestamp");
        let mut timestamp = 0;
        let err = AcErr::from(unsafe { super::bindings::acImageGetTimestampNs(buffer, &mut timestamp) });
        if err != AcErr::Success {
            return Err(error.pass(err.to_string()));
        };
        Ok(timestamp as usize)
    }
    ///
    /// Returns bits per pixel of the image.
    /// Images are self-describing, so the device does not need to be queried to get this information
    fn bpp(&self, buffer: acBuffer) -> Result<usize, Error> {
        let error = Error::new(&self.name, "bpp");
        let mut bpp = 0;
        let err = AcErr::from(unsafe { super::bindings::acImageGetBitsPerPixel(buffer, &mut bpp) });
        if err != AcErr::Success {
            return Err(error.pass(err.to_string()));
        }
        Ok(bpp)
    }
    ///
    /// Returns a pointer to the beginning of the image's payload data.
    /// The payload may include chunk data
    fn image_data(&self, buffer: acBuffer) -> Result<*mut u8, Error> {
        let error = Error::new(&self.name, "image_data");
        let mut result = std::ptr::null_mut();
        let err = AcErr::from(unsafe { super::bindings::acImageGetData(buffer, &mut result) });
        if err != AcErr::Success {
            return Err(error.pass(err.to_string()));
        };
        Ok(result)
    }
    ///
    /// Converts image format and color space from Arena SDK to OpenCv Mat
    fn convert(&self, len: usize, width: usize, height: usize, timestamp: usize, data: *mut std::ffi::c_void) -> Result<Image, Error>{
        let error = Error::new(&self.name, "convert");
        let src = unsafe { opencv::core::Mat::new_rows_cols_with_data_unsafe(
            height as i32,
            width as i32,
            self.pixel_format.cv_format(),
            data,
            opencv::core::Mat_AUTO_STEP,
        ) };
        match src {
            Ok(src) => match self.pixel_format {
                PixelFormat::BayerRG8 | PixelFormat::BayerBG8 | PixelFormat::BayerGB8 |
                PixelFormat::BayerRG10 | PixelFormat::BayerGR10 | PixelFormat::BayerBG10 | PixelFormat::BayerGB10 |
                PixelFormat::BayerRG12 | PixelFormat::BayerGR12 | PixelFormat::BayerBG12 | PixelFormat::BayerGB12 |
                PixelFormat::BayerRG16 | PixelFormat::BayerGR16 | PixelFormat::BayerBG16 | PixelFormat::BayerGB16 |
                PixelFormat::QoiBayerRG8 => {
                    let mut dst = src.clone();
                    match opencv::imgproc::cvt_color(
                        &src, 
                        &mut dst, 
                        opencv::imgproc::COLOR_BayerRG2RGB,
                        3,
                    ) {
                        Ok(_) => Ok(Image { width, height, timestamp: timestamp, mat: dst, bytes: len }),
                        Err(err) => Err(error.pass_with("OpenCv COLOR_BayerRG2RGB conversion Error", err.to_string())),
                    }
                }
                _ => Ok(Image { width, height, timestamp, mat: src, bytes: len })
            }
            Err(err) => Err(error.pass_with("Create OpenCv Mat Error", err.to_string())),
        }
    }
    ///
    /// Returns image (acBuffer).
    /// 
    /// If required decompresses a compressed image (acBuffer).
    /// In doing so, it creates a completely new image,
    /// similar to a deep copy but with an uncompressed pixel format.
    pub fn image(&mut self) -> Result<Image, Error> {
        let error = Error::new(&self.name, "image");
        let (buffer, len) = match self.pixel_format {
            PixelFormat::QoiBayerRG8 | PixelFormat::QoiMono8 |
            PixelFormat::QoiRGB8 | PixelFormat::QoiBGR8 |
            PixelFormat::QoiYCbCr8 => {
                let err = AcErr::from( unsafe { super::bindings::acImageFactoryDecompressImage(self.input, &mut self.decompressed) } );
                if err != AcErr::Success {
                    return Err(error.pass_with("FactoryDecompress error", err.to_string()));
                }
                log::debug!("{}.image | BitsPerPixel; {:?}", self.name, self.bpp(self.decompressed));
                let len = self.len(self.input).unwrap_or(0);
                Ok::<(acBuffer, usize), Error>((self.decompressed, len))
            }
            _ => {
                let len = self.len(self.input).unwrap_or(0);
                Ok::<(acBuffer, usize), Error>((self.input, len))
            }
        }?;
        let (width, height, timestamp, data) = (
            self.width(buffer)?,
            self.height(buffer)?,
            self.timestamp(buffer)?,
            self.image_data(buffer)?,
        );
        self.convert(len, width, height, timestamp, data as _)
    }
}
//
//
impl Drop for AcBuffer {
    fn drop(&mut self) {
        let err = AcErr::from(unsafe { super::bindings::acDeviceRequeueBuffer(self.device, self.input) });
        if err != AcErr::Success {
            log::warn!("{}.drop | Error; {}", self.name, err);
        };
        // !!! Images created with the image factory must be destroyed (acImageFactoryDestroy) when no longer needed;
        // !!! otherwise, memory will leak.
        if !self.decompressed.is_null() {
            let err = AcErr::from(unsafe { super::bindings::acImageFactoryDestroy(self.decompressed) });
            if err != AcErr::Success {
                log::warn!("{}.drop | Error; {}", self.name, err);
            };
        }
    }
}
