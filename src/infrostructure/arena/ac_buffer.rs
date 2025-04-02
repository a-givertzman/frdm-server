use opencv::imgproc::cvt_color;
use sal_core::error::Error;
use sal_sync::services::entity::name::Name;
use crate::infrostructure::arena::{
    ac_err::AcErr, ac_image::AcImage,
};

use super::{bindings::{acBuffer, acDevice}, pixel_format::PixelFormat};

pub struct AcBuffer {
    name: Name,
    // device: acDevice,
    buffer: ImgData,
}
impl AcBuffer {
    ///
    /// Returns new instance of the Device Node Map of kind:
    /// - DeviceNodeMap
    /// - TLStreamNodeMap
    pub fn new(parent: impl Into<String>, device: acDevice, buffer: acBuffer, pixel_format: PixelFormat) -> Self {
        let name = Name::new(parent.into(), format!("AcBuffer"));
        let buffer = ImgData::new(&name, device, buffer, pixel_format);
        Self {
            name,
            // device,
            buffer,
        }
    }
    ///
    /// Returns filled bytes
    pub fn len(&self) -> Result<usize, Error> {
        self.buffer.len()
    }
    // ///
    // /// Returns decompressed buffer (QOI compression)
    // /// 
    // /// Decompresses a compressed image (acBuffer).
    // /// In doing so, it creates a completely new image,
    // /// similar to a deep copy but with an uncompressed pixel format.
    // /// Images created with the image factory must be destroyed (acImageFactoryDestroy)
    // /// when no longer needed; otherwise, memory will leak.
    // pub fn decompress(&self, buffer: acBuffer) -> Result<acBuffer, Error> {
    //     let error = Error::new(&self.name, "decompress");
    //     let mut result: acBuffer = std::ptr::null_mut();
    //     let err = AcErr::from( unsafe { super::bindings::acImageFactoryDecompressImage(buffer, &mut result) } );
    //     if err != AcErr::Success {
    //         return Err(error.err(err));
    //     };
    //     Ok(result)
    // }
    ///
    /// Retorns single image
    pub fn image(&mut self) -> Result<AcImage, Error> {
        let error = Error::new(&self.name, "get_image");
        let bytes = self.buffer.len()?;
        log::trace!("{}.get_image | bytes: {} mb", self.name, (bytes as f64) / 1048576.0);
        let width = self.buffer.width()?;
        log::trace!("{}.get_image | width: {}; ", self.name, width);
        let height = self.buffer.height()?;
        log::trace!("{}.get_image | height: {}; ", self.name, height);
        let timestamp_ns = self.buffer.timestamp()?;
        log::trace!("{}.get_image | timestamp: {} ns)", self.name, timestamp_ns);
        log::trace!("{}.get_image | {}x{}, {:.2} MB", self.name, width, height, (bytes as f64) / 1048576.0);
        match self.buffer.get() {
            Ok(img) => {
                let src = unsafe { opencv::core::Mat::new_rows_cols_with_data_unsafe(
                    height as i32,
                    width as i32,
                    self.buffer.pixel_format.cv_format(),
                    img as *mut std::ffi::c_void,
                    opencv::core::Mat_AUTO_STEP,
                ) };
                match src {
                    Ok(src) => match self.buffer.pixel_format {
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
                    Err(err) => Err(error.pass_with("Create Error", err.to_string())),
                }
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}
//
// 
impl Drop for AcBuffer {
    fn drop(&mut self) {
    }
}




pub struct ImgData {
    name: Name,
    device: acDevice,
    input: acBuffer,
    out: Option<*mut u8>,
    decompressed: Option<acBuffer>,
    pub pixel_format: PixelFormat,
}
//
//
impl ImgData {
    ///
    /// Returns new instance of the Device Node Map of kind:
    /// - DeviceNodeMap
    /// - TLStreamNodeMap
    pub fn new(parent: impl Into<String>, device: acDevice, buffer: acBuffer, pixel_format: PixelFormat) -> Self {
        let name = Name::new(parent.into(), format!("ImgData"));
        Self {
            name,
            device,
            input: buffer,
            out: None,
            decompressed: None,
            pixel_format,
        }
    }
    ///
    /// Returns filled bytes
    pub fn len(&self) -> Result<usize, Error> {
        let error = Error::new(&self.name, "len");
        let mut bytes = 0;
        let err = AcErr::from(unsafe { super::bindings::acBufferGetSizeFilled(self.input, &mut bytes) });
        if err != AcErr::Success {
            return Err(error.err(err));
        };
        Ok(bytes)
    }
    ///
    /// Returns width of image buffer
    /// Images are self-describing, so the device does not need to be queried to get this information.
    pub fn width(&self) -> Result<usize, Error> {
        let error = Error::new(&self.name, "width");
        let mut width = 0;
        let err = AcErr::from(unsafe { super::bindings::acImageGetWidth(self.input, &mut width) });
        if err != AcErr::Success {
            return Err(error.err(err));
        };
        Ok(width)
    }
    ///
    /// Returns height of image buffer
    /// Images are self-describing, so the device does not need to be queried to get this information.
    pub fn height(&self) -> Result<usize, Error> {
        let error = Error::new(&self.name, "height");
        let mut height = 0;
        let err = AcErr::from(unsafe { super::bindings::acImageGetHeight(self.input, &mut height) });
        if err != AcErr::Success {
            return Err(error.err(err));
        };
        Ok(height)
    }
    ///
    /// Returns the timestamp of the image in nanoseconds.
    /// Images are self-describing, so the device does not need to be queried to get this information
    pub fn timestamp(&self) -> Result<usize, Error> {
        let error = Error::new(&self.name, "timestamp");
        let mut timestamp = 0;
        let err = AcErr::from(unsafe { super::bindings::acImageGetTimestampNs(self.input, &mut timestamp) });
        if err != AcErr::Success {
            return Err(error.err(err));
        };
        Ok(timestamp as usize)
    }
    ///
    /// Returns a pointer to the beginning of the image's payload data.
    /// The payload may include chunk data.
    pub fn get(&mut self) -> Result<*mut u8, Error> {
        let error = Error::new(&self.name, "get");
        match self.out {
            Some(out) => Ok(out),
            None => {
                match self.pixel_format {
                    PixelFormat::QoiBayerRG8 | PixelFormat::QoiMono8 |
                    PixelFormat::QoiRGB8 | PixelFormat::QoiBGR8 |
                    PixelFormat::QoiYCbCr8 => {
                        let mut decompressed = std::ptr::null_mut();
                        self.decompressed = Some(decompressed);
                        let err = AcErr::from( unsafe { super::bindings:: acImageFactoryDecompressImage(self.input, &mut decompressed) } );
                        if err != AcErr::Success {
                            return Err(error.err(err));
                        }
                        let mut result = std::ptr::null_mut();
                        let err = AcErr::from(unsafe { super::bindings::acImageGetData(decompressed, &mut result) });
                        if err != AcErr::Success {
                            return Err(error.err(err));
                        };
                        self.out = Some(result);
                        Ok(result)
                    }
                    _ => {
                        let mut result = std::ptr::null_mut();
                        let err = AcErr::from(unsafe { super::bindings::acImageGetData(self.input, &mut result) });
                        if err != AcErr::Success {
                            return Err(error.err(err));
                        };
                        self.out = Some(result);
                        Ok(result)
                    }
                }
            }
        }
    }
}
//
//
impl Drop for ImgData {
    fn drop(&mut self) {
        let err = AcErr::from(unsafe { super::bindings::acDeviceRequeueBuffer(self.device, self.input) });
        if err != AcErr::Success {
            log::warn!("{}.drop | Error; {}", self.name, err);
        };
        if let Some(decompressed) = self.decompressed {
            let err = AcErr::from(unsafe { super::bindings::acImageFactoryDestroy(decompressed) });
            if err != AcErr::Success {
                log::warn!("{}.drop | Error; {}", self.name, err);
            };
        }
    }
}
