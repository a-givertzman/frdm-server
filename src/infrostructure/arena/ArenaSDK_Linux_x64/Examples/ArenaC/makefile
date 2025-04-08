SUBDIRS =   C_Acquisition                                 \
            C_Acquisition_CompressedImageHandling         \
            C_Acquisition_CompressedImageLoading          \
	        C_Acquisition_TCP                             \
	        C_Acquisition_RDMA                            \
            C_Acquisition_MultiDevice                     \
	        C_Acquisition_MultithreadedAcquisitionAndSave \
            C_Acquisition_RapidAcquisition                \
	        C_Acquisition_SensorBinning                   \
            C_Callback_ImageCallbacks                     \
            C_Callback_MultithreadedImageCallbacks        \
            C_Callback_OnDeviceDisconnected               \
            C_Callback_OnEvent                            \
            C_Callback_OnNodeChange                       \
            C_Callback_Polling                            \
            C_ChunkData                                   \
            C_ChunkData_CRCValidation                     \
            C_Enumeration                                 \
	        C_Enumeration_CcpSwitchover                   \
            C_Enumeration_HandlingDisconnections          \
	        C_Enumeration_Unicast                         \
            C_Explore_NodeMaps                            \
            C_Explore_Nodes                               \
            C_Explore_NodeTypes                           \
            C_Exposure                                    \
	        C_Exposure_Long                               \
            C_ForceIp                                     \
            C_Helios_HeatMap                              \
            C_Helios_MinMaxDepth                          \
            C_Helios_SmoothResults                        \
            C_ImageProcessing_SplitChannels               \
	        C_IpConfig_Auto                               \
            C_IpConfig_Manual                             \
            C_LUT                                         \
            C_Multicast                                   \
            C_PixelCorrection                             \
            C_Polarization_DolpAolp                       \
            C_Polarization_ColorDolpAolp                  \
            C_Record                                      \
            C_Save                                        \
	        C_Save_Bmp                                    \
	        C_Save_Jpeg                                   \
	        C_Save_Png                                    \
	        C_Save_Raw                                    \
	        C_Save_Tiff                                   \
            C_Save_Ply                                    \
            C_Save_FileNamePattern                        \
            C_ScheduledActionCommands                     \
            C_Sequencer_HDR                               \
            C_SimpleAcquisition                           \
            C_Streamables                                 \
            C_Trigger                                     \
            C_Trigger_NextLeader                          \
            C_Trigger_OverlappingTrigger                  \
            C_UserSets                                    \


SRCS = $(wildcard *.c)
OBJS = $(SRCS:%.c=%.o)
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
