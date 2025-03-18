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

#include <string>
#include <sstream>

#include "NodeMapData/NodeMapData.h"
#include "PropertyID.h"
#include "NodeMapDataEnums.h"
#include "NodeMapDataTypes.h"
#include <Base/GCBase.h>
#include "NodeDataMap.h"


#ifndef C_ASSERT
#   define C_ASSERT3( e, l ) typedef char assertion_failed_at_line##l [ (e) ? 1 : -1 ]
#   define C_ASSERT2( e, l ) C_ASSERT3( e, l )
#   define C_ASSERT( e ) C_ASSERT2( e, __LINE__ )
#endif

namespace GENAPI_NAMESPACE
{

    class NODEMAPDATA_DECL CProperty
    {
        //friend std::ostream& operator<<(std::ostream& output, const CProperty& Property);

    public:
        //! Default constructor
        CProperty(INodeDataMap *pNodeDataMap);

        //! Copy constructor
        CProperty(CProperty *pTheProperty);

        //! Another copy constructor
        CProperty(CProperty *pSourceProperty, INodeDataMap *pTargetNodeDataMap);

        //! Construct used when loading from cache
        CProperty(std::istream &InStream, INodeDataMap *pNodeDataMap);

        //! Destructor
        virtual ~CProperty();

        //! Constructors initializing the property with a XSDe type and data x
        //CONSTRUCTOR_TPYE3( int64_t )

        CProperty(INodeDataMap *pNodeDataMap, const GENAPI_NAMESPACE::CPropertyID &PropertyID, const double x, CProperty *pAttribute = NULL) :
            m_pNodeDataMap(pNodeDataMap),
            m_pAttribute(pAttribute)
        {
            SetPropertyID(PropertyID);
            operator=(x);
        }

        // constructor for strings / reference to another node (also given as string)
        CProperty(INodeDataMap *pNodeDataMap, const CPropertyID &PropertyID, const std::string &x,
            CProperty *pAttribute = NULL, bool IsNodeRef = false) :
            m_pNodeDataMap(pNodeDataMap),
            m_pAttribute(pAttribute)
        {
            SetPropertyID(PropertyID);
            if (IsNodeRef)
            {
                NodeID_t NodeID = GetNodeDataMap()->GetNodeID(x, true);
                SetNodeID(NodeID);
            }
            else
            {
                operator=(x);
            }
        }

        // constructor for strings / reference to another node (given as NodeID)
        CProperty(INodeDataMap *pNodeDataMap, const CPropertyID &PropertyID, const int64_t x,
            CProperty *pAttribute = NULL, bool IsNodeRef = false) :
            m_pNodeDataMap(pNodeDataMap),
            m_pAttribute(pAttribute)
        {
            SetPropertyID(PropertyID);
            if (IsNodeRef)
            {
                SetNodeID(NodeID_t::FromIndex(static_cast<size_t>(x)));
            }
            else
            {
                operator=(x);
            }
        }


        CProperty(INodeDataMap *pNodeDataMap, const CPropertyID &PropertyID, NodeID_t x,
            CProperty *pAttribute = NULL, bool IsNodeRef = false) :
            m_pNodeDataMap(pNodeDataMap),
            m_pAttribute(pAttribute)
        {
            assert(IsNodeRef);
            (void)IsNodeRef;
            SetPropertyID(PropertyID);
            SetNodeID(x);
        }

        void operator=(const GENAPI_NAMESPACE::EAccessMode x)
        {
            m_DataType = EAccessMode_ID;
            m_Data.AccessMode = x;
            assert(GetDataSize() == sizeof x);
        }

        void operator=(const GENAPI_NAMESPACE::ECachingMode x)
        {
            m_DataType = ECachingMode_ID;
            m_Data.CachingMode = x;
            assert(GetDataSize() == sizeof x);
        }

        void operator=(const GENAPI_NAMESPACE::EDisplayNotation x)
        {
            m_DataType = EDisplayNotation_ID;
            m_Data.DisplayNotation = x;
            assert(GetDataSize() == sizeof x);
        }

        void operator=(const GENAPI_NAMESPACE::EEndianess x)
        {
            m_DataType = EEndianess_ID;
            m_Data.Endianess = x;
            assert(GetDataSize() == sizeof x);
        }

        void operator=(const GENAPI_NAMESPACE::ENameSpace x)
        {
            m_DataType = ENameSpace_ID;
            m_Data.NameSpace = x;
            assert(GetDataSize() == sizeof x);
        }

        void operator=(const GENAPI_NAMESPACE::ERepresentation x)
        {
            m_DataType = ERepresentation_ID;
            m_Data.Representation = x;
            assert(GetDataSize() == sizeof x);
        }

        void operator=(const GENAPI_NAMESPACE::ESign x)
        {
            m_DataType = ESign_ID;
            m_Data.Sign = x;
            assert(GetDataSize() == sizeof x);
        }

        void operator=(const GENAPI_NAMESPACE::ESlope x)
        {
            m_DataType = ESlope_ID;
            m_Data.Slope = x;
            assert(GetDataSize() == sizeof x);
        }

        void operator=(const GENAPI_NAMESPACE::EStandardNameSpace x)
        {
            m_DataType = EStandardNameSpace_ID;
            m_Data.StandardNameSpace = x;
            assert(GetDataSize() == sizeof x);
        }

        void operator=(const GENAPI_NAMESPACE::EVisibility x)
        {
            m_DataType = EVisibility_ID;
            m_Data.Visibility = x;
            assert(GetDataSize() == sizeof x);
        }

        void operator=(const GENAPI_NAMESPACE::EYesNo x)
        {
            m_DataType = EYesNo_ID;
            m_Data.YesNo = x;
            assert(GetDataSize() == sizeof x);
        }

        void operator=(const GENAPI_NAMESPACE::EInputDirection x)
        {
            m_DataType = EInputDirection_ID;
            m_Data.InputDirection = x;
            assert(GetDataSize() == sizeof x);
        }

        CProperty(INodeDataMap *pNodeDataMap, const CPropertyID& PropertyID, GENAPI_NAMESPACE::EAccessMode  x, CProperty *pAttribute = NULL) :
            m_pNodeDataMap(pNodeDataMap),
            m_pAttribute(pAttribute)
        {
            SetPropertyID(PropertyID);
            operator=(x);
        }

        CProperty(INodeDataMap *pNodeDataMap, const CPropertyID& PropertyID, GENAPI_NAMESPACE::ECachingMode  x, CProperty *pAttribute = NULL) :
            m_pNodeDataMap(pNodeDataMap),
            m_pAttribute(pAttribute)
        {
            SetPropertyID(PropertyID);
            operator=(x);
        }

        CProperty(INodeDataMap *pNodeDataMap, const CPropertyID& PropertyID, GENAPI_NAMESPACE::EDisplayNotation  x, CProperty *pAttribute = NULL) :
            m_pNodeDataMap(pNodeDataMap),
            m_pAttribute(pAttribute)
        {
            SetPropertyID(PropertyID);
            operator=(x);
        }

        CProperty(INodeDataMap *pNodeDataMap, const CPropertyID& PropertyID, GENAPI_NAMESPACE::EEndianess  x, CProperty *pAttribute = NULL) :
            m_pNodeDataMap(pNodeDataMap),
            m_pAttribute(pAttribute)
        {
            SetPropertyID(PropertyID);
            operator=(x);
        }

        CProperty(INodeDataMap *pNodeDataMap, const CPropertyID& PropertyID, GENAPI_NAMESPACE::ENameSpace  x, CProperty *pAttribute = NULL) :
            m_pNodeDataMap(pNodeDataMap),
            m_pAttribute(pAttribute)
        {
            SetPropertyID(PropertyID);
            operator=(x);
        }

        CProperty(INodeDataMap *pNodeDataMap, const CPropertyID& PropertyID, GENAPI_NAMESPACE::ERepresentation  x, CProperty *pAttribute = NULL) :
            m_pNodeDataMap(pNodeDataMap),
            m_pAttribute(pAttribute)
        {
            SetPropertyID(PropertyID);
            operator=(x);
        }

        CProperty(INodeDataMap *pNodeDataMap, const CPropertyID& PropertyID, GENAPI_NAMESPACE::ESign  x, CProperty *pAttribute = NULL) :
            m_pNodeDataMap(pNodeDataMap),
            m_pAttribute(pAttribute)
        {
            SetPropertyID(PropertyID);
            operator=(x);
        }

        CProperty(INodeDataMap *pNodeDataMap, const CPropertyID& PropertyID, GENAPI_NAMESPACE::ESlope  x, CProperty *pAttribute = NULL) :
            m_pNodeDataMap(pNodeDataMap),
            m_pAttribute(pAttribute)
        {
            SetPropertyID(PropertyID);
            operator=(x);
        }

        CProperty(INodeDataMap *pNodeDataMap, const CPropertyID& PropertyID, GENAPI_NAMESPACE::EStandardNameSpace  x, CProperty *pAttribute = NULL) :
            m_pNodeDataMap(pNodeDataMap),
            m_pAttribute(pAttribute)
        {
            SetPropertyID(PropertyID);
            operator=(x);
        }

        CProperty(INodeDataMap *pNodeDataMap, const CPropertyID& PropertyID, GENAPI_NAMESPACE::EVisibility  x, CProperty *pAttribute = NULL) :
            m_pNodeDataMap(pNodeDataMap),
            m_pAttribute(pAttribute)
        {
            SetPropertyID(PropertyID);
            operator=(x);
        }

        CProperty(INodeDataMap *pNodeDataMap, const CPropertyID& PropertyID, GENAPI_NAMESPACE::EYesNo  x, CProperty *pAttribute = NULL) :
            m_pNodeDataMap(pNodeDataMap),
            m_pAttribute(pAttribute)
        {
            SetPropertyID(PropertyID);
            operator=(x);
        }

        CProperty(INodeDataMap *pNodeDataMap, const CPropertyID& PropertyID, GENAPI_NAMESPACE::EInputDirection  x, CProperty *pAttribute = NULL) :
            m_pNodeDataMap(pNodeDataMap),
            m_pAttribute(pAttribute)
        {
            SetPropertyID(PropertyID);
            operator=(x);
        }

        GENAPI_NAMESPACE::EAccessMode EAccessMode() const
        {
            assert(m_DataType == EAccessMode_ID);
            return m_Data.AccessMode;
        }

        GENAPI_NAMESPACE::ECachingMode ECachingMode() const
        {
            assert(m_DataType == ECachingMode_ID);
            return m_Data.CachingMode;
        }

        GENAPI_NAMESPACE::EDisplayNotation EDisplayNotation() const
        {
            assert(m_DataType == EDisplayNotation_ID);
            return m_Data.DisplayNotation;
        }

        GENAPI_NAMESPACE::EEndianess EEndianess() const
        {
            assert(m_DataType == EEndianess_ID);
            return m_Data.Endianess;
        }

        GENAPI_NAMESPACE::ENameSpace ENameSpace() const
        {
            assert(m_DataType == ENameSpace_ID);
            return m_Data.NameSpace;
        }

        GENAPI_NAMESPACE::ERepresentation ERepresentation() const
        {
            assert(m_DataType == ERepresentation_ID);
            return m_Data.Representation;
        }

        GENAPI_NAMESPACE::ESign ESign() const
        {
            assert(m_DataType == ESign_ID);
            return m_Data.Sign;
        }

        GENAPI_NAMESPACE::ESlope ESlope() const
        {
            assert(m_DataType == ESlope_ID);
            return m_Data.Slope;
        }

        GENAPI_NAMESPACE::EStandardNameSpace EStandardNameSpace() const
        {
            assert(m_DataType == EStandardNameSpace_ID);
            return m_Data.StandardNameSpace;
        }

        GENAPI_NAMESPACE::EVisibility EVisibility() const
        {
            assert(m_DataType == EVisibility_ID);
            return m_Data.Visibility;
        }

        GENAPI_NAMESPACE::EYesNo EYesNo() const
        {
            assert(m_DataType == EYesNo_ID);
            return m_Data.YesNo;
        }

        GENAPI_NAMESPACE::EInputDirection EInputDirection() const
        {
            assert(m_DataType == EInputDirection_ID);
            return m_Data.InputDirection;
        }

        void operator=(const std::string &x)
        {
            m_DataType = String_ID;
            m_Data.StringIDValue = GetNodeDataMap()->SetString(x).ToIndex();
        }

        void operator=(const int64_t x)
        {
            m_DataType = Integer_ID;
            m_Data.Integer = x;
        }

        void operator=(const double x)
        {
            m_DataType = Float_ID;
            m_Data.Float = x;
        }

        void SetNodeID(const NodeID_t x)
        {
            m_DataType = Node_ID;
            m_Data.NodeIDValue = x.ToIndex();
        }

        int64_t Integer() const
        {
            assert(m_DataType == Integer_ID);
            return m_Data.Integer;
        }

        double Float() const
        {
            assert(m_DataType == Float_ID);
            return m_Data.Float;
        }

        virtual std::string String() const
        {
            assert(m_DataType == String_ID);
            return GetNodeDataMap()->GetString(StringID_t::FromIndex(m_Data.StringIDValue));
        }

        NodeID_t   NodeID() const
        {
            assert(m_DataType == Node_ID);
            return NodeID_t::FromIndex(m_Data.NodeIDValue);
        }

        StringID_t StringID() const
        {
            assert(m_DataType == String_ID);
            return StringID_t::FromIndex(m_Data.StringIDValue);
        }

        //! Sets the PropertyID
        void SetPropertyID(const CPropertyID &PropertyID)
        {
            assert(PropertyID.IsInitialized());
            m_PropertyID = PropertyID;
        }

        //! Get the PropertyID
        CPropertyID GetPropertyID() const;

        //! Verifies all members are initialized
        bool IsInitialized()
        {
            return (m_DataType != _Not_Initialized_ID) && m_PropertyID.IsInitialized();
        }

        //! Print helper
        std::string ToString(EToStringStyle_t ToStringStyle = tsText) const;

        //! Write to cache stream
        void ToFile(std::ostream &OutStream) const;

        //! Loads a CProperty object from cache stream
        void FromFile(std::istream &InStream);

        //! Creates a CProperty object from cache stream
        static CProperty *CreateFromFile(std::istream &InStream, INodeDataMap *pNodeDataMap);

        //! Gets the NodeDataMap
        inline INodeDataMap *GetNodeDataMap() const
        {
            assert(m_pNodeDataMap);
            return m_pNodeDataMap;
        }

        //! Sets an Attribute taking ownership of the object
        void SetAttribute(CProperty *pAttribute)
        {
            assert(pAttribute);
            m_pAttribute = pAttribute;
        }

        //! Gets an Attribute keeping the ownership
        //! Beware: the return value may be NULL
        CProperty *GetAttribute() const
        {
            return m_pAttribute;
        }

        inline int GetDataSize() const
        {
            if (m_DataType < _End_EDataType_uint8)
                return 1;
            else if (m_DataType < _End_EDataType_uint16)
                return 2;
            else if (m_DataType < _End_EDataType_uint32)
                return 4;
            else if (m_DataType < _End_EDataType_uint64)
                return 8;
            else
            {
                assert(false);
                return -1;
            }
        }

        //! ID reflecting the GenICam types used for property member variables, e.g. GenApi::ESign_t
        typedef enum
        {
            _Begin_EDataType,


            _End_EDataType_uint8,

            String_ID,
            Formula_ID,
            Node_ID,

            _End_EDataType_uint16,

            EAccessMode_ID,
            ECachingMode_ID,
            EDisplayNotation_ID,
            EEndianess_ID,
            ENameSpace_ID,
            ERepresentation_ID,
            ESign_ID,
            ESlope_ID,
            EStandardNameSpace_ID,
            EVisibility_ID,
            EYesNo_ID,
            EInputDirection_ID,

            _End_EDataType_uint32,

            Float_ID,
            Integer_ID,

            _End_EDataType_uint64,

            _Not_Initialized_ID,

            _End_EDataType

        } EDataType_t;

        EDataType_t GetDataType()
        {
            return m_DataType;
        }
        
        //! Returns a truth value of a proposition: The given two CProrpery objects are identical.
        bool operator==(const CProperty &rhs) const;
        
        //! Returns a truth value of a proposition: The given two CProrpery objects are NOT identical.
        bool operator!=(const CProperty &rhs) const { return !CProperty::operator==(rhs); };

        // Old versions of GCC (3.4.x and lower) cannot compile with these protected members
#if defined(__GNUC__) && GENICAM_GCC_VERSION < 30500
    public:
#else
    protected:
#endif
        //! ID reflecting the type of the property, e.g. <pValue>
        CPropertyID m_PropertyID;

        //! the data type
        EDataType_t m_DataType;

        C_ASSERT(sizeof(GENAPI_NAMESPACE::EAccessMode) == 4);
        C_ASSERT(sizeof(GENAPI_NAMESPACE::ECachingMode) == 4);
        C_ASSERT(sizeof(GENAPI_NAMESPACE::EDisplayNotation) == 4);
        C_ASSERT(sizeof(GENAPI_NAMESPACE::EEndianess) == 4);
        C_ASSERT(sizeof(GENAPI_NAMESPACE::ENameSpace) == 4);
        C_ASSERT(sizeof(GENAPI_NAMESPACE::ERepresentation) == 4);
        C_ASSERT(sizeof(GENAPI_NAMESPACE::ESign) == 4);
        C_ASSERT(sizeof(GENAPI_NAMESPACE::ESlope) == 4);
        C_ASSERT(sizeof(GENAPI_NAMESPACE::EStandardNameSpace) == 4);
        C_ASSERT(sizeof(GENAPI_NAMESPACE::EVisibility) == 4);
        C_ASSERT(sizeof(GENAPI_NAMESPACE::EYesNo) == 4);
        C_ASSERT(sizeof(GENAPI_NAMESPACE::EInputDirection) == 4);

        
        //! The actual data
        typedef union
        {
            GENAPI_NAMESPACE::EAccessMode AccessMode;
            GENAPI_NAMESPACE::ECachingMode CachingMode;
            GENAPI_NAMESPACE::EDisplayNotation DisplayNotation;
            GENAPI_NAMESPACE::EEndianess Endianess;
            GENAPI_NAMESPACE::ENameSpace NameSpace;
            GENAPI_NAMESPACE::ERepresentation Representation;
            GENAPI_NAMESPACE::ESign Sign;
            GENAPI_NAMESPACE::ESlope Slope;
            GENAPI_NAMESPACE::EStandardNameSpace StandardNameSpace;
            GENAPI_NAMESPACE::EVisibility Visibility;
            GENAPI_NAMESPACE::EYesNo YesNo;
            GENAPI_NAMESPACE::EInputDirection InputDirection;
            float64_t Float;
            int64_t Integer;
            StringID_t::value_type StringIDValue;
            NodeID_t::value_type NodeIDValue;
        } Data_t;

        //! the actual data
        Data_t m_Data;

        //! Pointer to the NodeDataMap
        INodeDataMap *m_pNodeDataMap;

        //! Attribute of this property
        CProperty *m_pAttribute;

    };

    std::ostream& operator<<(std::ostream& output, const CProperty& p);

}
