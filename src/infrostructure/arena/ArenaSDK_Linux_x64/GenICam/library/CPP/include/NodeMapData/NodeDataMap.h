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
#include "NodeMapDataTypes.h"
#include "NodeMapDataEnums.h"

#include <string>
#include <map>
#include <vector>
#include <assert.h>


//! List of NodeData objects which corresponds to a GenICam node map
/*! Flow during NodeData creation and registration

    #   Some node contains a pointer to another node which has not yet been loaded (optional step)
        -   Call NodeID = NodeMapData.GetNodeID( NodeName )
        -   NodeMapData checks if NodeName exists; if not creates one
        -   NodeMapData returns NodEID associated with the NodeName

    #   A new node is loaded (callback pre())
        -   A new NodeData object is created by the parser implementation
        -   The NodeData Object is filled with data
        -   When the loading is finished (callback post_XXX()) the NodeData object is set
            to the NodeMapData by calling SetNodeData( NodeID )
            The NodeID is retrieved during the Name() callback via GetNodeID function

*/

namespace GENAPI_NAMESPACE
{
    class CNodeMapFactory;
    class CNodeData;

    class NODEMAPDATA_DECL INodeDataMap
    {
    public:

        virtual NodeID_t GetNodeID(const std::string& NodeName, bool BuildMode = false) = 0;
        virtual const std::string &GetNodeName(const NodeID_t NodeID) const = 0;
        virtual StringID_t SetString(const std::string &String) = 0;
        virtual const std::string &GetString(const StringID_t StringID) = 0;

    };

    class NODEMAPDATA_DECL CNodeDataMap : public INodeDataMap
    {
        // The CNodeDataMapFactory contains the pre-processing code and needs to dig into the internal data structures
        friend class CNodeDataMapFactory;
        friend class GENAPI_NAMESPACE::CNodeMapFactory;

    public:

        /*!
            \brief Retrieves the NodeID belonging to a given \a NodeName

            If the NodeName was never presented to the NodeMapData up to this point a new
            NodeID is created and the NodeMame is remembered

            \param [in] NodeName The Name attribute of the node
            \return The NodeID belonging to Name
            */
        NodeID_t GetNodeID(const std::string& NodeName, bool BuildMode = false);

        /*!
            \brief Retrieves the NodeID belonging to a given \a NodeName.

            \param [in] NodeName The Name attribute of the node
            \return The NodeID belonging to \a NodeName; invalid if not present.
            */
        NodeID_t GetNodeID(const std::string& NodeName) const;

        //! Retrieves the name belonging to a NodeID
        const std::string &GetNodeName(const NodeID_t NodeID) const;

        /*!
            \brief Hands over the NodeData to the NodeDataMap where it is stored with the NodeID as key

            The NodeDataMap takes ownership of the NodeData and will destroy it later

            \param [in] pNodeData A pointer to the NodeData object
            \param [in] replace If true an existing NodeData is replaced
            */
        void SetNodeData(GENAPI_NAMESPACE::CNodeData* pNodeData, bool replace = false);

        //! Sets a string to the string table returning a StringID
        StringID_t SetString(const std::string &String);

        //! Retrieves a string belonging to a StringID 
        std::string &GetString(const StringID_t StringID);

        /*!
            \brief Returns a string representation of all NodeData objects stored in the NodeDataMap

            This is for debug proposes only

            \return The string representation of the NodeDataMap
            */
        GENICAM_NAMESPACE::gcstring ToString(GENAPI_NAMESPACE::EToStringStyle_t ToStringStyle = tsText) const;

        //! Write to cache stream
        void ToFile(std::ostream &OutStream);

        //! Read from cache stream
        void FromFile(std::istream &InStream);

        //! prints a NodeDataMap
        static void Print(std::ostream &Out, const CNodeDataMap *pNodeDataMap, GENAPI_NAMESPACE::EToStringStyle_t ToStringStyle = tsText);


        //! Sets a pointer to the node which is currently parsed
        /*! This is used for the embedded nodes to know what their parent node is */
        void SetContextNode(CNodeData *pContextNode)
        {
            //OutputDebugString( (Name + "\n").c_str());
            assert((pContextNode == NULL && m_pContextNode != NULL)
                || (pContextNode != NULL && m_pContextNode == NULL));
            m_pContextNode = pContextNode;
        }

        //! Retrieves the context node
        CNodeData *GetContextNode()
        {
            return m_pContextNode;
        }

        //! Retrieves the node last added
        /*! This is used by nodes to know about embedded nodes */
        CNodeData *GetLastAddedNode();

        //! Retrieves the context node
        CNodeData *GetNodeData(NodeID_t NodeID)
        {
            return (*m_pNodeID2NodeData)[static_cast<size_t>(NodeID.ToIndex())];
        }

        //! Retrieves the context node
        const CNodeData *GetNodeData(NodeID_t NodeID) const
        {
            return (*m_pNodeID2NodeData)[static_cast<size_t>(NodeID.ToIndex())];
        }

        //! Iterator access to NodeData objects
        typedef std::vector<CNodeData*> NodeID2NodeData_t;

        NodeID2NodeData_t::iterator NodeDataBegin()
        {
            return m_pNodeID2NodeData->begin();
        }

        NodeID2NodeData_t::const_iterator NodeDataBegin() const
        {
            return m_pNodeID2NodeData->begin();
        }

        NodeID2NodeData_t::iterator NodeDataEnd()
        {
            return m_pNodeID2NodeData->end();
        }

        NodeID2NodeData_t::const_iterator NodeDataEnd() const
        {
            return m_pNodeID2NodeData->end();
        }

        //! Beware: is valid only after parsing has finished
        size_t GetNumNodes() const
        {
            return m_pNodeID2NodeData->size();
        }

        std::string GetCacheFileName()
        {
            assert(m_pCacheFileName);
            return *m_pCacheFileName;
        }

        void SetCacheFileName(std::string &CacheFileName)
        {
            assert(m_pCacheFileName);
            *m_pCacheFileName = CacheFileName;
        }


        void Clear();

        void Inject(CNodeDataMap& InjectedNodeDataMap);
        void ExtractIndependentSubtree(CNodeDataMap& toExtractTree, const std::string& SubTreeRootNodeName, bool doRenameToRoot) const;

        typedef struct
        {
            uint32_t NumNodes;
            uint32_t NumProperties;
            uint32_t NumLinks;
            uint32_t NumStrings;
        } NodeStatistics_t;

        void GetNodeStatistics(NodeStatistics_t &NodeStatistics);

        //! Will fire an event if the data structures are not consistent
        void CheckConsistency();


        // Old versions of GCC (3.4.x and lower) cannot compile with these protected members
#if defined(__GNUC__) && GENICAM_GCC_VERSION < 30500
    public:
#else
    protected:
#endif

        typedef std::map<std::string, NodeID_t> NodeName2NodeID_t;
        typedef std::vector<std::string> NodeID2NodeName_t;
        typedef std::vector<std::string> StringID2String_t;

        //! Pointer to a map associating the NodeID with the NodeName as key
        NodeName2NodeID_t *m_pNodeName2NodeID;

        //! Pointer to a vector associating the NodeName with the NodeID as key
        NodeID2NodeName_t *m_pNodeID2NodeName;

        //! Pointer to a vector associating the NodeData object with the NodeID as key
        NodeID2NodeData_t *m_pNodeID2NodeData;

        //! Pointer to a vector associating Strings with a StringID as key
        StringID2String_t *m_pStringID2String;

        //! if a complex element is embedded in a node this pointer temporarily holds the NodeData element of
        //! the outer node
        CNodeData *m_pContextNode;

        //! The NodeID of the most recently added NodeData object
        NodeID_t m_LastAddedNodeID;
     private:   

         //! The path to the cache file
        std::string *m_pCacheFileName;

        // Constructor/Destructor
    public:
        //! Constructor
        CNodeDataMap();

        //! Destructor
        virtual ~CNodeDataMap();

    };

}
