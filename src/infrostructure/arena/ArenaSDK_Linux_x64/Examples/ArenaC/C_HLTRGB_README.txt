Please read for reference:
https://support.thinklucid.com/knowledgebase/using-lucid-3d-rgb-ip67-kit-in-arenaview/
https://support.thinklucid.com/app-note-helios-3d-point-cloud-with-rgb-color/#gs-opencv2
https://support.thinklucid.com/using-opencv-with-arena-sdk-on-windows/


How to run HLTRGB examples on Linux:

1) Install OpenCV library
OpenCV-2.4 is available for Linux on website

https://support.thinklucid.com/using-opencv-with-arena-sdk-on-linux/


2) Open folder with HLTRGB example
ex. /ArenaSDK_Linux_x64/Examples/ArenaC/C_HLTRGB_1_Calibration

3) Run "make" then run example
   
How to run HLTRGB examples on Windows:

1) Install OpenCV library.
https://opencv.org/releases/

Or install recommended version.
https://sourceforge.net/projects/opencvlibrary/files/opencv-win/2.4.13/opencv-2.4.13.4-vc14.exe/download

Choose extract to - C:\OpenCV-2.4.13

2) Add OpenCV binaries to your System path.
    - default path if OpenCV installed from recommended link - C:\OpenCV-2.4.13\opencv\build\x64\vc14\bin

3) Configurate OpenCV for Visual Sdudio projects:
	- Go to project properties.
	- Go to Configuration Properties/VC++ Directories to add the include and library directories for OpenCV.
					- Configuration Properties -> VC++ Directories -> Include Dirictories - add - default path - C:\OpenCV-2.4.13\opencv\build\include
					- Configuration Properties -> VC++ Directories -> Library Dirictories - add - default path - C:\OpenCV-2.4.13\opencv\build\x64\vc14\lib
	-  Edit the VC++ project linker with the opencv_worldxxxx.lib OpenCV dynamic library.
					- debug - Linker -> Input -> Additional Dependencies - add - default lib - 
						opencv_core2413d.lib, opencv_highgui2413d.lib, opencv_imgproc2413d.lib, opencv_calib3d2413d.lib
					- release - Linker -> Input -> Additional Dependencies - add - default lib - 
						opencv_core2413.lib, opencv_highgui2413.lib, opencv_imgproc2413.lib, opencv_calib3d2413.lib

!!! Notes for running HLTRGB C examples (require tuning parameters manually, lots of patience needed):

1. C_HLTRGB_1_Calibration  (until you can see 20 points clearly in the displayed image)

	- The findCalibrationPoints function written in C for OpenCV significantly differs from its C++ counterpart
	due to the nature of OpenCV's use of C++ features, such as classes and methods, which are not available in C. 

	-  Due to the absence of a direct SimpleBlobDetector equivalent in C, the function uses contour detection 
	and filtering based on geometric properties to identify potential calibration points.

	- Steps to adjust threshold: 
	 	- Start with Visualization: uncomment the visulization block to display the thresholded image
		- Initial Threshold Selection: Begin with a recommended threshold value, like 80, as a starting point. 
		- Incremental Adjustments:
					- If too many noise points are detected as calibration points, increase the threshold value.
					 This makes the criteria for a pixel to be considered part of a calibration point more stringent.

					- If calibration points are missed (not detected), decrease the threshold value. 
					This allows more pixels to be considered as part of calibration points, at the risk of including more noise.
	
		-  Adjust the Kernel Size:
					-  If the calibration points or other important features are becoming too blurred, leading to difficulties in detection or analysis, 
					reducing the kernel size (e.g., to (5, 5) or (3, 3)) might help preserve more detail.

					- In cases where the image is very noisy and the initial smoothing is insufficient to clean up the noise, 
					increasing the kernel size (e.g., to (11, 11) or (15, 15)) might improve the situation by applying a stronger smoothing effect.

2. C_HLTRGB_2_Orientation examples: (until you can get 20 points clearly in both cameras)

	- two functions that need to adjust the threshold

		- "findCalibrationPointsHLT"
		   
		   			- set VERBOSE_PRINTOUT to 1 for more verbose debugging printouts

		   			- use the same adjusting threshold and kernel methods as the first C_HLTRGB_1_Calibration example
		  			 until you found 20 circles

		- "findCalibrationPointsTRI"

		  			- set VERBOSE_PRINTOUT to 1 for more verbose debugging printouts

		   			- use the same adjusting threshold and kernel methods as the first C_HLTRGB_1_Calibration example
		  	 		until you found 20 circles

			

		


	


