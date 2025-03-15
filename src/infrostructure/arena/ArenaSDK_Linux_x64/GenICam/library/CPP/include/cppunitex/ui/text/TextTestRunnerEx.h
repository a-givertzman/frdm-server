#ifndef CPPUNITEX_UI_TEXT_TEXTTESTRUNNEREX_H
#define CPPUNITEX_UI_TEXT_TEXTTESTRUNNEREX_H//
#include <cppunit/ui/text/TextTestRunner.h>

CPPUNIT_NS_BEGIN

class CPPUNIT_API TextTestRunnerEx : public TextTestRunner
{
public:
    TextTestRunnerEx( Outputter *outputter = NULL, bool addConsoleLogger = true);

    bool
    run( std::string testName = "" ,
         bool doWait = false,
         bool doPrintResult = true,
         bool doPrintProgress = true );

};


CPPUNIT_NS_END





#endif
