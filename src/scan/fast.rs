///
///
pub struct FastScan{
    gray_frame: Mat,
    gradient_x: Mat,
    gradient_y: Mat,
    abs_x: Mat,
    abs_y: Mat,
    gradient: Mat
}
//
//
impl FastScan{
    pub fn new(&self, frame: PImage) -> Self{
        imgproc::cvt_color(&frame, &mut gray_frame, imgproc::COLOR_BGR2GRAY, 0)?;
        imgproc::sobel(&gray_frame,&mut gradient_x, core::CV_16S, 1, 0, 1, 1.0, 0.0, core::BORDER_DEFAULT)?;
        imgproc::sobel(&gray_frame,&mut gradient_y, core::CV_16S, 0, 1, 1, 1.0, 0.0, core::BORDER_DEFAULT)?;
        core::convert_scale_abs(&gradient_x, &mut abs_x, 1.0, 0.0)?;
        core::convert_scale_abs(&gradient_y, &mut abs_y, 1.0, 0.0)?;
        core::add_weighted(&abs_x, 0.5, &abs_y, 0.5, 0.0, &mut gradient, -1)?;
    }
}