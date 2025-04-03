use std::{sync::{atomic::{AtomicBool, Ordering}, mpsc, Arc}, thread::JoinHandle, time::Duration};

use opencv::videoio::VideoCaptureTrait;
use sal_core::error::Error;
use sal_sync::services::entity::{dbg_id::DbgId, name::Name};
use crate::infrostructure::arena::{ac_device::AcDevice, ac_system::AcSystem, image::Image};
use super::{camera_conf::CameraConf, pimage::PImage};
///
/// # Description to the [Camera] class
/// - Connecting to the IP Camra
/// - Receive frames from the `Camera`
pub struct Camera {
    dbg: DbgId,
    name: Name,
    conf: CameraConf,
    send: mpsc::Sender<Image>,
    recv: Option<mpsc::Receiver<Image>>,
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
        let dbg = DbgId(conf.name.join());
        log::trace!("{}.new | : ", dbg);
        let (send, recv) = mpsc::channel();
        Self {
            dbg,
            name: conf.name.clone(),
            conf,
            send,
            recv: Some(recv),
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// Returns channel recv to access farmes from camera
    /// - call `read` to start reading frames from camera
    /// - call `close` to stop reading and cleen up
    pub fn stream(&mut self) -> mpsc::Receiver<Image> {
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
                                        log::info!(
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
                                                    break;
                                                }
                                            } else {
                                                log::warn!("{}.read | Specified device index '{}' out of found devices count '{}'", dbg, index, devices);
                                                break;
                                            }
                                        }
                                        None => {
                                            log::error!("{}.read | Device index - is not specified in the camera conf", dbg);
                                            break;
                                        }
                                    }
                                } else {
                                    log::warn!("{}.read | No devices detected on current network interface", dbg);
                                }
                            }
                            None => {
                                log::warn!("{}.read | No devices detected, Possible AcSystem is not executed first", dbg);
                                break;
                            }
                        }
                    }
                    Err(err) => {
                        log::warn!("{}.read | Error: {}", dbg, err);
                        break;
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
    /// Receive frames from IP camera
    pub fn from_file(&self, path: impl Into<String>) -> Result<CameraIntoIterator, Error> {
        match opencv::videoio::VideoCapture::from_file(&path.into(), opencv::videoio::CAP_ANY) {
            Ok(mut video) => {
                let mut frames = vec![];
                let mut frame = opencv::core::Mat::default();
                while let Ok(result) = video.read(&mut frame) {
                    if result {
                        frames.push(PImage::new(frame.clone()));
                    } else {
                        break;
                    }
                }
                Ok(CameraIntoIterator { frames })
            }
            Err(err) => Err(Error::new(&self.dbg, "from_file").err(err.to_string())),
        }
    }
    ///
    /// Sends `Exit` signal to stop reading.
    pub fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}
///
/// Camera Iterator
pub struct CameraIntoIterator {
    // camera: Camera,
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
            // camera: self,
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