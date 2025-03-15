#ifndef RTmalloc_h__
#define RTmalloc_h__

#include <new>

#define OVERRIDE_MALLOC_FREE 0

#pragma warning(push)
#pragma warning(disable : 4290)

// This is to make sure to generate prolog and epilog for appverifier to 
// trace stack correctly when looking for memory leak. 
// Otherwise we do not see the real caller.
#pragma optimize("g",off)

// New operator overload
void * operator new(size_t size);

// Delete operator overload
void operator delete(void *ptr);

// New operator overload
void * operator new[](size_t size);

// Delete operator overload
void operator delete[](void *ptr);

#if OVERRIDE_MALLOC_FREE
#ifdef malloc
#undef malloc // To remove the debug overload
#endif // malloc
void * malloc(size_t size);

#ifdef free
#undef free // To remove the debug overload
#endif // free
void free(void * ptr);
#endif // OVERRIDE_MALLOC_FREE

#pragma warning(pop)
#endif // RTmalloc_h__