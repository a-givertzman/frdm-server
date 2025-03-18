#pragma once

#include "NodeMapData/NodeDataMap.h"
#include "NodeMapData/NodeData.h"
#include "NodeMapDataPtrs.h"
#include "GenApiSchema_Version_1_1-pskel.hxx"
#include "XmlParserStringToEnum.h"
#include "XmlParserValue2String.h"

#ifdef _MSC_VER
#   pragma warning(push)
#   pragma warning(disable : 4100)
#   pragma warning(disable : 4189)
#endif

namespace GENAPI_NAMESPACE
{

    class CNodeMapDataBuilder
    {
		static Version_t m_SchemaVersion;

    public:
        static bool NeedsContextNode(CNodeData::ENodeType_t nodeType)
        {
            if (
                CNodeData::StructReg_ID == nodeType
                || CNodeData::Enumeration_ID == nodeType
                || CNodeData::FloatReg_ID == nodeType
                || CNodeData::Register_ID == nodeType
                || CNodeData::MaskedIntReg_ID == nodeType
                || CNodeData::IntReg_ID == nodeType
                || CNodeData::ConfRom_ID == nodeType
                )
            {
                return true;
            }
            return false;
        }


        template<typename ParserPimplT>
        static void ProcessPre(CNodeMapDataPtrs& nodeMapDataPtrs, CNodeData::ENodeType_t nodeType, ParserPimplT*)
        {
            assert(nodeMapDataPtrs.pNodeData == NULL);
            assert(nodeMapDataPtrs.pNodeDataMap);

            if (nodeType != CNodeData::StructEntry_ID)
            {
                // normal operation
                nodeMapDataPtrs.pNodeData = new CNodeData(nodeType, nodeMapDataPtrs.pNodeDataMap);
            }
            else//special for StructEntry_ID
            {
                nodeMapDataPtrs.pNodeData = new CNodeData(CNodeData::MaskedIntReg_ID, nodeMapDataPtrs.pNodeDataMap);
            }

			if (CNodeData::EnumEntry_ID == nodeType)
			{
				// for sake of backwards cmopatibility we allow not embedded EnumEntry nodes but only in schema v1.0
				if (!(m_SchemaVersion.Major == 1 && m_SchemaVersion.Minor == 0))  // is not schmea v1.0?
					if (!nodeMapDataPtrs.pNodeDataMap->GetContextNode())  // is not embedded?
						throw RUNTIME_EXCEPTION("Standalone <EnumEntry> node detected which is ignored in schema v1.0 but forbidden otherwise");
			}

            if (CNodeData::RegisterDescription_ID == nodeType)
            {
                nodeMapDataPtrs.pNodeData->SetName("_RegisterDescription");
            }

            if (NeedsContextNode(nodeType))
            {
                nodeMapDataPtrs.pNodeDataMap->SetContextNode(nodeMapDataPtrs.pNodeData);
            }
        }

        template<typename T>
        static bool IsDisguisedStrucEntry(T*)
        {
            return false;
        }

        static bool IsDisguisedStrucEntry(::GENAPI_NAMESPACE::Version_1_1::StructEntryType_pimpl*)
        {
            return true;
        }

        template<typename ParserPimplT>
        static void ProcessPost(CNodeMapDataPtrs& nodeMapDataPtrs, ParserPimplT* pimpl)
        {
            ::GENAPI_NAMESPACE::Version_1_1::Key_t_pimpl* pCurrenValueParser = dynamic_cast< ::GENAPI_NAMESPACE::Version_1_1::Key_t_pimpl*>(pimpl);
            assert(nodeMapDataPtrs.pNodeData);
            assert(nodeMapDataPtrs.pNodeDataMap);
            try
            {
                if (nodeMapDataPtrs.pNodeData->GetNodeType() == CNodeData::StructReg_ID)
                {
                    nodeMapDataPtrs.pNodeDataMap->SetContextNode(NULL);
                    delete nodeMapDataPtrs.pNodeData;
                }
                else if ((nodeMapDataPtrs.pNodeData->GetNodeType() == CNodeData::TextDesc_ID
                    || nodeMapDataPtrs.pNodeData->GetNodeType() == CNodeData::IntKey_ID) && pCurrenValueParser != NULL)
                {
                    AddProperty(nodeMapDataPtrs, CPropertyID::p1212Parser_ID, nodeMapDataPtrs.pNodeDataMap->GetContextNode()->GetName());

                    int64_t Key;
                    bool res = XMLParser_String2Value(pCurrenValueParser->post_string(), &Key);
                    if (!res)
                        throw RUNTIME_EXCEPTION("%S it not a valid integer", pCurrenValueParser->post_string().c_str());
                    AddProperty(nodeMapDataPtrs, CPropertyID::Key_ID, Key);

                    nodeMapDataPtrs.pNodeDataMap->SetNodeData(nodeMapDataPtrs.pNodeData);

                    if (NeedsContextNode(nodeMapDataPtrs.pNodeData->GetNodeType()) && !IsDisguisedStrucEntry(pimpl))
                        nodeMapDataPtrs.pNodeDataMap->SetContextNode(NULL);

                }
                else // normal operation 
                {
					// Check for double EnumEntries
					const NodeID_t nodeID = nodeMapDataPtrs.pNodeData->GetNodeID();
					assert(nodeID.IsValid());
					CNodeData *pExistingNodeData = nodeMapDataPtrs.pNodeDataMap->GetNodeData(nodeID);
					if (    pExistingNodeData 
						&&  (nodeMapDataPtrs.pNodeData->GetNodeType() == GENAPI_NAMESPACE::CNodeData::EnumEntry_ID)
						&&  (*(nodeMapDataPtrs.pNodeData) == (*pExistingNodeData)) )
					{
						// if the data objects are really identical, ignore them; see @2720
						delete nodeMapDataPtrs.pNodeData;
						nodeMapDataPtrs.pNodeData = NULL;
						return;
					}

                    nodeMapDataPtrs.pNodeDataMap->SetNodeData(nodeMapDataPtrs.pNodeData);

					if (NeedsContextNode(nodeMapDataPtrs.pNodeData->GetNodeType())
						&& !IsDisguisedStrucEntry(pimpl)
						)
					{
						nodeMapDataPtrs.pNodeDataMap->SetContextNode(NULL);
					}
				}
				nodeMapDataPtrs.pNodeData = NULL;
			}
			catch (const GENICAM_NAMESPACE::GenericException &)
			{
				throw;
			}

					/*			

					try
					{
						NodeID2NodeData_t::reference ref = m_pNodeID2NodeData->at(nodeIdIndex);
						if (replace)
						{
							GENAPI_NAMESPACE::CNodeData *pOldNodeData = ref; 
							ref = NULL;
							assert(pOldNodeData);
							delete pOldNodeData;
						}
						if (ref != NULL)
						{
							// Check if the CNodeData object is actually an EnumEntry.
							if (pNodeData->GetNodeType() == GENAPI_NAMESPACE::CNodeData::EnumEntry_ID)
							{
								// The existence has not been locked in yet though...
								NodeExistence = DuplicatedAndIdentical;
								// If so the CNodeData object which already exists is as the variable "ref".
								// Check if "ref" and the new CNodeData objects are identical.
								if (*pNodeData != *ref)
								{
									NodeExistence = DuplicatedAndNotIdentical;
								}
							}
							else
							{
								// Treat the new CNodeData as duplicated & not identical anyway.
								NodeExistence = DuplicatedAndNotIdentical;
							}
						}
						if (NodeExistence == Unique)
						{
							ref = pNodeData;
						}
						else if (NodeExistence == DuplicatedAndNotIdentical)
						{
							throw RUNTIME_EXCEPTION("Found a duplicate node '%s'.", m_pNodeID2NodeName->at(nodeIdIndex).c_str());
						}
						else
						{
							// If and only if the existing and the new CNodeData object are identical
							// then ignore the new data object by not calling ref = pNodeData; instead
							// delete pNodeData.
							delete pNodeData;
							pNodeData = NULL;
						}
					}
					catch (const std::out_of_range& e)
					{
						throw RUNTIME_EXCEPTION("Inconsistent node map, index out of range: %s", e.what());
					}
					catch (const std::exception& e)
					{
						throw RUNTIME_EXCEPTION("Failed to SetNodeData: %s", e.what());
					}
					if (NodeExistence == Unique)
					{
						// Update the last added node ID if the node was unique.
						m_LastAddedNodeID = nodeID;
					}
					*/

        }


        template<typename ParserPimplT>
        static void ProcessPre(CNodeMapDataPtrsForConverterWithCleanup& nodeMapDataPtrs, CNodeData::ENodeType_t nodeType, ParserPimplT*)
        {
            assert(nodeMapDataPtrs.pNodeData == NULL);
            assert(nodeMapDataPtrs.pNodeDataMap);

            nodeMapDataPtrs.pNodeData = new CNodeData(nodeType, nodeMapDataPtrs.pNodeDataMap);

            // Registers can have embedded IntSwissKnife Elements
            // in order to get them their correct name the register's CNodeData object is made available
            // as context node
            nodeMapDataPtrs.pNodeDataMap->SetContextNode(nodeMapDataPtrs.pNodeData);

            CNodeData::ENodeType_t skType = CNodeData::IntSwissKnife_ID;
            if (nodeType == CNodeData::Converter_ID)
            {
                skType = CNodeData::SwissKnife_ID;
            }

            // Create and initialize worker SwissKnifest
            nodeMapDataPtrs.pConverterTo = new CNodeData(skType, nodeMapDataPtrs.pNodeDataMap);
            nodeMapDataPtrs.pConverterTo->AddProperty(new CProperty(nodeMapDataPtrs.pNodeDataMap, CPropertyID::InputDirection_ID, GENAPI_NAMESPACE::idTo));

            nodeMapDataPtrs.pConverterFrom = new CNodeData(skType, nodeMapDataPtrs.pNodeDataMap);
            nodeMapDataPtrs.pConverterFrom->AddProperty(new CProperty(nodeMapDataPtrs.pNodeDataMap, CPropertyID::InputDirection_ID, GENAPI_NAMESPACE::idFrom));

        }


        template<typename ParserPimplT>
        static void ProcessPost(CNodeMapDataPtrsForConverterWithCleanup& nodeMapDataPtrs, ParserPimplT*)
        {
            assert(nodeMapDataPtrs.pNodeData);
            assert(nodeMapDataPtrs.pNodeDataMap);

            // Clear the context node 
            nodeMapDataPtrs.pNodeDataMap->SetContextNode(NULL);

            // Set the NodeData to the map
            nodeMapDataPtrs.pNodeDataMap->SetNodeData(nodeMapDataPtrs.pNodeData);

            // give the SwissKnife nodes (see below) the same MergePriority as the Converter 
            int64_t MergePriority = 0;
            for (CNodeData::PropertyVector_t::iterator itProperty = nodeMapDataPtrs.pNodeData->GetPropertiesListBegin();
                itProperty != nodeMapDataPtrs.pNodeData->GetPropertiesListEnd(); itProperty++)
            {
                if ((*itProperty)->GetPropertyID() == CPropertyID::MergePriority_ID)
                    MergePriority = (*itProperty)->Integer();
            }

            // Name and set the worker SwissKnifes
            nodeMapDataPtrs.pConverterTo->SetName(nodeMapDataPtrs.pNodeData->GetName() + "_ConvertTo");
            {
                CProperty *pProperty = new CProperty(nodeMapDataPtrs.pNodeDataMap);
                pProperty->SetPropertyID(CPropertyID::pSwissKnifeTo_ID);
                pProperty->SetNodeID(nodeMapDataPtrs.pConverterTo->GetNodeID());
                nodeMapDataPtrs.pNodeData->AddProperty(pProperty);
                if (MergePriority != 0)
                    nodeMapDataPtrs.pConverterTo->AddProperty(new CProperty(nodeMapDataPtrs.pNodeDataMap, CPropertyID::MergePriority_ID, MergePriority));
            }
            nodeMapDataPtrs.pNodeDataMap->SetNodeData(nodeMapDataPtrs.pConverterTo);

            nodeMapDataPtrs.pConverterFrom->SetName(nodeMapDataPtrs.pNodeData->GetName() + "_ConvertFrom");
            {
                CProperty *pProperty = new CProperty(nodeMapDataPtrs.pNodeDataMap);
                pProperty->SetPropertyID(CPropertyID::pSwissKnifeFrom_ID);
                pProperty->SetNodeID(nodeMapDataPtrs.pConverterFrom->GetNodeID());
                nodeMapDataPtrs.pNodeData->AddProperty(pProperty);
                if (MergePriority != 0)
                    nodeMapDataPtrs.pConverterFrom->AddProperty(new CProperty(nodeMapDataPtrs.pNodeDataMap, CPropertyID::MergePriority_ID, MergePriority));
            }
            nodeMapDataPtrs.pNodeDataMap->SetNodeData(nodeMapDataPtrs.pConverterFrom);


            nodeMapDataPtrs.pNodeData = NULL;
            nodeMapDataPtrs.pConverterTo = NULL;
            nodeMapDataPtrs.pConverterFrom = NULL;
        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, bool Value, ParserPimplT*)
        {
            AddProperty(nodeMapDataPtrs, id, Value ? (int64_t)1 : (int64_t)0);
        }

        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, double Value, ParserPimplT*)
        {
            AddProperty(nodeMapDataPtrs, id, Value);
        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::std::string Value, ParserPimplT*)
        {
            AddProperty(nodeMapDataPtrs, id, StringToStringID(nodeMapDataPtrs, Value));
        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrsForConverterWithCleanup& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::std::string Value, ParserPimplT* pimpl)
        {
            assert(nodeMapDataPtrs.pNodeData);
            assert(nodeMapDataPtrs.pNodeDataMap);
            assert(nodeMapDataPtrs.pConverterTo);
            assert(nodeMapDataPtrs.pConverterFrom);

            if (id == CPropertyID::FormulaTo_ID)
            {
                AddProperty(nodeMapDataPtrs, id, StringToStringID(nodeMapDataPtrs, Value));
                CNodeMapDataPtrs to(nodeMapDataPtrs);
                to.pNodeData = nodeMapDataPtrs.pConverterTo;
                AddProperty(to, CPropertyID::Formula_ID, StringToStringID(nodeMapDataPtrs, Value));
            }
            else if (id == CPropertyID::FormulaFrom_ID)
            {
                AddProperty(nodeMapDataPtrs, id, StringToStringID(nodeMapDataPtrs, Value));
                CNodeMapDataPtrs from(nodeMapDataPtrs);
                from.pNodeData = nodeMapDataPtrs.pConverterFrom;
                AddProperty(from, CPropertyID::Formula_ID, StringToStringID(nodeMapDataPtrs, Value));
            }
            else
            {
                Process((const CNodeMapDataPtrs&)nodeMapDataPtrs, id, Value, pimpl);
            }


        }
        static int64_t StringToInt64(const CPropertyID::EProperty_ID_t id, const std::string& text)
        {
            int64_t val = 0;
            if (!XMLParser_String2Value(text, val))
            {
                throw PROPERTY_EXCEPTION("Error in property of type '%hs': cannot convert '%hs' to int64_t",
                    CPropertyID(id).ToString().c_str(), text.c_str());
            }
            return val;
        }


        static const std::string& StringToNodeID(const CNodeMapDataPtrs& /*nodeMapDataPtrs*/, const std::string& text)
        {
            return text;
        }


        static const std::string&  StringToStringID(const CNodeMapDataPtrs& /*nodeMapDataPtrs*/, const std::string& text)
        {
            return text;
        }


        static CProperty* NewProperty(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, const std::string& v, CProperty* pA = NULL)
        {
            return new CProperty(nodeMapDataPtrs.pNodeDataMap, id, v, pA, CPropertyID(id).IsPointer());
        }


        template <typename T>
        static CProperty* NewProperty(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, const T& v, CProperty* pA = NULL)
        {
            CProperty* p = new CProperty(nodeMapDataPtrs.pNodeDataMap, id, v);
            if (pA)
            {
                p->SetAttribute(pA);
            }
            return p;
        }


        template <typename T>
        static void AddProperty(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, const T& v)
        {
            assert(nodeMapDataPtrs.pNodeData);
            assert(nodeMapDataPtrs.pNodeDataMap);
            if (id > CPropertyID::_End_Property_ID)
            {
                assert(false);
                return;
            }
            nodeMapDataPtrs.pNodeData->AddProperty(NewProperty(nodeMapDataPtrs, id, v));
        }



        template <typename T, typename U>
        static void AddPropertyWithAttribute(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, const T& v, const CPropertyID::EProperty_ID_t idOfAttribute, const U& valueOfAttribute)
        {
            assert(nodeMapDataPtrs.pNodeData);
            assert(nodeMapDataPtrs.pNodeDataMap);
            if (id > CPropertyID::_End_Property_ID)
            {
                assert(false);
                return;
            }
            nodeMapDataPtrs.pNodeData->AddProperty(NewProperty(nodeMapDataPtrs, id, v, NewProperty(nodeMapDataPtrs, idOfAttribute, valueOfAttribute)));
        }


        //////////////////////////////////////////////////////////////////////////
        //////////////////////////////////////////////////////////////////////////
        //////////////////////////////////////////////////////////////////////////

        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::RegisterDescription_pskel* pskel, ParserPimplT*)
        {
            ::GENAPI_NAMESPACE::Version_1_1::RegisterDescription_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::RegisterDescription_pimpl*>(pskel);

        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::CName_t_pskel* pskel, ParserPimplT*)
        {
            ::GENAPI_NAMESPACE::Version_1_1::CName_t_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::CName_t_pimpl*>(pskel);
            assert(nodeMapDataPtrs.pNodeData);
            assert(nodeMapDataPtrs.pNodeDataMap);

            //note: pValueCopy1 is mapped to pValueCopy by caller

            if (CPropertyID(id).IsPointer())
            {
                AddProperty(nodeMapDataPtrs, id, StringToNodeID(nodeMapDataPtrs, pCurrenValueParser->text));
            }
            else
            {
                if (id == CPropertyID::Name_ID)
                {
                    CNodeData *pContextNode = nodeMapDataPtrs.pNodeDataMap->GetContextNode();

                    // if this is an embedded node prefix the name
                    // Example: MyRegister contains MyIntSwissKnife as embedded complex element
                    // In this case the IntSwissKnife gets the name _MyRegister_MyIntSwissKnife
                    if (pContextNode
                        && pContextNode != nodeMapDataPtrs.pNodeData
                        && pContextNode->GetNodeID().IsValid())
                    {
                        std::string Name;
                        if (pContextNode->GetNodeType() == CNodeData::Enumeration_ID)
                        {
                            // Give the EnumEntry node a special name
                            Name = "EnumEntry_" + pContextNode->GetName() + "_" + pCurrenValueParser->text;

                            // Use the name as default symbolic
                            AddProperty(nodeMapDataPtrs, CPropertyID::Symbolic_ID, StringToNodeID(nodeMapDataPtrs, pCurrenValueParser->text));

                            // give the EnumEntry node the same MergePriority than the Enumeration has
                            int64_t MergePriority = 0;
                            for (CNodeData::PropertyVector_t::iterator itProperty = pContextNode->GetPropertiesListBegin();
                                itProperty != pContextNode->GetPropertiesListEnd(); itProperty++)
                            {
                                if ((*itProperty)->GetPropertyID() == CPropertyID::MergePriority_ID)
                                    MergePriority = (*itProperty)->Integer();
                            }
                            if (MergePriority != 0)
                                AddProperty(nodeMapDataPtrs, CPropertyID::MergePriority_ID, MergePriority);
                        }
                        else if (pContextNode->GetNodeType() == CNodeData::ConfRom_ID)
                        {
                            // this is a bit dangerous because the schema does not guarantee that the name
                            // is ambiguous - however for sake of backward compatibility we cannot change this
                            Name = pCurrenValueParser->text;
                        }
                        else
                        {
                            // this makes the name unambigious
                            Name = "_" + pContextNode->GetName() + "_" + pCurrenValueParser->text;
                        }
                        nodeMapDataPtrs.pNodeData->SetName(Name);
                    }
                    else
                    {
                        nodeMapDataPtrs.pNodeData->SetName(pCurrenValueParser->text);
                    }
                }
                else if (id == CPropertyID::Symbolic_ID)
                {
                    // On Name callback a default symbolic is set
                    // Replace its value here

                    // Walk all properties
                    for (CNodeData::PropertyVector_t::const_iterator itProperty = nodeMapDataPtrs.pNodeData->GetPropertiesListBegin();
                        itProperty != nodeMapDataPtrs.pNodeData->GetPropertiesListEnd(); ++itProperty)
                    {
                        if ((*itProperty)->GetPropertyID() == CPropertyID::Symbolic_ID)
                        {
                            (*itProperty)->operator =(pCurrenValueParser->text);
                        }
                    }
                }
                else
                {
                    AddProperty(nodeMapDataPtrs, id, StringToStringID(nodeMapDataPtrs, pCurrenValueParser->text));
                }
            }
        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrsForConverterWithCleanup& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::CName_t_pskel* pskel, ParserPimplT* pimpl)
        {
            ::GENAPI_NAMESPACE::Version_1_1::CName_t_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::CName_t_pimpl*>(pskel);
            assert(nodeMapDataPtrs.pNodeData);
            assert(nodeMapDataPtrs.pNodeDataMap);
            assert(nodeMapDataPtrs.pConverterTo);
            assert(nodeMapDataPtrs.pConverterFrom);

            if (id == CPropertyID::pInvalidator_ID)
            {
                AddProperty(nodeMapDataPtrs, id, StringToNodeID(nodeMapDataPtrs, pCurrenValueParser->text));
            }
            else
            {
                Process((const CNodeMapDataPtrs&)nodeMapDataPtrs, id, pskel, pimpl);
            }
        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::StandardNameSpace_t_pskel* pskel, ParserPimplT*)
        {
            ::GENAPI_NAMESPACE::Version_1_1::StandardNameSpace_t_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::StandardNameSpace_t_pimpl*>(pskel);
            AddProperty(nodeMapDataPtrs, id, StringToStandardNameSpace(pCurrenValueParser->text));
        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::nonNegativeHexOrDecimal_t_pskel* pskel, ParserPimplT*)
        {
            ::GENAPI_NAMESPACE::Version_1_1::nonNegativeHexOrDecimal_t_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::nonNegativeHexOrDecimal_t_pimpl*>(pskel);
            if (id == CPropertyID::Bit_ID)
            {
                AddProperty(nodeMapDataPtrs, CPropertyID::LSB_ID, StringToInt64(id, pCurrenValueParser->text));
                AddProperty(nodeMapDataPtrs, CPropertyID::MSB_ID, StringToInt64(id, pCurrenValueParser->text));
            }
            else
            {
				int64_t number = StringToInt64(id, pCurrenValueParser->text);
                AddProperty(nodeMapDataPtrs, id, number);
				switch (id)
				{
				case CPropertyID::SchemaMajorVersion_ID:
					m_SchemaVersion.Major = (uint16_t)number;
					break;
				case CPropertyID::SchemaMinorVersion_ID:
					m_SchemaVersion.Minor = (uint16_t)number;
					break;
				case CPropertyID::SchemaSubMinorVersion_ID:
					m_SchemaVersion.SubMinor = (uint16_t)number;
					break;
				default:
					break;
				}
				

            }
        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::GUID_t_pskel* pskel, ParserPimplT*)
        {
            ::GENAPI_NAMESPACE::Version_1_1::GUID_t_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::GUID_t_pimpl*>(pskel);
            AddProperty(nodeMapDataPtrs, id, StringToStringID(nodeMapDataPtrs, pCurrenValueParser->text));
        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::NameSpace_t_pskel* pskel, ParserPimplT*)
        {
            ::GENAPI_NAMESPACE::Version_1_1::NameSpace_t_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::NameSpace_t_pimpl*>(pskel);
            AddProperty(nodeMapDataPtrs, id, StringToNameSpace(pCurrenValueParser->text));
        }

        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::NameSpace_te_pskel* pskel, ParserPimplT*)
        {
            ::GENAPI_NAMESPACE::Version_1_1::NameSpace_te_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::NameSpace_te_pimpl*>(pskel);
            if (pCurrenValueParser->text != "")
                AddProperty(nodeMapDataPtrs, id, StringToNameSpace(pCurrenValueParser->text));
        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::MergePriority_t_pskel* pskel, ParserPimplT*)
        {
            ::GENAPI_NAMESPACE::Version_1_1::MergePriority_t_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::MergePriority_t_pimpl*>(pskel);
            AddProperty(nodeMapDataPtrs, id, static_cast<int64_t>(pCurrenValueParser->longValue));
        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::YesNo_t_pskel* pskel, ParserPimplT*)
        {
            ::GENAPI_NAMESPACE::Version_1_1::YesNo_t_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::YesNo_t_pimpl*>(pskel);
            AddProperty(nodeMapDataPtrs, id, StringToYesNo(pCurrenValueParser->text));
        }

        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::YesNo_te_pskel* pskel, ParserPimplT*)
        {
            ::GENAPI_NAMESPACE::Version_1_1::YesNo_te_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::YesNo_te_pimpl*>(pskel);
            if (pCurrenValueParser->text != "" )
                AddProperty(nodeMapDataPtrs, id, StringToYesNo(pCurrenValueParser->text));
        }



        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::nonEmptyString_t_pskel* pskel, ParserPimplT*)
        {
            if (nodeMapDataPtrs.SuppressStringsOnLoading)
            {
                if ((id == CPropertyID::Description_ID) ||
                    (id == CPropertyID::ToolTip_ID) ||
                    (id == CPropertyID::DisplayName_ID))
                {
                    return;
                }
            }

            ::GENAPI_NAMESPACE::Version_1_1::nonEmptyString_t_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::nonEmptyString_t_pimpl*>(pskel);
            AddProperty(nodeMapDataPtrs, id, StringToStringID(nodeMapDataPtrs, pCurrenValueParser->text));
        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::Visibility_te_pskel* pskel, ParserPimplT*)
        {
            ::GENAPI_NAMESPACE::Version_1_1::Visibility_te_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::Visibility_te_pimpl*>(pskel);
            if (pCurrenValueParser->text != "")
                AddProperty(nodeMapDataPtrs, id, StringToVisibility(pCurrenValueParser->text));
        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::DocURL_t_pskel* pskel, ParserPimplT*)
        {
            ::GENAPI_NAMESPACE::Version_1_1::DocURL_t_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::DocURL_t_pimpl*>(pskel);
            AddProperty(nodeMapDataPtrs, id, StringToStringID(nodeMapDataPtrs, pCurrenValueParser->text));
        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::Hex_t_pskel* pskel, ParserPimplT*)
        {
            ::GENAPI_NAMESPACE::Version_1_1::Hex_t_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::Hex_t_pimpl*>(pskel);
            AddProperty(nodeMapDataPtrs, id, /* Hex_t values are processed as strings. */ StringToStringID(nodeMapDataPtrs, pCurrenValueParser->text));
        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::Access_te_pskel* pskel, ParserPimplT*)
        {
            ::GENAPI_NAMESPACE::Version_1_1::Access_te_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::Access_te_pimpl*>(pskel);
            if (pCurrenValueParser->text != "")
                AddProperty(nodeMapDataPtrs, id, StringToAccessMode(pCurrenValueParser->text));
        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::HexOrDecimal_t_pskel* pskel, ParserPimplT*)
        {
            ::GENAPI_NAMESPACE::Version_1_1::HexOrDecimal_t_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::HexOrDecimal_t_pimpl*>(pskel);
            AddProperty(nodeMapDataPtrs, id, StringToInt64(id, pCurrenValueParser->text));
        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::ValueIndexed_pskel* pskel, ParserPimplT*)
        {
            ::GENAPI_NAMESPACE::Version_1_1::ValueIndexed_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::ValueIndexed_pimpl*>(pskel);
            AddPropertyWithAttribute(nodeMapDataPtrs, id, StringToInt64(id, pCurrenValueParser->text), CPropertyID::Index_ID, StringToInt64(id, pCurrenValueParser->textIndex));
        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::pValueIndexed_pskel* pskel, ParserPimplT*)
        {
            ::GENAPI_NAMESPACE::Version_1_1::pValueIndexed_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::pValueIndexed_pimpl*>(pskel);
            AddPropertyWithAttribute(nodeMapDataPtrs, id, StringToNodeID(nodeMapDataPtrs, pCurrenValueParser->text), CPropertyID::Index_ID, StringToInt64(id, pCurrenValueParser->textIndex));
        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::PositiveHexOrDecimal_t_pskel* pskel, ParserPimplT*)
        {
            ::GENAPI_NAMESPACE::Version_1_1::PositiveHexOrDecimal_t_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::PositiveHexOrDecimal_t_pimpl*>(pskel);
            AddProperty(nodeMapDataPtrs, id, StringToInt64(id, pCurrenValueParser->text));
        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::IntRepresentation_t_pskel* pskel, ParserPimplT*)
        {
            ::GENAPI_NAMESPACE::Version_1_1::IntRepresentation_t_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::IntRepresentation_t_pimpl*>(pskel);
            AddProperty(nodeMapDataPtrs, id, StringToRepresentation(pCurrenValueParser->text));
        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, ::GENAPI_NAMESPACE::Version_1_1::IntSwissKnifeType_pskel* pskel, ParserPimplT*)
        {
            assert(nodeMapDataPtrs.pNodeData);
            assert(nodeMapDataPtrs.pNodeDataMap);
            AddProperty(nodeMapDataPtrs, CPropertyID::pAddress_ID, nodeMapDataPtrs.pNodeDataMap->GetLastAddedNode()->GetName());
        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::pVariable3_pskel* pskel, ParserPimplT*)
        {
            ::GENAPI_NAMESPACE::Version_1_1::pVariable3_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::pVariable3_pimpl*>(pskel);
            AddPropertyWithAttribute(nodeMapDataPtrs, id, StringToNodeID(nodeMapDataPtrs, pCurrenValueParser->text), CPropertyID::Name_ID, StringToStringID(nodeMapDataPtrs, pCurrenValueParser->textName));
        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::VariableName_t_pskel* pskel, ParserPimplT*)
        {
            ::GENAPI_NAMESPACE::Version_1_1::VariableName_t_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::VariableName_t_pimpl*>(pskel);
            AddProperty(nodeMapDataPtrs, id, StringToStringID(nodeMapDataPtrs, pCurrenValueParser->text));
        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::pIndexType_pskel* pskel, ParserPimplT*)
        {
            ::GENAPI_NAMESPACE::Version_1_1::pIndexType_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::pIndexType_pimpl*>(pskel);
            assert(nodeMapDataPtrs.pNodeData);
            assert(nodeMapDataPtrs.pNodeDataMap);

            if (!pCurrenValueParser->textOffset.empty())
            {
                AddPropertyWithAttribute(nodeMapDataPtrs, CPropertyID::pIndex_ID, StringToNodeID(nodeMapDataPtrs, pCurrenValueParser->text), CPropertyID::Offset_ID, StringToInt64(CPropertyID::Offset_ID, pCurrenValueParser->textOffset));
            }
            else if (!pCurrenValueParser->textpOffset.empty())
            {
                AddPropertyWithAttribute(nodeMapDataPtrs, CPropertyID::pIndex_ID, StringToNodeID(nodeMapDataPtrs, pCurrenValueParser->text), CPropertyID::pOffset_ID, StringToNodeID(nodeMapDataPtrs, pCurrenValueParser->textpOffset));
            }
            else
            {
                AddProperty(nodeMapDataPtrs, CPropertyID::pIndex_ID, StringToNodeID(nodeMapDataPtrs, pCurrenValueParser->text));
            }
        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::CachingMode_te_pskel* pskel, ParserPimplT*)
        {
            ::GENAPI_NAMESPACE::Version_1_1::CachingMode_te_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::CachingMode_te_pimpl*>(pskel);
            if (pCurrenValueParser->text != "")
                AddProperty(nodeMapDataPtrs, id, StringToCachingMode(pCurrenValueParser->text));
        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::Sign_te_pskel* pskel, ParserPimplT*)
        {
            ::GENAPI_NAMESPACE::Version_1_1::Sign_te_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::Sign_te_pimpl*>(pskel);
            if (pCurrenValueParser->text != "")
                AddProperty(nodeMapDataPtrs, id, StringToSign(pCurrenValueParser->text));
        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::Endianess_te_pskel* pskel, ParserPimplT*)
        {
            ::GENAPI_NAMESPACE::Version_1_1::Endianess_te_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::Endianess_te_pimpl*>(pskel);
            if (pCurrenValueParser->text != "")
                AddProperty(nodeMapDataPtrs, id, StringToEndianess(pCurrenValueParser->text));
        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, ::GENAPI_NAMESPACE::Version_1_1::EnumEntryType_pskel* pskel, ParserPimplT*)
        {
            assert(nodeMapDataPtrs.pNodeData);
            assert(nodeMapDataPtrs.pNodeDataMap);
            // Add pEnumEntry entry 
            AddProperty(nodeMapDataPtrs, CPropertyID::pEnumEntry_ID, StringToNodeID(nodeMapDataPtrs, nodeMapDataPtrs.pNodeDataMap->GetLastAddedNode()->GetName()));
        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::ValueIndexed1_pskel* pskel, ParserPimplT*)
        {
            ::GENAPI_NAMESPACE::Version_1_1::ValueIndexed1_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::ValueIndexed1_pimpl*>(pskel);
            AddPropertyWithAttribute(nodeMapDataPtrs, id, pCurrenValueParser->doubleValue, CPropertyID::Index_ID, StringToInt64(id, pCurrenValueParser->textIndex));
        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::pValueIndexed1_pskel* pskel, ParserPimplT*)
        {
            ::GENAPI_NAMESPACE::Version_1_1::pValueIndexed1_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::pValueIndexed1_pimpl*>(pskel);
            AddPropertyWithAttribute(nodeMapDataPtrs, id, StringToNodeID(nodeMapDataPtrs, pCurrenValueParser->text), CPropertyID::Index_ID, StringToInt64(id, pCurrenValueParser->textIndex));
        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::FloatRepresentation_t_pskel* pskel, ParserPimplT*)
        {
            ::GENAPI_NAMESPACE::Version_1_1::FloatRepresentation_t_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::FloatRepresentation_t_pimpl*>(pskel);
            AddProperty(nodeMapDataPtrs, id, StringToRepresentation(pCurrenValueParser->text));
        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::DisplayNotation_t_pskel* pskel, ParserPimplT*)
        {
            ::GENAPI_NAMESPACE::Version_1_1::DisplayNotation_t_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::DisplayNotation_t_pimpl*>(pskel);
            AddProperty(nodeMapDataPtrs, id, StringToDisplayNotation(pCurrenValueParser->text));
        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::Slope_te_pskel* pskel, ParserPimplT*)
        {
            ::GENAPI_NAMESPACE::Version_1_1::Slope_te_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::Slope_te_pimpl*>(pskel);
            if (pCurrenValueParser->text != "")
                AddProperty(nodeMapDataPtrs, id, StringToSlope(pCurrenValueParser->text));
        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrsForConverterWithCleanup& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::pVariable_pskel* pskel, ParserPimplT*)
        {
            ::GENAPI_NAMESPACE::Version_1_1::pVariable_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::pVariable_pimpl*>(pskel);
            assert(nodeMapDataPtrs.pNodeData);
            assert(nodeMapDataPtrs.pNodeDataMap);
            assert(nodeMapDataPtrs.pConverterTo);
            assert(nodeMapDataPtrs.pConverterFrom);

            AddPropertyWithAttribute(nodeMapDataPtrs, id, StringToNodeID(nodeMapDataPtrs, pCurrenValueParser->text), CPropertyID::Name_ID, StringToStringID(nodeMapDataPtrs, pCurrenValueParser->textName));
            CNodeMapDataPtrs to(nodeMapDataPtrs);
            to.pNodeData = nodeMapDataPtrs.pConverterTo;
            AddPropertyWithAttribute(to, id, StringToNodeID(to, pCurrenValueParser->text), CPropertyID::Name_ID, StringToStringID(to, pCurrenValueParser->textName));
            CNodeMapDataPtrs from(nodeMapDataPtrs);
            from.pNodeData = nodeMapDataPtrs.pConverterFrom;
            AddPropertyWithAttribute(from, id, StringToNodeID(from, pCurrenValueParser->text), CPropertyID::Name_ID, StringToStringID(from, pCurrenValueParser->textName));
        }



        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::pVariable1_pskel* pskel, ParserPimplT*)
        {
            ::GENAPI_NAMESPACE::Version_1_1::pVariable1_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::pVariable1_pimpl*>(pskel);
            AddPropertyWithAttribute(nodeMapDataPtrs, id, StringToNodeID(nodeMapDataPtrs, pCurrenValueParser->text), CPropertyID::Name_ID, StringToStringID(nodeMapDataPtrs, pCurrenValueParser->textName));
        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrsForConverterWithCleanup& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::pVariable1_pskel* pskel, ParserPimplT*)
        {
            ::GENAPI_NAMESPACE::Version_1_1::pVariable1_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::pVariable1_pimpl*>(pskel);
            assert(nodeMapDataPtrs.pNodeData);
            assert(nodeMapDataPtrs.pNodeDataMap);
            assert(nodeMapDataPtrs.pConverterTo);
            assert(nodeMapDataPtrs.pConverterFrom);

            AddPropertyWithAttribute(nodeMapDataPtrs, id, StringToNodeID(nodeMapDataPtrs, pCurrenValueParser->text), CPropertyID::Name_ID, StringToStringID(nodeMapDataPtrs, pCurrenValueParser->textName));
            CNodeMapDataPtrs to(nodeMapDataPtrs);
            to.pNodeData = nodeMapDataPtrs.pConverterTo;
            AddPropertyWithAttribute(to, id, StringToNodeID(to, pCurrenValueParser->text), CPropertyID::Name_ID, StringToStringID(to, pCurrenValueParser->textName));
            CNodeMapDataPtrs from(nodeMapDataPtrs);
            from.pNodeData = nodeMapDataPtrs.pConverterFrom;
            AddPropertyWithAttribute(from, id, StringToNodeID(from, pCurrenValueParser->text), CPropertyID::Name_ID, StringToStringID(from, pCurrenValueParser->textName));
        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::pVariable2_pskel* pskel, ParserPimplT*)
        {
            ::GENAPI_NAMESPACE::Version_1_1::pVariable2_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::pVariable2_pimpl*>(pskel);
            AddPropertyWithAttribute(nodeMapDataPtrs, id, StringToNodeID(nodeMapDataPtrs, pCurrenValueParser->text), CPropertyID::Name_ID, StringToStringID(nodeMapDataPtrs, pCurrenValueParser->textName));
        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrsForConverterWithCleanup& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::Constant_pskel* pskel, ParserPimplT*)
        {
            ::GENAPI_NAMESPACE::Version_1_1::Constant_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::Constant_pimpl*>(pskel);
            assert(nodeMapDataPtrs.pNodeData);
            assert(nodeMapDataPtrs.pNodeDataMap);
            assert(nodeMapDataPtrs.pConverterTo);
            assert(nodeMapDataPtrs.pConverterFrom);

            // create a new Float node
            CNodeData *pNodeData = new CNodeData(CNodeData::Float_ID, nodeMapDataPtrs.pNodeDataMap);

            // Add Name entry
            std::string Name;
            Name = "_" + nodeMapDataPtrs.pNodeDataMap->GetNodeName(nodeMapDataPtrs.pNodeData->GetNodeID()) + "_" + pCurrenValueParser->textName;
            pNodeData->SetName(Name);

            // Add Value entry
            pNodeData->AddProperty(new CProperty(nodeMapDataPtrs.pNodeDataMap, CPropertyID::Value_ID, pCurrenValueParser->doubleValue));

            // Register the node
            nodeMapDataPtrs.pNodeDataMap->SetNodeData(pNodeData);

            // Create a pVariable reference entry to the Float node in the SwissKnife node
            AddPropertyWithAttribute(nodeMapDataPtrs, CPropertyID::pVariable_ID, StringToNodeID(nodeMapDataPtrs, Name), CPropertyID::Name_ID, StringToStringID(nodeMapDataPtrs, pCurrenValueParser->textName));
            CNodeMapDataPtrs to(nodeMapDataPtrs);
            to.pNodeData = nodeMapDataPtrs.pConverterTo;
            AddPropertyWithAttribute(to, CPropertyID::pVariable_ID, StringToNodeID(to, Name), CPropertyID::Name_ID, StringToStringID(to, pCurrenValueParser->textName));
            CNodeMapDataPtrs from(nodeMapDataPtrs);
            from.pNodeData = nodeMapDataPtrs.pConverterFrom;
            AddPropertyWithAttribute(from, CPropertyID::pVariable_ID, StringToNodeID(from, Name), CPropertyID::Name_ID, StringToStringID(from, pCurrenValueParser->textName));

        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrsForConverterWithCleanup& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::Constant1_pskel* pskel, ParserPimplT*)
        {
            ::GENAPI_NAMESPACE::Version_1_1::Constant1_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::Constant1_pimpl*>(pskel);
            assert(nodeMapDataPtrs.pNodeData);
            assert(nodeMapDataPtrs.pNodeDataMap);
            assert(nodeMapDataPtrs.pConverterTo);
            assert(nodeMapDataPtrs.pConverterFrom);

            // create a new Integer node
            CNodeData *pNodeData = new CNodeData(CNodeData::Integer_ID, nodeMapDataPtrs.pNodeDataMap);

            // Add Name entry
            std::string Name;
            Name = "_" + nodeMapDataPtrs.pNodeDataMap->GetNodeName(nodeMapDataPtrs.pNodeData->GetNodeID()) + "_" + pCurrenValueParser->textName;
            pNodeData->SetName(Name);

            // Add Value entry
            pNodeData->AddProperty(new CProperty(nodeMapDataPtrs.pNodeDataMap, CPropertyID::Value_ID, StringToInt64(id, pCurrenValueParser->text)));

            // Register the node
            nodeMapDataPtrs.pNodeDataMap->SetNodeData(pNodeData);

            // Create a pVariable reference entry to the Integer node in the IntSwissKnife node
            AddPropertyWithAttribute(nodeMapDataPtrs, CPropertyID::pVariable_ID, StringToNodeID(nodeMapDataPtrs, Name), CPropertyID::Name_ID, StringToStringID(nodeMapDataPtrs, pCurrenValueParser->textName));
            CNodeMapDataPtrs to(nodeMapDataPtrs);
            to.pNodeData = nodeMapDataPtrs.pConverterTo;
            AddPropertyWithAttribute(to, CPropertyID::pVariable_ID, StringToNodeID(to, Name), CPropertyID::Name_ID, StringToStringID(to, pCurrenValueParser->textName));
            CNodeMapDataPtrs from(nodeMapDataPtrs);
            from.pNodeData = nodeMapDataPtrs.pConverterFrom;
            AddPropertyWithAttribute(from, CPropertyID::pVariable_ID, StringToNodeID(from, Name), CPropertyID::Name_ID, StringToStringID(from, pCurrenValueParser->textName));
        }




        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::Constant2_pskel* pskel, ParserPimplT*)
        {
            ::GENAPI_NAMESPACE::Version_1_1::Constant2_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::Constant2_pimpl*>(pskel);
            assert(nodeMapDataPtrs.pNodeData);
            assert(nodeMapDataPtrs.pNodeDataMap);

            // create a new Integer node
            CNodeData *pNodeData = new CNodeData(CNodeData::Float_ID, nodeMapDataPtrs.pNodeDataMap);

            // Add Name entry
            std::string Name;
            Name = "_" + nodeMapDataPtrs.pNodeDataMap->GetNodeName(nodeMapDataPtrs.pNodeData->GetNodeID()) + "_" + pCurrenValueParser->textName;
            pNodeData->SetName(Name);

            // Add Value entry
            pNodeData->AddProperty(new CProperty(nodeMapDataPtrs.pNodeDataMap, CPropertyID::Value_ID, pCurrenValueParser->doubleValue));

            // Register the node
            nodeMapDataPtrs.pNodeDataMap->SetNodeData(pNodeData);

            // Create a pVariable reference entry to the Integer node in the IntSwissKnife node
            AddPropertyWithAttribute(nodeMapDataPtrs, CPropertyID::pVariable_ID, StringToNodeID(nodeMapDataPtrs, Name), CPropertyID::Name_ID, StringToStringID(nodeMapDataPtrs, pCurrenValueParser->textName));
        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::Constant3_pskel* pskel, ParserPimplT*)
        {
            ::GENAPI_NAMESPACE::Version_1_1::Constant3_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::Constant3_pimpl*>(pskel);
            assert(nodeMapDataPtrs.pNodeData);
            assert(nodeMapDataPtrs.pNodeDataMap);

            // create a new Integer node
            CNodeData *pNodeData = new CNodeData(CNodeData::Integer_ID, nodeMapDataPtrs.pNodeDataMap);

            // Add Name entry
            std::string Name;
            Name = "_" + nodeMapDataPtrs.pNodeDataMap->GetNodeName(nodeMapDataPtrs.pNodeData->GetNodeID()) + "_" + pCurrenValueParser->textName;
            pNodeData->SetName(Name);

            // Add Value entry
            pNodeData->AddProperty(new CProperty(nodeMapDataPtrs.pNodeDataMap, CPropertyID::Value_ID, StringToInt64(id, pCurrenValueParser->text)));

            // Register the node
            nodeMapDataPtrs.pNodeDataMap->SetNodeData(pNodeData);

            // Create a pVariable reference entry to the Integer node in the IntSwissKnife node
            AddPropertyWithAttribute(nodeMapDataPtrs, CPropertyID::pVariable_ID, StringToNodeID(nodeMapDataPtrs, Name), CPropertyID::Name_ID, StringToStringID(nodeMapDataPtrs, pCurrenValueParser->textName));
        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrsForConverterWithCleanup& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::Expression_pskel* pskel, ParserPimplT*)
        {
            ::GENAPI_NAMESPACE::Version_1_1::Expression_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::Expression_pimpl*>(pskel);
            assert(nodeMapDataPtrs.pNodeData);
            assert(nodeMapDataPtrs.pNodeDataMap);
            assert(nodeMapDataPtrs.pConverterTo);
            assert(nodeMapDataPtrs.pConverterFrom);

            // create a new SwissKnife node
            CNodeData *pNodeData = new CNodeData(CNodeData::SwissKnife_ID, nodeMapDataPtrs.pNodeDataMap);

            // Add Name entry
            std::string Name;
            Name = "_" + nodeMapDataPtrs.pNodeDataMap->GetNodeName(nodeMapDataPtrs.pNodeData->GetNodeID()) + "_" + pCurrenValueParser->textName;
            pNodeData->SetName(Name);

            // Copy all pVariable entries from the original SwissKnife to the Expression SwissKnife 
            CNodeData::PropertyVector_t::const_iterator itProperty;
            for (itProperty = nodeMapDataPtrs.pNodeData->GetPropertiesListBegin(); itProperty != nodeMapDataPtrs.pNodeData->GetPropertiesListEnd(); ++itProperty)
            {
                std::string Buffer = (*itProperty)->ToString();
                if ((*itProperty)->GetPropertyID() == CPropertyID::pVariable_ID)
                {
                    CProperty *pProperty = (*itProperty);
                    pNodeData->AddProperty(new CProperty(pProperty));
                }
            }

            // Add Formula entry
            pNodeData->AddProperty(new CProperty(nodeMapDataPtrs.pNodeDataMap, CPropertyID::Formula_ID, pCurrenValueParser->text));

            // Register the node
            nodeMapDataPtrs.pNodeDataMap->SetNodeData(pNodeData);

            // Create a pVariable reference entry to the SwissKnife node
            AddPropertyWithAttribute(nodeMapDataPtrs, CPropertyID::pVariable_ID, StringToNodeID(nodeMapDataPtrs, Name), CPropertyID::Name_ID, StringToStringID(nodeMapDataPtrs, pCurrenValueParser->textName));
            CNodeMapDataPtrs to(nodeMapDataPtrs);
            to.pNodeData = nodeMapDataPtrs.pConverterTo;
            AddPropertyWithAttribute(to, CPropertyID::pVariable_ID, StringToNodeID(to, Name), CPropertyID::Name_ID, StringToStringID(to, pCurrenValueParser->textName));
            CNodeMapDataPtrs from(nodeMapDataPtrs);
            from.pNodeData = nodeMapDataPtrs.pConverterFrom;
            AddPropertyWithAttribute(from, CPropertyID::pVariable_ID, StringToNodeID(from, Name), CPropertyID::Name_ID, StringToStringID(from, pCurrenValueParser->textName));
        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrsForConverterWithCleanup& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::Expression1_pskel* pskel, ParserPimplT*)
        {
            ::GENAPI_NAMESPACE::Version_1_1::Expression1_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::Expression1_pimpl*>(pskel);
            assert(nodeMapDataPtrs.pNodeData);
            assert(nodeMapDataPtrs.pNodeDataMap);
            assert(nodeMapDataPtrs.pConverterTo);
            assert(nodeMapDataPtrs.pConverterFrom);

            // create a new IntSwissKnife node
            CNodeData *pNodeData = new CNodeData(CNodeData::IntSwissKnife_ID, nodeMapDataPtrs.pNodeDataMap);

            // Add Name entry
            std::string Name;
            Name = "_" + nodeMapDataPtrs.pNodeDataMap->GetNodeName(nodeMapDataPtrs.pNodeData->GetNodeID()) + "_" + pCurrenValueParser->textName;
            pNodeData->SetName(Name);

            // Copy all pVariable entries from the original IntSwissKnife to the Expression IntSwissKnife 
            CNodeData::PropertyVector_t::const_iterator itProperty;
            for (itProperty = nodeMapDataPtrs.pNodeData->GetPropertiesListBegin(); itProperty != nodeMapDataPtrs.pNodeData->GetPropertiesListEnd(); ++itProperty)
            {
                std::string Buffer = (*itProperty)->ToString();
                if ((*itProperty)->GetPropertyID() == CPropertyID::pVariable_ID)
                {
                    CProperty *pProperty = (*itProperty);
                    pNodeData->AddProperty(new CProperty(pProperty));
                }
            }

            // Add Formula entry
            pNodeData->AddProperty(new CProperty(nodeMapDataPtrs.pNodeDataMap, CPropertyID::Formula_ID, pCurrenValueParser->text));

            // Register the node
            nodeMapDataPtrs.pNodeDataMap->SetNodeData(pNodeData);

            // Create a pVariable reference entry to the IntSwissKnife node
            AddPropertyWithAttribute(nodeMapDataPtrs, CPropertyID::pVariable_ID, StringToNodeID(nodeMapDataPtrs, Name), CPropertyID::Name_ID, StringToStringID(nodeMapDataPtrs, pCurrenValueParser->textName));
            CNodeMapDataPtrs to(nodeMapDataPtrs);
            to.pNodeData = nodeMapDataPtrs.pConverterTo;
            AddPropertyWithAttribute(to, CPropertyID::pVariable_ID, StringToNodeID(to, Name), CPropertyID::Name_ID, StringToStringID(to, pCurrenValueParser->textName));
            CNodeMapDataPtrs from(nodeMapDataPtrs);
            from.pNodeData = nodeMapDataPtrs.pConverterFrom;
            AddPropertyWithAttribute(from, CPropertyID::pVariable_ID, StringToNodeID(from, Name), CPropertyID::Name_ID, StringToStringID(from, pCurrenValueParser->textName));
        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::Expression2_pskel* pskel, ParserPimplT*)
        {
            ::GENAPI_NAMESPACE::Version_1_1::Expression2_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::Expression2_pimpl*>(pskel);
            assert(nodeMapDataPtrs.pNodeData);
            assert(nodeMapDataPtrs.pNodeDataMap);

            // create a new SwissKnife node
            CNodeData *pNodeData = new CNodeData(CNodeData::SwissKnife_ID, nodeMapDataPtrs.pNodeDataMap);

            // Add Name entry
            std::string Name;
            Name = "_" + nodeMapDataPtrs.pNodeDataMap->GetNodeName(nodeMapDataPtrs.pNodeData->GetNodeID()) + "_" + pCurrenValueParser->textName;
            pNodeData->SetName(Name);

            // Copy all pVariable entries from the original SwissKnife to the Expression SwissKnife 
            CNodeData::PropertyVector_t::const_iterator itProperty;
            for (itProperty = nodeMapDataPtrs.pNodeData->GetPropertiesListBegin(); itProperty != nodeMapDataPtrs.pNodeData->GetPropertiesListEnd(); ++itProperty)
            {
                std::string Buffer = (*itProperty)->ToString();
                if ((*itProperty)->GetPropertyID() == CPropertyID::pVariable_ID)
                {
                    CProperty *pProperty = (*itProperty);
                    pNodeData->AddProperty(new CProperty(pProperty));
                }
            }

            // Add Formula entry
            pNodeData->AddProperty(new CProperty(nodeMapDataPtrs.pNodeDataMap, CPropertyID::Formula_ID, pCurrenValueParser->text));

            // Register the node
            nodeMapDataPtrs.pNodeDataMap->SetNodeData(pNodeData);

            // Create a pVariable reference entry to the SwissKnife node
            AddPropertyWithAttribute(nodeMapDataPtrs, CPropertyID::pVariable_ID, StringToNodeID(nodeMapDataPtrs, Name), CPropertyID::Name_ID, StringToStringID(nodeMapDataPtrs, pCurrenValueParser->textName));
        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::Expression3_pskel* pskel, ParserPimplT*)
        {
            ::GENAPI_NAMESPACE::Version_1_1::Expression3_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::Expression3_pimpl*>(pskel);
            assert(nodeMapDataPtrs.pNodeData);
            assert(nodeMapDataPtrs.pNodeDataMap);

            // create a new IntSwissKnife node
            CNodeData *pNodeData = new CNodeData(CNodeData::IntSwissKnife_ID, nodeMapDataPtrs.pNodeDataMap);

            // Add Name entry
            std::string Name;
            Name = "_" + nodeMapDataPtrs.pNodeDataMap->GetNodeName(nodeMapDataPtrs.pNodeData->GetNodeID()) + "_" + pCurrenValueParser->textName;
            pNodeData->SetName(Name);

            // Copy all pVariable entries from the original IntSwissKnife to the Expression IntSwissKnife 
            CNodeData::PropertyVector_t::const_iterator itProperty;
            for (itProperty = nodeMapDataPtrs.pNodeData->GetPropertiesListBegin(); itProperty != nodeMapDataPtrs.pNodeData->GetPropertiesListEnd(); ++itProperty)
            {
                std::string Buffer = (*itProperty)->ToString();
                if ((*itProperty)->GetPropertyID() == CPropertyID::pVariable_ID)
                {
                    CProperty *pProperty = (*itProperty);
                    pNodeData->AddProperty(new CProperty(pProperty));
                }
            }

            // Add Formula entry
            pNodeData->AddProperty(new CProperty(nodeMapDataPtrs.pNodeDataMap, CPropertyID::Formula_ID, pCurrenValueParser->text));
  

            // Register the node
            nodeMapDataPtrs.pNodeDataMap->SetNodeData(pNodeData);

            // Create a pVariable reference entry to the IntSwissKnife node
            AddPropertyWithAttribute(nodeMapDataPtrs, CPropertyID::pVariable_ID, StringToNodeID(nodeMapDataPtrs, Name), CPropertyID::Name_ID, StringToStringID(nodeMapDataPtrs, pCurrenValueParser->textName));
        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::Key_t_pskel* pskel, ParserPimplT*)
        {
            ::GENAPI_NAMESPACE::Version_1_1::Key_t_pimpl* pCurrenValueParser = static_cast< ::GENAPI_NAMESPACE::Version_1_1::Key_t_pimpl*>(pskel);
            assert(nodeMapDataPtrs.pNodeData);
            assert(nodeMapDataPtrs.pNodeDataMap);

            // create node
            CNodeData *pNodeData = NULL;
            switch (id)
            {
            case CPropertyID::TextDesc_ID:
                pNodeData = new CNodeData(CNodeData::TextDesc_ID, nodeMapDataPtrs.pNodeDataMap);
                break;
            case CPropertyID::IntKey_ID:
                pNodeData = new CNodeData(CNodeData::IntKey_ID, nodeMapDataPtrs.pNodeDataMap);
                break;
            default:
                assert(false);
            }
            assert(pNodeData);

            assert(pCurrenValueParser);
            pCurrenValueParser->nodeMapDataPtrs.pNodeData = pNodeData;
        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, ::GENAPI_NAMESPACE::Version_1_1::StructEntryType_pskel* pskel, ParserPimplT*)
        {
            assert(nodeMapDataPtrs.pNodeData);
            assert(nodeMapDataPtrs.pNodeDataMap);

            CNodeData *pStructEntry = nodeMapDataPtrs.pNodeDataMap->GetLastAddedNode();
            assert(pStructEntry);

            CNodeData::PropertyVector_t::const_iterator itStructRegProperty;
            CNodeData::PropertyVector_t::const_iterator itStructEntryProperty;
            
            // Properties not yet present in the StructEntry are stored in a temporary list and then stored in the StructReg.
            typedef std::vector< GENAPI_NAMESPACE::CProperty* > PropertyVector_t; 
            PropertyVector_t propertiesToPropagate;
            propertiesToPropagate.reserve(std::distance(nodeMapDataPtrs.pNodeData->GetPropertiesListBegin(), nodeMapDataPtrs.pNodeData->GetPropertiesListEnd()));

            for (itStructRegProperty = nodeMapDataPtrs.pNodeData->GetPropertiesListBegin();
                itStructRegProperty != nodeMapDataPtrs.pNodeData->GetPropertiesListEnd();
                ++itStructRegProperty)
            {
                bool PropertyAlreadyExists = false;
                for (itStructEntryProperty = pStructEntry->GetPropertiesListBegin();
                    itStructEntryProperty != pStructEntry->GetPropertiesListEnd();
                    ++itStructEntryProperty)
                {
                    if ((*itStructRegProperty)->GetPropertyID() == (*itStructEntryProperty)->GetPropertyID())
                    {
                        PropertyAlreadyExists = true;
                        break;
                    }
                }
                if (!PropertyAlreadyExists)
                {
                    propertiesToPropagate.push_back( new CProperty(*itStructRegProperty));
                }

            }

            // Copy the properties and give up the ownership
            for (PropertyVector_t::iterator it = propertiesToPropagate.begin(), end = propertiesToPropagate.end(); it != end; ++it)
            {
                try{
                    pStructEntry->AddProperty( *it );
                    *it = NULL;
                }
                catch (...)
                {   // free those properties we are still owning
                    while (it != end) {
                        delete *it++;
                    }
                    throw;
                }
            }
            propertiesToPropagate.clear();

        }


        template<typename ParserPimplT>
        static void Process(const CNodeMapDataPtrs& nodeMapDataPtrs, const CPropertyID::EProperty_ID_t id, ::GENAPI_NAMESPACE::Version_1_1::ExtensionType_pskel* pskel, ParserPimplT*)
        {
            assert(nodeMapDataPtrs.pNodeData);
            assert(nodeMapDataPtrs.pNodeDataMap);
            ::GenApi::Version_1_1::ExtensionType_pimpl* pCurrentValueParser = static_cast< ::GenApi::Version_1_1::ExtensionType_pimpl*>(pskel);

#if 0       // This code was originally targeted for v3.0.1 but later removed 
            /*Code to handle <ValidValueSet> in the extension field*/
            static const char *valieValueStr[] = {"<ValidValueSet>","</ValidValueSet>"};
            size_t startPos = pCurrentValueParser->content.find(valieValueStr[0]);
            if( startPos != std::string::npos && pCurrentValueParser->content.find(valieValueStr[1]) != std::string::npos )
            {
                size_t endPos = pCurrentValueParser->content.find("<", startPos + 1) + startPos+1;
                AddProperty(nodeMapDataPtrs, CPropertyID::ValidValueSet_ID, pCurrentValueParser->content.substr(startPos + strlen(valieValueStr[0]), endPos));
            }
            /*End of ValidValueSet extension code*/
#endif

            AddProperty(nodeMapDataPtrs, id, pCurrentValueParser->content);

        }


        // Generated code, remove later if not needed:


        //template<typename ParserPimplT>
        //static void Process( const CNodeMapDataPtrs& nodeMapDataPtrs, ::GenApi::Version_1_1::FloatType_pskel* pskel, ParserPimplT*)
        //{
        //    ::GenApi::Version_1_1::FloatType_pimpl* pCurrenValueParser = static_cast< ::GenApi::Version_1_1::FloatType_pimpl*>(pskel);

        //}


        //template<typename ParserPimplT>
        //static void Process( const CNodeMapDataPtrs& nodeMapDataPtrs, ::GenApi::Version_1_1::IntRegType_pskel* pskel, ParserPimplT*)
        //{
        //    ::GenApi::Version_1_1::IntRegType_pimpl* pCurrenValueParser = static_cast< ::GenApi::Version_1_1::IntRegType_pimpl*>(pskel);

        //}


        //template<typename ParserPimplT>
        //static void Process( const CNodeMapDataPtrs& nodeMapDataPtrs, ::GenApi::Version_1_1::CategoryType_pskel* pskel, ParserPimplT*)
        //{
        //    ::GenApi::Version_1_1::CategoryType_pimpl* pCurrenValueParser = static_cast< ::GenApi::Version_1_1::CategoryType_pimpl*>(pskel);

        //}


        //template<typename ParserPimplT>
        //static void Process( const CNodeMapDataPtrs& nodeMapDataPtrs, ::GenApi::Version_1_1::IntegerType_pskel* pskel, ParserPimplT*)
        //{
        //    ::GenApi::Version_1_1::IntegerType_pimpl* pCurrenValueParser = static_cast< ::GenApi::Version_1_1::IntegerType_pimpl*>(pskel);

        //}


        //template<typename ParserPimplT>
        //static void Process( const CNodeMapDataPtrs& nodeMapDataPtrs, ::GenApi::Version_1_1::FloatRegType_pskel* pskel, ParserPimplT*)
        //{
        //    ::GenApi::Version_1_1::FloatRegType_pimpl* pCurrenValueParser = static_cast< ::GenApi::Version_1_1::FloatRegType_pimpl*>(pskel);

        //}


        //template<typename ParserPimplT>
        //static void Process( const CNodeMapDataPtrs& nodeMapDataPtrs, ::GenApi::Version_1_1::StringType_pskel* pskel, ParserPimplT*)
        //{
        //    ::GenApi::Version_1_1::StringType_pimpl* pCurrenValueParser = static_cast< ::GenApi::Version_1_1::StringType_pimpl*>(pskel);

        //}


        //template<typename ParserPimplT>
        //static void Process( const CNodeMapDataPtrs& nodeMapDataPtrs, ::GenApi::Version_1_1::StringRegType_pskel* pskel, ParserPimplT*)
        //{
        //    ::GenApi::Version_1_1::StringRegType_pimpl* pCurrenValueParser = static_cast< ::GenApi::Version_1_1::StringRegType_pimpl*>(pskel);

        //}


        //template<typename ParserPimplT>
        //static void Process( const CNodeMapDataPtrs& nodeMapDataPtrs, ::GenApi::Version_1_1::MaskedIntRegType_pskel* pskel, ParserPimplT*)
        //{
        //    ::GenApi::Version_1_1::MaskedIntRegType_pimpl* pCurrenValueParser = static_cast< ::GenApi::Version_1_1::MaskedIntRegType_pimpl*>(pskel);

        //}


        //template<typename ParserPimplT>
        //static void Process( const CNodeMapDataPtrs& nodeMapDataPtrs, ::GenApi::Version_1_1::BooleanType_pskel* pskel, ParserPimplT*)
        //{
        //    ::GenApi::Version_1_1::BooleanType_pimpl* pCurrenValueParser = static_cast< ::GenApi::Version_1_1::BooleanType_pimpl*>(pskel);

        //}


        //template<typename ParserPimplT>
        //static void Process( const CNodeMapDataPtrs& nodeMapDataPtrs, ::GenApi::Version_1_1::CommandType_pskel* pskel, ParserPimplT*)
        //{
        //    ::GenApi::Version_1_1::CommandType_pimpl* pCurrenValueParser = static_cast< ::GenApi::Version_1_1::CommandType_pimpl*>(pskel);

        //}


        //template<typename ParserPimplT>
        //static void Process( const CNodeMapDataPtrs& nodeMapDataPtrs, ::GenApi::Version_1_1::EnumerationType_pskel* pskel, ParserPimplT*)
        //{
        //    ::GenApi::Version_1_1::EnumerationType_pimpl* pCurrenValueParser = static_cast< ::GenApi::Version_1_1::EnumerationType_pimpl*>(pskel);

        //}


        //template<typename ParserPimplT>
        //static void Process( const CNodeMapDataPtrs& nodeMapDataPtrs, ::GenApi::Version_1_1::RegisterType_pskel* pskel, ParserPimplT*)
        //{
        //    ::GenApi::Version_1_1::RegisterType_pimpl* pCurrenValueParser = static_cast< ::GenApi::Version_1_1::RegisterType_pimpl*>(pskel);

        //}


        //template<typename ParserPimplT>
        //static void Process( const CNodeMapDataPtrs& nodeMapDataPtrs, ::GenApi::Version_1_1::ConverterType_pskel* pskel, ParserPimplT*)
        //{
        //    ::GenApi::Version_1_1::ConverterType_pimpl* pCurrenValueParser = static_cast< ::GenApi::Version_1_1::ConverterType_pimpl*>(pskel);

        //}


        //template<typename ParserPimplT>
        //static void Process( const CNodeMapDataPtrs& nodeMapDataPtrs, ::GenApi::Version_1_1::IntConverterType_pskel* pskel, ParserPimplT*)
        //{
        //    ::GenApi::Version_1_1::IntConverterType_pimpl* pCurrenValueParser = static_cast< ::GenApi::Version_1_1::IntConverterType_pimpl*>(pskel);

        //}


        //template<typename ParserPimplT>
        //static void Process( const CNodeMapDataPtrs& nodeMapDataPtrs, ::GenApi::Version_1_1::SwissKnifeType_pskel* pskel, ParserPimplT*)
        //{
        //    ::GenApi::Version_1_1::SwissKnifeType_pimpl* pCurrenValueParser = static_cast< ::GenApi::Version_1_1::SwissKnifeType_pimpl*>(pskel);

        //}


        //template<typename ParserPimplT>
        //static void Process( const CNodeMapDataPtrs& nodeMapDataPtrs, ::GenApi::Version_1_1::PortType_pskel* pskel, ParserPimplT*)
        //{
        //    ::GenApi::Version_1_1::PortType_pimpl* pCurrenValueParser = static_cast< ::GenApi::Version_1_1::PortType_pimpl*>(pskel);

        //}


        //template<typename ParserPimplT>
        //static void Process( const CNodeMapDataPtrs& nodeMapDataPtrs, ::GenApi::Version_1_1::ConfRomType_pskel* pskel, ParserPimplT*)
        //{
        //    ::GenApi::Version_1_1::ConfRomType_pimpl* pCurrenValueParser = static_cast< ::GenApi::Version_1_1::ConfRomType_pimpl*>(pskel);

        //}


        //template<typename ParserPimplT>
        //static void Process( const CNodeMapDataPtrs& nodeMapDataPtrs, ::GenApi::Version_1_1::NodeType_pskel* pskel, ParserPimplT*)
        //{
        //    ::GenApi::Version_1_1::NodeType_pimpl* pCurrenValueParser = static_cast< ::GenApi::Version_1_1::NodeType_pimpl*>(pskel);

        //}


        //template<typename ParserPimplT>
        //static void Process( const CNodeMapDataPtrs& nodeMapDataPtrs, ::GenApi::Version_1_1::TextDescType_pskel* pskel, ParserPimplT*)
        //{
        //    ::GenApi::Version_1_1::TextDescType_pimpl* pCurrenValueParser = static_cast< ::GenApi::Version_1_1::TextDescType_pimpl*>(pskel);

        //}


        //template<typename ParserPimplT>
        //static void Process( const CNodeMapDataPtrs& nodeMapDataPtrs, ::GenApi::Version_1_1::IntKeyType_pskel* pskel, ParserPimplT*)
        //{
        //    ::GenApi::Version_1_1::IntKeyType_pimpl* pCurrenValueParser = static_cast< ::GenApi::Version_1_1::IntKeyType_pimpl*>(pskel);

        //}


        //template<typename ParserPimplT>
        //static void Process( const CNodeMapDataPtrs& nodeMapDataPtrs, ::GenApi::Version_1_1::DcamLockType_pskel* pskel, ParserPimplT*)
        //{
        //    ::GenApi::Version_1_1::DcamLockType_pimpl* pCurrenValueParser = static_cast< ::GenApi::Version_1_1::DcamLockType_pimpl*>(pskel);

        //}


        //template<typename ParserPimplT>
        //static void Process( const CNodeMapDataPtrs& nodeMapDataPtrs, ::GenApi::Version_1_1::SmartFeatureAdrType_pskel* pskel, ParserPimplT*)
        //{
        //    ::GenApi::Version_1_1::SmartFeatureAdrType_pimpl* pCurrenValueParser = static_cast< ::GenApi::Version_1_1::SmartFeatureAdrType_pimpl*>(pskel);

        //}


        //template<typename ParserPimplT>
        //static void Process( const CNodeMapDataPtrs& nodeMapDataPtrs, ::GenApi::Version_1_1::GroupType_pskel* pskel, ParserPimplT*)
        //{
        //    ::GenApi::Version_1_1::GroupType_pimpl* pCurrenValueParser = static_cast< ::GenApi::Version_1_1::GroupType_pimpl*>(pskel);

        //}


        //template<typename ParserPimplT>
        //static void Process( const CNodeMapDataPtrs& nodeMapDataPtrs, ::GenApi::Version_1_1::StructRegType_pskel* pskel, ParserPimplT*)
        //{
        //    ::GenApi::Version_1_1::StructRegType_pimpl* pCurrenValueParser = static_cast< ::GenApi::Version_1_1::StructRegType_pimpl*>(pskel);

        //}



    };

#ifdef _MSC_VER
#   pragma warning(pop)
#endif


	Version_t CNodeMapDataBuilder::m_SchemaVersion = { 0 };

}

