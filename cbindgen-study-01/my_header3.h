#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

/// Add two numbers
int32_t add(int32_t a, int32_t b);

/// Print a message
void foo(int32_t a, bool b);

} // extern "C"
