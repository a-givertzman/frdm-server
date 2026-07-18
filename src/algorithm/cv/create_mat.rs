use opencv::core::{Mat, MatTraitManual};
use sal_core::error::Error;
use crate::{algorithm::cv::MatType, Eval};
///
/// Creates `OpenCv` new [Mat]
pub struct CreateMat {
    width: i32,
    height: i32,
    typ: MatType,
}
//
//
impl CreateMat {
    ///
    /// Returns [CreateMat] new instance
    /// - `width` - Number of columns in a 2D array.
    /// - `height` - Number of rows in a 2D array.
    /// - `typ` - Array type. Use CV_8UC1, ..., CV_64FC4 to create 1-4 channel matrices, or CV_8UC(n), ..., CV_64FC(n) to create multi-channel (up to CV_CN_MAX channels) matrices.
    #[allow(unused)]
    pub fn new(width: i32, height: i32, typ: MatType) -> Self {
        Self { width, height, typ }
    }
    ///
    /// Returns [CreateMat] new instance with type `CV_8UC1`
    /// - `width` - Number of columns in a 2D array.
    /// - `height` - Number of rows in a 2D array.
    #[allow(unused)]
    pub fn gray8(width: i32, height: i32) -> Self {
        Self { width, height, typ: MatType::Cv8uc1 }
    }
    ///
    /// Returns [CreateMat] new instance with type `CV_8UC3`
    /// - `width` - Number of columns in a 2D array.
    /// - `height` - Number of rows in a 2D array.
    #[allow(unused)]
    pub fn color8(width: i32, height: i32) -> Self {
        Self { width, height, typ: MatType::Cv8uc3 }
    }
    ///
    /// Returns [CreateMat] new instance with type `CV_8UC4`
    /// - `width` - Number of columns in a 2D array.
    /// - `height` - Number of rows in a 2D array.
    #[allow(unused)]
    pub fn color8alpha(width: i32, height: i32, typ: MatType) -> Self {
        Self { width, height, typ: MatType::Cv8uc4 }
    }
}
//
// Owned vec
impl Eval<(), Result<Mat, Error>> for CreateMat {
    fn eval(&self, _: ()) -> Result<Mat, Error> {
        unsafe { opencv::core::Mat::new_rows_cols(
            self.height,
            self.width,
            self.typ as i32,
        )}
        .map_err(|err| Error::new("CreateMat", "eval()").pass_with(
            format!("Can't create Mat with width {}, height {}, type {:?}", self.width, self.height, self.typ),
            err.to_string(),
        ))
    }
}
//
// Owned vec
impl Eval<&[u8], Result<Mat, Error>> for CreateMat {
    fn eval(&self, val: &[u8]) -> Result<Mat, Error> {
        let error = Error::new("CreateMat", "eval(val: &[u8])");
        let mut mat = self.eval(()).map_err(|err| error.clone().pass(err.to_string()))?;
        if val.is_empty() {
            return Ok(mat);
        }
        // Получаем сырой слайс памяти из OpenCV и копируем туда байты
        let data_bytes = mat.data_bytes_mut().map_err(|err| error.clone().pass_with(
            "Failed to get mutable byte slice from Mat".to_string(),
            err.to_string()
        ))?;
        // Обязательная проверка (Safety check), чтобы не словить панику при copy_from_slice
        if data_bytes.len() != val.len() {
            return Err(error.pass_with(
                format!("Data size mismatch. Expected {} bytes, got {}", data_bytes.len(), val.len()),
                "".to_string()
            ));
        }
        data_bytes.copy_from_slice(val);
        Ok(mat)
    }
}
//
// Ref to vec
impl Eval<Vec<u8>, Result<Mat, Error>> for CreateMat {
    fn eval(&self, val: Vec<u8>) -> Result<Mat, Error> {
        self.eval(val.as_slice())
        .map_err(|err| Error::new("CreateMat", "eval(val: Vec<u8>)").pass(err.to_string()))
    }
}
