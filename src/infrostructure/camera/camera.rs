use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, thread::JoinHandle, time::Duration};
use opencv::videoio::VideoCaptureTrait;
use sal_core::{dbg::Dbg, error::Error};
use sal_sync::services::entity::{Name, Object};
use crate::{domain::{channel_unbounded, Receiver, Sender, Image}, infrostructure::arena::{AcDevice, AcSystem}};
use super::camera_conf::CameraConf;
///
/// # Description to the [Camera] class
/// - Connecting to the IP Camra
/// - Receive frames from the `Camera`
pub struct Camera {
    dbg: Dbg,
    name: Name,
    conf: CameraConf,
    send: Sender<Image>,
    recv: Option<Receiver<Image>>,
    suspend: Arc<AtomicBool>,
    exit: Arc<AtomicBool>,
}
//
//
impl Camera {
    ///
    /// Returns [Camera] new instance
    /// - [parent] - DbgId of parent entitie
    /// - `conf` - configuration parameters
    pub fn new(conf: CameraConf) -> Self {
        let dbg = Dbg::new(conf.name.parent(), conf.name.me());
        log::trace!("{}.new | : ", dbg);
        let (send, recv) = channel_unbounded();
        Self {
            dbg,
            name: conf.name.clone(),
            conf,
            send,
            recv: Some(recv),
            suspend: Arc::new(AtomicBool::new(false)),
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// Returns channel recv to access farmes from camera
    /// - call `read` to start reading frames from camera
    /// - call `close` to stop reading and cleen up
    pub fn stream(&mut self) -> Receiver<Image> {
        match self.recv.take() {
            Some(recv) => recv,
            None => {
                panic!("{}.stream | Receiver can be returned only once", self.name);
            },
        }
    }
    ///
    /// Receive frames from IP camera
    pub fn read(&self) -> Result<JoinHandle<()>, Error> {
        let dbg = self.dbg.clone();
        let conf = self.conf.clone();
        let send = self.send.clone();
        let exit = self.exit.clone();
        let handle = std::thread::spawn(move || {
            log::info!("{}.read | Start", dbg);
            loop {
                let mut ac_system = AcSystem::new(&dbg);
                match ac_system.run() {
                    Ok(_) => {
                        match ac_system.devices() {
                            Some(devices) => {
                                if devices > 0 {
                                    log::debug!("{}.read | Devices found: {}", dbg, devices);
                                    for dev in 0..devices {
                                        // log::debug!("{}.read | Retriving Device {}...", dbg, dev);
                                        let device_vendor = ac_system.device_vendor(dev).unwrap();
                                        let device_model = ac_system.device_model(dev).unwrap();
                                        log::trace!("{}.read | Device {} model: {}", dbg, dev, device_model);
                                        let device_serial = ac_system.device_serial(dev).unwrap();
                                        log::trace!("{}.read | Device {} serial: {}", dbg, dev, device_serial);
                                        let device_mac = ac_system.device_mac(dev).unwrap();
                                        log::trace!("{}.read | Device {} MAC: {}", dbg, dev, device_mac);
                                        let device_ip = ac_system.device_ip(dev).unwrap();
                                        log::trace!("{}.read | Device {} IP: {}", dbg, dev, device_ip);
                                        let device_firmware = ac_system.device_firmware(dev).unwrap();
                                        log::trace!("{}.read | Device {} Firmware: {}", dbg, dev, device_firmware);
                                        log::debug!(
                                            "{}.read | Device {}: {:?} | {:?} | {:?} | {:?} | {:?} | {:?}",
                                            dbg, dev, device_vendor, device_model, device_serial, device_mac, device_ip, device_firmware);
                                    }
                                    match &conf.index {
                                        Some(index) => {
                                            if devices >= index + 1 {
                                                let mut device = AcDevice::new(&dbg, ac_system.system, *index, conf.clone(), Some(exit.clone()));
                                                let result = device.listen(|frame| {
                                                    if let Err(err) = send.send(frame) {
                                                        log::warn!("{}.read | Send Error: {}", dbg, err);
                                                    }
                                                });
                                                if let Err(err) = result {
                                                    log::warn!("{}.read | Error: {}", dbg, err);
                                                }
                                            } else {
                                                log::warn!("{}.read | Specified device index '{}' out of found devices count '{}'", dbg, index, devices);
                                            }
                                        }
                                        None => {
                                            log::error!("{}.read | Device index - is not specified in the camera conf", dbg);
                                        }
                                    }
                                } else {
                                    log::warn!("{}.read | No devices detected on current network interface", dbg);
                                }
                            }
                            None => {
                                log::warn!("{}.read | No devices detected, Possible AcSystem is not executed first", dbg);
                            }
                        }
                    }
                    Err(err) => {
                        log::warn!("{}.read | Error: {}", dbg, err);
                    }
                }
                std::thread::sleep(Duration::from_secs(1));
                if exit.load(Ordering::SeqCst) {
                    break;
                }
            }
            log::info!("{}.read | Exit", dbg);
        });
        Ok(handle)
    }
    ///
    /// Receive frames from video file
    #[allow(unused)]
    pub fn from_video(&self, path: impl Into<String>) -> Result<CameraIntoIterator, Error> {
        match opencv::videoio::VideoCapture::from_file(&path.into(), opencv::videoio::CAP_ANY) {
            Ok(mut video) => {
                let mut frames = vec![];
                let mut frame = opencv::core::Mat::default();
                while let Ok(result) = video.read(&mut frame) {
                    if result {
                        frames.push(Image::with(frame.clone()));
                    } else {
                        break;
                    }
                }
                Ok(CameraIntoIterator { frames })
            }
            Err(err) => Err(Error::new(&self.dbg, "from_video").err(err.to_string())),
        }
    }
    ///
    /// Receive frames from path containing image files
    #[allow(unused)]
    pub fn from_images(&self, path: impl Into<String>) -> Result<CameraIntoIterator, Error> {
        let mut frames = vec![];
        match std::fs::read_dir(path.into()) {
            Ok(paths) => {
                for path in paths {
                    match path {
                        Ok(path) => {
                            if path.path().is_file() {
                                let path = path.path();
                                let path = path.to_str().ok_or(Error::new(&self.dbg, "from_images").err(format!("Error in path {}", path.display())))?;
                                match Image::load(path) {
                                    Ok(img) => {
                                        log::debug!("{}.from_images | Read: {}", self.dbg, path);
                                        frames.push(img);
                                    }
                                    Err(err) => return Err(Error::new(&self.dbg, "from_images").pass(err.to_string())),
                                }
                            }
                        }
                        Err(err) => return Err(Error::new(&self.dbg, "from_images").pass(err.to_string())),
                    }
                }
            }
            Err(err) => return Err(Error::new(&self.dbg, "from_images").pass(err.to_string())),
        }
        Ok(CameraIntoIterator { frames })
    }
    ///
    /// Suspending receiving frames from camera
    pub fn suspend(&self) {
        self.suspend.store(true, Ordering::Release);
    }
    ///
    /// Resuming receiving frames from camera
    pub fn resume(&self) {
        self.suspend.store(false, Ordering::Release);
    }
    ///
    /// Sends `Exit` signal to stop reading.
    #[allow(unused)]
    pub fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}
//
//
impl Object for Camera {
    fn name(&self) -> Name {
        self.name.clone()
    }
}
///
/// Camera Iterator
pub struct CameraIntoIterator {
    // camera: Camera,
    frames: Vec<Image>,
}
//
//
impl CameraIntoIterator {
    #[allow(unused)]
    pub fn push_frame(&mut self, frame: Image) {
        self.frames.push(frame);
    }
    fn pop_first(&mut self) -> Option<Image> {
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
    type Item = Image;
    type IntoIter = CameraIntoIterator;
    fn into_iter(self) -> Self::IntoIter {
        CameraIntoIterator {
            // camera: self,
            frames: vec![] //cv::read_frames_from_file
        }
    }
}
//
//
impl Iterator for CameraIntoIterator {
    type Item = Image;
    fn next(&mut self) -> Option<Self::Item> {
        self.pop_first()
    }
}