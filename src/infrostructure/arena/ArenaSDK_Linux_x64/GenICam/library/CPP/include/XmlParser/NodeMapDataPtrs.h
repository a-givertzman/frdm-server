#pragma once

#include "../NodeMapData/NodeData.h"

namespace GENAPI_NAMESPACE
{

    class CNodeDataMap;

    class CNodeMapDataPtrs
    {
    public:
        CNodeMapDataPtrs()
            : pNodeData(0)
            , pNodeDataMap(0)
            , SuppressStringsOnLoading(false)
        {
        }

        ~CNodeMapDataPtrs()
        {
        }

        CNodeMapDataPtrs(const CNodeMapDataPtrs& rhs)
            : pNodeData(rhs.pNodeData)
            , pNodeDataMap(rhs.pNodeDataMap)
            , SuppressStringsOnLoading(rhs.SuppressStringsOnLoading)
        {
        }

        CNodeMapDataPtrs& operator=(const CNodeMapDataPtrs& rhs)
        {
            pNodeData = rhs.pNodeData;
            pNodeDataMap = rhs.pNodeDataMap;
            SuppressStringsOnLoading = rhs.SuppressStringsOnLoading;
            return *this;
        }

        virtual void reset()
        {
            pNodeData = NULL;
        }

        CNodeData* pNodeData;
        CNodeDataMap* pNodeDataMap;
        bool SuppressStringsOnLoading;
    };


    class CNodeMapDataPtrsWithCleanup : public CNodeMapDataPtrs
    {
    public:
        CNodeMapDataPtrsWithCleanup()
        {
        }

        ~CNodeMapDataPtrsWithCleanup()
        {
            reset();
        }

        virtual void reset()
        {
            if (pNodeData)
            {
                //delete in case if parsing is stopped throwing an exception
                delete pNodeData;
                pNodeData = 0;
            }
        }

        CNodeMapDataPtrsWithCleanup(const CNodeMapDataPtrs& rhs)
            : CNodeMapDataPtrs(rhs)
        {
        }


        CNodeMapDataPtrsWithCleanup& operator=(const CNodeMapDataPtrs& rhs)
        {
            pNodeData = rhs.pNodeData;
            pNodeDataMap = rhs.pNodeDataMap;
            SuppressStringsOnLoading = rhs.SuppressStringsOnLoading;
            return *this;
        }
    };


    class CNodeMapDataPtrsForConverterWithCleanup : public CNodeMapDataPtrsWithCleanup
    {
    public:
        CNodeMapDataPtrsForConverterWithCleanup()
            : pConverterTo(0)
            , pConverterFrom(0)
        {
        }

        ~CNodeMapDataPtrsForConverterWithCleanup()
        {
            reset();
        }

        virtual void reset()
        {
            if (pConverterTo)
            {
                //delete in case if parsing is stopped throwing an exception
                delete pConverterTo;
                pConverterTo = 0;
            }
            if (pConverterFrom)
            {
                //delete in case if parsing is stopped throwing an exception
                delete pConverterFrom;
                pConverterFrom = 0;
            }
        }

        CNodeMapDataPtrsForConverterWithCleanup(const CNodeMapDataPtrsForConverterWithCleanup& rhs)
            : CNodeMapDataPtrsWithCleanup(rhs)
            , pConverterTo(rhs.pConverterTo)
            , pConverterFrom(rhs.pConverterFrom)
        {
        }


        CNodeMapDataPtrsForConverterWithCleanup(const CNodeMapDataPtrs& rhs)
            : CNodeMapDataPtrsWithCleanup(rhs)
            , pConverterTo(0)
            , pConverterFrom(0)
        {
        }


        CNodeMapDataPtrsForConverterWithCleanup& operator=(const CNodeMapDataPtrsForConverterWithCleanup& rhs)
        {
            pNodeData = rhs.pNodeData;
            pConverterTo = rhs.pConverterTo;
            pConverterFrom = rhs.pConverterFrom;
            pNodeDataMap = rhs.pNodeDataMap;
            SuppressStringsOnLoading = rhs.SuppressStringsOnLoading;
            return *this;
        }

        CNodeData* pConverterTo;
        CNodeData* pConverterFrom;
    };

}
