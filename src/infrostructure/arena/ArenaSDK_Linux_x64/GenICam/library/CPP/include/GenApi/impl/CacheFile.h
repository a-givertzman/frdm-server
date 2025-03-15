#pragma once

#include <Base/GCTypes.h>
#include <GenICamFwd.h>
#include <iostream>

#if (_MSC_VER>=1600)
#define STATIC_ASSERT(COND,MSG) static_assert( (COND), #MSG )
#else
#define STATIC_ASSERT(COND,MSG) typedef char static_assertion_##MSG[(COND)?1:-1]
#endif
namespace GENAPI_NAMESPACE
{
#pragma pack(push,1)
    struct Preamble
    {
        GENICAM_NAMESPACE::Version_t version;
        char magic[40];
    };
#pragma pack(pop)
    STATIC_ASSERT(sizeof (Preamble) == 46, Size_of_Preamble_unexpected);

    inline bool operator == (const Preamble& lhs, const Preamble& rhs)
    {
        bool ret = true;
        if (memcmp( lhs.magic, rhs.magic, sizeof rhs.magic ) != 0 ||
            lhs.version.Major != rhs.version.Major ||
            lhs.version.Minor != rhs.version.Minor ||
            lhs.version.SubMinor != rhs.version.SubMinor)
        {
            ret = false;
        }

        return ret;

    }
    inline bool operator !=(const Preamble& lhs, const Preamble& rhs)
    {
        const bool equal(lhs == rhs);
        return ! equal;
    }
    const Preamble& CacheFilePreamble();

    std::ostream& WritePreamble(std::ostream& os, const Preamble&p );
    std::istream& ReadPreamble(std::istream& is, Preamble &p);

    inline const Preamble& CacheFilePreamble()
    {
        static const Preamble g_CacheFilePreamble =
        {
            { GENICAM_VERSION_MAJOR, GENICAM_VERSION_MINOR, GENICAM_VERSION_SUBMINOR },
            /* On multi-arch machines, we've observed problems when multiple
             * archs shared the same cache and locking, so we avoid the clash.
             * This means however that it's the user's responsibility to avoid
             * that processes of multiple archs interfere on such machines. */
#define GUID64 "{2E0E4C8C-EC35-407F-982B-0990B3499701}"
#define GUID32 "{C248B50C-452B-430C-B8CB-E112BDF30571}"
#if defined (_MSC_VER)
#   if defined (_WIN64)
            GUID64
#   else
            GUID32
#   endif
#elif defined (__GNUC__)
#   if defined (__LP64__)
#      if defined (__linux__)
            GUID64
#      elif defined (__APPLE__)
            GUID64
#      else
#       error Unknown Platform
#      endif
#   else
#      if defined (__linux__)
            GUID32
#      elif defined (__APPLE__)
#       error Unsupported Platform
#      elif defined (VXWORKS)
            GUID32
#      else
#       error Unknown Platform
#      endif
#   endif
#else
#   error Unknown Platform
#endif
        };

        return g_CacheFilePreamble;
    }

    inline std::ostream& WritePreamble(std::ostream& os, const Preamble&p/*=CacheFilePreamble() */)
    {
        os.write(reinterpret_cast<const char*>(&p), sizeof p);

        return os;

    }

    inline std::istream& ReadPreamble(std::istream& is, Preamble &p)
    {
        is.read(reinterpret_cast<char*>(&p), sizeof p);

        if (is.gcount() != sizeof p)
        {
            is.setstate(std::ios_base::badbit);
        }
 
        return is;
    }
    inline std::istream& ReadPreambleAndCheckCurrentVersion(std::istream& is, Preamble &p)
    {
        is.read(reinterpret_cast<char*>(&p), sizeof p);

        if (p != CacheFilePreamble())
        {
            is.setstate(std::ios_base::badbit);
        }

        return is;
    }


}

