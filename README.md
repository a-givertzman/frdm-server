# frdm-server

Fiber Rope Defects Monitoring

Web page: https://sa-lab.dev/projects-frdm.html#project__overview

## Lens

Lens focal length | Image width | Image hight | Field depth
------------------| ----------: | ----------: | -------------:
4 mm              |  40...50    |      -      |    10..15
12 mm             |     15.5    |      -      |       4.5
16 mm             |     11.9    |      -      |       3.0

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
        
        - Use install.sh script - Recomended
            - Copy `install.sh` into Arena SDK folder
            ```bash
            cp src/infrostructure/arena/ArenaSDK_Linux_x64/install.sh /path/to/ArenaSDK_Linux_x64/
            ```
            - Execute `install.sh`
            ```bash
            cd /path/to/ArenaSDK_Linux_x64/
            ./install.sh
            ```

        - Use original installation script - Not recomended
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


- Descover the IP address of the camera

    - Install network discovering tool [netdisciver](https://github.com/alexxy/netdiscover)

    ```bash
    sudo apt install netdiscover
    ```
    
    - Use it in passive listening mode and detect incoming ARP announcements (using the -p switch) 
    ```bash
    sudo netdiscover -p
    ```
    
    - Reconnect the camera on the network and you will reciev it new IP
    ```bash
    Currently scanning: (passive)   |   Screen View: Unique Hosts

    5 Captured ARP Req/Rep packets, from 2 hosts.   Total size: 300
    _____________________________________________________________________________
    IP            At MAC Address     Count     Len  MAC Vendor / Hostname      
    -----------------------------------------------------------------------------
    0.0.0.0         1c:0f:af:90:a1:71      3     180  Lucid Vision Labs
    169.254.114.161 1c:0f:af:90:a1:71      2     120  Lucid Vision Labs
    ```
    - After some timeout (few secinds) camera MAC: `1c:0f:af:90:a1:71` and IP: `169.254.114.161`

- Start the application
  
    - usinng cargo
    ```bash
    cargo --run --release
    ```
    
    - Or precompiled executable
    ```bash
    ./frdm-server
    ```

## Regenerate bindings

```bash
bindgen src/infrostructure/arena/wrappers.h -o src/infrostructure/arena/bindings.rs -- "-Ilucid_arena_sdk_include_path"
```