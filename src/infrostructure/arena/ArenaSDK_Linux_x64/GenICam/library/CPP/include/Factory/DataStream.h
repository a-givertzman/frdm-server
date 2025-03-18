/****************************************************************************
 (c) 2005 by STEMMER IMAGING

 License: This file is published under the license of the EMVA GenICam  Standard Group.
 A text file describing the legal terms is included in  your installation as 'license.txt'.
 If for some reason you are missing  this file please contact the EMVA or visit the website
 (http://www.genicam.org) for a full copy.

 THIS SOFTWARE IS PROVIDED BY THE EMVA GENICAM STANDARD GROUP "AS IS"
 AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,
 THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE EMVA GENICAM STANDARD  GROUP
 OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,  SPECIAL,
 EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT  LIMITED TO,
 PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,  DATA, OR PROFITS;
 OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY  THEORY OF LIABILITY,
 WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT  (INCLUDING NEGLIGENCE OR OTHERWISE)
 ARISING IN ANY WAY OUT OF THE USE  OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 POSSIBILITY OF SUCH DAMAGE.

****************************************************************************/

 /// \file     $Source: /cvs/genicam/genicam/library/CPP/include/Factory/DataStream.h,v $
 /// \brief    Data Stream class
 /// \version  $Revision: 1.6 $
 /// \date     $Date: 2006/07/18 13:31:52 $




#ifndef GC_FACTORY_DATASTREAM_H_
#define GC_FACTORY_DATASTREAM_H_

#include "GenApi/Types.h"
#include "GenApi/RegisterSetHelper.h"
#include "GenApi/Pointer.h"
#include "GenApi/INodeMapDyn.h"
#include "GenApi/NodeMapDynPtr.h"
#include "GenApi/Impl/Category.h"


#include "Base/GCBase.h"
#include <Factory/Factory.h>


namespace GenICam
{
  namespace Factory
  {
    class CDataStream
    {
      friend class CDevice;
    private:

    public:
                                         CDataStream             ( CDevice *pDev, TL::Client::DS_HANDLE hDS, uint32_t iIndex, GenApi::INodeMap *TLNodeMap, GenApi::INodeMap *DevNodeMap, HMODULE hGlueMod);
      virtual                            ~CDataStream            ( void );

      GC_IMPORT_EXPORT  void             AnnounceBuffer           ( void *pBuffer, int32_t iSize, void *pPrivate, uint32_t *iID );
      GC_IMPORT_EXPORT  void             AllocAndAnnounceBuffer   ( uint32_t iSize, void *pPrivate, void **pBuffer, uint32_t *iID );
      GC_IMPORT_EXPORT  void            *RevokeBuffer             ( uint32_t iID, void **pBuffer = NULL, void **pPrivate = NULL);
      GC_IMPORT_EXPORT  void             QueueBuffer              ( uint32_t iID);
      GC_IMPORT_EXPORT  void             FlushQueue               ( TL::Client::ACQ_QUEUE_TYPE QueueType );
      GC_IMPORT_EXPORT  void             StartAcquisition         ( TL::Client::ACQ_START_FLAGS iAcquisitionFlags = TL::Client::ACQ_START_FLAGS_NONE, int64_t iNumDatasToAcquire = -1);
      GC_IMPORT_EXPORT  void             WaitForBuffer            ( int32_t iTimeoutMs, TL::Client::ACQ_WAIT_FLAGS iWaitFlags, TL::Client::ACQ_INFO *pInfo, void **ppPrivate);
      GC_IMPORT_EXPORT  void             StopAcquisition          ( TL::Client::ACQ_STOP_FLAGS iFlags);
      GC_IMPORT_EXPORT  void             GetStatus                ( TL::Client::ACQ_STATUS_CMD iCmd, void *pBuffer, uint64_t *iSize);
      GC_IMPORT_EXPORT  void             GetBufferInfo            ( uint32_t iID, TL::Client::BUF_INFO *BufInfo, uint32_t *iSize );
      GC_IMPORT_EXPORT  void             GetBufferID              ( uint32_t iIndex, uint32_t *iID);

    private:

      /// pointer to the parent device class
      CDevice                             *m_pDev;
      /// Handle to the Datastream
      TL::Client::DS_HANDLE               m_hDs;

      /// Index of this image stream
      int32_t                             m_iIndex;

      /// GenApi TL NodeMap
      GenApi::INodeMap                    *m_TLNodeMap;

      /// GenApi Device Nodemap
      GenApi::INodeMap                    *m_DevNodeMap;


      typedef GC_ERROR      (GC_CALLTYPE *FG_DSAnnounceBuffer)        ( GenApi::INodeMap *TLNodeMap, GenApi::INodeMap *DevNodeMap, TL::Client::DS_HANDLE hDS, void *pBuffer, uint32_t iSize, void *pPrivate, uint32_t *iID );
      typedef GC_ERROR      (GC_CALLTYPE *FG_DSRevokeBuffer)          ( GenApi::INodeMap *TLNodeMap, GenApi::INodeMap *DevNodeMap, TL::Client::DS_HANDLE hDS, uint32_t iID, void **pBuffer, void **pPrivate);
      typedef GC_ERROR      (GC_CALLTYPE *FG_DSAllocAndAnnounceBuffer)( GenApi::INodeMap *TLNodeMap, GenApi::INodeMap *DevNodeMap, TL::Client::DS_HANDLE hDS, uint32_t iSize, void *pPrivate, void **pBuffer, uint32_t *iID );
      typedef GC_ERROR      (GC_CALLTYPE *FG_DSQueueBuffer)           ( GenApi::INodeMap *TLNodeMap, GenApi::INodeMap *DevNodeMap, TL::Client::DS_HANDLE hDS, uint32_t iID);
      typedef GC_ERROR      (GC_CALLTYPE *FG_DSFlushQueue)            ( GenApi::INodeMap *TLNodeMap, GenApi::INodeMap *DevNodeMap, TL::Client::DS_HANDLE hDS, TL::Client::ACQ_QUEUE_TYPE QueueType );
      typedef GC_ERROR      (GC_CALLTYPE *FG_DSStartAcquisition)      ( GenApi::INodeMap *TLNodeMap, GenApi::INodeMap *DevNodeMap, TL::Client::DS_HANDLE hDS, TL::Client::ACQ_START_FLAGS iAcquisitionFlags, uint64_t iNumImagesToAcquire );
      typedef GC_ERROR      (GC_CALLTYPE *FG_DSWaitForBuffer)         ( GenApi::INodeMap *TLNodeMap, GenApi::INodeMap *DevNodeMap, TL::Client::DS_HANDLE hDS, uint32_t iTimeoutMs, TL::Client::ACQ_WAIT_FLAGS iWaitFlags,  TL::Client::ACQ_INFO *AcqInfo, void **ppPrivate);
      typedef GC_ERROR      (GC_CALLTYPE *FG_DSStopAcquisition)       ( GenApi::INodeMap *TLNodeMap, GenApi::INodeMap *DevNodeMap, TL::Client::DS_HANDLE hDS, TL::Client::ACQ_STOP_FLAGS Flags );
      typedef GC_ERROR      (GC_CALLTYPE *FG_DSGetStatus)             ( GenApi::INodeMap *TLNodeMap, GenApi::INodeMap *DevNodeMap, TL::Client::DS_HANDLE hDS, TL::Client::ACQ_STATUS_CMD iCmd, void *pBuffer, uint64_t *iSize);
      typedef GC_ERROR      (GC_CALLTYPE *FG_DSGetBufferInfo)         ( GenApi::INodeMap *TLNodeMap, GenApi::INodeMap *DevNodeMap, TL::Client::DS_HANDLE hDS, uint32_t iID, TL::Client::BUF_INFO *BufInfo, uint32_t *iSize );
      typedef GC_ERROR      (GC_CALLTYPE *FG_DSClose)                 ( GenApi::INodeMap *TLNodeMap, GenApi::INodeMap *DevNodeMap, TL::Client::DS_HANDLE hDS);
      typedef GC_ERROR      (GC_CALLTYPE *FG_DSGetBufferID)           ( GenApi::INodeMap *TLNodeMap, GenApi::INodeMap *DevNodeMap, TL::Client::DS_HANDLE hDS, uint32_t iIndex, uint32_t *iID);


      FG_DSAnnounceBuffer         m_pfG_DSAnnounceBuffer;
      FG_DSRevokeBuffer           m_pfG_DSRevokeBuffer;
      FG_DSAllocAndAnnounceBuffer m_pfG_DSAllocAndAnnounceBuffer;
      FG_DSQueueBuffer            m_pfG_DSQueueBuffer;
      FG_DSFlushQueue             m_pfG_DSFlushQueue;
      FG_DSStartAcquisition       m_pfG_DSStartAcquisition;
      FG_DSWaitForBuffer          m_pfG_DSWaitForBuffer;
      FG_DSStopAcquisition        m_pfG_DSStopAcquisition;
      FG_DSGetStatus              m_pfG_DSGetStatus;
      FG_DSGetBufferInfo          m_pfG_DSGetBufferInfo;
      FG_DSClose                  m_pfG_DSClose;
      FG_DSGetBufferID            m_pfG_DSGetBufferID;

    };


  } // end of namespace Factory
} // end of namespace GC


#endif //GC_FACTORY_DATASTREAM_H_