#include <math.h>
#include <algorithm>
#include <cstdint>
#include <vector>
#include <unordered_map>
#include <unordered_set>
#include <set>
#include <map>
#include <iostream>
#include <regex>
#include <numeric>
#include <queue>
#include <deque>
#include <functional>
#include <bitset>
#include <ranges>
#include <filesystem>
#include <sstream>
#include <fstream>
#include "rapidjson/document.h"
#include "rapidjson/writer.h"
#include "rapidjson/stringbuffer.h"

using namespace std;

using namespace rapidjson;

typedef unsigned char uchar;
typedef unsigned int uint;
typedef unsigned short ushort;

struct int2 { int2() = default; int2( int a, int b ) : x( a ), y( b ) {} int x, y; };
struct uint2 { uint2() = default; uint2( int a, int b ) : x( a ), y( b ) {} uint x, y; };
struct float2 { float2() = default; float2( float a, float b ) : x( a ), y( b ) {} float x, y; };
struct int3 { int3() = default; int3( int a, int b, int c ) : x( a ), y( b ), z( c ) {} int x, y, z; int dummy; };
struct uint3 { uint3() = default; uint3( uint a, uint b, uint c ) : x( a ), y( b ), z( c ) {} uint x, y, z; uint dummy; };
struct float3 { float3() = default; float3( float a, float b, float c ) : x( a ), y( b ), z( c ) {} float x, y, z; float dummy; };
struct int4 { int x, y, z, w; };
struct uint4 { uint x, y, z, w; };
struct float4 { float x, y, z, w; };
struct uchar4 { uchar x, y, z, w; };

inline float fminf( float a, float b ) { return a < b ? a : b; }
inline float fmaxf( float a, float b ) { return a > b ? a : b; }
inline float rsqrtf( float x ) { return 1.0f / sqrtf( x ); }
inline float sqrf( float x ) { return x * x; }

inline float2 make_float2( const float a, float b ) { float2 f2; f2.x = a, f2.y = b; return f2; }
inline float2 make_float2( const float s ) { return make_float2( s, s ); }
inline float2 make_float2( const float3& a ) { return make_float2( a.x, a.y ); }
inline float2 make_float2( const int2& a ) { return make_float2( float( a.x ), float( a.y ) ); } // explicit casts prevent gcc warnings
inline float2 make_float2( const uint2& a ) { return make_float2( float( a.x ), float( a.y ) ); }
inline int2 make_int2( const int a, const int b ) { int2 i2; i2.x = a, i2.y = b; return i2; }
inline int2 make_int2( const int s ) { return make_int2( s, s ); }
inline int2 make_int2( const int3& a ) { return make_int2( a.x, a.y ); }
inline int2 make_int2( const uint2& a ) { return make_int2( int( a.x ), int( a.y ) ); }
inline int2 make_int2( const float2& a ) { return make_int2( int( a.x ), int( a.y ) ); }
inline uint2 make_uint2( const uint a, const uint b ) { uint2 u2; u2.x = a, u2.y = b; return u2; }
inline uint2 make_uint2( const uint s ) { return make_uint2( s, s ); }
inline uint2 make_uint2( const uint3& a ) { return make_uint2( a.x, a.y ); }
inline uint2 make_uint2( const int2& a ) { return make_uint2( uint( a.x ), uint( a.y ) ); }
inline float3 make_float3( const float& a, const float& b, const float& c ) { float3 f3; f3.x = a, f3.y = b, f3.z = c; return f3; }
inline float3 make_float3( const float& s ) { return make_float3( s, s, s ); }
inline float3 make_float3( const float2& a ) { return make_float3( a.x, a.y, 0.0f ); }
inline float3 make_float3( const float2& a, const float& s ) { return make_float3( a.x, a.y, s ); }
inline float3 make_float3( const float4& a ) { return make_float3( a.x, a.y, a.z ); }
inline float3 make_float3( const int3& a ) { return make_float3( float( a.x ), float( a.y ), float( a.z ) ); }
inline float3 make_float3( const uint3& a ) { return make_float3( float( a.x ), float( a.y ), float( a.z ) ); }
inline int3 make_int3( const int& a, const int& b, const int& c ) { int3 i3; i3.x = a, i3.y = b, i3.z = c; return i3; }
inline int3 make_int3( const int& s ) { return make_int3( s, s, s ); }
inline int3 make_int3( const int2& a ) { return make_int3( a.x, a.y, 0 ); }
inline int3 make_int3( const int2& a, const int& s ) { return make_int3( a.x, a.y, s ); }
inline int3 make_int3( const uint3& a ) { return make_int3( int( a.x ), int( a.y ), int( a.z ) ); }
inline int3 make_int3( const float3& a ) { return make_int3( int( a.x ), int( a.y ), int( a.z ) ); }
inline int3 make_int3( const float4& a ) { return make_int3( int( a.x ), int( a.y ), int( a.z ) ); }
inline uint3 make_uint3( const uint a, uint b, uint c ) { uint3 u3; u3.x = a, u3.y = b, u3.z = c; return u3; }
inline uint3 make_uint3( const uint s ) { return make_uint3( s, s, s ); }
inline uint3 make_uint3( const uint2& a ) { return make_uint3( a.x, a.y, 0 ); }
inline uint3 make_uint3( const uint2& a, const uint s ) { return make_uint3( a.x, a.y, s ); }
inline uint3 make_uint3( const uint4& a ) { return make_uint3( a.x, a.y, a.z ); }
inline uint3 make_uint3( const int3& a ) { return make_uint3( uint( a.x ), uint( a.y ), uint( a.z ) ); }
inline float4 make_float4( const float a, const float b, const float c, const float d ) { float4 f4; f4.x = a, f4.y = b, f4.z = c, f4.w = d; return f4; }
inline float4 make_float4( const float s ) { return make_float4( s, s, s, s ); }
inline float4 make_float4( const float3& a ) { return make_float4( a.x, a.y, a.z, 0.0f ); }
inline float4 make_float4( const float3& a, const float w ) { return make_float4( a.x, a.y, a.z, w ); }
inline float4 make_float4( const int3& a, const float w ) { return make_float4( (float)a.x, (float)a.y, (float)a.z, w ); }
inline float4 make_float4( const int4& a ) { return make_float4( float( a.x ), float( a.y ), float( a.z ), float( a.w ) ); }
inline float4 make_float4( const uint4& a ) { return make_float4( float( a.x ), float( a.y ), float( a.z ), float( a.w ) ); }
inline int4 make_int4( const int a, const int b, const int c, const int d ) { int4 i4; i4.x = a, i4.y = b, i4.z = c, i4.w = d; return i4; }
inline int4 make_int4( const int s ) { return make_int4( s, s, s, s ); }
inline int4 make_int4( const int3& a ) { return make_int4( a.x, a.y, a.z, 0 ); }
inline int4 make_int4( const int3& a, const int w ) { return make_int4( a.x, a.y, a.z, w ); }
inline int4 make_int4( const uint4& a ) { return make_int4( int( a.x ), int( a.y ), int( a.z ), int( a.w ) ); }
inline int4 make_int4( const float4& a ) { return make_int4( int( a.x ), int( a.y ), int( a.z ), int( a.w ) ); }
inline uint4 make_uint4( const uint a, const uint b, const uint c, const uint d ) { uint4 u4; u4.x = a, u4.y = b, u4.z = c, u4.w = d; return u4; }
inline uint4 make_uint4( const uint s ) { return make_uint4( s, s, s, s ); }
inline uint4 make_uint4( const uint3& a ) { return make_uint4( a.x, a.y, a.z, 0 ); }
inline uint4 make_uint4( const uint3& a, const uint w ) { return make_uint4( a.x, a.y, a.z, w ); }
inline uint4 make_uint4( const int4& a ) { return make_uint4( uint( a.x ), uint( a.y ), uint( a.z ), uint( a.w ) ); }
inline uchar4 make_uchar4( const uchar a, const uchar b, const uchar c, const uchar d ) { uchar4 c4; c4.x = a, c4.y = b, c4.z = c, c4.w = d; return c4; }

inline bool operator==(const int3& a, const int3& b) { return a.x == b.x && a.y == b.y && a.z == b.z; }
inline bool operator==(const int2& a, const int2& b) { return a.x == b.x && a.y == b.y; }
inline bool operator==(const int4& a, const int4& b) { return a.x == b.x && a.y == b.y && a.z == b.z && a.w == b.w; }

inline float2 operator-( const float2& a ) { return make_float2( -a.x, -a.y ); }
inline int2 operator-( const int2& a ) { return make_int2( -a.x, -a.y ); }
inline float3 operator-( const float3& a ) { return make_float3( -a.x, -a.y, -a.z ); }
inline int3 operator-( const int3& a ) { return make_int3( -a.x, -a.y, -a.z ); }
inline float4 operator-( const float4& a ) { return make_float4( -a.x, -a.y, -a.z, -a.w ); }
inline int4 operator-( const int4& a ) { return make_int4( -a.x, -a.y, -a.z, -a.w ); }
inline int2 operator << ( const int2& a, int b ) { return make_int2( a.x << b, a.y << b ); }
inline int2 operator >> ( const int2& a, int b ) { return make_int2( a.x >> b, a.y >> b ); }
inline int3 operator << ( const int3& a, int b ) { return make_int3( a.x << b, a.y << b, a.z << b ); }
inline int3 operator >> ( const int3& a, int b ) { return make_int3( a.x >> b, a.y >> b, a.z >> b ); }
inline int4 operator << ( const int4& a, int b ) { return make_int4( a.x << b, a.y << b, a.z << b, a.w << b ); }
inline int4 operator >> ( const int4& a, int b ) { return make_int4( a.x >> b, a.y >> b, a.z >> b, a.w >> b ); }

inline float2 operator+( const float2& a, const float2& b ) { return make_float2( a.x + b.x, a.y + b.y ); }
inline float2 operator+( const float2& a, const int2& b ) { return make_float2( a.x + (float)b.x, a.y + (float)b.y ); }
inline float2 operator+( const float2& a, const uint2& b ) { return make_float2( a.x + (float)b.x, a.y + (float)b.y ); }
inline float2 operator+( const int2& a, const float2& b ) { return make_float2( (float)a.x + b.x, (float)a.y + b.y ); }
inline float2 operator+( const uint2& a, const float2& b ) { return make_float2( (float)a.x + b.x, (float)a.y + b.y ); }
inline void operator+=( float2& a, const float2& b ) { a.x += b.x;	a.y += b.y; }
inline void operator+=( float2& a, const int2& b ) { a.x += (float)b.x; a.y += (float)b.y; }
inline void operator+=( float2& a, const uint2& b ) { a.x += (float)b.x; a.y += (float)b.y; }
inline float2 operator+( const float2& a, float b ) { return make_float2( a.x + b, a.y + b ); }
inline float2 operator+( const float2& a, int b ) { return make_float2( a.x + (float)b, a.y + (float)b ); }
inline float2 operator+( const float2& a, uint b ) { return make_float2( a.x + (float)b, a.y + (float)b ); }
inline float2 operator+( float b, const float2& a ) { return make_float2( a.x + b, a.y + b ); }
inline void operator+=( float2& a, float b ) { a.x += b; a.y += b; }
inline void operator+=( float2& a, int b ) { a.x += (float)b; a.y += (float)b; }
inline void operator+=( float2& a, uint b ) { a.x += (float)b;	a.y += (float)b; }
inline int2 operator+( const int2& a, const int2& b ) { return make_int2( a.x + b.x, a.y + b.y ); }
inline void operator+=( int2& a, const int2& b ) { a.x += b.x;	a.y += b.y; }
inline int2 operator+( const int2& a, int b ) { return make_int2( a.x + b, a.y + b ); }
inline int2 operator+( int b, const int2& a ) { return make_int2( a.x + b, a.y + b ); }
inline void operator+=( int2& a, int b ) { a.x += b;	a.y += b; }
inline uint2 operator+( const uint2& a, const uint2& b ) { return make_uint2( a.x + b.x, a.y + b.y ); }
inline void operator+=( uint2& a, const uint2& b ) { a.x += b.x;	a.y += b.y; }
inline uint2 operator+( const uint2& a, uint b ) { return make_uint2( a.x + b, a.y + b ); }
inline uint2 operator+( uint b, const uint2& a ) { return make_uint2( a.x + b, a.y + b ); }
inline void operator+=( uint2& a, uint b ) { a.x += b;	a.y += b; }
inline float3 operator+( const float3& a, const float3& b ) { return make_float3( a.x + b.x, a.y + b.y, a.z + b.z ); }
inline float3 operator+( const float3& a, const int3& b ) { return make_float3( a.x + (float)b.x, a.y + (float)b.y, a.z + (float)b.z ); }
inline float3 operator+( const float3& a, const uint3& b ) { return make_float3( a.x + (float)b.x, a.y + (float)b.y, a.z + (float)b.z ); }
inline float3 operator+( const int3& a, const float3& b ) { return make_float3( (float)a.x + b.x, (float)a.y + b.y, (float)a.z + b.z ); }
inline float3 operator+( const uint3& a, const float3& b ) { return make_float3( (float)a.x + b.x, (float)a.y + b.y, (float)a.z + b.z ); }
inline void operator+=( float3& a, const float3& b ) { a.x += b.x;	a.y += b.y;	a.z += b.z; }
inline void operator+=( float3& a, const int3& b ) { a.x += (float)b.x; a.y += (float)b.y; a.z += (float)b.z; }
inline void operator+=( float3& a, const uint3& b ) { a.x += (float)b.x; a.y += (float)b.y; a.z += (float)b.z; }
inline float3 operator+( const float3& a, float b ) { return make_float3( a.x + b, a.y + b, a.z + b ); }
inline float3 operator+( const float3& a, int b ) { return make_float3( a.x + (float)b, a.y + (float)b, a.z + (float)b ); }
inline float3 operator+( const float3& a, uint b ) { return make_float3( a.x + (float)b, a.y + (float)b, a.z + (float)b ); }
inline void operator+=( float3& a, float b ) { a.x += b; a.y += b;	a.z += b; }
inline void operator+=( float3& a, int b ) { a.x += (float)b; a.y += (float)b; a.z += (float)b; }
inline void operator+=( float3& a, uint b ) { a.x += (float)b; a.y += (float)b; a.z += (float)b; }
inline int3 operator+( const int3& a, const int3& b ) { return make_int3( a.x + b.x, a.y + b.y, a.z + b.z ); }
inline void operator+=( int3& a, const int3& b ) { a.x += b.x;	a.y += b.y;	a.z += b.z; }
inline int3 operator+( const int3& a, int b ) { return make_int3( a.x + b, a.y + b, a.z + b ); }
inline void operator+=( int3& a, int b ) { a.x += b;	a.y += b;	a.z += b; }
inline uint3 operator+( const uint3& a, const uint3& b ) { return make_uint3( a.x + b.x, a.y + b.y, a.z + b.z ); }
inline void operator+=( uint3& a, const uint3& b ) { a.x += b.x;	a.y += b.y;	a.z += b.z; }
inline uint3 operator+( const uint3& a, uint b ) { return make_uint3( a.x + b, a.y + b, a.z + b ); }
inline void operator+=( uint3& a, uint b ) { a.x += b;	a.y += b;	a.z += b; }
inline int3 operator+( int b, const int3& a ) { return make_int3( a.x + b, a.y + b, a.z + b ); }
inline uint3 operator+( uint b, const uint3& a ) { return make_uint3( a.x + b, a.y + b, a.z + b ); }
inline float3 operator+( float b, const float3& a ) { return make_float3( a.x + b, a.y + b, a.z + b ); }
inline float4 operator+( const float4& a, const float4& b ) { return make_float4( a.x + b.x, a.y + b.y, a.z + b.z, a.w + b.w ); }
inline float4 operator+( const float4& a, const int4& b ) { return make_float4( a.x + (float)b.x, a.y + (float)b.y, a.z + (float)b.z, a.w + (float)b.w ); }
inline float4 operator+( const float4& a, const uint4& b ) { return make_float4( a.x + (float)b.x, a.y + (float)b.y, a.z + (float)b.z, a.w + (float)b.w ); }
inline float4 operator+( const int4& a, const float4& b ) { return make_float4( (float)a.x + b.x, (float)a.y + b.y, (float)a.z + b.z, (float)a.w + b.w ); }
inline float4 operator+( const uint4& a, const float4& b ) { return make_float4( (float)a.x + b.x, (float)a.y + b.y, (float)a.z + b.z, (float)a.w + b.w ); }
inline void operator+=( float4& a, const float4& b ) { a.x += b.x;	a.y += b.y;	a.z += b.z;	a.w += b.w; }
inline void operator+=( float4& a, const int4& b ) { a.x += (float)b.x; a.y += (float)b.y; a.z += (float)b.z; a.w += (float)b.w; }
inline void operator+=( float4& a, const uint4& b ) { a.x += (float)b.x; a.y += (float)b.y; a.z += (float)b.z; a.w += (float)b.w; }
inline float4 operator+( const float4& a, float b ) { return make_float4( a.x + b, a.y + b, a.z + b, a.w + b ); }
inline float4 operator+( const float4& a, int b ) { return make_float4( a.x + (float)b, a.y + (float)b, a.z + (float)b, a.w + (float)b ); }
inline float4 operator+( const float4& a, uint b ) { return make_float4( a.x + (float)b, a.y + (float)b, a.z + (float)b, a.w + (float)b ); }
inline float4 operator+( float b, const float4& a ) { return make_float4( a.x + b, a.y + b, a.z + b, a.w + b ); }
inline void operator+=( float4& a, float b ) { a.x += b;	a.y += b;	a.z += b;	a.w += b; }
inline void operator+=( float4& a, int b ) { a.x += (float)b; a.y += (float)b; a.z += (float)b; a.w += (float)b; }
inline void operator+=( float4& a, uint b ) { a.x += (float)b; a.y += (float)b; a.z += (float)b; a.w += (float)b; }
inline int4 operator+( const int4& a, const int4& b ) { return make_int4( a.x + b.x, a.y + b.y, a.z + b.z, a.w + b.w ); }
inline void operator+=( int4& a, const int4& b ) { a.x += b.x;	a.y += b.y;	a.z += b.z;	a.w += b.w; }
inline int4 operator+( const int4& a, int b ) { return make_int4( a.x + b, a.y + b, a.z + b, a.w + b ); }
inline int4 operator+( int b, const int4& a ) { return make_int4( a.x + b, a.y + b, a.z + b, a.w + b ); }
inline void operator+=( int4& a, int b ) { a.x += b;	a.y += b;	a.z += b;	a.w += b; }
inline uint4 operator+( const uint4& a, const uint4& b ) { return make_uint4( a.x + b.x, a.y + b.y, a.z + b.z, a.w + b.w ); }
inline void operator+=( uint4& a, const uint4& b ) { a.x += b.x;	a.y += b.y;	a.z += b.z;	a.w += b.w; }
inline uint4 operator+( const uint4& a, uint b ) { return make_uint4( a.x + b, a.y + b, a.z + b, a.w + b ); }
inline uint4 operator+( uint b, const uint4& a ) { return make_uint4( a.x + b, a.y + b, a.z + b, a.w + b ); }
inline void operator+=( uint4& a, uint b ) { a.x += b;	a.y += b;	a.z += b;	a.w += b; }

inline float2 operator-( const float2& a, const float2& b ) { return make_float2( a.x - b.x, a.y - b.y ); }
inline float2 operator-( const float2& a, const int2& b ) { return make_float2( a.x - (float)b.x, a.y - (float)b.y ); }
inline float2 operator-( const float2& a, const uint2& b ) { return make_float2( a.x - (float)b.x, a.y - (float)b.y ); }
inline float2 operator-( const int2& a, const float2& b ) { return make_float2( (float)a.x - b.x, (float)a.y - b.y ); }
inline float2 operator-( const uint2& a, const float2& b ) { return make_float2( (float)a.x - b.x, (float)a.y - b.y ); }
inline void operator-=( float2& a, const float2& b ) { a.x -= b.x;	a.y -= b.y; }
inline void operator-=( float2& a, const int2& b ) { a.x -= (float)b.x; a.y -= (float)b.y; }
inline void operator-=( float2& a, const uint2& b ) { a.x -= (float)b.x; a.y -= (float)b.y; }
inline float2 operator-( const float2& a, float b ) { return make_float2( a.x - b, a.y - b ); }
inline float2 operator-( const float2& a, int b ) { return make_float2( a.x - (float)b, a.y - (float)b ); }
inline float2 operator-( const float2& a, uint b ) { return make_float2( a.x - (float)b, a.y - (float)b ); }
inline float2 operator-( float b, const float2& a ) { return make_float2( b - a.x, b - a.y ); }
inline void operator-=( float2& a, float b ) { a.x -= b; a.y -= b; }
inline void operator-=( float2& a, int b ) { a.x -= (float)b; a.y -= (float)b; }
inline void operator-=( float2& a, uint b ) { a.x -= (float)b; a.y -= (float)b; }
inline int2 operator-( const int2& a, const int2& b ) { return make_int2( a.x - b.x, a.y - b.y ); }
inline void operator-=( int2& a, const int2& b ) { a.x -= b.x;	a.y -= b.y; }
inline int2 operator-( const int2& a, int b ) { return make_int2( a.x - b, a.y - b ); }
inline int2 operator-( int b, const int2& a ) { return make_int2( b - a.x, b - a.y ); }
inline void operator-=( int2& a, int b ) { a.x -= b;	a.y -= b; }
inline uint2 operator-( const uint2& a, const uint2& b ) { return make_uint2( a.x - b.x, a.y - b.y ); }
inline void operator-=( uint2& a, const uint2& b ) { a.x -= b.x;	a.y -= b.y; }
inline uint2 operator-( const uint2& a, uint b ) { return make_uint2( a.x - b, a.y - b ); }
inline uint2 operator-( uint b, const uint2& a ) { return make_uint2( b - a.x, b - a.y ); }
inline void operator-=( uint2& a, uint b ) { a.x -= b;	a.y -= b; }
inline float3 operator-( const float3& a, const float3& b ) { return make_float3( a.x - b.x, a.y - b.y, a.z - b.z ); }
inline float3 operator-( const float3& a, const int3& b ) { return make_float3( a.x - (float)b.x, a.y - (float)b.y, a.z - (float)b.z ); }
inline float3 operator-( const float3& a, const uint3& b ) { return make_float3( a.x - (float)b.x, a.y - (float)b.y, a.z - (float)b.z ); }
inline float3 operator-( const int3& a, const float3& b ) { return make_float3( (float)a.x - b.x, (float)a.y - b.y, (float)a.z - b.z ); }
inline float3 operator-( const uint3& a, const float3& b ) { return make_float3( (float)a.x - b.x, (float)a.y - b.y, (float)a.z - b.z ); }
inline void operator-=( float3& a, const float3& b ) { a.x -= b.x;	a.y -= b.y;	a.z -= b.z; }
inline void operator-=( float3& a, const int3& b ) { a.x -= (float)b.x; a.y -= (float)b.y; a.z -= (float)b.z; }
inline void operator-=( float3& a, const uint3& b ) { a.x -= (float)b.x; a.y -= (float)b.y; a.z -= (float)b.z; }
inline float3 operator-( const float3& a, float b ) { return make_float3( a.x - b, a.y - b, a.z - b ); }
inline float3 operator-( const float3& a, int b ) { return make_float3( a.x - (float)b, a.y - (float)b, a.z - (float)b ); }
inline float3 operator-( const float3& a, uint b ) { return make_float3( a.x - (float)b, a.y - (float)b, a.z - (float)b ); }
inline float3 operator-( float b, const float3& a ) { return make_float3( b - a.x, b - a.y, b - a.z ); }
inline void operator-=( float3& a, float b ) { a.x -= b; a.y -= b; a.z -= b; }
inline void operator-=( float3& a, int b ) { a.x -= (float)b; a.y -= (float)b; a.z -= (float)b; }
inline void operator-=( float3& a, uint b ) { a.x -= (float)b;	a.y -= (float)b; a.z -= (float)b; }
inline int3 operator-( const int3& a, const int3& b ) { return make_int3( a.x - b.x, a.y - b.y, a.z - b.z ); }
inline void operator-=( int3& a, const int3& b ) { a.x -= b.x;	a.y -= b.y;	a.z -= b.z; }
inline int3 operator-( const int3& a, int b ) { return make_int3( a.x - b, a.y - b, a.z - b ); }
inline int3 operator-( int b, const int3& a ) { return make_int3( b - a.x, b - a.y, b - a.z ); }
inline void operator-=( int3& a, int b ) { a.x -= b;	a.y -= b;	a.z -= b; }
inline uint3 operator-( const uint3& a, const uint3& b ) { return make_uint3( a.x - b.x, a.y - b.y, a.z - b.z ); }
inline void operator-=( uint3& a, const uint3& b ) { a.x -= b.x;	a.y -= b.y;	a.z -= b.z; }
inline uint3 operator-( const uint3& a, uint b ) { return make_uint3( a.x - b, a.y - b, a.z - b ); }
inline uint3 operator-( uint b, const uint3& a ) { return make_uint3( b - a.x, b - a.y, b - a.z ); }
inline void operator-=( uint3& a, uint b ) { a.x -= b;	a.y -= b;	a.z -= b; }
inline float4 operator-( const float4& a, const float4& b ) { return make_float4( a.x - b.x, a.y - b.y, a.z - b.z, a.w - b.w ); }
inline float4 operator-( const float4& a, const int4& b ) { return make_float4( a.x - (float)b.x, a.y - (float)b.y, a.z - (float)b.z, a.w - (float)b.w ); }
inline float4 operator-( const float4& a, const uint4& b ) { return make_float4( a.x - (float)b.x, a.y - (float)b.y, a.z - (float)b.z, a.w - (float)b.w ); }
inline float4 operator-( const int4& a, const float4& b ) { return make_float4( (float)a.x - b.x, (float)a.y - b.y, (float)a.z - b.z, (float)a.w - b.w ); }
inline float4 operator-( const uint4& a, const float4& b ) { return make_float4( (float)a.x - b.x, (float)a.y - b.y, (float)a.z - b.z, (float)a.w - b.w ); }
inline void operator-=( float4& a, const float4& b ) { a.x -= b.x;	a.y -= b.y;	a.z -= b.z;	a.w -= b.w; }
inline void operator-=( float4& a, const int4& b ) { a.x -= (float)b.x; a.y -= (float)b.y; a.z -= (float)b.z; a.w -= (float)b.w; }
inline void operator-=( float4& a, const uint4& b ) { a.x -= (float)b.x; a.y -= (float)b.y; a.z -= (float)b.z; a.w -= (float)b.w; }
inline float4 operator-( const float4& a, float b ) { return make_float4( a.x - b, a.y - b, a.z - b, a.w - b ); }
inline float4 operator-( const float4& a, int b ) { return make_float4( a.x - (float)b, a.y - (float)b, a.z - (float)b, a.w - (float)b ); }
inline float4 operator-( const float4& a, uint b ) { return make_float4( a.x - (float)b, a.y - (float)b, a.z - (float)b, a.w - (float)b ); }
inline void operator-=( float4& a, float b ) { a.x -= b; a.y -= b; a.z -= b; a.w -= b; }
inline void operator-=( float4& a, int b ) { a.x -= (float)b; a.y -= (float)b; a.z -= (float)b; a.w -= (float)b; }
inline void operator-=( float4& a, uint b ) { a.x -= (float)b; a.y -= (float)b; a.z -= (float)b; a.w -= (float)b; }
inline int4 operator-( const int4& a, const int4& b ) { return make_int4( a.x - b.x, a.y - b.y, a.z - b.z, a.w - b.w ); }
inline void operator-=( int4& a, const int4& b ) { a.x -= b.x;	a.y -= b.y;	a.z -= b.z;	a.w -= b.w; }
inline int4 operator-( const int4& a, int b ) { return make_int4( a.x - b, a.y - b, a.z - b, a.w - b ); }
inline int4 operator-( int b, const int4& a ) { return make_int4( b - a.x, b - a.y, b - a.z, b - a.w ); }
inline void operator-=( int4& a, int b ) { a.x -= b;	a.y -= b;	a.z -= b;	a.w -= b; }
inline uint4 operator-( const uint4& a, const uint4& b ) { return make_uint4( a.x - b.x, a.y - b.y, a.z - b.z, a.w - b.w ); }
inline void operator-=( uint4& a, const uint4& b ) { a.x -= b.x;	a.y -= b.y;	a.z -= b.z;	a.w -= b.w; }
inline uint4 operator-( const uint4& a, uint b ) { return make_uint4( a.x - b, a.y - b, a.z - b, a.w - b ); }
inline uint4 operator-( uint b, const uint4& a ) { return make_uint4( b - a.x, b - a.y, b - a.z, b - a.w ); }
inline void operator-=( uint4& a, uint b ) { a.x -= b;	a.y -= b;	a.z -= b;	a.w -= b; }

inline float2 operator*( const float2& a, const float2& b ) { return make_float2( a.x * b.x, a.y * b.y ); }
inline void operator*=( float2& a, const float2& b ) { a.x *= b.x;	a.y *= b.y; }
inline float2 operator*( const float2& a, float b ) { return make_float2( a.x * b, a.y * b ); }
inline float2 operator*( float b, const float2& a ) { return make_float2( b * a.x, b * a.y ); }
inline void operator*=( float2& a, float b ) { a.x *= b;	a.y *= b; }
inline int2 operator*( const int2& a, const int2& b ) { return make_int2( a.x * b.x, a.y * b.y ); }
inline void operator*=( int2& a, const int2& b ) { a.x *= b.x;	a.y *= b.y; }
inline int2 operator*( const int2& a, int b ) { return make_int2( a.x * b, a.y * b ); }
inline int2 operator*( int b, const int2& a ) { return make_int2( b * a.x, b * a.y ); }
inline void operator*=( int2& a, int b ) { a.x *= b;	a.y *= b; }
inline uint2 operator*( const uint2& a, const uint2& b ) { return make_uint2( a.x * b.x, a.y * b.y ); }
inline void operator*=( uint2& a, const uint2& b ) { a.x *= b.x;	a.y *= b.y; }
inline uint2 operator*( const uint2& a, uint b ) { return make_uint2( a.x * b, a.y * b ); }
inline uint2 operator*( uint b, const uint2& a ) { return make_uint2( b * a.x, b * a.y ); }
inline void operator*=( uint2& a, uint b ) { a.x *= b;	a.y *= b; }
inline float3 operator*( const float3& a, const float3& b ) { return make_float3( a.x * b.x, a.y * b.y, a.z * b.z ); }
inline void operator*=( float3& a, const float3& b ) { a.x *= b.x;	a.y *= b.y;	a.z *= b.z; }
inline float3 operator*( const float3& a, float b ) { return make_float3( a.x * b, a.y * b, a.z * b ); }
inline float3 operator*( float b, const float3& a ) { return make_float3( b * a.x, b * a.y, b * a.z ); }
inline void operator*=( float3& a, float b ) { a.x *= b;	a.y *= b;	a.z *= b; }
inline int3 operator*( const int3& a, const int3& b ) { return make_int3( a.x * b.x, a.y * b.y, a.z * b.z ); }
inline void operator*=( int3& a, const int3& b ) { a.x *= b.x;	a.y *= b.y;	a.z *= b.z; }
inline int3 operator*( const int3& a, int b ) { return make_int3( a.x * b, a.y * b, a.z * b ); }
inline int3 operator*( int b, const int3& a ) { return make_int3( b * a.x, b * a.y, b * a.z ); }
inline void operator*=( int3& a, int b ) { a.x *= b;	a.y *= b;	a.z *= b; }
inline uint3 operator*( const uint3& a, const uint3& b ) { return make_uint3( a.x * b.x, a.y * b.y, a.z * b.z ); }
inline void operator*=( uint3& a, const uint3& b ) { a.x *= b.x;	a.y *= b.y;	a.z *= b.z; }
inline uint3 operator*( const uint3& a, uint b ) { return make_uint3( a.x * b, a.y * b, a.z * b ); }
inline uint3 operator*( uint b, const uint3& a ) { return make_uint3( b * a.x, b * a.y, b * a.z ); }
inline void operator*=( uint3& a, uint b ) { a.x *= b;	a.y *= b;	a.z *= b; }
inline float4 operator*( const float4& a, const float4& b ) { return make_float4( a.x * b.x, a.y * b.y, a.z * b.z, a.w * b.w ); }
inline void operator*=( float4& a, const float4& b ) { a.x *= b.x;	a.y *= b.y;	a.z *= b.z;	a.w *= b.w; }
inline float4 operator*( const float4& a, float b ) { return make_float4( a.x * b, a.y * b, a.z * b, a.w * b ); }
inline float4 operator*( float b, const float4& a ) { return make_float4( b * a.x, b * a.y, b * a.z, b * a.w ); }
inline void operator*=( float4& a, float b ) { a.x *= b;	a.y *= b;	a.z *= b;	a.w *= b; }
inline int4 operator*( const int4& a, const int4& b ) { return make_int4( a.x * b.x, a.y * b.y, a.z * b.z, a.w * b.w ); }
inline void operator*=( int4& a, const int4& b ) { a.x *= b.x;	a.y *= b.y;	a.z *= b.z;	a.w *= b.w; }
inline int4 operator*( const int4& a, int b ) { return make_int4( a.x * b, a.y * b, a.z * b, a.w * b ); }
inline int4 operator*( int b, const int4& a ) { return make_int4( b * a.x, b * a.y, b * a.z, b * a.w ); }
inline void operator*=( int4& a, int b ) { a.x *= b;	a.y *= b;	a.z *= b;	a.w *= b; }
inline uint4 operator*( const uint4& a, const uint4& b ) { return make_uint4( a.x * b.x, a.y * b.y, a.z * b.z, a.w * b.w ); }
inline void operator*=( uint4& a, const uint4& b ) { a.x *= b.x;	a.y *= b.y;	a.z *= b.z;	a.w *= b.w; }
inline uint4 operator*( const uint4& a, uint b ) { return make_uint4( a.x * b, a.y * b, a.z * b, a.w * b ); }
inline uint4 operator*( uint b, const uint4& a ) { return make_uint4( b * a.x, b * a.y, b * a.z, b * a.w ); }
inline void operator*=( uint4& a, uint b ) { a.x *= b;	a.y *= b;	a.z *= b;	a.w *= b; }

inline float2 operator/( const float2& a, const float2& b ) { return make_float2( a.x / b.x, a.y / b.y ); }
inline void operator/=( float2& a, const float2& b ) { a.x /= b.x;	a.y /= b.y; }
inline float2 operator/( const float2& a, float b ) { return make_float2( a.x / b, a.y / b ); }
inline void operator/=( float2& a, float b ) { a.x /= b;	a.y /= b; }
inline float2 operator/( float b, const float2& a ) { return make_float2( b / a.x, b / a.y ); }
inline float3 operator/( const float3& a, const float3& b ) { return make_float3( a.x / b.x, a.y / b.y, a.z / b.z ); }
inline void operator/=( float3& a, const float3& b ) { a.x /= b.x;	a.y /= b.y;	a.z /= b.z; }
inline float3 operator/( const float3& a, float b ) { return make_float3( a.x / b, a.y / b, a.z / b ); }
inline void operator/=( float3& a, float b ) { a.x /= b;	a.y /= b;	a.z /= b; }
inline float3 operator/( float b, const float3& a ) { return make_float3( b / a.x, b / a.y, b / a.z ); }
inline float4 operator/( const float4& a, const float4& b ) { return make_float4( a.x / b.x, a.y / b.y, a.z / b.z, a.w / b.w ); }
inline void operator/=( float4& a, const float4& b ) { a.x /= b.x;	a.y /= b.y;	a.z /= b.z;	a.w /= b.w; }
inline float4 operator/( const float4& a, float b ) { return make_float4( a.x / b, a.y / b, a.z / b, a.w / b ); }
inline void operator/=( float4& a, float b ) { a.x /= b;	a.y /= b;	a.z /= b;	a.w /= b; }
inline float4 operator/( float b, const float4& a ) { return make_float4( b / a.x, b / a.y, b / a.z, b / a.w ); }

inline float2 fminf( const float2& a, const float2& b ) { return make_float2( fminf( a.x, b.x ), fminf( a.y, b.y ) ); }
inline float3 fminf( const float3& a, const float3& b ) { return make_float3( fminf( a.x, b.x ), fminf( a.y, b.y ), fminf( a.z, b.z ) ); }
inline float4 fminf( const float4& a, const float4& b ) { return make_float4( fminf( a.x, b.x ), fminf( a.y, b.y ), fminf( a.z, b.z ), fminf( a.w, b.w ) ); }
inline int2 min( const int2& a, const int2& b ) { return make_int2( min( a.x, b.x ), min( a.y, b.y ) ); }
inline int3 min( const int3& a, const int3& b ) { return make_int3( min( a.x, b.x ), min( a.y, b.y ), min( a.z, b.z ) ); }
inline int4 min( const int4& a, const int4& b ) { return make_int4( min( a.x, b.x ), min( a.y, b.y ), min( a.z, b.z ), min( a.w, b.w ) ); }
inline uint2 min( const uint2& a, const uint2& b ) { return make_uint2( min( a.x, b.x ), min( a.y, b.y ) ); }
inline uint3 min( const uint3& a, const uint3& b ) { return make_uint3( min( a.x, b.x ), min( a.y, b.y ), min( a.z, b.z ) ); }
inline uint4 min( const uint4& a, const uint4& b ) { return make_uint4( min( a.x, b.x ), min( a.y, b.y ), min( a.z, b.z ), min( a.w, b.w ) ); }

inline float2 fmaxf( const float2& a, const float2& b ) { return make_float2( fmaxf( a.x, b.x ), fmaxf( a.y, b.y ) ); }
inline float3 fmaxf( const float3& a, const float3& b ) { return make_float3( fmaxf( a.x, b.x ), fmaxf( a.y, b.y ), fmaxf( a.z, b.z ) ); }
inline float4 fmaxf( const float4& a, const float4& b ) { return make_float4( fmaxf( a.x, b.x ), fmaxf( a.y, b.y ), fmaxf( a.z, b.z ), fmaxf( a.w, b.w ) ); }
inline int2 max( const int2& a, const int2& b ) { return make_int2( max( a.x, b.x ), max( a.y, b.y ) ); }
inline int3 max( const int3& a, const int3& b ) { return make_int3( max( a.x, b.x ), max( a.y, b.y ), max( a.z, b.z ) ); }
inline int4 max( const int4& a, const int4& b ) { return make_int4( max( a.x, b.x ), max( a.y, b.y ), max( a.z, b.z ), max( a.w, b.w ) ); }
inline uint2 max( const uint2& a, const uint2& b ) { return make_uint2( max( a.x, b.x ), max( a.y, b.y ) ); }
inline uint3 max( const uint3& a, const uint3& b ) { return make_uint3( max( a.x, b.x ), max( a.y, b.y ), max( a.z, b.z ) ); }
inline uint4 max( const uint4& a, const uint4& b ) { return make_uint4( max( a.x, b.x ), max( a.y, b.y ), max( a.z, b.z ), max( a.w, b.w ) ); }

// inline float lerp( float a, float b, float t ) { return a + t * (b - a); }
// inline float2 lerp( const float2& a, const float2& b, float t ) { return a + t * (b - a); }
// inline float3 lerp( const float3& a, const float3& b, float t ) { return a + t * (b - a); }
// inline float4 lerp( const float4& a, const float4& b, float t ) { return a + t * (b - a); }

inline float clamp( float f, float a, float b ) { return fmaxf( a, fminf( f, b ) ); }
inline int clamp( int f, int a, int b ) { return max( a, min( f, b ) ); }
inline uint clamp( uint f, uint a, uint b ) { return max( a, min( f, b ) ); }
inline float2 clamp( const float2& v, float a, float b ) { return make_float2( clamp( v.x, a, b ), clamp( v.y, a, b ) ); }
inline float2 clamp( const float2& v, const float2& a, const float2& b ) { return make_float2( clamp( v.x, a.x, b.x ), clamp( v.y, a.y, b.y ) ); }
inline float3 clamp( const float3& v, float a, float b ) { return make_float3( clamp( v.x, a, b ), clamp( v.y, a, b ), clamp( v.z, a, b ) ); }
inline float3 clamp( const float3& v, const float3& a, const float3& b ) { return make_float3( clamp( v.x, a.x, b.x ), clamp( v.y, a.y, b.y ), clamp( v.z, a.z, b.z ) ); }
inline float4 clamp( const float4& v, float a, float b ) { return make_float4( clamp( v.x, a, b ), clamp( v.y, a, b ), clamp( v.z, a, b ), clamp( v.w, a, b ) ); }
inline float4 clamp( const float4& v, const float4& a, const float4& b ) { return make_float4( clamp( v.x, a.x, b.x ), clamp( v.y, a.y, b.y ), clamp( v.z, a.z, b.z ), clamp( v.w, a.w, b.w ) ); }
inline int2 clamp( const int2& v, int a, int b ) { return make_int2( clamp( v.x, a, b ), clamp( v.y, a, b ) ); }
inline int2 clamp( const int2& v, const int2& a, const int2& b ) { return make_int2( clamp( v.x, a.x, b.x ), clamp( v.y, a.y, b.y ) ); }
inline int3 clamp( const int3& v, int a, int b ) { return make_int3( clamp( v.x, a, b ), clamp( v.y, a, b ), clamp( v.z, a, b ) ); }
inline int3 clamp( const int3& v, const int3& a, const int3& b ) { return make_int3( clamp( v.x, a.x, b.x ), clamp( v.y, a.y, b.y ), clamp( v.z, a.z, b.z ) ); }
inline int4 clamp( const int4& v, int a, int b ) { return make_int4( clamp( v.x, a, b ), clamp( v.y, a, b ), clamp( v.z, a, b ), clamp( v.w, a, b ) ); }
inline int4 clamp( const int4& v, const int4& a, const int4& b ) { return make_int4( clamp( v.x, a.x, b.x ), clamp( v.y, a.y, b.y ), clamp( v.z, a.z, b.z ), clamp( v.w, a.w, b.w ) ); }
inline uint2 clamp( const uint2& v, uint a, uint b ) { return make_uint2( clamp( v.x, a, b ), clamp( v.y, a, b ) ); }
inline uint2 clamp( const uint2& v, const uint2& a, const uint2& b ) { return make_uint2( clamp( v.x, a.x, b.x ), clamp( v.y, a.y, b.y ) ); }
inline uint3 clamp( const uint3& v, uint a, uint b ) { return make_uint3( clamp( v.x, a, b ), clamp( v.y, a, b ), clamp( v.z, a, b ) ); }
inline uint3 clamp( const uint3& v, const uint3& a, const uint3& b ) { return make_uint3( clamp( v.x, a.x, b.x ), clamp( v.y, a.y, b.y ), clamp( v.z, a.z, b.z ) ); }
inline uint4 clamp( const uint4& v, uint a, uint b ) { return make_uint4( clamp( v.x, a, b ), clamp( v.y, a, b ), clamp( v.z, a, b ), clamp( v.w, a, b ) ); }
inline uint4 clamp( const uint4& v, const uint4& a, const uint4& b ) { return make_uint4( clamp( v.x, a.x, b.x ), clamp( v.y, a.y, b.y ), clamp( v.z, a.z, b.z ), clamp( v.w, a.w, b.w ) ); }

inline float dot( const float2& a, const float2& b ) { return a.x * b.x + a.y * b.y; }
inline float dot( const float3& a, const float3& b ) { return a.x * b.x + a.y * b.y + a.z * b.z; }
inline float dot( const float4& a, const float4& b ) { return a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w; }
inline int dot( const int2& a, const int2& b ) { return a.x * b.x + a.y * b.y; }
inline int dot( const int3& a, const int3& b ) { return a.x * b.x + a.y * b.y + a.z * b.z; }
inline int dot( const int4& a, const int4& b ) { return a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w; }
inline uint dot( const uint2& a, const uint2& b ) { return a.x * b.x + a.y * b.y; }
inline uint dot( const uint3& a, const uint3& b ) { return a.x * b.x + a.y * b.y + a.z * b.z; }
inline uint dot( const uint4& a, const uint4& b ) { return a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w; }

inline float length( const float2& v ) { return sqrtf( dot( v, v ) ); }
inline float length( const float3& v ) { return sqrtf( dot( v, v ) ); }
inline float length( const float4& v ) { return sqrtf( dot( v, v ) ); }

inline float length( const int2& v ) { return sqrtf( (float)dot( v, v ) ); }
inline float length( const int3& v ) { return sqrtf( (float)dot( v, v ) ); }
inline float length( const int4& v ) { return sqrtf( (float)dot( v, v ) ); }

inline float2 normalize( const float2& v ) { float invLen = rsqrtf( dot( v, v ) ); return v * invLen; }
inline float3 normalize( const float3& v ) { float invLen = rsqrtf( dot( v, v ) ); return v * invLen; }
inline float4 normalize( const float4& v ) { float invLen = rsqrtf( dot( v, v ) ); return v * invLen; }

inline uint dominantAxis( const float2& v ) { float x = fabs( v.x ), y = fabs( v.y ); return x > y ? 0 : 1; } // for coherent grid traversal
inline uint dominantAxis( const float3& v ) { float x = fabs( v.x ), y = fabs( v.y ), z = fabs( v.z ); float m = max( max( x, y ), z ); return m == x ? 0 : (m == y ? 1 : 2); }

inline float2 floorf( const float2& v ) { return make_float2( floorf( v.x ), floorf( v.y ) ); }
inline float3 floorf( const float3& v ) { return make_float3( floorf( v.x ), floorf( v.y ), floorf( v.z ) ); }
inline float4 floorf( const float4& v ) { return make_float4( floorf( v.x ), floorf( v.y ), floorf( v.z ), floorf( v.w ) ); }

inline float fracf( float v ) { return v - floorf( v ); }
inline float2 fracf( const float2& v ) { return make_float2( fracf( v.x ), fracf( v.y ) ); }
inline float3 fracf( const float3& v ) { return make_float3( fracf( v.x ), fracf( v.y ), fracf( v.z ) ); }
inline float4 fracf( const float4& v ) { return make_float4( fracf( v.x ), fracf( v.y ), fracf( v.z ), fracf( v.w ) ); }

inline float2 fmodf( const float2& a, const float2& b ) { return make_float2( fmodf( a.x, b.x ), fmodf( a.y, b.y ) ); }
inline float3 fmodf( const float3& a, const float3& b ) { return make_float3( fmodf( a.x, b.x ), fmodf( a.y, b.y ), fmodf( a.z, b.z ) ); }
inline float4 fmodf( const float4& a, const float4& b ) { return make_float4( fmodf( a.x, b.x ), fmodf( a.y, b.y ), fmodf( a.z, b.z ), fmodf( a.w, b.w ) ); }

inline float2 fabs( const float2& v ) { return make_float2( fabs( v.x ), fabs( v.y ) ); }
inline float3 fabs( const float3& v ) { return make_float3( fabs( v.x ), fabs( v.y ), fabs( v.z ) ); }
inline float4 fabs( const float4& v ) { return make_float4( fabs( v.x ), fabs( v.y ), fabs( v.z ), fabs( v.w ) ); }
inline int2 abs( const int2& v ) { return make_int2( abs( v.x ), abs( v.y ) ); }
inline int3 abs( const int3& v ) { return make_int3( abs( v.x ), abs( v.y ), abs( v.z ) ); }
inline int4 abs( const int4& v ) { return make_int4( abs( v.x ), abs( v.y ), abs( v.z ), abs( v.w ) ); }

inline float3 reflect( const float3& i, const float3& n ) { return i - 2.0f * n * dot( n, i ); }

inline float3 cross( const float3& a, const float3& b ) { return make_float3( a.y * b.z - a.z * b.y, a.z * b.x - a.x * b.z, a.x * b.y - a.y * b.x ); }

inline float smoothstep( float a, float b, float x )
{
	float y = clamp( (x - a) / (b - a), 0.0f, 1.0f );
	return (y * y * (3.0f - (2.0f * y)));
}
inline float2 smoothstep( float2 a, float2 b, float2 x )
{
	float2 y = clamp( (x - a) / (b - a), 0.0f, 1.0f );
	return (y * y * (make_float2( 3.0f ) - (make_float2( 2.0f ) * y)));
}
inline float3 smoothstep( float3 a, float3 b, float3 x )
{
	float3 y = clamp( (x - a) / (b - a), 0.0f, 1.0f );
	return (y * y * (make_float3( 3.0f ) - (make_float3( 2.0f ) * y)));
}
inline float4 smoothstep( float4 a, float4 b, float4 x )
{
	float4 y = clamp( (x - a) / (b - a), 0.0f, 1.0f );
	return (y * y * (make_float4( 3.0f ) - (make_float4( 2.0f ) * y)));
}

using i64 = int64_t;
using u64 = uint64_t;
using i32 = int32_t;
using u32 = uint32_t;
using i16 = int16_t;
using u16 = uint16_t;
using i8 = int8_t;
using u8 = uint8_t;

template <typename T>
bool string_to(const std::string& s, T& result)
{
	std::istringstream ss(s);
	ss >> result >> std::ws;
	if (ss.eof()) return true;			// success
	return false;						// failure
}

template <typename T> vector<T> StringToInts(string s)
{
	static regex digit_regex = regex("-?\\d+");
	auto result = vector<T>();
	std::regex_token_iterator<std::string::iterator> rend;
	std::regex_token_iterator<std::string::iterator> a1(s.begin(), s.end(), digit_regex);
	while (a1 != rend) {
		T res = 0;
		const string _s = *a1++;
		if (string_to<T>(_s, res)) {
			result.push_back(res);
		}
	}
	return result;
}

template <typename T> size_t simple_hash(const initializer_list<T>& list)
{
	std::size_t seed = list.size();
	for (auto x : list) {
		x = u32((x >> 16) ^ x) * 0x45d9f3b;
		x = u32((x >> 16) ^ x) * 0x45d9f3b;
		x = (x >> 16) ^ x;
		seed ^= x + 0x9e3779b9 + (seed << 6) + (seed >> 2);
	}
	return seed;
}

inline std::string ReplaceAll(std::string str, const std::string& from, const std::string& to) {
	size_t start_pos = 0;
	while ((start_pos = str.find(from, start_pos)) != std::string::npos) {
		str.replace(start_pos, from.length(), to);
		start_pos += to.length(); // Handles case where 'to' is a substring of 'from'
	}
	return str;
}

inline void ReplaceAllInPlace(std::string& str, const std::string& from, const std::string& to)
{
	size_t start_pos = 0;
	while ((start_pos = str.find(from, start_pos)) != std::string::npos) {
		str.replace(start_pos, from.length(), to);
		start_pos += to.length(); // Handles case where 'to' is a substring of 'from'
	}
}

vector<string> StringSplit(string s, string delimiter)
{
	size_t pos_start = 0, pos_end, delim_len = delimiter.length();
	string token;
	vector<string> res;

	while ((pos_end = s.find(delimiter, pos_start)) != string::npos) {
		token = s.substr(pos_start, pos_end - pos_start);
		pos_start = pos_end + delim_len;
		res.push_back(token);
	}

	res.push_back(s.substr(pos_start));
	return res;
}
bool StartsWith(string s, string pattern);
template <typename T> int sgn(T val) {	return (T(0) < val) - (val < T(0)); }
template <typename T> T nmod(T val, T modulo) { return (modulo + (val % modulo)) % modulo; }
template <typename T> void move(std::vector<T>& v, size_t oldIndex, size_t newIndex)
{
	if (oldIndex > newIndex) 
	{
		std::rotate(v.rend() - oldIndex - 1, v.rend() - oldIndex, v.rend() - newIndex);
	}
	else { std::rotate(v.begin() + oldIndex, v.begin() + oldIndex + 1, v.begin() + newIndex + 1); }
}

bool will_overflow(long long a, long long b)
{
	return (b > LLONG_MAX / a);
}

void ltrim(std::string& s)
{
	s.erase(s.begin(), std::find_if(s.begin(), s.end(), [](unsigned char ch) {
		return !std::isspace(ch);
		}));
}
void rtrim(std::string& s)
{
	s.erase(std::find_if(s.rbegin(), s.rend(), [](unsigned char ch) {
		return !std::isspace(ch);
		}).base(), s.end());
}
void trim(std::string& s)
{
	rtrim(s);
	ltrim(s);
}
std::string ltrim_copy(std::string s)
{
	ltrim(s);
	return s;
}
std::string rtrim_copy(std::string s)
{
	rtrim(s);
	return s;
}
std::string trim_copy(std::string s)
{
	trim(s);
	return s;
}

// inclusive
inline bool Inbetween(int a, int lower, int upper)
{
	return a >= lower && a <= upper;
}

inline int ManhattanDistance(int x1, int y1, int x2, int y2)
{
	return abs(x1 - x2) + abs(y1 - y2);
}

template<typename T> bool all_equal(const initializer_list<T>& list1, const initializer_list<T>& list2)
{
	for (u64 i = 0; i < list1.size(); i++)
	{
		T item1 = *(list1.begin() + i);
		T item2 = *(list2.begin() + i);
		if (item1 != item2) return false;
	}
	return true;
}

struct Point
{
	i64 x = -1, y = -1;
};
inline bool operator==(const Point& lhs, const Point& rhs) { return lhs.x == rhs.x && lhs.y == rhs.y; }
struct PointKeyHasher
{
	std::size_t operator()(const Point& k) const
	{
		return simple_hash({ k.x, k.y });
	}
};

string TextFileRead(const char* _File)
{
	ifstream s(_File);
	string str((istreambuf_iterator<char>(s)), istreambuf_iterator<char>());
	s.close();
	return str;
}

int LineCount(const string s)
{
	const char* p = s.c_str();
	int lines = 0;
	while (*p) if (*p++ == '\n') lines++;
	return lines;
}

template<typename T> u64 indexOfOrSmaller(vector<T>& indices, T index) {
    auto const it = lower_bound(indices.begin(), indices.end(), index);
    return it - indices.begin();
}

template<typename T> u64 indexOfFirstGreater(vector<T>& indices, T index) {
    auto const it = upper_bound(indices.begin(), indices.end(), index);
    return it - indices.begin();
}

template<typename T> string joinArrayWith(vector<T>& arr) {
	stringstream s;
	copy(arr.begin(), arr.end(), ostream_iterator<int>(s, string(",").c_str()));
	return s.str();
}