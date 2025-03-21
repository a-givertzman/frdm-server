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

/// \file     $Source: /cvs/genicam/genicam/library/CPP/include/Factory/Registry/RegRegistry.h,v $
/// \brief    GenICam Registry definition
/// \version  $Revision: 1.7 $
/// \date     $Date: 2006/07/18 13:31:52 $




#ifndef GC_REGISTRY_DEF
#define GC_REGISTRY_DEF

#include "Base/GCBase.h"

#include <string>

#include <xercesc/parsers/XercesDOMParser.hpp>
#include <xercesc/dom/DOM.hpp>
#include <xercesc/sax/HandlerBase.hpp>
#include <xercesc/util/XMLString.hpp>
#include <xercesc/util/PlatformUtils.hpp>

#ifdef _DEBUG
    #pragma comment(lib, "xerces-c_2D.lib")
#else
    #pragma comment(lib, "xerces-c_2.lib")
#endif


#include <Factory/Registry/RegTLClient.h>
#include <Factory/Registry/RegGlue.h>
#include <Factory/Registry/RegDevice.h>
#include <Factory/Registry/RegXmlDescription.h>

namespace GenICam {
  namespace Registry {

    #define TRANSPORTINTERFACETYPE_GENERIC      "Generic"
    #define TRANSPORTINTERFACETYPE_IEEE1394     "IEEE1394"
    #define TRANSPORTINTERFACETYPE_GIGEVISION   "GigEVision"
    #define TRANSPORTINTERFACETYPE_CAMLINK      "CamLink"

    class GCRegistry
    {
    private:
    public:
                              GCRegistry                    ( void );
      virtual                ~GCRegistry                    ( void );

      virtual void            GetTLClientList               ( gcstring_vector  &TLCList );

      virtual void            GetTLClientInfo               ( const gcstring &gstrString, RegTLClientInfo &TLInfo);

      virtual void            GetGlueInfo                   ( const gcstring &gstrFQName, RegGlueInfo &GlueInfo);

      virtual void            GetDevInfo                    ( const gcstring &gstrFQName, RegDeviceInfo &DevInfo);

      virtual void            GetXmlInfo                    ( const gcstring &gstrFQName, RegXmlInfo &XmlInfo);

      virtual bool8_t         HasBinding                    ( const gcstring &gstrDevId);

    private:
      /// Name of the Registry file until we move to XML
      gcstring                m_strGCRegIniFile;

      /// GenICam_Root
      gcstring                m_strGenICamRoot;


      /// XERCES Parser
      XERCES_CPP_NAMESPACE::XercesDOMParser               *m_pParser;
      /// XERCES
          XERCES_CPP_NAMESPACE::ErrorHandler                  *m_pErr;

    };

  }
}


#endif //GC_REGISTRY__DEF

