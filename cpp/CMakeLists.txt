project( ProjectEulerCpp )
cmake_minimum_required(VERSION 3.12)

add_compile_options(-Wall -Wextra -Wpedantic -Ofast)

add_executable( p0001 "P0001/mult35.cpp" )
add_executable( p0002 "P0002/fibeven.cpp" )
add_executable( p0003 "P0003/primefact.cpp" )
add_executable( p0004 "P0004/palindromicnumber.cpp" )
add_executable( p0005 "P0005/smallestdivisible.cpp" )
add_executable( p0006 "P0006/sumsquare.cpp" )
add_executable( p0007 "P0007/prime10001.cpp" )
add_executable( p0008 "P0008/adjacent.cpp" )
add_executable( p0009 "P0009/triplet.cpp" )
add_executable( p0010 "P0010/primesum.cpp" )
add_executable( p0011 "P0011/grid.cpp" )
add_executable( p0012 "P0012/trianglenumbers.cpp" )

