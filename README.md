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


cargo --run --release

```log

error: linking with `cc` failed: exit status: 1
  |
  = note: LC_ALL="C" PATH="/home/lobanov/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin:/home/lobanov/.nvm/versions/node/v23.3.0/bin:/home/lobanov/.cargo/bin:/opt/flutter/bin:/usr/local/bin:/usr/bin:/bin:/usr/local/games:/usr/games" VSLANG="1033" "cc" "-m64" "/tmp/rustc3FC4zg/symbols.o" "<17 object files omitted>" "-Wl,--as-needed" "-Wl,-Bstatic" "/home/lobanov/code/rust/frdm-server/target/release/deps/{libstrum-0b7bf07ed3be7e80.rlib}" "/home/lobanov/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/{libtest-bb17ba1fa02ea08e.rlib,libgetopts-d04d0c542852b7d7.rlib,libunicode_width-7748d1fe0f8acd00.rlib,librustc_std_workspace_std-6cf585dc4073d549.rlib}" "/home/lobanov/code/rust/frdm-server/target/release/deps/{libdebugging-7a4c48cf6c66502b.rlib,libenv_logger-83328ca0d6a4eb0c.rlib,libanstream-7f9c39cc16105ac2.rlib,libanstyle_query-0ec2b25140b6d7ad.rlib,libis_terminal_polyfill-bf761cd4dc0b389b.rlib,libcolorchoice-1e6c555edcee5f25.rlib,libanstyle_parse-916f91a0d31394cd.rlib,libutf8parse-92507dd790123542.rlib,libjiff-473589abbf03506b.rlib,libenv_filter-38e9b9939c461500.rlib,libanstyle-319a8ccf41848125.rlib,libtesting-10cae0948b1b1543.rlib,libopencv-114e024fb7dda441.rlib,libsal_sync-e9b6c6d50e5a82e0.rlib,libserde_yaml-55f2b6cde78c9c20.rlib,libunsafe_libyaml-b45e9684ba67bb97.rlib,libregex-b19fe03f86732b4c.rlib,libregex_automata-0ea2c2e990d000f8.rlib,libaho_corasick-3a453165868f5eb0.rlib,libregex_syntax-4df818b07f17da16.rlib,libapi_tools-8c84db3c73dab772.rlib,libpostgres-6b6386af6c5741ce.rlib,libtokio_postgres-d021276b9a5b15d2.rlib,libphf-5bad353a46b856f8.rlib,libphf_shared-c42c3fea4fc27df2.rlib,libsiphasher-e089f424824f1a01.rlib,libwhoami-9c95ab043efb959f.rlib,libpercent_encoding-62ca22eb4f35439b.rlib,libtokio_util-5f458ea1887a1555.rlib,libparking_lot-01b9f4b230714582.rlib,libparking_lot_core-92d0f85353d1c114.rlib,libsmallvec-97a0aef71691e276.rlib,liblock_api-4cd32ae3b326a62a.rlib,libscopeguard-60a1171477bdd3ce.rlib,libfutures_channel-c05a178f429b7dcf.rlib,libtokio-51d4d20f381976ee.rlib,libsocket2-4c01ce6e148fe7b5.rlib,libmio-1b8a07862540e316.rlib,libpostgres_types-9e22ee810b59e5fc.rlib,libserde_json-0e1451c6fac9f1e1.rlib,libitoa-5e2be848989b08aa.rlib,libryu-faf6d12fb8cc5562.rlib,libpostgres_protocol-fb971c77fc2220bf.rlib,libstringprep-21aabfeec7798039.rlib,libunicode_properties-b2ba0179e051d6c4.rlib,libunicode_normalization-309cf3d46cd6b8ac.rlib,libtinyvec-025ce083cb9e06ed.rlib,libtinyvec_macros-e403c844cd3ad9eb.rlib,libunicode_bidi-4de7d47cce945665.rlib,libmemchr-c8388e68dc42a0fb.rlib,libsha2-703b2ec968fec22c.rlib,libcpufeatures-57b6db1c5fa82529.rlib,librand-c668656c2f7dd217.rlib,librand_chacha-d2fd79394012b7ef.rlib,librand_core-a6cf681881750990.rlib,libgetrandom-3b06f1f07df844ad.rlib,libhmac-5ce4b1735e26933b.rlib,libbase64-b804dc57cea28435.rlib,libmd5-b0bf03b41be7b239.rlib,libdigest-b481a1e2548a5990.rlib,libsubtle-f737cd1a17195a5d.rlib,libblock_buffer-bf51be6242240f45.rlib,libcrypto_common-04003abf4e7ac147.rlib,libgeneric_array-bf0cc6f9804a12f3.rlib,libtypenum-ad6f52ecc6416362.rlib,libfutures_util-38242913b67a3a53.rlib,libslab-25380e9912a74dba.rlib,libpin_project_lite-da4b768b555dfb71.rlib,libfutures_sink-71f585249bf7ed8c.rlib,libfutures_task-6db7b99001e2b038.rlib,libpin_utils-7b53b70439efde7e.rlib,libfutures_core-1c4bfbfaddd0db2d.rlib,libbytes-d27ec48e375f99fe.rlib,libfallible_iterator-234f5bffb7bac3ac.rlib,libconcat_in_place-139cc37e41e6e0ed.rlib,libtesting-ab4807de5dee8840.rlib,librand-4d9ec72c6dec85b6.rlib,librand_chacha-4407c9c6566d0411.rlib,libppv_lite86-5f0076ffe42dec9f.rlib,libzerocopy-dbbbce475e7dad8c.rlib,librand_core-52889763d5972100.rlib,libgetrandom-44f9b45642b92a1b.rlib,liblibc-2afee566be1a6e5e.rlib,libcfg_if-73dbcf6a2d0ddc7f.rlib,libchrono-04a2d6eed8cf6b61.rlib,libnum_traits-25ee7693f7064064.rlib,libiana_time_zone-f94afee3d77de1ee.rlib,libconcat_string-2a5dc1e7a1928988.rlib,libindexmap-44aa254cb606c742.rlib,libequivalent-f9833901e3016d9d.rlib,libhashbrown-f61664884c23f1e5.rlib,libserde-f8377c7291106128.rlib,libhashers-573022c2e39806ae.rlib,libfxhash-d9a1c370e7bb1809.rlib,libbyteorder-6d1fd29486dc3b61.rlib,liblog-eeeeba1bbfa2ffb1.rlib}" "/home/lobanov/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/{libstd-6273572f18644c87.rlib,libpanic_unwind-267e668abf74a283.rlib,libobject-ec6154ccae37a33e.rlib,libmemchr-500edd5521c440d4.rlib,libaddr2line-86d8d9428792e8ef.rlib,libgimli-10f06487503767c2.rlib,librustc_demangle-6a38424de1e5bca5.rlib,libstd_detect-de9763ea1c19dca3.rlib,libhashbrown-a7f5bb2f736d3c49.rlib,librustc_std_workspace_alloc-7e368919bdc4a44c.rlib,libminiz_oxide-376454d49910c786.rlib,libadler-fa99f5692b5dce85.rlib,libunwind-91cafdaf16f7fe40.rlib,libcfg_if-f7ee3f1ea78d9dae.rlib,liblibc-d3a35665f881365a.rlib,liballoc-715bc629a88bca60.rlib,librustc_std_workspace_core-ae70165d1278cff7.rlib,libcore-406129d0e3fbc101.rlib,libcompiler_builtins-1af05515ab19524a.rlib}" "-Wl,-Bdynamic" "-lstdc++" "-lopencv_stitching" "-lopencv_alphamat" "-lopencv_aruco" "-lopencv_barcode" "-lopencv_bgsegm" "-lopencv_bioinspired" "-lopencv_ccalib" "-lopencv_cvv" "-lopencv_dnn_objdetect" "-lopencv_dnn_superres" "-lopencv_dpm" "-lopencv_face" "-lopencv_freetype" "-lopencv_fuzzy" "-lopencv_hdf" "-lopencv_hfs" "-lopencv_img_hash" "-lopencv_intensity_transform" "-lopencv_line_descriptor" "-lopencv_mcc" "-lopencv_quality" "-lopencv_rapid" "-lopencv_reg" "-lopencv_rgbd" "-lopencv_saliency" "-lopencv_shape" "-lopencv_stereo" "-lopencv_structured_light" "-lopencv_phase_unwrapping" "-lopencv_superres" "-lopencv_optflow" "-lopencv_surface_matching" "-lopencv_tracking" "-lopencv_highgui" "-lopencv_datasets" "-lopencv_text" "-lopencv_plot" "-lopencv_ml" "-lopencv_videostab" "-lopencv_videoio" "-lopencv_viz" "-lopencv_wechat_qrcode" "-lopencv_ximgproc" "-lopencv_video" "-lopencv_xobjdetect" "-lopencv_objdetect" "-lopencv_calib3d" "-lopencv_imgcodecs" "-lopencv_features2d" "-lopencv_dnn" "-lopencv_flann" "-lopencv_xphoto" "-lopencv_photo" "-lopencv_imgproc" "-lopencv_core" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-z,noexecstack" "-L" "./src/infrostructure/arena/ArenaSDK_Linux_x64/lib64" "-L" "./src/infrostructure/arena/ArenaSDK_Linux_x64/GenICam/library/lib/Linux64_x64" "-L" "./src/infrostructure/arena/ArenaSDK_Linux_x64/ffmpeg" "-L" "/home/lobanov/code/rust/frdm-server/target/release/build/opencv-34d22e8b1d462377/out" "-L" "/usr/lib/x86_64-linux-gnu" "-L" "/home/lobanov/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/home/lobanov/code/rust/frdm-server/target/release/deps/frdm_server-a6baeeb64f9f584f" "-Wl,--gc-sections" "-pie" "-Wl,-z,relro,-z,now" "-Wl,-O1" "-Wl,--strip-debug" "-nodefaultlibs"
  = note: some arguments are omitted. use `--verbose` to show all linker arguments
  = note: /usr/bin/ld: /home/lobanov/code/rust/frdm-server/target/release/deps/frdm_server-a6baeeb64f9f584f.frdm_server.d7c868e34937a04e-cgu.05.rcgu.o: in function `frdm_server::test::unit::infrostructure::camera::arena_test::tests::test_task_cycle':
          frdm_server.d7c868e34937a04e-cgu.05:(.text._ZN11frdm_server4test4unit14infrostructure6camera10arena_test5tests15test_task_cycle17hc211f2ef89646215E+0x1cb): undefined reference to `acOpenSystem'
          /usr/bin/ld: frdm_server.d7c868e34937a04e-cgu.05:(.text._ZN11frdm_server4test4unit14infrostructure6camera10arena_test5tests15test_task_cycle17hc211f2ef89646215E+0x204): undefined reference to `acSystemUpdateDevices'
          /usr/bin/ld: frdm_server.d7c868e34937a04e-cgu.05:(.text._ZN11frdm_server4test4unit14infrostructure6camera10arena_test5tests15test_task_cycle17hc211f2ef89646215E+0x237): undefined reference to `acSystemGetNumDevices'
          /usr/bin/ld: frdm_server.d7c868e34937a04e-cgu.05:(.text._ZN11frdm_server4test4unit14infrostructure6camera10arena_test5tests15test_task_cycle17hc211f2ef89646215E+0x300): undefined reference to `acCloseSystem'
          collect2: error: ld returned 1 exit status
          
  = note: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
  = note: use the `-l` flag to specify native libraries to link
  = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-lib)

warning: `frdm-server` (bin "frdm-server" test) generated 1634 warnings
error: could not compile `frdm-server` (bin "frdm-server" test) due to 1 previous error; 1634 warnings emitted
```