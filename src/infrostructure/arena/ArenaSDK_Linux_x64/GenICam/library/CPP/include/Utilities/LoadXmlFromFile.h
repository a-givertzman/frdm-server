#pragma once

#ifndef INCLUDED_UTILITY_LOADXMLFROMFILE
#define INCLUDED_UTILITY_LOADXMLFROMFILE

#include <GenApi/GenApi.h>
#include <Base/GCString.h>
#include <GenApi/NodeMapRef.h>
#include <Utilities/GenICamTestRoot.h>


template<class TCameraParams>
inline void LoadTestXML(GENAPI_NAMESPACE::CNodeMapRefT<TCameraParams> &Ref, const GENICAM_NAMESPACE::gcstring &TestCategory, const GENICAM_NAMESPACE::gcstring &FileName)
{
    Ref._LoadXMLFromFile(GenICam::Test::MakeTestXMLPath(TestCategory, FileName));
}

#endif // INCLUDED_UTILITY_LOADXMLFROMFILE

