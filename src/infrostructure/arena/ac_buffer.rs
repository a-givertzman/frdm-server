use opencv::imgproc::cvt_color;
use sal_sync::services::entity::{error::str_err::StrErr, name::Name};

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
    /// 
    pub fn get_image(&self) -> Result<AcImage, StrErr> {
        // get and display size filled
        let mut bytes = 0;
        let err = AcErr::from(unsafe { acBufferGetSizeFilled(self.buffer, &mut bytes) });
        if err != AcErr::Success {
            return Err(StrErr(format!("{}.get_image | Error: {}", self.name, err)));
        };
        log::trace!("{}.get_image | bytes: {}; ", self.name, bytes);
        // get and display width
        let mut width = 0;
        let err = AcErr::from(unsafe { acImageGetWidth(self.buffer, &mut width) });
        if err != AcErr::Success {
            return Err(StrErr(format!("{}.get_image | Error: {}", self.name, err)));
        };
        log::trace!("{}.get_image | width: {}; ", self.name, width);
        // get and display height
        let mut height = 0;
        let err = AcErr::from(unsafe { acImageGetHeight(self.buffer, &mut height) });
        if err != AcErr::Success {
            return Err(StrErr(format!("{}.get_image | Error: {}", self.name, err)));
        };
        log::trace!("{}.get_image | height: {}; ", self.name, height);
        // get and display timestamp
        let mut timestamp_ns = 0;
        let err = AcErr::from(unsafe { acImageGetTimestampNs(self.buffer, &mut timestamp_ns) });
        if err != AcErr::Success {
            return Err(StrErr(format!("{}.get_image | Error: {}", self.name, err)));
        };
        log::trace!("{}.get_image | timestamp (ns): {})", self.name, timestamp_ns);
        let mut buf = Vec::with_capacity(bytes);
        let mut p_input  = buf.as_mut_ptr();
        let err = AcErr::from(unsafe { acImageGetData(self.buffer, &mut p_input) });
        if err != AcErr::Success {
            return Err(StrErr(format!("{}.get_image | Error: {}", self.name, err)));
        };
        let src = unsafe { opencv::core::Mat::new_rows_cols_with_data_unsafe(
            height as i32,
            width as i32,
            self.pixel_format.cv_format(),
            p_input as *mut std::ffi::c_void,
            opencv::core::Mat_AUTO_STEP,
        ) };
        match src {
            Ok(src) => {
                let mut dst = src.clone();
                // let mut dst = opencv::core::Mat::default();
                // let mut dst = unsafe { opencv::core::Mat::new_rows_cols(height as i32, width as i32, opencv::core::CV_8UC3).unwrap() };
                match cvt_color(
                    &src, 
                    &mut dst, 
                    opencv::imgproc::COLOR_BayerRG2RGB,
                    3,
                ) {
                    Ok(_) => Ok(AcImage {
                        width,
                        height,
                        timestamp: timestamp_ns as usize,
                        mat: dst,
                    }),
                    Err(err) => Err(StrErr(format!("{}.get_image | Convert Error: {}", self.name, err))),
                }
            }
            Err(err) => Err(StrErr(format!("{}.get_image | Create src Error: {}", self.name, err))),
        }

        // match opencv::core::Mat::new_size_with_data(
        //     opencv::core::Size_ { width: width as i32, height: height as i32 },
        //      dst,
        // ) {
        //     Ok(mat) => Ok(AcImage {
        //         width,
        //         height,
        //         timestamp: timestamp_ns as usize,
        //         mat: mat.clone_pointee(),
        //     }),
        //     Err(err) => Err(StrErr(format!("{}.get_image | Error: {}", self.name, err))),
        // };

        // match unsafe { opencv::core::Mat::new_rows_cols_with_data_unsafe(
        //     height as i32,
        //     width as i32,
        //      self.pixel_format.cv_format(),
        //     p_input as *mut std::ffi::c_void,
        //     opencv::core::Mat_AUTO_STEP,
        // ) } {
        //     Ok(mat) => Ok(AcImage {
        //         width,
        //         height,
        //         timestamp: timestamp_ns as usize,
        //         mat,
        //     }),
        //     Err(err) => Err(StrErr(format!("{}.get_image | Error: {}", self.name, err))),
        // }
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
