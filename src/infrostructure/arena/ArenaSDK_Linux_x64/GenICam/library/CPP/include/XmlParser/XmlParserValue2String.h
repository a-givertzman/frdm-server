#pragma once

#include "XmlParser/XmlParser.h"
#include <sstream>
#include <string>
#include <cctype>
#include <iomanip>
#include <Base/GCTypes.h>

namespace GENAPI_NAMESPACE
{

    //! Converts an T property to a string
    template< typename T >
    void XMLParser_Value2String(T Value, std::string &ValueStr)
    {
        static const int precision(36);  // For the MS STL, values > 36 ( == _MAX_SIG_DIG ) are leading to trailing zeroes
        std::ostringstream s;
        s << std::setprecision(precision) << Value;
        ValueStr = s.str();
    }

    //! Converts a string to an T property
    template< typename T >
    bool XMLParser_String2Value(const std::string &ValueStr, T* Value)
    {
        std::istringstream s(ValueStr.c_str());
        if (ValueStr.length() > 2
            && (ValueStr[0] == '0' && (ValueStr[1] == 'x' || ValueStr[1] == 'X')))
        {
            s.ignore(2);
            s >> std::hex >> *Value;
        }
        else
            s >> *Value;

        bool b = (!s.fail());
        return b;
    }

    //! Converts a string to an integer handling hex number
    GENICAMXMLPARSER_DECL bool XMLParser_String2Value(const std::string &ValueStr, int64_t &Value);

    //! Converts a byte array property to a string
    GENICAMXMLPARSER_DECL void XMLParser_Value2String(uint8_t *pValue, std::string &ValueStr, int64_t len);

    //! Converts a string to a byte array
    GENICAMXMLPARSER_DECL bool XMLParser_String2Value(const std::string &ValueStr, uint8_t *pValue, int64_t len);

}
