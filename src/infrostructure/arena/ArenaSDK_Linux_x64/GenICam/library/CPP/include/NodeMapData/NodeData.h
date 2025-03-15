//-----------------------------------------------------------------------------
//  (c) 2014 by Basler 
//  Project: GenApi
//  Author:  Fritz Dierks
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
#include <GenApi/GenApiDll.h>
#include "NodeMapDataEnums.h"
#include "NodeMapDataTypes.h"
#include "Property.h"

#if defined(UNDER_RTSS)
#include "RTmalloc\RTmalloc.h"
#endif // defined(UNDER_RTSS)

#include <string>
#include <assert.h>
#include <list>
#include <vector>
#include <set>

namespace GENAPI_NAMESPACE
{
    class CNodeDataMap;
    class CNodeMapFactory;

    //! Object corresponding to a GenICam node and holding a list of the node's properties 
    class NODEMAPDATA_DECL CNodeData
    {
        // The CNodeDataMapFactory contains the pre-processing code and needs to dig into the internal data structures
        friend class CNodeDataMapFactory;
        friend class CNodeDataMap;
        friend class CNodeMapFactory;
    public:

        //! ID describing the type of the GenICam node
        //! For example MaskedIntReg_ID
        typedef enum
        {
            Node_ID,
            Category_ID,
            Integer_ID,
            Enumeration_ID,
            EnumEntry_ID,
            MaskedIntReg_ID,
            Register_ID,
            IntReg_ID,
            Float_ID,
            FloatReg_ID,
            SwissKnife_ID,
            IntSwissKnife_ID,
            IntKey_ID,
            TextDesc_ID,
            Port_ID,
            ConfRom_ID,
            AdvFeatureLock_ID,
            SmartFeature_ID,
            String_ID,
            StringReg_ID,
            Boolean_ID,
            Command_ID,
            Converter_ID,
            IntConverter_ID,

            // specials
            StructReg_ID,           // this is used temporarily only
            StructEntry_ID,           // this is used temporarily only
            Group_ID,                // this is used temporarily only
            RegisterDescription_ID, // the outer node
            _NotImpl_ID            // uninitialized

        } ENodeType_t;

        //! Constructor 
        /*! requires a back-pointer to the node map where the NodeData object is going to be registered */
        CNodeData(const ENodeType_t NodeType, CNodeDataMap *pNodeDataMap);

        /*! copy constructor  */
        CNodeData(const CNodeData &SourceNodeData, const NodeID_t &TargetNodeID, CNodeDataMap *pTargetNodeDataMap, bool extractedNode = false);

        //! Destructor
        virtual ~CNodeData();

        //! Sets the NodeID
        void SetNodeID(NodeID_t NodeID);

        //! Retrieves the NodeID
        virtual NodeID_t GetNodeID() const;

        //! Sets the Name of a node
        virtual void SetName(const std::string &Name);

        //! Gets the Name of a node
        virtual std::string GetName() const;

        //! Adds a property to the node
        void AddProperty(CProperty *pProperty);

        //! Removes all properties with the given \a propertyID
        void RemoveProperties(CPropertyID propertyID);

        //! Print helper
        virtual std::string ToString(EToStringStyle_t ToStringStyle = tsText);

        //! Write to cache stream
        void ToFile1(std::ostream &OutStream);
        void ToFile2(std::ostream &OutStream);

        //! Creates a CNodeData object from cache stream
        static CNodeData *FromFile1(std::istream &InStream, CNodeDataMap *pNodeDataMap);
        void FromFile2(std::istream &InStream);

        //! Helper to retrieve the NodeDataMap
        inline CNodeDataMap *GetNodeDataMap()
        {
            return m_pNodeDataMap;
        }

        //! Helper to retrieve the NodeDataMap
        inline const CNodeDataMap *GetNodeDataMap() const
        {
            return m_pNodeDataMap;
        }

        ENodeType_t GetNodeType() const
        {
            return m_NodeType;
        }

        //! Some container types 
        typedef std::vector< CProperty *> PropertyVector_t;
        typedef std::vector<NodeID_t> NodeIDVector_t;
        typedef std::set<NodeID_t> NodeIDSet_t;

        //! Retrieves a const iterator pointing to the begin of the property list
        PropertyVector_t::iterator GetPropertiesListBegin()
        {
            assert(m_pPropertyVector);
            return m_pPropertyVector->begin();
        }

        //! Retrieves a const iterator pointing to the begin of the property list
        PropertyVector_t::const_iterator GetPropertiesListBegin() const
        {
            assert(m_pPropertyVector);
            return m_pPropertyVector->begin();
        }

        //! Retrieves a const iterator pointing to the end of the property list
        PropertyVector_t::iterator GetPropertiesListEnd()
        {
            assert(m_pPropertyVector);
            return m_pPropertyVector->end();
        }

        //! Retrieves a const iterator pointing to the end of the property list
        PropertyVector_t::const_iterator GetPropertiesListEnd() const
        {
            assert(m_pPropertyVector);
            return m_pPropertyVector->end();
        }

        // Deletes the elements in the property list
        static void FreePropertiesList(PropertyVector_t &PropertyList)
        {
            for (PropertyVector_t::iterator it = PropertyList.begin(); it != PropertyList.end(); it++)
                delete (*it);
            PropertyList.clear();
        }

        //! Called by CNodeDataMapFactory::BuildDependencyList
        void PropagateDependency(NodeIDVector_t &NextNodesToProcess);

        //! Called by CNodeDataMapFactory::BuildDependencyList
        bool PushDependencies(const NodeIDSet_t& DependingNodes, NodeID_t pParent);

        //! Called by CNodeDataMapFactory::BuildTerminalList
        const NodeIDSet_t * PropagateTerminals();

        //! Beware: is valid only after m_pWritingChildren has been filled
        bool IsTerminalNode();

        //! Retrieves the MergePriority of this node
        int64_t GetMergePriority();

        // iteratively called function checking for cyclical selectors
        void CheckSelectedCycle(CNodeDataMap::NodeID2NodeData_t &NodeStack); // should be const
        
        // iteratively called function checking for cyclical selectors
        void CheckReadingCycle(CNodeDataMap::NodeID2NodeData_t &NodeStack); // should be const
        
        //! Is other CNodeData object identical.
        bool operator==(const CNodeData &rhs) const;

        //! Is other CNodeData object NOT identical.
        bool operator!=(const CNodeData &rhs) const { return !CNodeData::operator==(rhs); };
        
        // Old versions of GCC (3.4.x and lower) cannot compile with these protected members
#if defined(__GNUC__) && GENICAM_GCC_VERSION < 30500
    public:
#else
    protected:
#endif
        //! The type of the node, e.g. MaskedIntReg_ID
        ENodeType_t m_NodeType;

        //! The ID of the node, e.g. 4711
        NodeID_t m_NodeID;

        //! the list of properties
        PropertyVector_t *m_pPropertyVector;

        //! Back pointer to NodeDataMap
        CNodeDataMap *m_pNodeDataMap;

        //! All nodes for which this node is at least an invalidating child
        //! For details see GenICam wiki : GenApi/SoftwareArchitecture/NodeDependencies
        NodeIDVector_t *m_pParents;

        //! all nodes which can be read from 
        //! For details see GenICam wiki : GenApi/SoftwareArchitecture/NodeDependencies
        NodeIDVector_t *m_pReadingChildren;

        //! all nodes which can write a value further down the node stack
        //! For details see GenICam wiki : GenApi/SoftwareArchitecture/NodeDependencies
        NodeIDVector_t *m_pWritingChildren;

        //! All directly connected nodes which invalidate this node
        //! For details see GenICam wiki : GenApi/SoftwareArchitecture/NodeDependencies
        NodeIDSet_t *m_pInvalidatingChildren;

        //! All directly or indirectly connected nodes which are invalidated by this nodes (i.e. which are dependent on this node)
        //! For details see GenICam wiki : GenApi/SoftwareArchitecture/NodeDependencies
        NodeIDSet_t *m_pAllDependingNodes;

        //! All indirectly connected terminal nodes
        //! For details see GenICam wiki : GenApi/SoftwareArchitecture/NodeDependencies
        NodeIDSet_t *m_pAllTerminalNodes;

        //! States when propagating terminals
        enum ETerminalPropagationState
        {
            eNotVisited, //!< PropagateTerminals not run for node
            eBeingVisited, //!< PropagateTerminals is in progress for node, used for detecting cycles
            eTerminalPropagationDone //! PropagateTerminals is done for node, can just copy from m_AllTerminalNodes
        };

        //! Current state for propagating terminals
        ETerminalPropagationState m_propagationState;

        //! True of the read cycle detection has passed through this node.
        bool m_readCycleChecked;

        //! True of the selector cycle detection has passed through this node.
        bool m_selectorCycleChecked;


    };

    struct PropertyCreator
    {
        template< typename T >
        static CProperty*  Create(CNodeDataMap *pNodeDataMap, CPropertyID::EProperty_ID_t PropertyID, const T& t)
        {
                return new CProperty(pNodeDataMap, PropertyID, t);
        }
        template< typename T, typename U >
        static CProperty*  Create(CNodeDataMap *pNodeDataMap, CPropertyID::EProperty_ID_t PropertyID, const T& t, const U& u)
        {
                return new CProperty(pNodeDataMap, PropertyID, t, u);
        }
        template< typename T, typename U, typename V >
        static CProperty*  Create(CNodeDataMap *pNodeDataMap, CPropertyID::EProperty_ID_t PropertyID, T& t, const U& u, const V&v)
        {
                return new CProperty(pNodeDataMap, PropertyID, t, u, v);
        }
        template< typename T, typename U, typename V >
        static CProperty*  Create(CNodeDataMap *pNodeDataMap, CPropertyID::EProperty_ID_t PropertyID, T& t, int i, bool b)
        {
                return new CProperty(pNodeDataMap, PropertyID, t, i, b);
        }
        static CProperty*  Create(CNodeDataMap *pNodeDataMap, CPropertyID::EProperty_ID_t PropertyID, NodeID_t x, CProperty *pAttribute = NULL, bool IsNodeRef = false)
        {
                return new CProperty(pNodeDataMap, PropertyID, x, pAttribute,IsNodeRef);
        }

    };

    struct PropertyListHelper{
        static bool Add(CNodeData::PropertyVector_t &PropertyList, CProperty* Property)
        {
            bool success = false;
            if (Property)
                try{
                PropertyList.push_back(Property);
                success = true;
            }
            catch (...)
            {
                delete Property; // we took ownership hence have to clean-up
                throw;
            }

            return success;
        }
    };

    //! Converts an NodeID_t property to a string
    void NODEMAPDATA_DECL Value2String(CNodeData::ENodeType_t Value, std::string &ValueStr);

    //! Add property to property list
    void NODEMAPDATA_DECL AddProperty(CNodeData::PropertyVector_t& pl, CNodeDataMap* pd, CPropertyID::EProperty_ID_t p, CNodeData::ENodeType_t nt);
    void NODEMAPDATA_DECL AddProperty(CNodeData::PropertyVector_t& pl, CNodeDataMap* pd, CPropertyID::EProperty_ID_t pid, EYesNo e);
    void NODEMAPDATA_DECL AddProperty(CNodeData::PropertyVector_t& pl, CNodeDataMap* pd, CPropertyID::EProperty_ID_t pid, EVisibility e);
    void NODEMAPDATA_DECL AddProperty(CNodeData::PropertyVector_t& pl, CNodeDataMap* pd, CPropertyID::EProperty_ID_t p, const std::string& s);

}
