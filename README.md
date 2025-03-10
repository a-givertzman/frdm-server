# frdm-server

Fiber Rope Defects Monitoring

Web page: https://sa-lab.dev/projects-frdm.html#project__overview

## Installation

- Based on the [Rust OpenCV lib](https://github.com/twistedfall/opencv-rust?tab=readme-ov-file)

   Check [README](https://github.com/twistedfall/opencv-rust/blob/master/INSTALL.md) to get started

- Used Arena SDK on Linux (integrated using OpenCV)
    - [Original instructions](https://support.thinklucid.com/using-opencv-with-arena-sdk-on-linux/)
    - Download [Arena SDK](https://thinklucid.com/downloads-hub/) (registration required)
    - Extract the tarball to your desired location:  
      ```bash
      $ tar -xvzf ArenaSDK_v0.1.95_Linux_x64.tar.gz --directory /tmp/arena/
      ```
    - Run the Arena_SDK.conf file
      > WARNING: Pass -cti argument to set the GENICAM_GENTL64_PATH environment variable.
            Reboot the PC before running applications that use .cti files.
      ```bash
      $ cd /path/to/ArenaSDK_Linux_x64/
      $ chmod +x ./Arena_SDK_Linux_x64.conf 
      $ sudo sh Arena_SDK_Linux_x64.conf
      ```
      This will make the Arena SDK shared library files accessible by the run-time linker (ld.so or ld-linux.so).
    - Be shure the MTU for ethernet interfgace used by camera is set to 900 bytes