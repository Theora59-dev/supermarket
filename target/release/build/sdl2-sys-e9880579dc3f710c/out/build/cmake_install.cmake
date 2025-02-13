# Install script for directory: /home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "/home/noa/supermarket/target/release/build/sdl2-sys-e9880579dc3f710c/out")
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

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "/home/noa/supermarket/target/release/build/sdl2-sys-e9880579dc3f710c/out/build/libSDL2main.a")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "/home/noa/supermarket/target/release/build/sdl2-sys-e9880579dc3f710c/out/build/libSDL2.a")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "/home/noa/supermarket/target/release/build/sdl2-sys-e9880579dc3f710c/out/build/libSDL2_test.a")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/SDL2/SDL2mainTargets.cmake")
    file(DIFFERENT _cmake_export_file_changed FILES
         "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/SDL2/SDL2mainTargets.cmake"
         "/home/noa/supermarket/target/release/build/sdl2-sys-e9880579dc3f710c/out/build/CMakeFiles/Export/f084604df1a27ef5b4fef7c7544737d1/SDL2mainTargets.cmake")
    if(_cmake_export_file_changed)
      file(GLOB _cmake_old_config_files "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/SDL2/SDL2mainTargets-*.cmake")
      if(_cmake_old_config_files)
        string(REPLACE ";" ", " _cmake_old_config_files_text "${_cmake_old_config_files}")
        message(STATUS "Old export file \"$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/SDL2/SDL2mainTargets.cmake\" will be replaced.  Removing files [${_cmake_old_config_files_text}].")
        unset(_cmake_old_config_files_text)
        file(REMOVE ${_cmake_old_config_files})
      endif()
      unset(_cmake_old_config_files)
    endif()
    unset(_cmake_export_file_changed)
  endif()
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/cmake/SDL2" TYPE FILE FILES "/home/noa/supermarket/target/release/build/sdl2-sys-e9880579dc3f710c/out/build/CMakeFiles/Export/f084604df1a27ef5b4fef7c7544737d1/SDL2mainTargets.cmake")
  if(CMAKE_INSTALL_CONFIG_NAME MATCHES "^([Rr][Ee][Ll][Ee][Aa][Ss][Ee])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/cmake/SDL2" TYPE FILE FILES "/home/noa/supermarket/target/release/build/sdl2-sys-e9880579dc3f710c/out/build/CMakeFiles/Export/f084604df1a27ef5b4fef7c7544737d1/SDL2mainTargets-release.cmake")
  endif()
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/SDL2/SDL2staticTargets.cmake")
    file(DIFFERENT _cmake_export_file_changed FILES
         "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/SDL2/SDL2staticTargets.cmake"
         "/home/noa/supermarket/target/release/build/sdl2-sys-e9880579dc3f710c/out/build/CMakeFiles/Export/f084604df1a27ef5b4fef7c7544737d1/SDL2staticTargets.cmake")
    if(_cmake_export_file_changed)
      file(GLOB _cmake_old_config_files "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/SDL2/SDL2staticTargets-*.cmake")
      if(_cmake_old_config_files)
        string(REPLACE ";" ", " _cmake_old_config_files_text "${_cmake_old_config_files}")
        message(STATUS "Old export file \"$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/SDL2/SDL2staticTargets.cmake\" will be replaced.  Removing files [${_cmake_old_config_files_text}].")
        unset(_cmake_old_config_files_text)
        file(REMOVE ${_cmake_old_config_files})
      endif()
      unset(_cmake_old_config_files)
    endif()
    unset(_cmake_export_file_changed)
  endif()
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/cmake/SDL2" TYPE FILE FILES "/home/noa/supermarket/target/release/build/sdl2-sys-e9880579dc3f710c/out/build/CMakeFiles/Export/f084604df1a27ef5b4fef7c7544737d1/SDL2staticTargets.cmake")
  if(CMAKE_INSTALL_CONFIG_NAME MATCHES "^([Rr][Ee][Ll][Ee][Aa][Ss][Ee])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/cmake/SDL2" TYPE FILE FILES "/home/noa/supermarket/target/release/build/sdl2-sys-e9880579dc3f710c/out/build/CMakeFiles/Export/f084604df1a27ef5b4fef7c7544737d1/SDL2staticTargets-release.cmake")
  endif()
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/SDL2/SDL2testTargets.cmake")
    file(DIFFERENT _cmake_export_file_changed FILES
         "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/SDL2/SDL2testTargets.cmake"
         "/home/noa/supermarket/target/release/build/sdl2-sys-e9880579dc3f710c/out/build/CMakeFiles/Export/f084604df1a27ef5b4fef7c7544737d1/SDL2testTargets.cmake")
    if(_cmake_export_file_changed)
      file(GLOB _cmake_old_config_files "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/SDL2/SDL2testTargets-*.cmake")
      if(_cmake_old_config_files)
        string(REPLACE ";" ", " _cmake_old_config_files_text "${_cmake_old_config_files}")
        message(STATUS "Old export file \"$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/SDL2/SDL2testTargets.cmake\" will be replaced.  Removing files [${_cmake_old_config_files_text}].")
        unset(_cmake_old_config_files_text)
        file(REMOVE ${_cmake_old_config_files})
      endif()
      unset(_cmake_old_config_files)
    endif()
    unset(_cmake_export_file_changed)
  endif()
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/cmake/SDL2" TYPE FILE FILES "/home/noa/supermarket/target/release/build/sdl2-sys-e9880579dc3f710c/out/build/CMakeFiles/Export/f084604df1a27ef5b4fef7c7544737d1/SDL2testTargets.cmake")
  if(CMAKE_INSTALL_CONFIG_NAME MATCHES "^([Rr][Ee][Ll][Ee][Aa][Ss][Ee])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/cmake/SDL2" TYPE FILE FILES "/home/noa/supermarket/target/release/build/sdl2-sys-e9880579dc3f710c/out/build/CMakeFiles/Export/f084604df1a27ef5b4fef7c7544737d1/SDL2testTargets-release.cmake")
  endif()
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Devel" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/cmake/SDL2" TYPE FILE FILES
    "/home/noa/supermarket/target/release/build/sdl2-sys-e9880579dc3f710c/out/build/SDL2Config.cmake"
    "/home/noa/supermarket/target/release/build/sdl2-sys-e9880579dc3f710c/out/build/SDL2ConfigVersion.cmake"
    )
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include/SDL2" TYPE FILE FILES
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_assert.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_atomic.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_audio.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_bits.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_blendmode.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_clipboard.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_copying.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_cpuinfo.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_egl.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_endian.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_error.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_events.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_filesystem.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_gamecontroller.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_gesture.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_guid.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_haptic.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_hidapi.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_hints.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_joystick.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_keyboard.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_keycode.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_loadso.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_locale.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_log.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_main.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_messagebox.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_metal.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_misc.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_mouse.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_mutex.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_name.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_opengl.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_opengl_glext.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_opengles.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_opengles2.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_opengles2_gl2.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_opengles2_gl2ext.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_opengles2_gl2platform.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_opengles2_khrplatform.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_pixels.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_platform.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_power.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_quit.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_rect.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_render.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_rwops.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_scancode.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_sensor.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_shape.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_stdinc.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_surface.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_system.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_syswm.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_test.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_test_assert.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_test_common.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_test_compare.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_test_crc32.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_test_font.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_test_fuzzer.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_test_harness.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_test_images.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_test_log.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_test_md5.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_test_memory.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_test_random.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_thread.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_timer.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_touch.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_types.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_version.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_video.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/SDL_vulkan.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/begin_code.h"
    "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/include/close_code.h"
    "/home/noa/supermarket/target/release/build/sdl2-sys-e9880579dc3f710c/out/build/include/SDL_revision.h"
    "/home/noa/supermarket/target/release/build/sdl2-sys-e9880579dc3f710c/out/build/include-config-release/SDL_config.h"
    )
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/licenses/SDL2" TYPE FILE FILES "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/LICENSE.txt")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/pkgconfig" TYPE FILE FILES "/home/noa/supermarket/target/release/build/sdl2-sys-e9880579dc3f710c/out/build/sdl2.pc")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/bin" TYPE PROGRAM FILES "/home/noa/supermarket/target/release/build/sdl2-sys-e9880579dc3f710c/out/build/sdl2-config")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/aclocal" TYPE FILE FILES "/home/noa/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sdl2-sys-0.37.0/SDL/sdl2.m4")
endif()

if(CMAKE_INSTALL_COMPONENT)
  set(CMAKE_INSTALL_MANIFEST "install_manifest_${CMAKE_INSTALL_COMPONENT}.txt")
else()
  set(CMAKE_INSTALL_MANIFEST "install_manifest.txt")
endif()

string(REPLACE ";" "\n" CMAKE_INSTALL_MANIFEST_CONTENT
       "${CMAKE_INSTALL_MANIFEST_FILES}")
file(WRITE "/home/noa/supermarket/target/release/build/sdl2-sys-e9880579dc3f710c/out/build/${CMAKE_INSTALL_MANIFEST}"
     "${CMAKE_INSTALL_MANIFEST_CONTENT}")
