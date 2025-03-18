#ifndef CPPUNIT_TESTCASEEX_H
#define CPPUNIT_TESTCASEEX_H

#include <cppunit/Portability.h>
#include <cppunit/TestLeaf.h>
#include <cppunit/TestAssert.h>
#include <cppunit/TestFixture.h>
#include <cppunit/TestCase.h>

#include "LogEx.h"

CPPUNIT_NS_BEGIN


class TestResult;

class CPPUNIT_API TestCaseEx : public CPPUNIT_NS::TestCase
{
 //friend class TestCallerEx;

public:
    TestCaseEx();

    TestCaseEx( const std::string &name );

    virtual ~TestCaseEx();

    void openParams(const std::string & fixtureName);

    void closeParams();

    //~log4cplus::helpers::Properties & getParams()
    //~{
    //~ return m_fixtureParams;
    //~};

    //~const log4cplus::helpers::Properties& getParams() const
    //~{
    //~    return m_fixtureParams;
    //~}

protected:
    std::string getFixtureName() const
    {
        return m_fixtureName;
    };

    std::string getPropertyFilename() const
    {
        return m_propertyFilename;
    };

    void debug(const char* stringFormat, ...) throw();

private:
    TestCaseEx( const TestCaseEx &other );
    TestCaseEx &operator=( const TestCaseEx &other );

    std::string m_fixtureName;
    std::string m_propertyFilename;
    //~log4cplus::helpers::Properties m_fixtureParams;
    GENICAM_NAMESPACE::ILogger *m_pLogger;

};


CPPUNIT_NS_END

#endif // CPPUNIT_TESTCASE_H
