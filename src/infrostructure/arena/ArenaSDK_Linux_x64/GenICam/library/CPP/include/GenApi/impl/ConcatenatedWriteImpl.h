//-----------------------------------------------------------------------------
//  (c) 2017 by Teledyne DALSA
//  Section: Digital Imaging
//  Project: GenAPI
//  Author:  Eric Bourbonnais
//
//  License: This file is published under the license of the EMVA GenICam  Standard Group. 
//  A text file describing the legal terms is included in  your installation as 'GenICam_license.pdf'. 
//  If for some reason you are missing  this file please contact the EMVA or visit the website
//  (http://www.genicam.org) for a full copy.
// 
//  THIS SOFTWARE IS PROVIDED BY THE EMVA GENICAM STANDARD GROUP "AS IS"
//  AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,  
//  THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR  
//  PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE EMVA GENICAM STANDARD  GROUP 
//  OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,  SPECIAL, 
//  EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT  LIMITED TO, 
//  PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,  DATA, OR PROFITS; 
//  OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY  THEORY OF LIABILITY, 
//  WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT  (INCLUDING NEGLIGENCE OR OTHERWISE) 
//  ARISING IN ANY WAY OUT OF THE USE  OF THIS SOFTWARE, EVEN IF ADVISED OF THE 
//  POSSIBILITY OF SUCH DAMAGE.
//-----------------------------------------------------------------------------
//
/*!
\file
\brief   This file contains the implementation definition of the node write concatenator classes.
\ingroup GenApi_PublicInterface
*/

#ifndef GENAPI_NODE_WRITE_CONCATENATOR_IMPL_H
#define GENAPI_NODE_WRITE_CONCATENATOR_IMPL_H

#include "GenApi/ConcatenatedWrite.h"


namespace GENAPI_NAMESPACE
{
    class CValuePolyContainer
    {
    public:
        typedef enum
        {
            VALUE_POLY_TYPE_INT,
            VALUE_POLY_TYPE_DOUBLE,
            VALUE_POLY_TYPE_STR,
            VALUE_POLY_TYPE_BOOL
        }VALUE_POLY_TYPE;
        CValuePolyContainer(const GENICAM_NAMESPACE::gcstring &nodeName, const GENICAM_NAMESPACE::gcstring &nodeValue) : m_type(VALUE_POLY_TYPE_STR), m_nodeName(nodeName), m_nodeValueS(nodeValue) {};
        explicit CValuePolyContainer(const GENICAM_NAMESPACE::gcstring &nodeName, const int64_t nodeValue): m_type(VALUE_POLY_TYPE_INT), m_nodeName(nodeName), m_nodeValueI(nodeValue) {} ;
        explicit CValuePolyContainer(const GENICAM_NAMESPACE::gcstring &nodeName, const double nodeValue) : m_type(VALUE_POLY_TYPE_DOUBLE), m_nodeName(nodeName), m_nodeValueD(nodeValue) {};
        explicit CValuePolyContainer(const GENICAM_NAMESPACE::gcstring &nodeName, const bool   nodeValue) : m_type(VALUE_POLY_TYPE_BOOL), m_nodeName(nodeName), m_nodeValueB(nodeValue) {};

        GENICAM_NAMESPACE::gcstring &GetStr()
        {
            switch (m_type)
            {
            case VALUE_POLY_TYPE_INT:
                Value2String(m_nodeValueI, m_nodeValueS );
                break;
            case VALUE_POLY_TYPE_DOUBLE:
                Value2String(m_nodeValueD, m_nodeValueS );
                break;
            case VALUE_POLY_TYPE_STR:
                break;
            case VALUE_POLY_TYPE_BOOL:
                Value2String(m_nodeValueB, m_nodeValueS );
                break;
            }
            return m_nodeValueS;
        }
        int64_t GetInt()
        {
            switch (m_type)
            {
            case VALUE_POLY_TYPE_INT:
                break;
            case VALUE_POLY_TYPE_DOUBLE:
                m_nodeValueI = (int64_t)m_nodeValueD;
                break;
            case VALUE_POLY_TYPE_STR:
                String2Value(m_nodeValueS, &m_nodeValueI );
                break;
            case VALUE_POLY_TYPE_BOOL:
                m_nodeValueI = (m_nodeValueB == true);
                break;
            }
            return m_nodeValueI;
        }
        double GetFloat()
        {
            switch(m_type)
            {
            case VALUE_POLY_TYPE_INT:
                m_nodeValueD = (double)m_nodeValueI;
                break;
            case VALUE_POLY_TYPE_DOUBLE:
                break;
            case VALUE_POLY_TYPE_STR:
                String2Value(m_nodeValueS, &m_nodeValueD );
                break;
            case VALUE_POLY_TYPE_BOOL:
                m_nodeValueD = (double)(m_nodeValueB == true);
                break;
            }
            return m_nodeValueD;
        }
        bool GetBool()
        {
            switch (m_type)
            {
            case VALUE_POLY_TYPE_INT:
                m_nodeValueB = (m_nodeValueI == 0);
                break;
            case VALUE_POLY_TYPE_DOUBLE:
                m_nodeValueB = (m_nodeValueD == 0);
                break;
            case VALUE_POLY_TYPE_STR:
                String2Value(m_nodeValueS, &m_nodeValueB);
                break;
            case VALUE_POLY_TYPE_BOOL:
                break;
            }
            return m_nodeValueB;
        }
        VALUE_POLY_TYPE GetType() const { return m_type; }
        const GENICAM_NAMESPACE::gcstring& GetNodeName() const { return m_nodeName; }
    private:
        VALUE_POLY_TYPE m_type;
        GENICAM_NAMESPACE::gcstring m_nodeName;
        GENICAM_NAMESPACE::gcstring m_nodeValueS;
        int64_t  m_nodeValueI;
        double   m_nodeValueD;
        bool     m_nodeValueB;
    };

    inline CNodeWriteConcatenator::~CNodeWriteConcatenator(void)
    {
    }

    class CNodeWriteConcatenatorImpl : public CNodeWriteConcatenator
    {
    public:
        explicit CNodeWriteConcatenatorImpl() {};
        virtual  ~CNodeWriteConcatenatorImpl(void) {};
        virtual  void Add(const GENICAM_NAMESPACE::gcstring &nodeName, const GENICAM_NAMESPACE::gcstring &nodeValue)
        {
            m_values.push_back(CValuePolyContainer(nodeName, nodeValue));
        }
        virtual  void Add(const GENICAM_NAMESPACE::gcstring &nodeName, const char *nodeValue)
        {
            m_values.push_back(CValuePolyContainer(nodeName, GENICAM_NAMESPACE::gcstring(nodeValue)));
        }
        virtual  void Add( const GENICAM_NAMESPACE::gcstring &nodeName, const int64_t nodeValue)
        {
            m_values.push_back(CValuePolyContainer(nodeName, nodeValue));
        }
        virtual  void Add( const GENICAM_NAMESPACE::gcstring &nodeName, const double nodeValue)
        {
            m_values.push_back(CValuePolyContainer(nodeName, nodeValue));
        }
        virtual  void Add(const GENICAM_NAMESPACE::gcstring &nodeName, const bool nodeValue)
        {
            m_values.push_back(CValuePolyContainer(nodeName, nodeValue));
        }
        virtual  void Clear()
        {
            m_values.clear();
        }
        virtual std::list<CValuePolyContainer>::iterator begin(void)
        {
            return m_values.begin();
        }
        virtual std::list<CValuePolyContainer>::iterator end(void)
        {
            return m_values.end();
        }
        virtual  void Destroy()
        {
            delete this;
        }

    private:
        std::list<CValuePolyContainer> m_values;
    };

}

#endif // GENAPI_NODE_WRITE_CONCATENATOR_IMPL_H
