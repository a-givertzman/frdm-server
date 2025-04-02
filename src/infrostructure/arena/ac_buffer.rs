use opencv::imgproc::cvt_color;
use sal_core::error::Error;
use sal_sync::services::entity::name::Name;
use crate::infrostructure::arena::{
    ac_err::AcErr, ac_image::AcImage,
    bindings::{acBufferGetSizeFilled, acDeviceRequeueBuffer, acImageGetData, acImageGetHeight, acImageGetTimestampNs, acImageGetWidth},
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
    /// Retorns single image
    pub fn get_image(&self) -> Result<AcImage, Error> {
        let error = Error::new(&self.name, "get_image");
        let mut bytes = 0;
        let err = AcErr::from(unsafe { acBufferGetSizeFilled(self.buffer, &mut bytes) });
        if err != AcErr::Success {
            return Err(error.err(err));
        };
        log::trace!("{}.get_image | bytes: {} mb", self.name, (bytes as f64) / 1048576.0);
        // get and display width
        let mut width = 0;
        let err = AcErr::from(unsafe { acImageGetWidth(self.buffer, &mut width) });
        if err != AcErr::Success {
            return Err(error.err(err));
        };
        log::trace!("{}.get_image | width: {}; ", self.name, width);
        // get and display height
        let mut height = 0;
        let err = AcErr::from(unsafe { acImageGetHeight(self.buffer, &mut height) });
        if err != AcErr::Success {
            return Err(error.err(err));
        };
        log::trace!("{}.get_image | height: {}; ", self.name, height);
        // get and display timestamp
        let mut timestamp_ns = 0;
        let err = AcErr::from(unsafe { acImageGetTimestampNs(self.buffer, &mut timestamp_ns) });
        if err != AcErr::Success {
            return Err(error.err(err));
        };
        log::trace!("{}.get_image | timestamp (ns): {})", self.name, timestamp_ns);
        let mut buf = Vec::with_capacity(bytes);
        let mut p_input  = buf.as_mut_ptr();
        let err = AcErr::from(unsafe { acImageGetData(self.buffer, &mut p_input) });
        if err != AcErr::Success {
            return Err(error.err(err));
        };
        log::trace!("{}.get_image | {}x{}, {:.2} MB", self.name, width, height, (bytes as f64) / 1048576.0);
        let src = unsafe { opencv::core::Mat::new_rows_cols_with_data_unsafe(
            height as i32,
            width as i32,
            self.pixel_format.cv_format(),
            p_input as *mut std::ffi::c_void,
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
}
///
/// 
impl Drop for AcBuffer {
    fn drop(&mut self) {
        let err = AcErr::from(unsafe { acDeviceRequeueBuffer(self.device, self.buffer) });
        if err != AcErr::Success {
            log::warn!("{}.drop | Error; {}", self.name, err);
        };
    }
}
