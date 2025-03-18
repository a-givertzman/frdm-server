//-----------------------------------------------------------------------------
//  (c) 2014 by Basler Vision Technologies
//  Section: Vision Components
//  Project: NodeMapData
//  Author:  Fritz Dierks
//  $Header$
//
//  License: This file is published under the license of the EMVA GenICam  Standard Group.
//  A text file describing the legal terms is included in  your installation as 'GenICam_license.pdf'.
//  If for some reason you are missing  this file please contact the EMVA or visit the website
//  (http://www.genicam.org) for a full copy.
//
//  THIS SOFTWARE IS PROVIDED BY THE EMVA GENICAM STANDARD GROUP "AS IS"
//  AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,
//  THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
//  PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE EMVA GENICAM STANDARD  GROUP
//  OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,  SPECIAL,
//  EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT  LIMITED TO,
//  PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,  DATA, OR PROFITS;
//  OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY  THEORY OF LIABILITY,
//  WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT  (INCLUDING NEGLIGENCE OR OTHERWISE)
//  ARISING IN ANY WAY OUT OF THE USE  OF THIS SOFTWARE, EVEN IF ADVISED OF THE
//  POSSIBILITY OF SUCH DAMAGE.
//-----------------------------------------------------------------------------
/*!
\file
\brief    declaration of the functions exported by this DLL
*/
#ifndef NODEMAPDATA_H
#define NODEMAPDATA_H

#if defined (_MSC_VER)
#   ifdef NODEMAPDATA_EXPORTS
#       define NODEMAPDATA_DECL   __declspec(dllexport)
#   else
#       define NODEMAPDATA_DECL   __declspec(dllimport)
#   endif
#elif defined (__GNUC__)
#   ifdef NODEMAPDATA_EXPORTS
#       define NODEMAPDATA_DECL   __attribute__ ((visibility("default")))
#   else
#       define NODEMAPDATA_DECL   
#   endif
#else
#   error Unknown/unsupported platform
#endif

#include "GenApi/GenApiDll.h"
#include "Base/GCTypes.h"

#endif
