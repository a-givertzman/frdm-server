SUBDIRS =   Cpp_Acquisition                                 \
            Cpp_Acquisition_CompressedImageHandling         \
            Cpp_Acquisition_CompressedImageLoading          \
	    Cpp_Acquisition_TCP                             \
	    Cpp_Acquisition_RDMA                            \
	    Cpp_Acquisition_MultiDevice                     \
	    Cpp_Acquisition_MultithreadedAcquisitionAndSave \
	    Cpp_Acquisition_RapidAcquisition                \
	    Cpp_Acquisition_SensorBinning                   \
	    Cpp_Callback_ImageCallbacks                     \
	    Cpp_Callback_MultithreadedImageCallbacks        \
	    Cpp_Callback_OnEvent                            \
	    Cpp_Callback_OnDeviceDisconnected               \
	    Cpp_Callback_OnNodeChange                       \
	    Cpp_Callback_Polling                            \
	    Cpp_ChunkData                                   \
	    Cpp_ChunkData_CRCValidation                     \
	    Cpp_Enumeration                                 \
	    Cpp_Enumeration_CcpSwitchover                   \
	    Cpp_Enumeration_HandlingDisconnections          \
	    Cpp_Enumeration_Unicast                         \
	    Cpp_Explore_NodeMaps                            \
	    Cpp_Explore_Nodes                               \
	    Cpp_Explore_NodeTypes                           \
	    Cpp_Exposure                                    \
	    Cpp_Exposure_ForHDR                             \
	    Cpp_Exposure_Long                               \
	    Cpp_ForceIp                                     \
	    Cpp_Helios_HeatMap                              \
	    Cpp_Helios_MinMaxDepth                          \
	    Cpp_Helios_SmoothResults                        \
	    Cpp_ImageProcessing_SplitChannels               \
	    Cpp_IpConfig_Auto                               \
	    Cpp_IpConfig_Manual                             \
	    Cpp_LUT                                         \
	    Cpp_Multicast                                   \
	    Cpp_PixelCorrection                             \
	    Cpp_Polarization_DolpAolp                       \
	    Cpp_Polarization_ColorDolpAolp                  \
	    Cpp_Record                                      \
	    Cpp_Save                                        \
	    Cpp_Save_Bmp                                    \
	    Cpp_Save_Jpeg                                   \
	    Cpp_Save_Png                                    \
	    Cpp_Save_Raw                                    \
	    Cpp_Save_Tiff                                   \
	    Cpp_Save_Ply                                    \
	    Cpp_Save_FileNamePattern                        \
	    Cpp_ScheduledActionCommands                     \
	    Cpp_Sequencer_HDR                               \
	    Cpp_SimpleAcquisition                           \
	    Cpp_Streamables                                 \
	    Cpp_Trigger                                     \
	    Cpp_Trigger_NextLeader                          \
	    Cpp_Trigger_OverlappingTrigger                  \
	    Cpp_UserSets                                    \
		IpConfigUtility



SRCS = $(wildcard *.cpp)
OBJS = $(SRCS:%.cpp=%.o)
DEPS = $(OBJS:%.o=%.d)

.PHONY: subdirs $(SUBDIRS)

subdirs: $(SUBDIRS)

$(SUBDIRS):
	@echo '\n'[$@]
	$(MAKE) -C $@

SUBCLEAN = $(addsuffix .clean,$(SUBDIRS))

.PHONY: clean $(SUBCLEAN)
clean: $(SUBCLEAN)

$(SUBCLEAN): %.clean:
	$(MAKE) -C $* clean


SUBRELEASE = $(addsuffix .release,$(SUBDIRS))

.PHONY: release $(SUBRELEASE)
release: $(SUBRELEASE)

$(SUBRELEASE): %.release:
	$(MAKE) -C $* release


SUBDEBUG = $(addsuffix .debug,$(SUBDIRS))

.PHONY: debug $(SUBDEBUG)
debug: $(SUBDEBUG)

$(SUBDEBUG): %.debug:
	$(MAKE) -C $* debug
