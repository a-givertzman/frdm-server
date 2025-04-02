use opencv::imgproc::cvt_color;
use sal_core::error::Error;
use sal_sync::services::entity::name::Name;
use crate::infrostructure::arena::{
    ac_err::AcErr, ac_image::AcImage,
};

use super::{bindings::{acBuffer, acDevice}, pixel_format::PixelFormat};

pub struct AcBuffer {
    name: Name,
    device: acDevice,
    buffer: acBuffer,
    pixel_format: PixelFormat,
}
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
            buffer,
            pixel_format,
        }
    }
    ///
    /// Returns filled bytes
    pub fn len(&self) -> Result<usize, Error> {
        let error = Error::new(&self.name, "len");
        let mut bytes = 0;
        let err = AcErr::from(unsafe { super::bindings::acBufferGetSizeFilled(self.buffer, &mut bytes) });
        if err != AcErr::Success {
            return Err(error.err(err));
        };
        Ok(bytes)
    }
    ///
    /// Returns decompressed buffer (QOI compression)
    /// 
    /// Decompresses a compressed image (acBuffer).
    /// In doing so, it creates a completely new image,
    /// similar to a deep copy but with an uncompressed pixel format.
    /// Images created with the image factory must be destroyed (acImageFactoryDestroy)
    /// when no longer needed; otherwise, memory will leak.
    pub fn decompress(&self, buffer: acBuffer) -> Result<acBuffer, Error> {
        let error = Error::new(&self.name, "decompress");
        let mut result: acBuffer = std::ptr::null_mut();
        let err = AcErr::from( unsafe { super::bindings::acImageFactoryDecompressImage(buffer, &mut result) } );
        if err != AcErr::Success {
            return Err(error.err(err));
        };
        Ok(result)
    }
    ///
    /// Returns width of image buffer
    /// Images are self-describing, so the device does not need to be queried to get this information.
    pub fn width(&self, buffer: acBuffer) -> Result<usize, Error> {
        let error = Error::new(&self.name, "width");
        let mut width = 0;
        let err = AcErr::from(unsafe { super::bindings::acImageGetWidth(buffer, &mut width) });
        if err != AcErr::Success {
            return Err(error.err(err));
        };
        Ok(width)
    }
    ///
    /// Returns height of image buffer
    /// Images are self-describing, so the device does not need to be queried to get this information.
    pub fn height(&self, buffer: acBuffer) -> Result<usize, Error> {
        let error = Error::new(&self.name, "height");
        let mut height = 0;
        let err = AcErr::from(unsafe { super::bindings::acImageGetHeight(buffer, &mut height) });
        if err != AcErr::Success {
            return Err(error.err(err));
        };
        Ok(height)
    }
    ///
    /// Returns the timestamp of the image in nanoseconds.
    /// Images are self-describing, so the device does not need to be queried to get this information
    pub fn timestamp(&self, buffer: acBuffer) -> Result<usize, Error> {
        let error = Error::new(&self.name, "timestamp");
        let mut timestamp = 0;
        let err = AcErr::from(unsafe { super::bindings::acImageGetTimestampNs(buffer, &mut timestamp) });
        if err != AcErr::Success {
            return Err(error.err(err));
        };
        Ok(timestamp as usize)
    }
    ///
    /// Returns a pointer to the beginning of the image's payload data.
    /// The payload may include chunk data.
    fn image_data(&self, buffer: acBuffer, len: usize) -> Result<Vec<u8>, Error> {
        let error = Error::new(&self.name, "image_data");
        fn get_data(self_name: &str, buffer: acBuffer, len: usize) -> Result<Vec<u8>, Error> {
            let error = Error::new(self_name, "get_data");
            let mut buf = Vec::with_capacity(len);
            let mut p_input  = buf.as_mut_ptr();
            let err = AcErr::from(unsafe { super::bindings::acImageGetData(buffer, &mut p_input) });
            if err != AcErr::Success {
                return Err(error.err(err));
            };
            Ok(buf)
        }
        match self.pixel_format {
            PixelFormat::QoiBayerRG8 | PixelFormat::QoiMono8 |
            PixelFormat::QoiRGB8 | PixelFormat::QoiBGR8 |
            PixelFormat::QoiYCbCr8 => {
                let mut decompressed: acBuffer = std::ptr::null_mut();
                let err = AcErr::from( unsafe { super::bindings:: acImageFactoryDecompressImage(buffer, &mut decompressed) } );
                if err != AcErr::Success {
                    return Err(error.err(err));
                }
                let result = match get_data(&self.name.join(), decompressed, len) {
                    Ok(data) => Ok(data),
                    Err(err) => Err(error.pass(err)),
                };
                let err = AcErr::from(unsafe { super::bindings::acImageFactoryDestroy(decompressed) });
                if err != AcErr::Success {
                    return Err(error.err(err));
                };
                result
            }
            _ => {
                match get_data(&self.name.join(), buffer, len) {
                    Ok(data) => Ok(data),
                    Err(err) => Err(error.pass(err)),
                }
            }
        }
    }
    ///
    /// Retorns single image
    pub fn image(&self) -> Result<AcImage, Error> {
        let error = Error::new(&self.name, "get_image");
        let bytes = self.len()?;
        log::trace!("{}.get_image | bytes: {} mb", self.name, (bytes as f64) / 1048576.0);
        let buffer: acBuffer = match self.pixel_format {
            PixelFormat::QoiBayerRG8 | PixelFormat::QoiMono8 |
            PixelFormat::QoiRGB8 | PixelFormat::QoiBGR8 |
            PixelFormat::QoiYCbCr8 => {
                self.decompress(self.buffer)?
            }
            _ => {
                self.buffer
            }
        };
        let width = self.width(buffer)?;
        log::trace!("{}.get_image | width: {}; ", self.name, width);
        let height = self.height(buffer)?;
        log::trace!("{}.get_image | height: {}; ", self.name, height);
        let timestamp_ns = self.timestamp(buffer)?;
        log::trace!("{}.get_image | timestamp: {} ns)", self.name, timestamp_ns);
        log::trace!("{}.get_image | {}x{}, {:.2} MB", self.name, width, height, (bytes as f64) / 1048576.0);
        match self.image_data(buffer, bytes) {
            Ok(mut img) => {
                let src = unsafe { opencv::core::Mat::new_rows_cols_with_data_unsafe(
                    height as i32,
                    width as i32,
                    self.pixel_format.cv_format(),
                    img.as_mut_ptr() as *mut std::ffi::c_void,
                    opencv::core::Mat_AUTO_STEP,
                ) };
                match src {
                    Ok(src) => {
                        match self.pixel_format {
                            PixelFormat::BayerRG8 | PixelFormat::BayerBG8 | PixelFormat::BayerGB8 |
                            PixelFormat::BayerRG10 | PixelFormat::BayerGR10 | PixelFormat::BayerBG10 | PixelFormat::BayerGB10 |
                            PixelFormat::BayerRG12 | PixelFormat::BayerGR12 | PixelFormat::BayerBG12 | PixelFormat::BayerGB12 |
                            PixelFormat::BayerRG16 | PixelFormat::BayerGR16 | PixelFormat::BayerBG16 | PixelFormat::BayerGB16 => {
                                let mut dst = src.clone();
                                match cvt_color(
                                    &src, 
                                    &mut dst, 
                                    opencv::imgproc::COLOR_BayerRG2RGB,
                                    3,
                                ) {
                                    Ok(_) => Ok(AcImage { width, height, timestamp: timestamp_ns as usize, mat: dst, bytes }),
                                    Err(err) => Err(error.pass_with("Convert Error", err.to_string())),
                                }
                            }
                            _ => Ok(AcImage { width, height, timestamp: timestamp_ns as usize, mat: src, bytes })
                        }
                    }
                    Err(err) => Err(error.pass_with("Create Error", err.to_string())),
                }
            }
            Err(_) => todo!(),
        }
    }
}
///
/// 
impl Drop for AcBuffer {
    fn drop(&mut self) {
        let err = AcErr::from(unsafe { super::bindings::acDeviceRequeueBuffer(self.device, self.buffer) });
        if err != AcErr::Success {
            log::warn!("{}.drop | Error; {}", self.name, err);
        };
    }
}
