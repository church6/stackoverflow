// Mon Dec  5 10:33:53 AM HKT 2022
// Church.ZHONG

// https://learn.microsoft.com/en-us/cpp/c-language/cpp-integer-limits?view=msvc-170

// Constant	Meaning	Value

// CHAR_BIT	Number of bits in the smallest variable that is not a bit field.
// 8

// SCHAR_MIN	Minimum value for a variable of type signed char.	-128

// SCHAR_MAX	Maximum value for a variable of type signed char.	127

// UCHAR_MAX	Maximum value for a variable of type unsigned char.	255
// (0xff)

// CHAR_MIN	Minimum value for a variable of type char.	-128; 0 if /J
// option used

// CHAR_MAX	Maximum value for a variable of type char.	127; 255 if /J
// option used

// MB_LEN_MAX	Maximum number of bytes in a multicharacter constant.	5

// SHRT_MIN	Minimum value for a variable of type short.	-32768

// SHRT_MAX	Maximum value for a variable of type short.	32767

// USHRT_MAX	Maximum value for a variable of type unsigned short.	65535
// (0xffff)

// INT_MIN	Minimum value for a variable of type int.	-2147483647 - 1

// INT_MAX	Maximum value for a variable of type int.	2147483647

// UINT_MAX	Maximum value for a variable of type unsigned int. 4294967295
// (0xffffffff)

// LONG_MIN	Minimum value for a variable of type long.	-2147483647 - 1

// LONG_MAX	Maximum value for a variable of type long.	2147483647

// ULONG_MAX	Maximum value for a variable of type unsigned long. 4294967295
// (0xffffffff)

// LLONG_MIN	Minimum value for a variable of type long long.
// -9,223,372,036,854,775,807 - 1

// LLONG_MAX	Maximum value for a variable of type long long.
// 9,223,372,036,854,775,807

// ULLONG_MAX	Maximum value for a variable of type unsigned long long.
// 18,446,744,073,709,551,615 (0xffffffffffffffff)

#include <climits>
#include <cstdio>
#include <limits>

#undef NDEBUG
//#define NDEBUG
#include <cassert>

// Check windows
#if _WIN32 || _WIN64
#if _WIN64
#define ENVIRONMENT64
#else
#define ENVIRONMENT32
#endif
#endif

// Check GCC
#if __GNUC__
#if __x86_64__ || __ppc64__
#define ENVIRONMENT64
#else
#define ENVIRONMENT32
#endif
#endif

int main(int argc, char *argv[]) {
  assert(8 == CHAR_BIT);
  assert(-128 == SCHAR_MIN);
  assert(127 == SCHAR_MAX);
  assert(0xff == UCHAR_MAX);
  assert(-128 == CHAR_MIN);
  assert(127 == CHAR_MAX);
#ifdef ENVIRONMENT32
  assert(5 == MB_LEN_MAX);
#else
  assert(16 == MB_LEN_MAX);
#endif
  assert(-32768 == SHRT_MIN);
  assert(32767 == SHRT_MAX);
  assert(65535 == USHRT_MAX);
  assert((-2147483647 - 1) == INT_MIN);
  assert(2147483647 == INT_MAX);
  assert(0xffffffff == UINT_MAX); // 4294967295 (0xffffffff)
  // FIXME
  // assert((-2147483647 - 1) == LONG_MIN);
  // FIXME
  // assert(2147483647 == LONG_MAX);
  // FIXME
  // assert(0xffffffff == ULONG_MAX); // 4294967295 (0xffffffff)
  assert((-9223372036854775807L - 1) == LLONG_MIN);
  assert(9223372036854775807L == LLONG_MAX);
  assert(0xffffffffffffffff ==
         ULLONG_MAX); // 18,446,744,073,709,551,615 (0xffffffffffffffff)

  printf("MB_LEN_MAX=%d\n", MB_LEN_MAX);
  printf("OK\n");
  // FIXME
  //  assert(0 == 1);

  return 0;
}
