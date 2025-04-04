//! ```ignore
//!      Baumer format                             |    OpenCV format
//! -----------------------------------------------|-----------------------------------
//!     Mono8                                      |    CV_8UC1, (8 bit, 1 channel)
//! -----------------------------------------------|-----------------------------------
//!     Mono10                                     |    Convert to Mono16 (bit-shift)
//! -----------------------------------------------|-----------------------------------
//!     Mono12	                                   |    Convert to Mono16 (bit-shift)
//! -----------------------------------------------|-----------------------------------
//!     Mono16                                     |	CV_16UC1, (16 bit, 1 channel)
//! -----------------------------------------------|-----------------------------------
//!     BGR8                                       |	CV_8UC3, (8 bit, 3 channels)
//! -----------------------------------------------|-----------------------------------
//!     BGR10                                      |	Convert to BGR16 (bit-shift)
//! -----------------------------------------------|-----------------------------------
//!     BGR12                                      |	Convert to BGR16 (bit-shift)
//! -----------------------------------------------|-----------------------------------
//!     BGR16                                      |	CV_16UC3, (16 bit, 3 channels (BGR) )
//! -----------------------------------------------|-----------------------------------
//!     RGB8                                       |	Convert to BGR8 using cv::cvtColor
//! -----------------------------------------------|-----------------------------------
//!     RGB10                                      |	Convert to BGR16 bit-shift and cv::cvtColor
//! -----------------------------------------------|-----------------------------------
//!     RGB12                                      |	Convert to BGR16 bit-shift and cv::cvtColor
//! -----------------------------------------------|-----------------------------------
//!     RGB16                                      |	Convert to BGR16 using cv::cvtColor
//! -----------------------------------------------|-----------------------------------
//!     BayerGB8, BayerRG8,                        |    Convert to BGR8 using cv::cvtColor
//!     BayerGR8, BayerBG8                         |
//! -----------------------------------------------|-----------------------------------
//!     BayerGB10, BayerRG10,                      |    Convert to BGR16 bit-shift and cv::cvtColor
//!     BayerGR10, BayerBG10
//! -----------------------------------------------|-----------------------------------
//!     BayerGB12, BayerRG12,                      |    Convert to BGR16 bit-shift and cv::cvtColor
//!     BayerGR12, BayerBG12                       |
//! -----------------------------------------------|-----------------------------------
//!     BayerGB16, BayerRG16,                      |    Convert to BGR16 bit-shift and cv::cvtColor
//!     BayerGR16, BayerBG16                       |
//! ```
//! 
//! See also:
//! 
//! [Color pixel formats](https://www.1stvision.com/cameras/IDS/IDS-manuals/en/basics-color-pixel-formats.html)
//! [RAW Bayer pixel formats](https://www.1stvision.com/cameras/IDS/IDS-manuals/en/basics-raw-bayer-pixel-formats.html)
//! 
//! Format conversions:
//! https://docs.opencv.org/3.4/de/d25/imgproc_color_conversions.html
//! https://stackoverflow.com/questions/7734469/converting-basler-image-to-opencv

use serde::Deserialize;

///
/// Pixel format
/// - Mono8/10/12/16,
/// - Bayer8/10/12/16,
/// - RGB8, BGR8,
/// - YCbCr8, YCbCr411, 
/// - YUV422, YUV411,
/// - Default and fastest BayerRG8
#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum PixelFormat {
    Mono8, Mono10, Mono12, Mono16,
    RGB8, BGR8,
    RGB10, BGR10,
    RGB12, BGR12,
    RGB16, BGR16,
    BayerRG8, BayerGR8, BayerBG8, BayerGB8,
    BayerRG10, BayerGR10, BayerBG10, BayerGB10,
    BayerRG12, BayerGR12, BayerBG12, BayerGB12,
    BayerRG16, BayerGR16, BayerBG16, BayerGB16,
    #[serde(alias="QOIBayerRG8", alias="QOI_BayerRG8")]
    QoiBayerRG8,
    #[serde(alias="QOIMono8", alias="QOI_Mono8")]
    QoiMono8,
    #[serde(alias="QOIRGB8", alias="QOI_RGB8")]
    QoiRGB8,
    #[serde(alias="QOIBGR8", alias="QOI_BGR8")]
    QoiBGR8,
    #[serde(alias="QOIYCbCr8", alias="QOI_YCbCr8")]
    QoiYCbCr8,
}
impl PixelFormat {
    ///
    /// Returns the string name of the format``
    pub fn format(&self) -> String {
        match self {
            Self::Mono8 => String::from("Mono8"),
            Self::Mono10 => String::from("Mono10"),
            Self::Mono12 => String::from("Mono12"),
            Self::Mono16 => String::from("Mono16"),
            
            Self::RGB8 => String::from("RGB8"),
            Self::BGR8 => String::from("BGR8"),
            
            Self::RGB10 => String::from("RGB10"),
            Self::BGR10 => String::from("BGR10"),
            
            Self::RGB12 => String::from("RGB12"),
            Self::BGR12 => String::from("BGR12"),

            Self::RGB16 => String::from("RGB16"),
            Self::BGR16 => String::from("BGR16"),

            Self::BayerRG8 => String::from("BayerRG8"),
            Self::BayerGR8 => String::from("BayerGR8"),
            Self::BayerBG8 => String::from("BayerBG8"),
            Self::BayerGB8 => String::from("BayerGB8"),

            Self::BayerRG10 => String::from("BayerRG10"),
            Self::BayerGR10 => String::from("BayerGR10"),
            Self::BayerBG10 => String::from("BayerBG10"),
            Self::BayerGB10 => String::from("BayerGB10"),

            Self::BayerRG12 => String::from("BayerRG12"),
            Self::BayerGR12 => String::from("BayerGR12"),
            Self::BayerBG12 => String::from("BayerBG12"),
            Self::BayerGB12 => String::from("BayerGB12"),

            Self::BayerRG16 => String::from("BayerRG16"),
            Self::BayerGR16 => String::from("BayerGR16"),
            Self::BayerBG16 => String::from("BayerBG16"),
            Self::BayerGB16 => String::from("BayerGB16"),

            Self::QoiBayerRG8 => String::from("QOI_BayerRG8"),
            Self::QoiMono8    => String::from("QOI_Mono8"),
            Self::QoiRGB8     => String::from("QOI_RGB8"),
            Self::QoiBGR8     => String::from("QOI_BGR8"),
            Self::QoiYCbCr8   => String::from("QOI_YCbCr8"),
        }
    }
    ///
    /// Returns the OpenCV color format
    pub fn cv_format(&self) -> i32 {
        match self {
            Self::Mono8 => opencv::core::CV_8UC1,
            Self::Mono10 => opencv::core::CV_16UC1,
            Self::Mono12 => opencv::core::CV_16UC1,
            Self::Mono16 => opencv::core::CV_16UC1,

            Self::RGB8 => opencv::core::CV_8UC3,
            Self::BGR8 => opencv::core::CV_8UC3,
            
            Self::RGB10 => opencv::core::CV_16UC3,
            Self::BGR10 => opencv::core::CV_16UC3,
            
            Self::RGB12 => opencv::core::CV_16UC3,
            Self::BGR12 => opencv::core::CV_16UC3,

            Self::RGB16 => opencv::core::CV_16UC3,
            Self::BGR16 => opencv::core::CV_16UC3,

            Self::BayerRG8 => opencv::core::CV_8UC1,
            Self::BayerGR8 => opencv::core::CV_8UC1,
            Self::BayerBG8 => opencv::core::CV_8UC1,
            Self::BayerGB8 => opencv::core::CV_8UC1,

            Self::BayerRG10 => opencv::core::CV_16UC1,
            Self::BayerGR10 => opencv::core::CV_16UC1,
            Self::BayerBG10 => opencv::core::CV_16UC1,
            Self::BayerGB10 => opencv::core::CV_16UC1,

            Self::BayerRG12 => opencv::core::CV_16UC1,
            Self::BayerGR12 => opencv::core::CV_16UC1,
            Self::BayerBG12 => opencv::core::CV_16UC1,
            Self::BayerGB12 => opencv::core::CV_16UC1,

            Self::BayerRG16 => opencv::core::CV_16UC1,
            Self::BayerGR16 => opencv::core::CV_16UC1,
            Self::BayerBG16 => opencv::core::CV_16UC1,
            Self::BayerGB16 => opencv::core::CV_16UC1,

            Self::QoiBayerRG8 => opencv::core::CV_8UC1,
            Self::QoiMono8    => opencv::core::CV_8UC1,
            Self::QoiRGB8     => opencv::core::CV_8UC3,
            Self::QoiBGR8     => opencv::core::CV_8UC3,
            Self::QoiYCbCr8   => opencv::core::CV_8UC3,
        }
    }
}
