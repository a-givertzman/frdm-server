Please read for reference:
https://support.thinklucid.com/knowledgebase/using-lucid-3d-rgb-ip67-kit-in-arenaview/
https://support.thinklucid.com/app-note-helios-3d-point-cloud-with-rgb-color/#gs-opencv2
https://support.thinklucid.com/using-opencv-with-arena-sdk-on-windows/


How to run HLTRGB examples on Linux:

1) Install OpenCV library
https://support.thinklucid.com/using-opencv-with-arena-sdk-on-linux/

2) Open folder with HLTRGB example
ex. /ArenaSDK_Linux_x64/Examples/Arena/Cpp_HLTRGB_1_Calibration

3) Run "make" then run example
   
How to run HLTRGB examples on Windows:

1) Install OpenCV library.
https://opencv.org/releases/

Or install recommended version.
https://sourceforge.net/projects/opencvlibrary/files/4.5.3/opencv-4.5.3-vc14_vc15.exe/download

Choose extract to - C:\OpenCV-4.5.3

2) Add OpenCV binaries to your System path.
    - default path if OpenCV installed from recommended link - C:\OpenCV-4.5.3\opencv\build\x64\vc15\bin

3) Configurate OpenCV for Visual Studio projects:
	- Go to project properties.
	- Go to Configuration Properties/VC++ Directories to add the include and library directories for OpenCV.
					- Configuration Properties -> VC++ Directories -> Include Directories - add - default path - C:\OpenCV-4.5.3\opencv\build\include
					- Configuration Properties -> VC++ Directories -> Library Directories - add - default path - C:\OpenCV-4.5.3\opencv\build\x64\vc15\lib
	-  Edit the VC++ project linker with the opencv_worldxxxx.lib OpenCV dynamic library.
					- debug - Linker -> Input -> Additional Dependencies - add - default lib - opencv_world453d.lib
					- release - Linker -> Input -> Additional Dependencies - add - default lib - opencv_world453.lib
