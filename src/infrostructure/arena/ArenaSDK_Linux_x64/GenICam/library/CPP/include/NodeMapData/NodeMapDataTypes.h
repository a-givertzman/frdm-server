//-----------------------------------------------------------------------------
//  (c) 2014 by Basler 
//  Project: GenApi
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

#pragma once

#include "NodeMapData/NodeMapData.h"

#include <GenApi/Types.h>  //WARNING: NodeMapData must not have a dependency to GenApi. Only using PODs and Enums here.
#include <Base/GCBase.h>   //Using GcBase for exceptions.

namespace GENAPI_NAMESPACE
{

    const int32_t InvalidID = -1;

    struct NODEMAPDATA_DECL  StringID_t
    {
        typedef int32_t value_type;

        StringID_t()
            : index(InvalidID)
        {
        }

        StringID_t(const StringID_t& rhs)
            : index(rhs.index)
        {
        }

        StringID_t& operator=(const StringID_t& rhs)
        {
            index = rhs.index;
            return *this;
        }

        bool operator==(const StringID_t& rhs) const
        {
            return index == rhs.index;
        }

        bool operator!=(const StringID_t& rhs) const
        {
            return index != rhs.index;
        }

        bool operator<(const StringID_t& rhs) const
        {
            return index < rhs.index;
        }

        bool operator<=(const StringID_t& rhs) const
        {
            return index <= rhs.index;
        }

        bool operator>(const StringID_t& rhs) const
        {
            return index > rhs.index;
        }

        bool operator>=(const StringID_t& rhs) const
        {
            return index >= rhs.index;
        }

        static StringID_t FromIndex(size_t i)
        {
            StringID_t sid;
            sid.index = static_cast<value_type>(i);
            return sid;
        }

        static StringID_t FromIndex(value_type i)
        {
            StringID_t sid;
            sid.index = i;
            return sid;
        }

        value_type ToIndex() const
        {
            return index;
        }

        bool IsValid() const
        {
            return index >= 0;
        }

    private:
        value_type index;
    };



    typedef int64_t IntNumber_t;
    typedef double FloatNumber_t;

    struct NODEMAPDATA_DECL NodeID_t
    {
        typedef int32_t value_type;

        NodeID_t()
            : index(InvalidID)
        {
        }

        NodeID_t(const NodeID_t& rhs)
            : index(rhs.index)
        {
        }

        NodeID_t& operator=(const NodeID_t& rhs)
        {
            index = rhs.index;
            return *this;
        }

        bool operator==(const NodeID_t& rhs) const
        {
            return index == rhs.index;
        }

        bool operator<(const NodeID_t& rhs) const
        {
            return index < rhs.index;
        }

        bool operator!=(const NodeID_t& rhs) const
        {
            return index != rhs.index;
        }

        static NodeID_t FromIndex(size_t i)
        {
            NodeID_t sid;
            sid.index = static_cast<value_type>(i);
            return sid;
        }

        static NodeID_t FromIndex(value_type i)
        {
            NodeID_t sid;
            sid.index = i;
            return sid;
        }

        value_type ToIndex() const
        {
            return index;
        }

        bool IsValid() const
        {
            return index >= 0;
        }

    private:
        value_type index;
    };

}
