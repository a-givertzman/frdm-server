use std::sync::mpsc;
use sal_sync::services::entity::name::Name;
use crate::domain::dbg::dbgid::DbgId;
use super::{camera_conf::CameraConf, pimage::PImage};
///
/// # Description to the [Camera] class
/// - Connecting to the IP Camra
/// - Receive frames from the `Camera`
pub struct Camera {
    dbg: DbgId,
    name: Name,
    conf: CameraConf,
}
//
//
impl Camera {
    ///
    /// Returns [Camera] new instance
    /// - [parent] - DbgId of parent entitie
    /// - `conf` - configuration parameters
    pub fn new(conf: CameraConf) -> Self {
        let dbg = DbgId::root(conf.name.join());
        log::debug!("{}.new | : ", dbg);
        Self {
            dbg,
            name: conf.name.clone(),
            conf,
        }
    }
    ///
    /// Receive frames from IP camera
    pub fn read(&self, path: Into<Path>) -> mpsc::Receiver<PImage> {
        let video = videoio::VideoCapture::from_file("src/video/video_test.mp4", videoio::CAP_ANY).unwrap();
        video.read(&mut frame).unwrap();
        if frame.empty() {
            break;
        }
        camera.push_frame(PImage::new(frame));
    }
}
///
/// Camera Iterator
pub struct CameraIntoIterator {
    camera: Camera,
    frames: Vec<PImage>,
}
//
//
impl CameraIntoIterator {
    pub fn push_frame(&mut self, frame: PImage) {
        self.frames.push(frame);
    }
    fn pop_first(&mut self) -> Option<PImage> {
        if self.frames.is_empty() {
            None
        } else {
            Some(self.frames.remove(0))
        }
    }
}
//
//
impl IntoIterator for Camera {
    type Item = PImage;
    type IntoIter = CameraIntoIterator;
    fn into_iter(self) -> Self::IntoIter {
        CameraIntoIterator {
            camera: self,
            frames: vec![] //cv::read_frames_from_file
        }
    }
}
//
//
impl Iterator for CameraIntoIterator {
    type Item = PImage;
    fn next(&mut self) -> Option<Self::Item> {
        self.pop_first()
    }
}