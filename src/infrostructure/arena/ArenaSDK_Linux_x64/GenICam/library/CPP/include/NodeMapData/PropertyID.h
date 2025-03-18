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
#include <string>
#include <sstream>
#include <assert.h>

namespace GENAPI_NAMESPACE
{

    //! Holds the ID describing what kind a property is, e.g. <Value> or <pValue>
    class NODEMAPDATA_DECL CPropertyID
    {
        friend std::ostream& operator<<(std::ostream& output, const CPropertyID& p);

    public:
        //! These ID represent the functional role of a property within a GenICam node object
        //! For example: an Integer property can have the role Value_ID or Max_ID
        //! BEWARE : the sequence of the enum entries is crucial; the _End_Of_XXX entries are used
        //!          to determine the pointer type
        typedef enum
        {
            _Begin_Property_ID,

            // Pointers establishing a writing, reading, and invalidating dependency
            pValue_ID,
            pValueCopy_ID,
            pValueIndexed_ID,
            pValueDefault_ID,
            pSwissKnifeTo_ID,
            pSwissKnifeFrom_ID,
            _End_Of_Writing_Pointers,

            // Pointers establishing a reading and invalidating dependency
            pIsAvailable_ID,
            pIsImplemented_ID,
            pIsLocked_ID,
            pError_ID,
            pMin_ID,
            pMax_ID,
            pInc_ID,
            pCommandValue_ID,
            pVariable_ID,
            pLength_ID,
            pAddress_ID,
            pIndex_ID,
            pPort_ID,
            pChunkID_ID,
            p1212Parser_ID,
            pBlockPolling_ID,
            pOffset_ID,
            pFeature_ID,
            _End_Of_Reading_Pointers,

            // Pointers establishing an invalidating dependency
            pInvalidator_ID,
            pSelecting_ID,
            pEnumEntry_ID,
            _End_Of_Invalidating_Pointers,

            // Pointers establishing no dependency
            pDependent_ID,  // this is a created dependency which does not count as direct dependency
            pAlias_ID,
            pCastAlias_ID,
            pSelected_ID,
            pTerminal_ID,
            _End_Of_Pointers,

            // Elements
            _Elements,
            NodeType_ID,
            Name_ID,
            NameSpace_ID,
            IsFeature_ID,
            Visibility_ID,
            Streamable_ID,
            Cachable_ID,
            Description_ID,
            ToolTip_ID,
            EventID_ID,
            DeviceName_ID,
            ImposedAccessMode_ID,
            ImposedVisibility_ID,
            PollingTime_ID,
            DocuURL_ID,
            IsDeprecated_ID,
            DisplayName_ID,
            Value_ID,
            OnValue_ID,
            OffValue_ID,
            AccessMode_ID,
            Length_ID,
            Address_ID,
            CommandValue_ID,
            Min_ID,
            Max_ID,
            Inc_ID,
            ValueDefault_ID,
            Representation_ID,
            ValidValueSet_ID,
            Unit_ID,
            ValueIndexed_ID,
            DisplayPrecision_ID,
            DisplayNotation_ID,
            NumericValue_ID,
            Symbolic_ID,
            IsSelfClearing_ID,
            Key_ID,
            FeatureID_ID,
            Timeout_ID,
            Endianess_ID,
            ChunkID_ID,
            SwapEndianess_ID,
            CacheChunkData_ID,
            Sign_ID,
            LSB_ID,
            MSB_ID,
            Formula_ID,
            Input_ID,
            FormulaTo_ID,
            FormulaFrom_ID,
            Slope_ID,
            IsLinear_ID,
            MergePriority_ID,
            ExposeStatic_ID,
            Extension_ID,
            Offset_ID,
            Index_ID,
            Comment_ID,
            InputDirection_ID,

            // RegisterDescription attributes
            ModelName_ID,
            VendorName_ID,
            StandardNameSpace_ID,
            SchemaMajorVersion_ID,
            SchemaMinorVersion_ID,
            SchemaSubMinorVersion_ID,
            MajorVersion_ID,
            MinorVersion_ID,
            SubMinorVersion_ID,
            ProductGuid_ID,
            VersionGuid_ID,

            // Special
            _Not_Initialized_ID,

            _End_Property_ID,

            AdvFeatureLock_ID,
            SmartFeature_ID,
            Bit_ID,
            Constant_ID,
            Expression_ID,
            TextDesc_ID,
            IntKey_ID
        } EProperty_ID_t;

        inline bool IsWritingPointer() const
        {
            return IsWritingPointer(m_PropertyID);
        }

        inline static bool IsWritingPointer(EProperty_ID_t PropertyID)
        {
            return PropertyID < _End_Of_Writing_Pointers;
        }

        inline bool IsReadingPointer() const
        {
            return IsReadingPointer(m_PropertyID);
        }

        inline static bool IsReadingPointer(EProperty_ID_t PropertyID)
        {
            return PropertyID < _End_Of_Reading_Pointers;
        }

        inline bool IsInvalidatingPointer() const
        {
            return IsInvalidatingPointer(m_PropertyID);
        }

        inline static bool IsInvalidatingPointer(EProperty_ID_t PropertyID)
        {
            return PropertyID < _End_Of_Invalidating_Pointers;
        }

        inline bool IsPointer() const
        {
            return m_PropertyID < _End_Of_Pointers;
        }

        inline EProperty_ID_t GetID() const
        {
            return m_PropertyID;
        }

        //! Constructor
        CPropertyID();

        //! Constructor
        CPropertyID(const EProperty_ID_t PropertyID);

        //! Assignment
        void operator=(const EProperty_ID_t PropertyID);

        //! is equal operator
        bool operator==(const CPropertyID& rhs) const;

        //! Tests if the object has set to a valid ID
        bool IsInitialized() const;

        //! Print helper
        std::string ToString() const;

    protected:
        //! The actual Property ID
        EProperty_ID_t m_PropertyID;
    };

    //! Streaming operator allowing a PropertyID to be streamed for example directly to cout
    std::ostream& operator<<(std::ostream& output, const CPropertyID& PropertyID);

}
