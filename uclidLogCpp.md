# Uclid C++ Log

## 09/06/26

### Header files 

At the most basic level a C++ header file (.h or .hpp) acts as a menu or a table of contents for the code. It tells the compiler *what* tools exist (e.g., the names of functions, classes, or variables) and how to interact with them, without spelling out exactly *how* they work behind the scenes.

## What needs to be inside a Header File?

**Include guards:** This is a safety guard and is usually written at the top of the header file as '#pragma once'. This prevents the compiler from accidentally copying the header's contents more than once, causing an error.

**Declarations (Prototypes):** The exact name, return type, and parameters of the function or class you want to share.

*Note: For the source file to actually use the header, the source file must physically link them by using the #include "filename.h" directive at the top.*

## Example

### 1 The Header File (math_tools.h)

This tells the program that an `add` function exists, but doesn't do the math yet.

'''
#pragma once

// This is the declaration (the "what")
int add(int a, int b);
'''

### 2 The Source File (math_tools.cpp)

This includes the header and provides the actual logic for the function.

'''
**cpp**

#include "math_tools.h"

// This is the implementation (the "how")
int add(int a, int b){
    return a + b;
}
'''

### 3 The Main Project (main.cpp)

This file just wants to use the tool. It only needs to look at the header file to know how to use it.


