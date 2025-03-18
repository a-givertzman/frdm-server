//-----------------------------------------------------------------------------
//  (c) 2006 by Basler Vision Technologies
//  Section: Vision Components
//  Project: GenApi
//  Author:  Margret Albrecht
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
    \brief    Definition of macros for cross-platform compatibility (internal part)
    \ingroup GenApi_Implementation
*/

#ifndef GENAPI_COMPATIBILITY_H
#   include <GenApi/Compatibility.h>
#endif

#ifndef GENAPI_IMPL_COMPATIBILITY_H
#define GENAPI_IMPL_COMPATIBILITY_H
#if defined (_MSC_VER) && defined (_WIN32)

#ifdef PHARLAP_WIN32
    #pragma comment( lib, "wsock32.lib")
#else
    #pragma comment( lib, "ws2_32.lib")
#endif

#include <winsock.h>

#define __LITTLE_ENDIAN 1234
#define __BIG_ENDIAN    4321
#if defined( _M_IX86 ) || defined( _M_X64 ) // *JS*
    #define __BYTE_ORDER __LITTLE_ENDIAN
#else
    #error Unsupported architecture
#endif

#include <guiddef.h>

#pragma intrinsic(_byteswap_ushort)
#pragma intrinsic(_byteswap_ulong)
#pragma intrinsic(_byteswap_uint64)
#define bswap_16(s) _byteswap_ushort(s)
#define bswap_32(s) _byteswap_ulong(s)
#define bswap_64(s) _byteswap_uint64(s)

#define _DEFINE_GUID(name, l, w0, w1, c0, c1, c2, c3, c4, c5, c6, c7) \
    GUID name = { l, w0, w1, { c0, c1, c2, c3, c4, c5, c6, c7 } };

#elif defined (__GNUC__) && (defined (__linux__) || defined (__APPLE__) || defined(VXWORKS))

#include <string.h>
#include <netinet/in.h>
  #if defined(__linux__)
    #include <endian.h>
    #include <byteswap.h>
  #elif defined(__APPLE__) && defined(__DARWIN_BYTE_ORDER)
    #define __LITTLE_ENDIAN 1234
    #define __BIG_ENDIAN    4321
    #if  __DARWIN_BYTE_ORDER == __DARWIN_BIG_ENDIAN
      #error Unsupported architecture
    #else
      #define __BYTE_ORDER __LITTLE_ENDIAN
      inline long long swapbytes64(const long long ll) {
          int lowBytes = 0x00000000ffffffff & ll;
          int highBytes = (0xffffffff00000000 & ll) >> 32;
          lowBytes = ntohl(lowBytes);
          highBytes = ntohl(highBytes);
          return (static_cast<long long>(lowBytes) << 32) | 
                  static_cast<long long>(highBytes);
      }
      #define bswap_16(s) ntohs(s)
      #define bswap_32(l) ntohl(l)
      #define bswap_64(ll) swapbytes64(ll)
    #endif
  #elif defined (VXWORKS)
      #define bswap_16(s) ((((s) << 8) & 0xff00U) | (((s) >> 8) & 0x00ffU))
      #define bswap_32(l) ((((l) << 24) & 0xff000000) | (((l) << 8) & 0x00ff0000) | (((l) >> 8) & 0x0000ff00) | (((l) >> 24) & 0x000000ff))
      #define bswap_64(ll) (                          \
           (((ll) << 56) & 0xff00000000000000LL) |    \
           (((ll) << 40) & 0x00ff000000000000LL) |    \
           (((ll) << 24) & 0x0000ff0000000000LL) |    \
           (((ll) <<  8) & 0x000000ff00000000LL) |    \
           (((ll) >>  8) & 0x00000000ff000000LL) |    \
           (((ll) >> 24) & 0x0000000000ff0000LL) |    \
           (((ll) >> 40) & 0x000000000000ff00LL) |    \
           (((ll) >> 56) & 0x00000000000000ffLL))
  
    #define __LITTLE_ENDIAN 1234
    #define __BIG_ENDIAN    4321
    #define __BYTE_ORDER __BIG_ENDIAN
  #else // !linux and !apple, or apple but __DARWIN_BYTE_ORDER undefined
    #error Unsupported architecture
  #endif

#define _snprintf snprintf
#define _strtoi64 strtoll
#define _vsnprintf vsnprintf
#define _strdup strdup
#ifndef VXWORKS
   #include <cmath>
   #define _isnan(val) std::isnan(val)
#else
#  include <private/mathP.h>
#  define _isnan isNan
#endif

// these definitions break the stl-base code and should not be used!
//#define min(x, y) (((x) < (y)) ? (x) : (y))
//#define max(x, y) (((x) > (y)) ? (x) : (y))


#if (__BYTE_ORDER == __BIG_ENDIAN)
    #define _DEFINE_GUID(name, l, w0, w1, c0, c1, c2, c3, c4, c5, c6, c7)   \
        GUID name = { bswap_32(l), bswap_16(w0), bswap_16(w1), { c0, c1, c2, c3, c4, c5, c6, c7 } }
#else
    #define _DEFINE_GUID(name, l, w0, w1, c0, c1, c2, c3, c4, c5, c6, c7)   \
        GUID name = { l, w0, w1, { c0, c1, c2, c3, c4, c5, c6, c7 } }
#endif

#define IsEqualGUID(a, b) (!memcmp(&a, &b, sizeof (GUID)))

inline bool
operator ==(const GUID &a, const GUID &b)
{
    return IsEqualGUID(a, b);
}

#else
#   error Unsupported platform
#endif

#if !(defined(__BYTE_ORDER) && defined(__LITTLE_ENDIAN) && defined(__BIG_ENDIAN))
  #error Byte order macros must be defined.
#endif

/* Endianness handling, using "own" names since there are too many
 * "standards" around. */
#if __BYTE_ORDER == __LITTLE_ENDIAN
  /* Little endian to host */
  #define letoh_16(val) (val)
  #define letoh_32(val) (val)
  #define letoh_64(val) (val)
  /* Host to little endian */
  #define htole_16(val) (val)
  #define htole_32(val) (val)
  #define htole_64(val) (val)
  /* Big endian to host */
  #define betoh_16(val) bswap_16(val)
  #define betoh_32(val) bswap_32(val)
  #define betoh_64(val) bswap_64(val)
  /* Host to big endian */
  #define htobe_16(val) bswap_16(val)
  #define htobe_32(val) bswap_32(val)
  #define htobe_64(val) bswap_64(val)
#else  /* __BYTE_ORDER == __LITTLE_ENDIAN */
  /* Little endian to host */
  #define letoh_16(val) bswap_16(val)
  #define letoh_32(val) bswap_32(val)
  #define letoh_64(val) bswap_64(val)
  /* Host to little endian */
  #define htole_16(val) bswap_16(val)
  #define htole_32(val) bswap_32(val)
  #define htole_64(val) bswap_64(val)
  /* Big endian to host */
  #define betoh_16(val) (val)
  #define betoh_32(val) (val)
  #define betoh_64(val) (val)
  /* Host to big endian */
  #define htobe_16(val) (val)
  #define htobe_32(val) (val)
  #define htobe_64(val) (val)
#endif /* __BYTE_ORDER == __LITTLE_ENDIAN */

#endif // ifndef GENAPI_IMPL_COMPATIBILITY_H
