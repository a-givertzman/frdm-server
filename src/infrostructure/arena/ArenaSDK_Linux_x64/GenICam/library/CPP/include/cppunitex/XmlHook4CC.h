#pragma once
#include <cppunit/XmlOutputterHook.h>

class CPPUNIT_API  CXmlHook4CC :
    public CppUnit::XmlOutputterHook
{
public:
    CXmlHook4CC(void);
    ~CXmlHook4CC(void);
private:
    // overrides
    /* virtual */ void beginDocument( CppUnit::XmlDocument *document );
    /* virtual */ void endDocument( CppUnit::XmlDocument *document );
    /* virtual */ void failTestAdded( CppUnit::XmlDocument* document
        , CppUnit::XmlElement * testElement
        , CppUnit::Test * test
        , CppUnit::TestFailure * failure );
    /* virtual */ void successfulTestAdded( CppUnit::XmlDocument *document
        , CppUnit::XmlElement *testElement
        , CppUnit::Test *test );

private:
    CppUnit::XmlElement *m_Failed;
    CppUnit::XmlElement *m_Succeeded;
};
