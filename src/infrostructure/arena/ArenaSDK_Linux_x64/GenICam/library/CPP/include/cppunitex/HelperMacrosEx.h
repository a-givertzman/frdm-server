// //////////////////////////////////////////////////////////////////////////
// Header file HelperMacrosEx.h
// (c)Copyright 2004
// //////////////////////////////////////////////////////////////////////////
#ifndef CPPUNIT_EXTENSIONS__HELPERMACROSEX_H
#define CPPUNIT_EXTENSIONS__HELPERMACROSEX_H

#include <cppunit/extensions/HelperMacros.h>
#include "TestCallerEx.h"

#define CPPUNITEX_TEST( testMethod)                        \
    CPPUNIT_TEST_SUITE_ADD_TEST(                           \
        ( new CPPUNIT_NS::TestCallerEx<TestFixtureType>(   \
                  context.getTestNameFor( #testMethod),    \
                  &TestFixtureType::testMethod,            \
                  context.makeFixture() ) ) )

#define CPPUNIT_ASSERT_THROW_EX( expression, ExceptionType )          \
   do {                                                             \
      bool cpputExceptionThrown_ = false;                           \
      try {                                                         \
         expression;                                                \
      } catch ( const ExceptionType &e ) {                          \
        GenICam::ILogger *m_pLogger = &(GenICam::CLog::GetLogger("CppUnit")); \
        cpputExceptionThrown_ = true;                              \
        GCLOGINFO( m_pLogger, "!!!!!!!! CHECK MANUALLY : exception Message = '%s'\n", e.what());  \
      }                                                             \
                                                                    \
      if ( cpputExceptionThrown_ )                                  \
         break;                                                     \
                                                                    \
      CPPUNIT_NS::Asserter::fail(                                   \
                     "Expected exception: " #ExceptionType          \
                     " not thrown.",                                \
                     CPPUNIT_SOURCELINE() );                        \
   } while ( *"a" == 'b' /* 'false' replacement without cond expr is constant warning*/)

#undef CPPUNIT_TEST
#define CPPUNIT_TEST(dummy) Use_CPPUNITEX_TEST


/*! \brief Macro for primitive value comparisons
 * \ingroup Assertions
 */
#define CPPUNIT_ASSERT_DOUBLES_EQUAL_EX(expected,actual)        \
  ( CPPUNIT_NS::assertDoubleEquals( (expected),        \
                                    (actual),          \
                                    std::numeric_limits<double>::epsilon(),           \
                                    CPPUNIT_SOURCELINE() ) )


#endif  // CPPUNIT_EXTENSIONS__HELPERMACROSEX_H
