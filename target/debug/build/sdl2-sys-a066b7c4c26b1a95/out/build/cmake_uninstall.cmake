if (NOT EXISTS "/home/noa/supermarket/target/debug/build/sdl2-sys-a066b7c4c26b1a95/out/build/install_manifest.txt")
    message(FATAL_ERROR "Cannot find install manifest: \"/home/noa/supermarket/target/debug/build/sdl2-sys-a066b7c4c26b1a95/out/build/install_manifest.txt\"")
endif(NOT EXISTS "/home/noa/supermarket/target/debug/build/sdl2-sys-a066b7c4c26b1a95/out/build/install_manifest.txt")

file(READ "/home/noa/supermarket/target/debug/build/sdl2-sys-a066b7c4c26b1a95/out/build/install_manifest.txt" files)
string(REGEX REPLACE "\n" ";" files "${files}")
foreach (file ${files})
    message(STATUS "Uninstalling \"$ENV{DESTDIR}${file}\"")
    execute_process(
        COMMAND /usr/bin/cmake -E remove "$ENV{DESTDIR}${file}"
        OUTPUT_VARIABLE rm_out
        RESULT_VARIABLE rm_retval
    )
    if(NOT ${rm_retval} EQUAL 0)
        message(FATAL_ERROR "Problem when removing \"$ENV{DESTDIR}${file}\"")
    endif (NOT ${rm_retval} EQUAL 0)
endforeach(file)

