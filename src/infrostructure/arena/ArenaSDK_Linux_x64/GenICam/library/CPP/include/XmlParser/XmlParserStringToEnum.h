#pragma once

#include <GenApi/Types.h>
#include <string.h>

namespace GENAPI_NAMESPACE
{

    inline ::GENAPI_NAMESPACE::ESign StringToSign(const std::string& text)
    {
        ::GENAPI_NAMESPACE::ESign v =
            static_cast< ::GENAPI_NAMESPACE::ESign> (0);
        const char* s = text.c_str();

        if (strcmp(s, "Signed") == 0)
            v = ::GENAPI_NAMESPACE::Signed;
        else if (strcmp(s, "Unsigned") == 0)
            v = ::GENAPI_NAMESPACE::Unsigned;
        else if (strcmp(s, "_UndefinedSign") == 0)
            v = ::GENAPI_NAMESPACE::_UndefinedSign;
        else
        {
            assert(false); //should not happen, due to parser
        }

        return v;

    }


    inline ::GENAPI_NAMESPACE::EAccessMode StringToAccessMode(const std::string& text)
    {
        ::GENAPI_NAMESPACE::EAccessMode v =
            static_cast< ::GENAPI_NAMESPACE::EAccessMode> (0);
        const char* s = text.c_str();

        if (strcmp(s, "NI") == 0)
            v = ::GENAPI_NAMESPACE::NI;
        else if (strcmp(s, "NA") == 0)
            v = ::GENAPI_NAMESPACE::NA;
        else if (strcmp(s, "WO") == 0)
            v = ::GENAPI_NAMESPACE::WO;
        else if (strcmp(s, "RO") == 0)
            v = ::GENAPI_NAMESPACE::RO;
        else if (strcmp(s, "RW") == 0)
            v = ::GENAPI_NAMESPACE::RW;
        else if (strcmp(s, "_UndefinedAccesMode") == 0)
            v = ::GENAPI_NAMESPACE::_UndefinedAccesMode;
        else if (strcmp(s, "_CycleDetectAccesMode") == 0)
            v = ::GENAPI_NAMESPACE::_CycleDetectAccesMode;
        else
        {
            assert(false); //should not happen, due to parser
        }

        return v;

    }


    inline ::GENAPI_NAMESPACE::EVisibility StringToVisibility(const std::string& text)
    {
        ::GENAPI_NAMESPACE::EVisibility v =
            static_cast< ::GENAPI_NAMESPACE::EVisibility> (0);
        const char* s = text.c_str();

        if (strcmp(s, "Beginner") == 0)
            v = ::GENAPI_NAMESPACE::Beginner;
        else if (strcmp(s, "Expert") == 0)
            v = ::GENAPI_NAMESPACE::Expert;
        else if (strcmp(s, "Guru") == 0)
            v = ::GENAPI_NAMESPACE::Guru;
        else if (strcmp(s, "Invisible") == 0)
            v = ::GENAPI_NAMESPACE::Invisible;
        else if (strcmp(s, "_UndefinedVisibility") == 0)
            v = ::GENAPI_NAMESPACE::_UndefinedVisibility;
        else
        {
            assert(false); //should not happen, due to parser
        }

        return v;

    }


    inline ::GENAPI_NAMESPACE::ECachingMode StringToCachingMode(const std::string& text)
    {
        ::GENAPI_NAMESPACE::ECachingMode v =
            static_cast< ::GENAPI_NAMESPACE::ECachingMode> (0);
        const char* s = text.c_str();

        if (strcmp(s, "NoCache") == 0)
            v = ::GENAPI_NAMESPACE::NoCache;
        else if (strcmp(s, "WriteThrough") == 0)
            v = ::GENAPI_NAMESPACE::WriteThrough;
        else if (strcmp(s, "WriteAround") == 0)
            v = ::GENAPI_NAMESPACE::WriteAround;
        else if (strcmp(s, "_UndefinedCachingMode") == 0)
            v = ::GENAPI_NAMESPACE::_UndefinedCachingMode;
        else
        {
            assert(false); //should not happen, due to parser
        }

        return v;

    }


    inline ::GENAPI_NAMESPACE::ERepresentation StringToRepresentation(const std::string& text)
    {
        ::GENAPI_NAMESPACE::ERepresentation v =
            static_cast< ::GENAPI_NAMESPACE::ERepresentation> (0);
        const char* s = text.c_str();

        if (strcmp(s, "Linear") == 0)
            v = ::GENAPI_NAMESPACE::Linear;
        else if (strcmp(s, "Logarithmic") == 0)
            v = ::GENAPI_NAMESPACE::Logarithmic;
        else if (strcmp(s, "Boolean") == 0)
            v = ::GENAPI_NAMESPACE::Boolean;
        else if (strcmp(s, "PureNumber") == 0)
            v = ::GENAPI_NAMESPACE::PureNumber;
        else if (strcmp(s, "HexNumber") == 0)
            v = ::GENAPI_NAMESPACE::HexNumber;
        else if (strcmp(s, "IPV4Address") == 0)
            v = ::GENAPI_NAMESPACE::IPV4Address;
        else if (strcmp(s, "MACAddress") == 0)
            v = ::GENAPI_NAMESPACE::MACAddress;
        else if (strcmp(s, "_UndefinedRepresentation") == 0)
            v = ::GENAPI_NAMESPACE::_UndefinedRepresentation;
        else
        {
            assert(false); //should not happen, due to parser
        }

        return v;

    }


    inline ::GENAPI_NAMESPACE::EEndianess StringToEndianess(const std::string& text)
    {
        ::GENAPI_NAMESPACE::EEndianess v =
            static_cast< ::GENAPI_NAMESPACE::EEndianess> (0);
        const char* s = text.c_str();

        if (strcmp(s, "BigEndian") == 0)
            v = ::GENAPI_NAMESPACE::BigEndian;
        else if (strcmp(s, "LittleEndian") == 0)
            v = ::GENAPI_NAMESPACE::LittleEndian;
        else if (strcmp(s, "_UndefinedEndian") == 0)
            v = ::GENAPI_NAMESPACE::_UndefinedEndian;
        else
        {
            assert(false); //should not happen, due to parser
        }

        return v;

    }


    inline ::GENAPI_NAMESPACE::ENameSpace StringToNameSpace(const std::string& text)
    {
        ::GENAPI_NAMESPACE::ENameSpace v =
            static_cast< ::GENAPI_NAMESPACE::ENameSpace> (0);
        const char* s = text.c_str();

        if (strcmp(s, "Custom") == 0)
            v = ::GENAPI_NAMESPACE::Custom;
        else if (strcmp(s, "Standard") == 0)
            v = ::GENAPI_NAMESPACE::Standard;
        else if (strcmp(s, "_UndefinedNameSpace") == 0)
            v = ::GENAPI_NAMESPACE::_UndefinedNameSpace;
        else
        {
            assert(false); //should not happen, due to parser
        }

        return v;

    }


    inline ::GENAPI_NAMESPACE::EStandardNameSpace StringToStandardNameSpace(const std::string& text)
    {
        ::GENAPI_NAMESPACE::EStandardNameSpace v =
            static_cast< ::GENAPI_NAMESPACE::EStandardNameSpace> (0);
        const char* s = text.c_str();

        if (strcmp(s, "None") == 0)
            v = ::GENAPI_NAMESPACE::None;
        else if (strcmp(s, "GEV") == 0)
            v = ::GENAPI_NAMESPACE::GEV;
        else if (strcmp(s, "IIDC") == 0)
            v = ::GENAPI_NAMESPACE::IIDC;
        else if (strcmp(s, "CL") == 0)
            v = ::GENAPI_NAMESPACE::CL;
        else if (strcmp(s, "USB") == 0)
            v = ::GENAPI_NAMESPACE::USB;
        else if (strcmp(s, "_UndefinedStandardNameSpace") == 0)
            v = ::GENAPI_NAMESPACE::_UndefinedStandardNameSpace;
        else
        {
            assert(false); //should not happen, due to parser
        }

        return v;

    }


    inline ::GENAPI_NAMESPACE::EYesNo StringToYesNo(const std::string& text)
    {
        ::GENAPI_NAMESPACE::EYesNo v =
            static_cast< ::GENAPI_NAMESPACE::EYesNo> (0);
        const char* s = text.c_str();

        if (strcmp(s, "Yes") == 0)
            v = ::GENAPI_NAMESPACE::Yes;
        else if (strcmp(s, "No") == 0)
            v = ::GENAPI_NAMESPACE::No;
        else if (strcmp(s, "_UndefinedYesNo") == 0)
            v = ::GENAPI_NAMESPACE::_UndefinedYesNo;
        else
        {
            assert(false); //should not happen, due to parser
        }

        return v;

    }


    inline ::GENAPI_NAMESPACE::ESlope StringToSlope(const std::string& text)
    {
        ::GENAPI_NAMESPACE::ESlope v =
            static_cast< ::GENAPI_NAMESPACE::ESlope> (0);
        const char* s = text.c_str();

        if (strcmp(s, "Increasing") == 0)
            v = ::GENAPI_NAMESPACE::Increasing;
        else if (strcmp(s, "Decreasing") == 0)
            v = ::GENAPI_NAMESPACE::Decreasing;
        else if (strcmp(s, "Varying") == 0)
            v = ::GENAPI_NAMESPACE::Varying;
        else if (strcmp(s, "Automatic") == 0)
            v = ::GENAPI_NAMESPACE::Automatic;
        else if (strcmp(s, "_UndefinedESlope") == 0)
            v = ::GENAPI_NAMESPACE::_UndefinedESlope;
        else
        {
            assert(false); //should not happen, due to parser
        }

        return v;

    }


    inline ::GENAPI_NAMESPACE::EXMLValidation StringToXMLValidation(const std::string& text)
    {
        ::GENAPI_NAMESPACE::EXMLValidation v =
            static_cast< ::GENAPI_NAMESPACE::EXMLValidation> (0);
        const char* s = text.c_str();

        if (strcmp(s, "xvLoad") == 0)
            v = ::GENAPI_NAMESPACE::xvLoad;
        else if (strcmp(s, "xvCycles") == 0)
            v = ::GENAPI_NAMESPACE::xvCycles;
        else if (strcmp(s, "xvSFNC") == 0)
            v = ::GENAPI_NAMESPACE::xvSFNC;
        else if (strcmp(s, "xvDefault") == 0)
            v = ::GENAPI_NAMESPACE::xvDefault;
        else if (strcmp(s, "xvAll") == 0)
            v = ::GENAPI_NAMESPACE::xvAll;
        else if (strcmp(s, "_UndefinedEXMLValidation") == 0)
            v = ::GENAPI_NAMESPACE::_UndefinedEXMLValidation;
        else
        {
            assert(false); //should not happen, due to parser
        }

        return v;

    }


    inline ::GENAPI_NAMESPACE::EDisplayNotation StringToDisplayNotation(const std::string& text)
    {
        ::GENAPI_NAMESPACE::EDisplayNotation v =
            static_cast< ::GENAPI_NAMESPACE::EDisplayNotation> (0);
        const char* s = text.c_str();

        if (strcmp(s, "Automatic") == 0)
            v = ::GENAPI_NAMESPACE::fnAutomatic;
        else if (strcmp(s, "Fixed") == 0)
            v = ::GENAPI_NAMESPACE::fnFixed;
        else if (strcmp(s, "Scientific") == 0)
            v = ::GENAPI_NAMESPACE::fnScientific;
        else if (strcmp(s, "_UndefinedEDisplayNotation") == 0)
            v = ::GENAPI_NAMESPACE::_UndefinedEDisplayNotation;
        else
        {
            assert(false); //should not happen, due to parser
        }

        return v;

    }


    inline ::GENAPI_NAMESPACE::EInterfaceType StringToInterfaceType(const std::string& text)
    {
        ::GENAPI_NAMESPACE::EInterfaceType v =
            static_cast< ::GENAPI_NAMESPACE::EInterfaceType> (0);
        const char* s = text.c_str();

        if (strcmp(s, "intfIValue") == 0)
            v = ::GENAPI_NAMESPACE::intfIValue;
        else if (strcmp(s, "intfIBase") == 0)
            v = ::GENAPI_NAMESPACE::intfIBase;
        else if (strcmp(s, "intfIInteger") == 0)
            v = ::GENAPI_NAMESPACE::intfIInteger;
        else if (strcmp(s, "intfIBoolean") == 0)
            v = ::GENAPI_NAMESPACE::intfIBoolean;
        else if (strcmp(s, "intfICommand") == 0)
            v = ::GENAPI_NAMESPACE::intfICommand;
        else if (strcmp(s, "intfIFloat") == 0)
            v = ::GENAPI_NAMESPACE::intfIFloat;
        else if (strcmp(s, "intfIString") == 0)
            v = ::GENAPI_NAMESPACE::intfIString;
        else if (strcmp(s, "intfIRegister") == 0)
            v = ::GENAPI_NAMESPACE::intfIRegister;
        else if (strcmp(s, "intfICategory") == 0)
            v = ::GENAPI_NAMESPACE::intfICategory;
        else if (strcmp(s, "intfIEnumeration") == 0)
            v = ::GENAPI_NAMESPACE::intfIEnumeration;
        else if (strcmp(s, "intfIEnumEntry") == 0)
            v = ::GENAPI_NAMESPACE::intfIEnumEntry;
        else if (strcmp(s, "intfIPort") == 0)
            v = ::GENAPI_NAMESPACE::intfIPort;
        else
        {
            assert(false); //should not happen, due to parser
        }

        return v;

    }


    inline ::GENAPI_NAMESPACE::ELinkType StringToLinkType(const std::string& text)
    {
        ::GENAPI_NAMESPACE::ELinkType v =
            static_cast< ::GENAPI_NAMESPACE::ELinkType> (0);
        const char* s = text.c_str();

        if (strcmp(s, "ctParentNodes") == 0)
            v = ::GENAPI_NAMESPACE::ctParentNodes;
        else if (strcmp(s, "ctReadingChildren") == 0)
            v = ::GENAPI_NAMESPACE::ctReadingChildren;
        else if (strcmp(s, "ctWritingChildren") == 0)
            v = ::GENAPI_NAMESPACE::ctWritingChildren;
        else if (strcmp(s, "ctInvalidatingChildren") == 0)
            v = ::GENAPI_NAMESPACE::ctInvalidatingChildren;
        else if (strcmp(s, "ctDependingNodes") == 0)
            v = ::GENAPI_NAMESPACE::ctDependingNodes;
        else if (strcmp(s, "ctTerminalNodes") == 0)
            v = ::GENAPI_NAMESPACE::ctTerminalNodes;
        else
        {
            assert(false); //should not happen, due to parser
        }

        return v;

    }


    inline ::GENAPI_NAMESPACE::EInputDirection StringToInputDirection(const std::string& text)
    {
        ::GENAPI_NAMESPACE::EInputDirection v =
            static_cast< ::GENAPI_NAMESPACE::EInputDirection> (0);
        const char* s = text.c_str();

        if (strcmp(s, "idFrom") == 0)
            v = ::GENAPI_NAMESPACE::idFrom;
        else if (strcmp(s, "idTo") == 0)
            v = ::GENAPI_NAMESPACE::idTo;
        else if (strcmp(s, "idNone") == 0)
            v = ::GENAPI_NAMESPACE::idNone;
        else
        {
            assert(false); //should not happen, due to parser
        }

        return v;

    }




    inline std::string EnumToString(::GENAPI_NAMESPACE::ESign e)
    {
        switch (e)
        {
        case ::GENAPI_NAMESPACE::Signed:
            return "Signed";
        case ::GENAPI_NAMESPACE::Unsigned:
            return "Unsigned";
        case ::GENAPI_NAMESPACE::_UndefinedSign:
            return "_UndefinedSign";
        default:
            assert(false);
        }

        return "ESign?";

    }


    inline std::string EnumToString(::GENAPI_NAMESPACE::EAccessMode e)
    {
        switch (e)
        {
        case ::GENAPI_NAMESPACE::NI:
            return "NI";
        case ::GENAPI_NAMESPACE::NA:
            return "NA";
        case ::GENAPI_NAMESPACE::WO:
            return "WO";
        case ::GENAPI_NAMESPACE::RO:
            return "RO";
        case ::GENAPI_NAMESPACE::RW:
            return "RW";
        case ::GENAPI_NAMESPACE::_UndefinedAccesMode:
            return "_UndefinedAccesMode";
        case ::GENAPI_NAMESPACE::_CycleDetectAccesMode:
            return "_CycleDetectAccesMode";
        default:
            assert(false);
        }

        return "EAccessMode?";

    }


    inline std::string EnumToString(::GENAPI_NAMESPACE::EVisibility e)
    {
        switch (e)
        {
        case ::GENAPI_NAMESPACE::Beginner:
            return "Beginner";
        case ::GENAPI_NAMESPACE::Expert:
            return "Expert";
        case ::GENAPI_NAMESPACE::Guru:
            return "Guru";
        case ::GENAPI_NAMESPACE::Invisible:
            return "Invisible";
        case ::GENAPI_NAMESPACE::_UndefinedVisibility:
            return "_UndefinedVisibility";
        default:
            assert(false);
        }

        return "EVisibility?";

    }


    inline std::string EnumToString(::GENAPI_NAMESPACE::ECachingMode e)
    {
        switch (e)
        {
        case ::GENAPI_NAMESPACE::NoCache:
            return "NoCache";
        case ::GENAPI_NAMESPACE::WriteThrough:
            return "WriteThrough";
        case ::GENAPI_NAMESPACE::WriteAround:
            return "WriteAround";
        case ::GENAPI_NAMESPACE::_UndefinedCachingMode:
            return "_UndefinedCachingMode";
        default:
            assert(false);
        }

        return "ECachingMode?";

    }


    inline std::string EnumToString(::GENAPI_NAMESPACE::ERepresentation e)
    {
        switch (e)
        {
        case ::GENAPI_NAMESPACE::Linear:
            return "Linear";
        case ::GENAPI_NAMESPACE::Logarithmic:
            return "Logarithmic";
        case ::GENAPI_NAMESPACE::Boolean:
            return "Boolean";
        case ::GENAPI_NAMESPACE::PureNumber:
            return "PureNumber";
        case ::GENAPI_NAMESPACE::HexNumber:
            return "HexNumber";
        case ::GENAPI_NAMESPACE::IPV4Address:
            return "IPV4Address";
        case ::GENAPI_NAMESPACE::MACAddress:
            return "MACAddress";
        case ::GENAPI_NAMESPACE::_UndefinedRepresentation:
            return "_UndefinedRepresentation";
        default:
            assert(false);
        }

        return "ERepresentation?";

    }


    inline std::string EnumToString(::GENAPI_NAMESPACE::EEndianess e)
    {
        switch (e)
        {
        case ::GENAPI_NAMESPACE::BigEndian:
            return "BigEndian";
        case ::GENAPI_NAMESPACE::LittleEndian:
            return "LittleEndian";
        case ::GENAPI_NAMESPACE::_UndefinedEndian:
            return "_UndefinedEndian";
        default:
            assert(false);
        }

        return "EEndianess?";

    }


    inline std::string EnumToString(::GENAPI_NAMESPACE::ENameSpace e)
    {
        switch (e)
        {
        case ::GENAPI_NAMESPACE::Custom:
            return "Custom";
        case ::GENAPI_NAMESPACE::Standard:
            return "Standard";
        case ::GENAPI_NAMESPACE::_UndefinedNameSpace:
            return "_UndefinedNameSpace";
        default:
            assert(false);
        }

        return "ENameSpace?";

    }


    inline std::string EnumToString(::GENAPI_NAMESPACE::EStandardNameSpace e)
    {
        switch (e)
        {
        case ::GENAPI_NAMESPACE::None:
            return "None";
        case ::GENAPI_NAMESPACE::GEV:
            return "GEV";
        case ::GENAPI_NAMESPACE::IIDC:
            return "IIDC";
        case ::GENAPI_NAMESPACE::CL:
            return "CL";
        case ::GENAPI_NAMESPACE::USB:
            return "USB";
        case ::GENAPI_NAMESPACE::_UndefinedStandardNameSpace:
            return "_UndefinedStandardNameSpace";
        default:
            assert(false);
        }

        return "EStandardNameSpace?";

    }


    inline std::string EnumToString(::GENAPI_NAMESPACE::EYesNo e)
    {
        switch (e)
        {
        case ::GENAPI_NAMESPACE::Yes:
            return "Yes";
        case ::GENAPI_NAMESPACE::No:
            return "No";
        case ::GENAPI_NAMESPACE::_UndefinedYesNo:
            return "_UndefinedYesNo";
        default:
            assert(false);
        }

        return "EYesNo?";

    }


    inline std::string EnumToString(::GENAPI_NAMESPACE::ESlope e)
    {
        switch (e)
        {
        case ::GENAPI_NAMESPACE::Increasing:
            return "Increasing";
        case ::GENAPI_NAMESPACE::Decreasing:
            return "Decreasing";
        case ::GENAPI_NAMESPACE::Varying:
            return "Varying";
        case ::GENAPI_NAMESPACE::Automatic:
            return "Automatic";
        case ::GENAPI_NAMESPACE::_UndefinedESlope:
            return "_UndefinedESlope";
        default:
            assert(false);
        }

        return "ESlope?";

    }


    inline std::string EnumToString(::GENAPI_NAMESPACE::EXMLValidation e)
    {
        switch (e)
        {
        case ::GENAPI_NAMESPACE::xvLoad:
            return "xvLoad";
        case ::GENAPI_NAMESPACE::xvCycles:
            return "xvCycles";
        case ::GENAPI_NAMESPACE::xvSFNC:
            return "xvSFNC";
        case ::GENAPI_NAMESPACE::xvDefault:
            return "xvDefault";
        case ::GENAPI_NAMESPACE::xvAll:
            return "xvAll";
        case ::GENAPI_NAMESPACE::_UndefinedEXMLValidation:
            return "_UndefinedEXMLValidation";
        default:
            assert(false);
        }

        return "EXMLValidation?";

    }


    inline std::string EnumToString(::GENAPI_NAMESPACE::EDisplayNotation e)
    {
        switch (e)
        {
        case ::GENAPI_NAMESPACE::fnAutomatic:
            return "Automatic";
        case ::GENAPI_NAMESPACE::fnFixed:
            return "Fixed";
        case ::GENAPI_NAMESPACE::fnScientific:
            return "Scientific";
        case ::GENAPI_NAMESPACE::_UndefinedEDisplayNotation:
            return "_UndefinedEDisplayNotation";
        default:
            assert(false);
        }

        return "EDisplayNotation?";

    }


    inline std::string EnumToString(::GENAPI_NAMESPACE::EInterfaceType e)
    {
        switch (e)
        {
        case ::GENAPI_NAMESPACE::intfIValue:
            return "intfIValue";
        case ::GENAPI_NAMESPACE::intfIBase:
            return "intfIBase";
        case ::GENAPI_NAMESPACE::intfIInteger:
            return "intfIInteger";
        case ::GENAPI_NAMESPACE::intfIBoolean:
            return "intfIBoolean";
        case ::GENAPI_NAMESPACE::intfICommand:
            return "intfICommand";
        case ::GENAPI_NAMESPACE::intfIFloat:
            return "intfIFloat";
        case ::GENAPI_NAMESPACE::intfIString:
            return "intfIString";
        case ::GENAPI_NAMESPACE::intfIRegister:
            return "intfIRegister";
        case ::GENAPI_NAMESPACE::intfICategory:
            return "intfICategory";
        case ::GENAPI_NAMESPACE::intfIEnumeration:
            return "intfIEnumeration";
        case ::GENAPI_NAMESPACE::intfIEnumEntry:
            return "intfIEnumEntry";
        case ::GENAPI_NAMESPACE::intfIPort:
            return "intfIPort";
        default:
            assert(false);
        }

        return "EInterfaceType?";

    }


    inline std::string EnumToString(::GENAPI_NAMESPACE::ELinkType e)
    {
        switch (e)
        {
        case ::GENAPI_NAMESPACE::ctParentNodes:
            return "ctParentNodes";
        case ::GENAPI_NAMESPACE::ctReadingChildren:
            return "ctReadingChildren";
        case ::GENAPI_NAMESPACE::ctWritingChildren:
            return "ctWritingChildren";
        case ::GENAPI_NAMESPACE::ctInvalidatingChildren:
            return "ctInvalidatingChildren";
        case ::GENAPI_NAMESPACE::ctDependingNodes:
            return "ctDependingNodes";
        case ::GENAPI_NAMESPACE::ctTerminalNodes:
            return "ctTerminalNodes";
        default:
            assert(false);
        }

        return "ELinkType?";

    }


    inline std::string EnumToString(::GENAPI_NAMESPACE::EInputDirection e)
    {
        switch (e)
        {
        case ::GENAPI_NAMESPACE::idFrom:
            return "From";
        case ::GENAPI_NAMESPACE::idTo:
            return "To";
        case ::GENAPI_NAMESPACE::idNone:
            return "None";
        default:
            assert(false);
        }

        return "EInputDirection?";

    }

}
