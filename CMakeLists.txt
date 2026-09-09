cmake_minimum_required(Version 3.15)

# Name the project and specify C++ as the language 
project(uclid Version 1.0 LANGUAGES CXX)

# Enforce modern C++ (C++17 is great for libraries like toml++)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARDS_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON) # Generates a file to help VS Code's IntelliSense

# Download toml++ directly from GitHub
include(FetchContent)
FetchContent_Declare(
	tomlplusplus
	GIT_REPOSITORY http://github.com/marzer/tomlplusplus.git
	GIT_TAG        v3.4.0
)

# Explicitly list you source files
# (It is generally better practice to list them manually rather than using GLOB,
# so CMake knows exactly when a new file is added)
set(SOURCES
	src/main.cpp
	src/World.cpp
)

# Create the executable
add_executable(${PROJECT_NAME} ${SOURCES})

# Tell the compiler where to find the header files
# PRIVATE means these headers are only needed to build this specific executable
target_include_directories(${PROJECT_NAME} PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/include)

#---------------------------------------------------------
# Placeholder for future libraries (TOML, rendering, etc.)
#---------------------------------------------------------
# find_package(ExternalLibrary REQUIRED)
# target_link_libraries(${PROJECT_NAME} PRIVATE ExternalLibrary)

# Link the downloaded library to your uclid executable
target_link_libraries(${PROJECT_NAME} PRIVATE tomlplusplus::tomlplusplus)
