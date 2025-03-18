#ifndef __LOG_OUTPUTTER_H__
#define __LOG_OUTPUTTER_H__

#ifdef _MSC_VER
#   pragma warning( push )
#   pragma warning( disable: 4251 )  // X needs to have dll-interface to be used by clients of class Z
#endif

#include <cppunit/Outputter.h>
#include "LogEx.h"

CPPUNIT_NS_BEGIN

class Exception;
class SourceLine;
class TestResultCollector;
class TestFailure;

/*! \brief Prints a TestResultCollector to the CppunitEx logger
 */
class CPPUNIT_API LogOutputter : public Outputter
{

public:
  LogOutputter( TestResultCollector *result);

  /// Destructor.
  virtual ~LogOutputter();

  void write();

  virtual void printFailure( TestFailure *failure,
                             int failureNumber );
protected:
  TestResultCollector *m_result;
  GENICAM_NAMESPACE::ILogger* m_pLogger;

private:
  /// Prevents the use of the copy constructor.
  LogOutputter( const LogOutputter &copy );

  /// Prevents the use of the copy operator.
  void operator =( const LogOutputter &copy );
};





CPPUNIT_NS_END

#ifdef _MSC_VER
#   pragma warning( pop )
#endif

#endif
