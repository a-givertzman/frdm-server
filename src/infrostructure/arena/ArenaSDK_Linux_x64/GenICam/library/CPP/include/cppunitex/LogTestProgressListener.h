#ifndef CPPUNIT_LOGTESTPROGRESSLISTENER_H
#define CPPUNIT_LOGTESTPROGRESSLISTENER_H

#include <cppunit/TestListener.h>
#include "LogEx.h"


CPPUNIT_NS_BEGIN


/*! \brief TestListener that prints the name of each test before running it.
* \ingroup TrackingTestExecution
*/
class CPPUNIT_API LogTestProgressListener : public TestListener
{
public:
    /*! Constructs a LogTestProgressListener object.
    */
    LogTestProgressListener();

    /// Destructor.
    virtual ~LogTestProgressListener();

    void startTest( Test *test );

    void addFailure( const TestFailure &failure );

    void endTest( Test *test );

private:
    /// Prevents the use of the copy constructor.
    LogTestProgressListener( const LogTestProgressListener &copy );

    /// Prevents the use of the copy operator.
    void operator =( const LogTestProgressListener &copy );

private:
    bool m_lastTestFailed;
    GENICAM_NAMESPACE::ILogger* m_pLogger;
};


CPPUNIT_NS_END

#endif  // CPPUNIT_LOGTESTPROGRESSLISTENER_H
