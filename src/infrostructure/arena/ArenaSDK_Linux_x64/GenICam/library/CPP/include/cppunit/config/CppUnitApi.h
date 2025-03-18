#ifndef CPPUNIT_CONFIG_CPPUNITAPI
#define CPPUNIT_CONFIG_CPPUNITAPI

#undef CPPUNIT_API

// define CPPUNIT_DLL_BUILD when building CppUnit dll.
#ifdef CPPUNIT_BUILD_DLL
    #if defined(_MSC_VER)
    #    define CPPUNIT_API __declspec(dllexport)
    #    define CPPUNIT_RTTI_CLASS_API __declspec( dllexport )
    #elif defined (__GNUC__)
    #    if (defined (__linux__) || defined(__APPLE__) || defined(VXWORKS))
    #        define CPPUNIT_API __attribute__((visibility("default")))
    #        define CPPUNIT_RTTI_CLASS_API __attribute__((visibility("default")))
    #   endif
    #else
    #    error Unsupported compiler or platform
    #endif
#else
    // If CPPUNIT_DLL_BUILD is not defined, we are linking to it
    #if defined(_MSC_VER)
    #    define CPPUNIT_API __declspec(dllimport)
    #    define CPPUNIT_RTTI_CLASS_API __declspec( dllimport )
    #elif defined (__GNUC__)
    #    if (defined (__linux__) || defined(__APPLE__) || defined(VXWORKS))
             // Classes that need RTTI info across library boundaries (like Exception classes)
             // need to always use default visibility, even when symbols are being imported.
    #        define CPPUNIT_RTTI_CLASS_API __attribute__((visibility("default")))
    #    endif
    #else
    #    error Unsupported compiler or platform
    #endif
#endif

#ifdef CPPUNIT_API
#undef CPPUNIT_NEED_DLL_DECL
#define CPPUNIT_NEED_DLL_DECL 1
#endif

#ifndef CPPUNIT_API
#define CPPUNIT_API
#undef CPPUNIT_NEED_DLL_DECL
#define CPPUNIT_NEED_DLL_DECL 0
#endif


#endif  // CPPUNIT_CONFIG_CPPUNITAPI
