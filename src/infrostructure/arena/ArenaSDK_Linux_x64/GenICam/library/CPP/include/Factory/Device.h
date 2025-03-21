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

 /// \file     $Source: /cvs/genicam/genicam/library/CPP/include/Factory/Device.h,v $
 /// \brief    Device class, wrapping a single device
 /// \version  $Revision: 1.9 $
 /// \date     $Date: 2006/07/18 13:31:52 $




#ifndef GC_FACTORY_DEVICE_H_
#define GC_FACTORY_DEVICE_H_

#include "GenApi/Types.h"
#include "GenApi/RegisterSetHelper.h"
#include "GenApi/Pointer.h"
#include "GenApi/NodeMapDynPtr.h"
#include "GenApi/INodeMapDyn.h"
#include "GenApi/Impl/Category.h"


#include <Factory/Factory.h>

#include <string>
#include <list>
#include <vector>
#include <utility>
#include <map>

#include <TLI/TLI.h>

using namespace std;

namespace GenICam
{
  namespace Factory
  {
    // Forward declarations
    class CFactory;
    class CTLIPort;
    class CImageStream;

    class CDevice
    {
    protected:
                                                          CDevice            ( CFactory *, TL::Client::TL_HANDLE, TL::Client::DEV_HANDLE, gcstring &strGluePath );
      virtual                                            ~CDevice            ( void );
    public:
      typedef gcstring_vector                             StringVector_t;
      typedef gcstring_vector::iterator                   StringVectorIter_t;

      ///  \addtogroup GCFDeviceDestruction GenICam Factory Device Destruction
      ///  @{

      GC_IMPORT_EXPORT  void                              CloseDevice         ( void );

      ///  @}

      ///  \addtogroup GCFNodeTRee GenICam Factory Device Destruction
      ///  @{

      virtual GC_IMPORT_EXPORT  GenApi::INodeMap       * GetDevNodeMap    ( void );
      virtual GC_IMPORT_EXPORT  GenApi::INodeMap       * GetTLNodeMap     ( void );
      virtual GC_IMPORT_EXPORT  GenApi::INode          * GetNode          ( const gcstring &PropName );
      ///  @}




      ///  \addtogroup GCFPropertyAccess GenICam Factory Property Access
      ///  @{
      //virtual GC_IMPORT_EXPORT  uint32_t                GetNumProperties       ( void );
      //virtual GC_IMPORT_EXPORT  const string            GetPropertyName        ( uint32_t iIndex );


      ///  @}

      ///  \addtogroup GCFAcquisition GenICam Factory Acquisition
      ///  @{
      virtual GC_IMPORT_EXPORT  uint32_t                GetNumDataStreams     ( void );
      virtual GC_IMPORT_EXPORT  void                    GetDataStream         ( uint32_t iIndex, CDataStream **pDataStream);
      ///  @}

      ///  \addtogroup GCFCallBacks GenICam Factory CallBack
      ///  @{
      virtual GC_IMPORT_EXPORT  void                    SetTLCallBack          ( TL::Client::TL_Callback pCB, void *pPrivate );
      virtual GC_IMPORT_EXPORT  void                    RemoveTLCallBack       ( TL::Client::TL_Callback pCB );
      virtual GC_IMPORT_EXPORT  void                    SetDevCallBack         ( TL::Client::TL_Callback pCB, void *pPrivate );
      virtual GC_IMPORT_EXPORT  void                    RemoveDevCallBack      ( TL::Client::TL_Callback pCB );
      ///  @}

    private:

      /// combines a GENAPI node to a Factory Property
      //typedef std::pair<GenApi::INode *, CProperty *>         PropSet_t;

      /// maps a short name to a long name
      typedef std::pair<std::string, std::string>                 NameSet_t;
      typedef std::vector<NameSet_t>                              NameVector_t;
      typedef std::vector<NameSet_t>::iterator                    NameVectorIter_t;

      // define friend class to allow CFactory  to create CDevice Object
      friend class CFactory;
      friend class CDataStream;

      /// Remove Datastream
      void          RemoveDatastream(CDataStream *pDS);

      /// Handle to the parent Factory
      CFactory                            *m_pFactory;
      /// Hanlde to the used TLI Interface
      TL::Client::TL_HANDLE                m_hTL;
      /// Handle to the TLI Device
      TL::Client::DEV_HANDLE               m_hDev;

      /// Device Port
      CTLIPort                             *m_pDevPort;

      /// GenApi NodeMap
        GenApi::INodeMapDyn                  *m_DevNodeMapDyn;
      /// GenApi RootNode
      GenApi::INodeMap                     *m_DevNodeMap;

      NameVector_t                          m_ShortNames;

      void                                  CreateDevNodeTree         ( void );

      typedef std::map<int32_t, CDataStream *>              StreamMap_t;
      typedef std::map<int32_t, CDataStream *>::iterator    StreamMapIter_t;

      /// Map of all instantiaited Data Streams
      StreamMap_t                           m_StreamMap;

      /// Module Handle for the Glue DLL
      HMODULE                               m_hGlueModule;

      /*
      typedef GC_ERROR      (GC_CALLTYPE *FG_GCReadData) ( GenApi::INodeMap *TLNodeMap, GenApi::INodeMap *DevNodeMap, TL::Client::TL_PORTHANDLE hPort, TL::Client::TL_ACCESS_MODE AccessMode, uint64_t iAddress, void *pBuffer, uint64_t *iSize);
      typedef GC_ERROR      (GC_CALLTYPE *FG_GCWriteData) ( GenApi::INodeMap *TLNodeMap, GenApi::INodeMap *DevNodeMap, TL::Client::TL_PORTHANDLE hPort, TL::Client::TL_ACCESS_MODE AccessMode, uint64_t iAddress, const void *pBuffer, uint64_t *iSize);
      typedef GC_ERROR      (GC_CALLTYPE *FG_GCGetErrorText) ( GenApi::INodeMap *TLNodeMap, GenApi::INodeMap *DevNodeMap, TL::Client::TL_HANDLE hTL, uint32_t iID, const char **ppErrText );
      typedef GC_ERROR      (GC_CALLTYPE *FG_TLInfo) ( GenApi::INodeMap *TLNodeMap, GenApi::INodeMap *DevNodeMap, const char *psPath, TL::Client::TLINFO *pInfo, uint32_t *piSize);
      typedef GC_ERROR      (GC_CALLTYPE *FG_TLOpen) ( GenApi::INodeMap *TLNodeMap, GenApi::INodeMap *DevNodeMap, pcchar8_t psPath, const char *psPrivateData, TL::Client::TL_HANDLE *pHandle);
      typedef GC_ERROR      (GC_CALLTYPE *FG_TLClose) ( GenApi::INodeMap *TLNodeMap, GenApi::INodeMap *DevNodeMap, TL::Client::TL_HANDLE hTL);
      typedef GC_ERROR      (GC_CALLTYPE *FG_TLGetXml)  ( GenApi::INodeMap *TLNodeMap, GenApi::INodeMap *DevNodeMap, TL::Client::TL_HANDLE hTL, char *psXML, uint32_t *iSize);
      typedef GC_ERROR      (GC_CALLTYPE *FG_TLGetNumInterfaces) ( GenApi::INodeMap *TLNodeMap, GenApi::INodeMap *DevNodeMap, TL::Client::TL_HANDLE hTL, uint32_t *iNumInterfaces );
      typedef GC_ERROR      (GC_CALLTYPE *FG_TLGetInterfaceName)      ( GenApi::INodeMap *TLNodeMap, GenApi::INodeMap *DevNodeMap, TL::Client::TL_HANDLE hTL, uint32_t iIndex, char * psName, uint32_t iSize );
      typedef GC_ERROR      (GC_CALLTYPE *FG_TLUpdateInterfaceList)  ( GenApi::INodeMap *TLNodeMap, GenApi::INodeMap *DevNodeMap, TL::Client::TL_HANDLE hTL, bool8_t *bHasChanged );
      typedef GC_ERROR      (GC_CALLTYPE *FG_TLGetNumDevices) ( GenApi::INodeMap *TLNodeMap, GenApi::INodeMap *DevNodeMap, TL::Client::TL_HANDLE hTL, const char *psInterfaceName, uint32_t *iNumDevices );
      typedef GC_ERROR      (GC_CALLTYPE *FG_TLGetDeviceName) ( GenApi::INodeMap *TLNodeMap, GenApi::INodeMap *DevNodeMap, TL::Client::TL_HANDLE hTL, const char *psInterfaceName, uint32_t iIndex, char * psName, uint32_t iSize  );
      typedef GC_ERROR      (GC_CALLTYPE *FG_TLUpdateDeviceList) ( GenApi::INodeMap *TLNodeMap, GenApi::INodeMap *DevNodeMap, TL::Client::TL_HANDLE hTL, const char *psInterfaceName, bool8_t *bHasChanged );
      typedef GC_ERROR      (GC_CALLTYPE *FG_TLOpenDevice) ( GenApi::INodeMap *TLNodeMap, GenApi::INodeMap *DevNodeMap, TL::Client::TL_HANDLE hTL, const char *sInterfaceName, const char *sDevName, TL::Client::DEV_HANDLE *hDev );

      typedef GC_ERROR      (GC_CALLTYPE *FG_DevGetXml) ( GenApi::INodeMap *TLNodeMap, GenApi::INodeMap *DevNodeMap, TL::Client::DEV_HANDLE hDev, char *pXML, uint32_t *iSize);
      */
      typedef GC_ERROR      (GC_CALLTYPE *FG_TLSetCallback) ( GenApi::INodeMap *TLNodeMap, GenApi::INodeMap *DevNodeMap, TL::Client::TL_HANDLE hTL, TL::Client::TL_Callback pCB, void *pPrivate);
      typedef GC_ERROR      (GC_CALLTYPE *FG_TLRemoveCallback) ( GenApi::INodeMap *TLNodeMap, GenApi::INodeMap *DevNodeMap, TL::Client::TL_HANDLE hTL, TL::Client::TL_Callback pCB );
      typedef GC_ERROR      (GC_CALLTYPE *FG_DevGetNumDataStreams) ( GenApi::INodeMap *TLNodeMap, GenApi::INodeMap *DevNodeMap, TL::Client::DEV_HANDLE hDev, uint32_t *piNumChannel);
      typedef GC_ERROR      (GC_CALLTYPE *FG_DevCreateDataStream) ( GenApi::INodeMap *TLNodeMap, GenApi::INodeMap *DevNodeMap, TL::Client::DEV_HANDLE hDev, uint32_t iChannel, TL::Client::DS_HANDLE *hDS);
      typedef GC_ERROR      (GC_CALLTYPE *FG_DevGetInfo) ( GenApi::INodeMap *TLNodeMap, GenApi::INodeMap *DevNodeMap, TL::Client::DEV_HANDLE hDev, TL::Client::DEVINFO *pInfo, uint32_t *iSize);
      typedef GC_ERROR      (GC_CALLTYPE *FG_DevSetCallback) ( GenApi::INodeMap *TLNodeMap, GenApi::INodeMap *DevNodeMap, TL::Client::DEV_HANDLE hDev, TL::Client::Dev_Callback pCB, void *pPrivate);
      typedef GC_ERROR      (GC_CALLTYPE *FG_DevRemoveCallback) ( GenApi::INodeMap *TLNodeMap, GenApi::INodeMap *DevNodeMap, TL::Client::DEV_HANDLE hDev, TL::Client::Dev_Callback pCB );
      //typedef GC_ERROR      (GC_CALLTYPE *FG_DevClose) ( GenApi::INodeMap *TLNodeMap, GenApi::INodeMap *DevNodeMap, TL::Client::DEV_HANDLE hDev );
      typedef GC_ERROR      (GC_CALLTYPE *FG_DevQueryCustomFeature)  ( GenApi::INodeMap *TLNodeMap, GenApi::INodeMap *DevNodeMap, TL::Client::DEV_HANDLE hDev, void *pInBuf, uint32_t iInSize, void *pOutBuf, uint32_t *iOutSize);

      FG_TLSetCallback          m_pfG_TLSetCallback;
      FG_TLRemoveCallback       m_pfG_TLRemoveCallback;
      FG_DevGetNumDataStreams   m_pfG_DevGetNumDataStreams;
      FG_DevCreateDataStream    m_pfG_DevCreateDataStream;
      FG_DevGetInfo             m_pfG_DevGetInfo;
      FG_DevSetCallback         m_pfG_DevSetCallback;
      FG_DevRemoveCallback      m_pfG_DevRemoveCallback;
      FG_DevQueryCustomFeature  m_pfG_DevQueryCustomFeature;

    };
  } // end of namespace Factory
} // end of namespace GC


#endif //GC_FACTORY_DEVICE_H_