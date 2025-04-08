ARCH_TYPE = $(shell dpkg --print-architecture)

ifeq ($(ARCH_TYPE), amd64)

LDFLAGS = -L../../../lib64 \
          -L../../../GenICam/library/lib/Linux64_x64 \
          -L../../../ffmpeg \
		  $(OPENCV_LIB_PATH)
          
GENICAMLIBS = -lGCBase_gcc54_v3_3_LUCID \
              -lGenApi_gcc54_v3_3_LUCID \
              -lLog_gcc54_v3_3_LUCID \
              -llog4cpp_gcc54_v3_3_LUCID \
              -lMathParser_gcc54_v3_3_LUCID \
              -lNodeMapData_gcc54_v3_3_LUCID \
              -lXmlParser_gcc54_v3_3_LUCID

OUTDIR = ../../../OutputDirectory/Linux/x64Release/

else ifeq ($(ARCH_TYPE), armhf)

LDFLAGS = -L../../../lib \
          -L../../../GenICam/library/lib/Linux32_ARMhf \
          -L../../../ffmpeg \
		  $(OPENCV_LIB_PATH)

GENICAMLIBS = -lGCBase_gcc540_v3_3_LUCID \
              -lGenApi_gcc540_v3_3_LUCID \
              -lLog_gcc540_v3_3_LUCID \
              -llog4cpp_gcc540_v3_3_LUCID \
              -lMathParser_gcc540_v3_3_LUCID \
              -lNodeMapData_gcc540_v3_3_LUCID \
              -lXmlParser_gcc540_v3_3_LUCID


OUTDIR = ../../../OutputDirectory/armhf/x32Release/

else ifeq ($(ARCH_TYPE), arm64)

LDFLAGS = -L../../../lib \
          -L../../../GenICam/library/lib/Linux64_ARM \
          -L../../../ffmpeg \
		  $(OPENCV_LIB_PATH)

GENICAMLIBS = -lGCBase_gcc54_v3_3_LUCID \
              -lGenApi_gcc54_v3_3_LUCID \
              -lLog_gcc54_v3_3_LUCID \
              -llog4cpp_gcc54_v3_3_LUCID \
              -lMathParser_gcc54_v3_3_LUCID \
              -lNodeMapData_gcc54_v3_3_LUCID \
              -lXmlParser_gcc54_v3_3_LUCID


OUTDIR = ../../../OutputDirectory/arm64/x64Release/
endif

CC=g++

INCLUDE= -I../../../include/Arena \
         -I../../../include/Save \
         -I../../../include/GenTL \
         -I../../../GenICam/library/CPP/include \
		 $(OPENCV_INC_PATH)

CFLAGS=-Wall -g -O2 -std=c++11 -Wno-unknown-pragmas


FFMPEGLIBS = -lavcodec \
             -lavformat \
             -lavutil \
             -lswresample

LIBS= -larena -lsave -lgentl $(GENICAMLIBS) $(FFMPEGLIBS) -lpthread -llucidlog $(OPENCV_LIBS)
RM = rm -f

SRCS = $(wildcard *.cpp)
OBJS = $(SRCS:%.cpp=%.o)
DEPS = $(OBJS:%.o=%.d)

.PHONY: all
all: ${TARGET}

${TARGET}: ${OBJS}
	${CC} ${LDFLAGS} $^ -o $@ $(LIBS)
	-mkdir -p $(OUTDIR)
	-cp $(TARGET) $(OUTDIR)

%.o: %.cpp ${SRCS}
	${CC} ${INCLUDE}  ${LDFLAGS} -o $@ $< -c ${CFLAGS}

${DEPS}: %.cpp
	${CC} ${CFLAGS} ${INCLUDE} -MM $< >$@

-include $(OBJS:.o=.d)

.PHONY: clean
clean:
	-${RM} ${TARGET} ${OBJS} ${DEPS}
