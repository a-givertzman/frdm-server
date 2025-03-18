
#pragma once
#ifndef INCLUDED_GENICAMROOT_H
#define INCLUDED_GENICAMROOT_H

#include <Base/GCUtilities.h>
#include <Base/GCString.h>
#include <vector>
#include <string>

namespace GENICAM_NAMESPACE
{
    inline GenICam::gcstring GetGenICamRootFolderSafe(void)
    {
#if defined(VXWORKS)
        gcstring path;
        if (GetValueOfEnvironmentVariable(GENICAM_ROOT, path))
        {
            return path;
        }
        else
        {
            throw RUNTIME_EXCEPTION("GenICam root folder not set. Set the environment variable " GENICAM_ROOT);
        }
#else
        gcstring path;
        // determine value of GENICAM_ROOT based on module's location
        // assuming a canonical installation
        // <GenICam_ROOT>\bin\win32_x86\GCBase_xxx.dll
        gcstring const p = GetModulePathFromFunction(reinterpret_cast<void*>(&GetGenICamRootFolderSafe));
        gcstring const delimiters = "/\\";
        gcstring_vector v;
        v.reserve(16);
        Tokenize(p, v, delimiters);

        if (v.size() > 3)
        {
#if ! defined(_WIN32)
            path += "/";
#endif
            path += v.front();
            for (gcstring_vector::const_iterator it = v.begin()+1, end = v.end() - 3; it != end; ++it)
            {
                path += "/";
                path += *it;
            }
            return path;
        }

        // default : same directory
        return p.substr( 0, p.size() - v.back().size() - 1);
#endif
    }

}

#endif // INCLUDED_GENICAMROOT_H
