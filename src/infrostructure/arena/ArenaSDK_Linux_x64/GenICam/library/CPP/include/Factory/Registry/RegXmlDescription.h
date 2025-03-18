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

/// \file     $Source: /cvs/genicam/genicam/library/CPP/include/Factory/Registry/RegXmlDescription.h,v $
/// \brief    GenICam Registry XML description
/// \version  $Revision: 1.6 $
/// \date     $Date: 2006/07/18 13:31:52 $



#ifndef GC_REGISTRY__XMLDESCRIPTION_DEF
#define GC_REGISTRY__XMLDESCRIPTION_DEF


#include "Base/GCBase.h"


namespace GenICam {
  namespace Registry {

    class RegXmlInfo
    {
    public:
      gcstring          m_strPath;

      gcstring          m_strToolTip;

    private:

    };

  }
}


#endif //GC_REGISTRY__XMLDESCRIPTION_DEF