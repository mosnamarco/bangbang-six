# Install script for directory: /home/marco/Desktop/Projects/rustacian-na-ako/bangbang_six/target/debug/build/raylib-sys-cf148da2bc5a1b0f/out/raylib/src

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "/home/marco/Desktop/Projects/rustacian-na-ako/bangbang_six/target/debug/build/raylib-sys-cf148da2bc5a1b0f/out")
endif()
string(REGEX REPLACE "/$" "" CMAKE_INSTALL_PREFIX "${CMAKE_INSTALL_PREFIX}")

# Set the install configuration name.
if(NOT DEFINED CMAKE_INSTALL_CONFIG_NAME)
  if(BUILD_TYPE)
    string(REGEX REPLACE "^[^A-Za-z0-9_]+" ""
           CMAKE_INSTALL_CONFIG_NAME "${BUILD_TYPE}")
  else()
    set(CMAKE_INSTALL_CONFIG_NAME "Release")
  endif()
  message(STATUS "Install configuration: \"${CMAKE_INSTALL_CONFIG_NAME}\"")
endif()

# Set the component getting installed.
if(NOT CMAKE_INSTALL_COMPONENT)
  if(COMPONENT)
    message(STATUS "Install component: \"${COMPONENT}\"")
    set(CMAKE_INSTALL_COMPONENT "${COMPONENT}")
  else()
    set(CMAKE_INSTALL_COMPONENT)
  endif()
endif()

# Install shared libraries without execute permission?
if(NOT DEFINED CMAKE_INSTALL_SO_NO_EXE)
  set(CMAKE_INSTALL_SO_NO_EXE "1")
endif()

# Is this installation the result of a crosscompile?
if(NOT DEFINED CMAKE_CROSSCOMPILING)
  set(CMAKE_CROSSCOMPILING "FALSE")
endif()

# Set default install directory permissions.
if(NOT DEFINED CMAKE_OBJDUMP)
  set(CMAKE_OBJDUMP "/usr/bin/objdump")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "/home/marco/Desktop/Projects/rustacian-na-ako/bangbang_six/target/debug/build/raylib-sys-cf148da2bc5a1b0f/out/build/raylib/libraylib.a")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include" TYPE FILE FILES
    "/home/marco/Desktop/Projects/rustacian-na-ako/bangbang_six/target/debug/build/raylib-sys-cf148da2bc5a1b0f/out/raylib/src/raylib.h"
    "/home/marco/Desktop/Projects/rustacian-na-ako/bangbang_six/target/debug/build/raylib-sys-cf148da2bc5a1b0f/out/raylib/src/rlgl.h"
    "/home/marco/Desktop/Projects/rustacian-na-ako/bangbang_six/target/debug/build/raylib-sys-cf148da2bc5a1b0f/out/raylib/src/physac.h"
    "/home/marco/Desktop/Projects/rustacian-na-ako/bangbang_six/target/debug/build/raylib-sys-cf148da2bc5a1b0f/out/raylib/src/raymath.h"
    "/home/marco/Desktop/Projects/rustacian-na-ako/bangbang_six/target/debug/build/raylib-sys-cf148da2bc5a1b0f/out/raylib/src/raudio.h"
    )
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/pkgconfig" TYPE FILE FILES "/home/marco/Desktop/Projects/rustacian-na-ako/bangbang_six/target/debug/build/raylib-sys-cf148da2bc5a1b0f/out/build/raylib/raylib.pc")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/cmake/raylib" TYPE FILE FILES "/home/marco/Desktop/Projects/rustacian-na-ako/bangbang_six/target/debug/build/raylib-sys-cf148da2bc5a1b0f/out/build/raylib/raylib-config-version.cmake")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/cmake/raylib" TYPE FILE FILES "/home/marco/Desktop/Projects/rustacian-na-ako/bangbang_six/target/debug/build/raylib-sys-cf148da2bc5a1b0f/out/raylib/src/../cmake/raylib-config.cmake")
endif()

if(NOT CMAKE_INSTALL_LOCAL_ONLY)
  # Include the install script for each subdirectory.
  include("/home/marco/Desktop/Projects/rustacian-na-ako/bangbang_six/target/debug/build/raylib-sys-cf148da2bc5a1b0f/out/build/raylib/external/glfw/cmake_install.cmake")

endif()
