use opencv::core::MatTraitConst;
use sal_core::error::Error;

///
/// Contains a image with metadata
#[derive(Debug, Clone)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub timestamp: usize,
    pub mat: opencv::core::Mat,
    pub bytes: usize,
}
//
//
impl Image {
    ///
    /// Returns [Image] new instance
    /// - `width` - Origin size of image
    /// - `height` - Origin size of image
    /// - `mat` - The matrix of image
    /// - `timestamp` - Timstemp of image
    /// - `bytes` - Length of image payload in bytes
    pub fn new(
        width: usize,
        height: usize,
        mat: opencv::core::Mat,
        timestamp: usize,
    ) -> Self {
        Self {
            width,
            height,
            timestamp,
            bytes: mat.elem_size1(),
            mat,
        }
    }
    ///
    /// Used for testing only !!!
    /// To simply create [Image] and compare it by matrix
    /// 
    /// Use `Image::new` instead
    pub fn with(mat: opencv::core::Mat) -> Self {
        Self {
            width: 0,
            height: 0,
            timestamp: 0,
            mat,
            bytes: 0,
        }
    }
    ///
    /// ## Saves an image to a specified file.
    /// 
    /// The image format is chosen based on the filename extension (see cv::imread for the list of extensions).
    /// 
    /// In general, only 8-bit single-channel or 3-channel (with 'BGR' channel order) images can be saved using this function, with these exceptions:
    /// 
    /// - With OpenEXR encoder, only 32-bit float (CV_32F) images can be saved.
    ///     - 8-bit unsigned (CV_8U) images are not supported.
    /// - With Radiance HDR encoder, non 64-bit float (CV_64F) images can be saved.
    ///     - All images will be converted to 32-bit float (CV_32F).
    /// - With JPEG 2000 encoder, 8-bit unsigned (CV_8U) and 16-bit unsigned (CV_16U) images can be saved.
    /// - With JPEG XL encoder, 8-bit unsigned (CV_8U), 16-bit unsigned (CV_16U) and 32-bit float(CV_32F) images can be saved.
    ///     - JPEG XL images with an alpha channel can be saved using this function. To do this, create 8-bit (or 16-bit, 32-bit float) 4-channel image BGRA, where the alpha channel goes last. Fully transparent pixels should have alpha set to 0, fully opaque pixels should have alpha set to 255/65535/1.0.
    /// - With PAM encoder, 8-bit unsigned (CV_8U) and 16-bit unsigned (CV_16U) images can be saved.
    /// - With PNG encoder, 8-bit unsigned (CV_8U) and 16-bit unsigned (CV_16U) images can be saved.
    ///     - PNG images with an alpha channel can be saved using this function. To do this, create 8-bit (or 16-bit) 4-channel image BGRA, where the alpha channel goes last. Fully transparent pixels should have alpha set to 0, fully opaque pixels should have alpha set to 255/65535 (see the code sample below).
    /// - With PGM/PPM encoder, 8-bit unsigned (CV_8U) and 16-bit unsigned (CV_16U) images can be saved.
    /// - With TIFF encoder, 8-bit unsigned (CV_8U), 8-bit signed (CV_8S), 16-bit unsigned (CV_16U), 16-bit signed (CV_16S), 32-bit signed (CV_32S), 32-bit float (CV_32F) and 64-bit float (CV_64F) images can be saved.
    ///     - Multiple images (vector of Mat) can be saved in TIFF format (see the code sample below).
    ///     - 32-bit float 3-channel (CV_32FC3) TIFF images will be saved using the LogLuv high dynamic range encoding (4 bytes per pixel)
    /// 
    /// If the image format is not supported, the image will be converted to 8-bit unsigned (CV_8U) and saved that way.
    /// 
    /// If the format, depth or channel order is different, use Mat::convertTo and cv::cvtColor to convert it before saving. Or, use the universal FileStorage I/O functions to save the image to XML or YAML format.
    /// 
    /// The sample below shows how to create a BGRA image, how to set custom compression parameters and save it to a PNG file. It also demonstrates how to save multiple images in a TIFF file: @include snippets/imgcodecs_imwrite.cpp
    /// 
    /// ### Parameters
    /// 
    /// filename: Name of the file.
    /// img: (Mat or vector of Mat) Image or Images to be saved.
    /// params: Format-specific parameters encoded as pairs (paramId_1, paramValue_1, paramId_2, paramValue_2, ... .) see cv::ImwriteFlags
    pub fn save(&self, path: impl Into<String>) -> Result<(), Error> {
        let error = Error::new("Image", "save");
        let path = path.into();
        let params = opencv::core::Vector::new();
        opencv::imgcodecs::imwrite(&path, &self.mat, &params)
            .map(|_| ())
            .map_err(|err| error.pass_with(format!("Errorsaving image into '{path}'"), err.to_string()))
    }
    /// ## Loads an image from a file.
    /// 
    /// @anchor imread
    /// 
    /// The imread function loads an image from the specified file and returns OpenCV matrix. If the image cannot be read (because of a missing file, improper permissions, or unsupported/invalid format), the function returns an empty matrix.
    /// 
    /// Currently, the following file formats are supported:
    /// 
    /// - Windows bitmaps - *.bmp, *.dib (always supported)
    /// - GIF files - *.gif (always supported)
    /// - JPEG files - *.jpeg, *.jpg, *.jpe (see the Note section)
    /// - JPEG 2000 files - *.jp2 (see the Note section)
    /// - Portable Network Graphics - *.png (see the Note section)
    /// - WebP - *.webp (see the Note section)
    /// - AVIF - *.avif (see the Note section)
    /// - Portable image format - *.pbm, *.pgm, *.ppm, *.pxm, *.pnm (always supported)
    /// - PFM files - *.pfm (see the Note section)
    /// - Sun rasters - *.sr, *.ras (always supported)
    /// - TIFF files - *.tiff, *.tif (see the Note section)
    /// - OpenEXR Image files - *.exr (see the Note section)
    /// - Radiance HDR - *.hdr, *.pic (always supported)
    /// - Raster and Vector geospatial data supported by GDAL (see the Note section)
    /// 
    /// ### Note:
    /// 
    /// - The function determines the type of an image by its content, not by the file extension.
    /// - In the case of color images, the decoded images will have the channels stored in B G R order.
    /// - When using IMREAD_GRAYSCALE, the codec’s internal grayscale conversion will be used, if available. Results may differ from the output of cvtColor().
    /// - On Microsoft Windows* and Mac OS*, the codecs shipped with OpenCV (libjpeg, libpng, libtiff, and libjasper) are used by default. So, OpenCV can always read JPEGs, PNGs, and TIFFs. On Mac OS, there is also an option to use native Mac OS image readers. However, beware that currently these native image loaders give images with different pixel values because of the color management embedded into Mac OS.
    /// - On Linux*, BSD flavors, and other Unix-like open-source operating systems, OpenCV looks for codecs supplied with the OS. Ensure the relevant packages are installed (including development files, such as “libjpeg-dev” in Debian* and Ubuntu*) to get codec support, or turn on the OPENCV_BUILD_3RDPARTY_LIBS flag in CMake.
    /// - If the WITH_GDAL flag is set to true in CMake and IMREAD_LOAD_GDAL is used to load the image, the GDAL driver will be used to decode the image, supporting Raster and Vector formats.
    /// - If EXIF information is embedded in the image file, the EXIF orientation will be taken into account, and thus the image will be rotated accordingly unless the flags IMREAD_IGNORE_ORIENTATION or IMREAD_UNCHANGED are passed.
    /// - Use the IMREAD_UNCHANGED flag to preserve the floating-point values from PFM images.
    /// - By default, the number of pixels must be less than 2^30. This limit can be changed by setting the environment variable OPENCV_IO_MAX_IMAGE_PIXELS. See [tutorial_env_reference].
    /// 
    /// ### Parameters:
    /// 
    /// - filename: Name of the file to be loaded.
    /// - flags: Flag that can take values of cv::ImreadModes.
    /// 
    /// ### C++ default parameters
    /// 
    /// - flags: IMREAD_COLOR_BGR
    /// 
    /// 
    pub fn load(path: impl Into<String>) -> Result<Self, Error> {
        let error = Error::new("Image", "load");
        let path = path.into();
        opencv::imgcodecs::imread(&path, opencv::imgcodecs::IMREAD_UNCHANGED)
            .map(|mat| Image::with(mat))
            .map_err(|err| error.pass_with(format!("Errorsaving image into '{path}'"), err.to_string()))
    }
}
//
//
impl Default for Image {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            timestamp: 0,
            mat: opencv::core::Mat::default(),
            bytes: 0,
        }
    }
}
//
// TODO: Better way to compare matrixes:
// bool eq = std::equal(a.begin<uchar>(), a.end<uchar>(), b.begin<uchar>());
impl PartialEq for Image {
    fn eq(&self, other: &Self) -> bool {
        let mut dst = self.mat.clone();
        opencv::core::compare(&self.mat, &other.mat, &mut dst, opencv::core::CmpTypes::CMP_EQ as i32).is_ok()
    }
}
