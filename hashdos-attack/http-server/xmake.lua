add_rules("mode.debug", "mode.release")

add_requires("cpp-httplib")
add_requires("xxhash")

target("http-server")
    set_kind("binary")
    add_files("src/*.cpp")
    add_packages("cpp-httplib")
    add_packages("xxhash")

-- set language: c++17
set_languages("c++17")
