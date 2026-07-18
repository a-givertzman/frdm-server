#[derive(Debug ,Clone, Copy, PartialEq, Eq)]
///
/// The code of type of `Mat`
#[allow(unused)]
pub enum MatType {
    Cv8uc1 = opencv::core::CV_8UC1 as isize,
    Cv8uc2 = opencv::core::CV_8UC2 as isize,
    Cv8uc3 = opencv::core::CV_8UC3 as isize,
    Cv8uc4 = opencv::core::CV_8UC4 as isize,
    Cv8sc1 = opencv::core::CV_8SC1 as isize,
    Cv8sc2 = opencv::core::CV_8SC2 as isize,
    Cv8sc3 = opencv::core::CV_8SC3 as isize,
    Cv8sc4 = opencv::core::CV_8SC4 as isize,
    Cv16uc1 = opencv::core::CV_16UC1 as isize,
    Cv16uc2 = opencv::core::CV_16UC2 as isize,
    Cv16uc3 = opencv::core::CV_16UC3 as isize,
    Cv16uc4 = opencv::core::CV_16UC4 as isize,
    Cv16sc1 = opencv::core::CV_16SC1 as isize,
    Cv16sc2 = opencv::core::CV_16SC2 as isize,
    Cv16sc3 = opencv::core::CV_16SC3 as isize,
    Cv16sc4 = opencv::core::CV_16SC4 as isize,
    Cv32sc1 = opencv::core::CV_32SC1 as isize,
    Cv32sc2 = opencv::core::CV_32SC2 as isize,
    Cv32sc3 = opencv::core::CV_32SC3 as isize,
    Cv32sc4 = opencv::core::CV_32SC4 as isize,
    Cv32fc1 = opencv::core::CV_32FC1 as isize,
    Cv32fc2 = opencv::core::CV_32FC2 as isize,
    Cv32fc3 = opencv::core::CV_32FC3 as isize,
    Cv32fc4 = opencv::core::CV_32FC4 as isize,
    Cv64fc1 = opencv::core::CV_64FC1 as isize,
    Cv64fc2 = opencv::core::CV_64FC2 as isize,
    Cv64fc3 = opencv::core::CV_64FC3 as isize,
    Cv64fc4 = opencv::core::CV_64FC4 as isize,
}
//
//
impl MatType {
    ///
    /// Returns color depth number of bits
    #[allow(unused)]
    pub fn depth(&self) -> u8 {
        match self {
            Self::Cv8uc1 => 8,
            Self::Cv8uc2 => 8,
            Self::Cv8uc3 => 8,
            Self::Cv8uc4 => 8,
            Self::Cv8sc1 => 8,
            Self::Cv8sc2 => 8,
            Self::Cv8sc3 => 8,
            Self::Cv8sc4 => 8,
            Self::Cv16uc1 => 16,
            Self::Cv16uc2 => 16,
            Self::Cv16uc3 => 16,
            Self::Cv16uc4 => 16,
            Self::Cv16sc1 => 16,
            Self::Cv16sc2 => 16,
            Self::Cv16sc3 => 16,
            Self::Cv16sc4 => 16,
            Self::Cv32sc1 => 32,
            Self::Cv32sc2 => 32,
            Self::Cv32sc3 => 32,
            Self::Cv32sc4 => 32,
            Self::Cv32fc1 => 32,
            Self::Cv32fc2 => 32,
            Self::Cv32fc3 => 32,
            Self::Cv32fc4 => 32,
            Self::Cv64fc1 => 64,
            Self::Cv64fc2 => 64,
            Self::Cv64fc3 => 64,
            Self::Cv64fc4 => 64,
        }
    }
    ///
    /// Returns number of color channels
    #[allow(unused)]
    pub fn channels(&self) -> u8 {
        match self {
            Self::Cv8uc1 => 1,
            Self::Cv8uc2 => 2,
            Self::Cv8uc3 => 3,
            Self::Cv8uc4 => 4,
            Self::Cv8sc1 => 1,
            Self::Cv8sc2 => 2,
            Self::Cv8sc3 => 3,
            Self::Cv8sc4 => 4,
            Self::Cv16uc1 => 1,
            Self::Cv16uc2 => 2,
            Self::Cv16uc3 => 3,
            Self::Cv16uc4 => 4,
            Self::Cv16sc1 => 1,
            Self::Cv16sc2 => 2,
            Self::Cv16sc3 => 3,
            Self::Cv16sc4 => 4,
            Self::Cv32sc1 => 1,
            Self::Cv32sc2 => 2,
            Self::Cv32sc3 => 3,
            Self::Cv32sc4 => 4,
            Self::Cv32fc1 => 1,
            Self::Cv32fc2 => 2,
            Self::Cv32fc3 => 3,
            Self::Cv32fc4 => 4,
            Self::Cv64fc1 => 1,
            Self::Cv64fc2 => 2,
            Self::Cv64fc3 => 3,
            Self::Cv64fc4 => 4,
        }
    }
}