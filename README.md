# frdm-server

Fiber Rope Defects Monitoring

Web page: <https://sa-lab.dev/projects-frdm.html#project__overview>

## Lens

[LUCID Vision Labs](https://thinklucid.com/category/lenses-lens-tubes/)
[Optical calculations](https://www.vision-doctor.com/en/service-en/sen-t5/optical-calculations.html)
[Calculating the depth of field (DOF)](https://www.vision-doctor.com/en/service-en/sen-t5/optical-calculations/calculation-depth-of-field.html)


**Lens test on working distance 200 mm**

Lens focal length | Image width  | Image hight | Field depth    | Image deformation
------------------| ----------:  | ----------: | -------------: | ------------------:
4 mm              | 400...500 mm |      -      |  100..150 mm   |  approx 10..30 px
12 mm             |     150.5 mm |      -      |      45.0 mm   |  approx 2..5 px
16 mm             |     110.9 mm |      -      |      20.0 mm   |  approx 1..2 px
25 mm             |      60.9 mm |      -      |       1.0 mm   |  approx < 1 px

## Installation

- Based on the [Rust OpenCV lib](https://github.com/twistedfall/opencv-rust?tab=readme-ov-file)

   Install using [README instruction](https://github.com/twistedfall/opencv-rust/blob/master/INSTALL.md) (Recomended)

   Or execute in terminal
   ```bash
   sudo apt install libopencv-dev clang libclang-dev
    ```

- Used Arena SDK for Linux (integrated using OpenCV)
    - **Importent:** Be shure the MTU for ethernet interfgace used by camera is set to 900 bytes
    - **Importent:** Also folluw [this instruction](src/infrostructure/arena/readme.md) to properly setup network inteface
    - Download Arena SDK v0.1.95 or later from [Downloads](https://thinklucid.com/downloads-hub/) (for Ubuntu)
    - Execute install script with the path to the Arena SDK donloaded archive and project root
        ```bash
        src/infrostructure/arena/install.sh "path/ArenaSDK_v0.1.95_Linux_x64.tar.gz" ./
        ```

    - [Original instructions](https://support.thinklucid.com/using-opencv-with-arena-sdk-on-linux/) - **Not recomended**


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

## Algorithm descriptions

Ferst frame passed into the `FastScan` algorithm, which very fast (15..17ms for 1936 x 1464 image) will find rope contours and make it analysys.
If some defects are detected, then already prepared in the `FastScan` normalized gray scale image passed into the `FineScan` algorithm.
`FineScan` is more expensive in calculations (200..300ms) but much more precise in rope contours detection.
Finally we have an array of rope defect if found in the frame.

### FastScan Algorithm

#### Contour detection algorithms optimized for speed, tradeoff in result quality

- Convert into gray scale
- Apply autogamma
- First way (execute in the separate thread)
   - Find contours based on the sharpness (sopel gradient or laplacian)
- Second way (execute in the separate thread)
   - Find contours based on the moving objhect (diff of same pixel betwee current and previouse frame)
- Union contours of two ways using bitwise/add_weighted operation

### FineScan Algorithm

#### Basic futures

Contour detection algorithms optimized for quality, tradeoff in speed

- Gray scale image expected from `context.normalized.gray`
- First way (execute in the separate thread)
   - Threshold based on the sharpness (sobel gradient or laplacian)
   - Find contours (polilines) around white (using threshold) areas
   - Compose nierby areas by distance between
   - Find biggest area
   - Make a convex hall around found biggest area
   - Store convex hall to be used by future steps
- Second way (execute in the separate thread)
   - Find contours based on the moving objhect (diff of same pixel betwee current and previouse frame)
- Union contours of two ways using bitwise/add_weighted operation
- Crop outside convex hall
