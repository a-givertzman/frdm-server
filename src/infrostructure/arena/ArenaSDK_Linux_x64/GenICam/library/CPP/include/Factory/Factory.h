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

 /// \file     $Source: /cvs/genicam/genicam/library/CPP/include/Factory/Factory.h,v $
 /// \brief    Factory class
 /// \version  $Revision: 1.8 $
 /// \date     $Date: 2006/07/18 13:31:52 $




#ifndef GC_FACTORY_H_
#define GC_FACTORY_H_

#include <string>
#include <vector>
#include <map>

#include "Base/GCBase.h"

#include "GenApi/NodeMapDynPtr.h"

#include <TLI/Platform.h>
#include <TLI/Client.h>
#include <TLI/System.h>

#include <Factory/DataStream.h>
#include <Factory/Device.h>

#include <Factory/Registry/RegRegistry.h>



#if defined  _WIN32 && !defined GC_DONTLINKFACTORY
  #ifdef _DEBUG
    #ifdef _MT
      #ifdef _DLL
        #pragma message ("Using : MultiThreaded Dll Factory Debug LIB")
        #pragma comment(lib, "GCFactory_MDd_VC71.lib")
        #pragma message ("Using : MultiThreaded Dll Base Debug LIB")
        #pragma comment(lib, "GCBase_MDd_VC71.lib")
      #else
        #error MTd library not available

      #endif
    #else
      #ifdef _DLL
        #error Invalid library setup
      #else
        #error MLs library not available
      #endif
    #endif
  #else
    #ifdef _MT
      #ifdef _DLL
        #pragma message ("Using : MultiThreaded Dll Factory LIB")
        #pragma comment(lib, "GCFactory_MD_VC71.lib")
        #pragma message ("Using : MultiThreaded Dll Base LIB")
        #pragma comment(lib, "GCBase_MD_VC71.lib")
      #else
        #error MT library not available

      #endif
    #else
      #ifdef _DLL
        #error Invalid library setup
      #else
        #error ML library not available
      #endif
    #endif
  #endif

#endif



namespace GenICam
{
  namespace Factory
  {
    // Handles
    typedef void *FACTORY_HANDLE;

    class CTLIPort;

    class CFactory
    {
      friend class CDevice;

    public:
      ///  \addtogroup GCFFacCreate GenICam Factory Create / Destroy
      ///  @{
      GC_IMPORT_EXPORT  static CFactory     *CreateFactory          ( void );
      GC_IMPORT_EXPORT  void                DestroyFactory          ( void );
      ///  @}


      ///  \addtogroup GCFDevEnum GenICam Factory Enum Devices
      ///  @{
      virtual GC_IMPORT_EXPORT  void        UpdateDeviceList        ( void );
      virtual GC_IMPORT_EXPORT  uint32_t    GetNumDevices           ( void );

      virtual GC_IMPORT_EXPORT  gcstring    GetDeviceName           ( uint32_t iIndex );
      virtual GC_IMPORT_EXPORT  bool        DeviceHasBinding        ( const gcstring &gstrDevId );
      virtual GC_IMPORT_EXPORT  void        GetDeviceList           ( gcstring_vector &gstrvDevList );
      //virtual GC_IMPORT_EXPORT  const char *operator[]              ( int32_t iIndex );

      ///  @}


      ///  \addtogroup GCFDevCreate GenICam Factory Devices instatiation
      ///  @{
      virtual GC_IMPORT_EXPORT   CDevice    *OpenDevice             ( gcstring     &sName );
      ///  @}
    public:
      class CDeviList;

    public:

      GC_IMPORT_EXPORT                      CFactory                ( void );
      virtual GC_IMPORT_EXPORT              ~CFactory               ( void );

    private:

      /// Information on a specific Device
      class CDevLocation
      {
      public:
                        CDevLocation      ( const char *sTLName,
                                            const char *sInterfaceName,
                                            const char *sDevName,
                                            const char *sTLFilePathName,
                                            const char *sTLGlueDll,
                                            const char *sTLPrivateData);

        virtual         ~CDevLocation     ( void );

        /// TL Name
        gcstring                           m_strTLName;
        /// Interface Name
        gcstring                           m_strInterfaceName;
        /// Device Name
        gcstring                           m_strDevName;

        /// Path to TL
        gcstring                           m_strTLFilepath;

        /// Path to GlueDll
        gcstring                           m_strTLGlueDllName;

        /// TL Private Data
        gcstring                           m_strTLPrivateData;

      };

      /// TL Instance
      /// Only one Instance per TL Possible
      class CTLInstance
      {
      public:


                                            CTLInstance         (TL::Client::TL_HANDLE hTL, gcstring     strName);
        virtual                            ~CTLInstance         ( void );

        /// TL Handle
        TL::Client::TL_HANDLE               m_hTL;
        /// TL Name
        gcstring                            m_strName;

        /// GenApi Interface to TL NodeMap
          GenApi::INodeMapDyn                *m_TLNodeMapDyn;     // Balser NodeMap


        /// TL Port
        CTLIPort                           *m_pTLPort;


      private:
        void                                CreateTLNodeTree      ( void );

      };
      virtual CTLInstance                  *GetTLInstance         ( TL::Client::TL_HANDLE hTL){ return m_TLMap[hTL]; };

      typedef std::map<gcstring    , CDevLocation *>::iterator   DevMapIter_t;
      typedef std::map<gcstring    , CDevLocation *>             DevMap_t;

      DevMap_t                                                   m_DevMap;

      typedef std::map<TL::Client::TL_HANDLE, CTLInstance*>             TLMap_t;
      typedef std::map<TL::Client::TL_HANDLE, CTLInstance*>::iterator   TLMapIter_t;

      TLMap_t                                                    m_TLMap;


    private:
      /// GenICam Registry
      Registry::GCRegistry              *m_Registry;

    };

  } // end of namespace Factory
} // end of namespace GC


#endif //GC_FACTORY_H_
