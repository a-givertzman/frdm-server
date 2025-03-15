#ifndef CPPUNIT_TESTCALLEREX_H    // -*- C++ -*-
#define CPPUNIT_TESTCALLEREX_H

#include <iostream>

#include <cppunit/Exception.h>
#include "TestCaseEx.h"

#if CPPUNIT_USE_TYPEINFO_NAME
#  include <cppunit/extensions/TypeInfoHelper.h>
#endif

using namespace std;

CPPUNIT_NS_BEGIN

/*! \brief Generate a test case from a fixture method with call to paramSetup.
* \ingroup WritingTestFixture
*/

template <class Fixture>
class TestCallerEx : public TestCaseEx
{
    typedef void (Fixture::*TestMethod)();

public:
    /*!
    * Constructor for TestCallerEx. This constructor builds a new Fixture
    * instance owned by the TestCallerEx.
    * \param name name of this TestCallerEx
    * \param test the method this TestCallerEx calls in runTest()
    */
    TestCallerEx( std::string name, TestMethod test ) :
      TestCaseEx( name ),
          m_ownFixture( true ),
          m_fixture( new Fixture() ),
          m_test( test )
      {
      }

      /*!
      * Constructor for TestCallerEx.
      * This constructor does not create a new Fixture instance but accepts
      * an existing one as parameter. The TestCallerEx will not own the
      * Fixture object.
      * \param name name of this TestCallerEx
      * \param test the method this TestCallerEx calls in runTest()
      * \param fixture the Fixture to invoke the test method on.
      */
      TestCallerEx(std::string name, TestMethod test, Fixture& fixture) :
      TestCaseEx( name ),
          m_ownFixture( false ),
          m_fixture( &fixture ),
          m_test( test )
      {
      }

      /*!
      * Constructor for TestCallerEx.
      * This constructor does not create a new Fixture instance but accepts
      * an existing one as parameter. The TestCallerEx will own the
      * Fixture object and delete it in its destructor.
      * \param name name of this TestCallerEx
      * \param test the method this TestCallerEx calls in runTest()
      * \param fixture the Fixture to invoke the test method on.
      */
      TestCallerEx(std::string name, TestMethod test, Fixture* fixture) :
      TestCaseEx( name ),
          m_ownFixture( true ),
          m_fixture( fixture ),
          m_test( test )
      {
          m_pLogger = &(GenICam::CLog::GetLogger("CppUnit"));
      }

      ~TestCallerEx()
      {
          if (m_ownFixture)
              delete m_fixture;
      }

      void runTest()
      {
          if( !m_skipTest )
          {
              (m_fixture->*m_test)();
          }
      }

      void setUp()
      {
          std::string strPropertyFileName = getName();
          size_t pos;
          while( (pos=strPropertyFileName.find("::")) != std::string::npos ) {
              strPropertyFileName.erase(pos, 2);
              strPropertyFileName.insert(pos, "_");
          };
          strPropertyFileName.append(".props");
          // log the name of the property file

          GCLOGINFO( m_pLogger, "*** Property file: %s", strPropertyFileName.c_str());
          m_fixture->openParams(strPropertyFileName);

          // see whether to skip the test or not.
          // skipping the test is forced, when "cppunit_skip_test" property
          // is set to "yes" (case sensitive!),
          // e.g. in property file: "cppunit_skip_test=yes"
          //std::string skip_str = m_fixture->getParams().getProperty("cppunit_skip_test", "no");
          std::string skip_str = "no";  // temp
          if( skip_str==std::string("yes") )
          {
              GCLOGINFO( m_pLogger, "*** Test : %s : SKIPPED\n", getName().c_str() );
              m_skipTest = true;
          }
          else
          {
              m_skipTest = false;
          }

          if( !m_skipTest )
          {
              m_fixture->setUp ();
          }
      }

      void tearDown()
      {
          if( !m_skipTest )
          {
              m_fixture->tearDown ();
          }
          m_fixture->closeParams();
      }

      std::string toString() const
      {
          return "TestCallerEx " + getName();
      }

      GenICam::ILogger *m_pLogger;
private:
    TestCallerEx( const TestCallerEx &other );
    TestCallerEx &operator =( const TestCallerEx &other );

private:
    bool m_skipTest;
    bool m_ownFixture;
    Fixture *m_fixture;
    TestMethod m_test;

};



CPPUNIT_NS_END

#endif // CPPUNIT_TESTCALLER_H
