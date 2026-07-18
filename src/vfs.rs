/***************************************************************************************
This file creates a rood ==Directory== node. 
It then inserts a file named ==hello.txt==.
It creates a sub-directory which contains its own ==pendulum.sim== file.
It initializes the ==current_path== as an empty vector, meaning you are 
currently at the "top" level.
****************************************************************************************/

/* 
==use== is a ==path shortener==
is i wrote use Jerry::Smith as dummy;
from now on, when i use dummy, it knows i mean Jerry::Smith
A Rust specific rule
everything in Rust is private by default
if you want to ==use== something from another module, 
you have to explicitly mark it as ==pub== (public) in that module
use std::collections::HashMap;
*/

/* #[derive(Debug)] is a procedural macro in Rust. It tells the compiler 
to automatically write the code necessary for you to print your type 
using the {:?} formatter.
Rust is a compiled language and does not have "introspection" at runtime.
It needs to know how to convert your data into a string *at compile time*.
If you define a struct or an enum and try to pass it to println!(),
the compiler will panic because it does not know how to represent your 
custom type as text.

The solution:
In Rust, a "trait" is a set of capabilities you can give a type.
The Debug trait is a standard capability that says, "This type knows how to 
represent itself in a format suitable for debugging."

by adding #[derive(Debug)] above your enum or struct, you are instructing
the Rust compiler to automatically generate the implementation of the Debug 
trait for that type
*/

/* 
how to use it:
once you add that line, you can use the special {:?} (Debug) format 
specifier in println!.
Why it's useful:
without this, you would have to manually write a function that iterates though 
every field of your enum or struct and prints them out,
which is tedious and error-prone.
*/
#[derive(Debug)]
pub enum Node {
    File { content: String },
    Directory { children: HashMap<String, Node> },
}

pub struct VirtualFileSystem {
    pub root: Node,
    // This a path, e.g., vec!["home", "user", "documents"]
    pub current_path: Vec<String>.
}

impl VirtualFileSystem {
    pub fn new() -> Self {
        // Create the initial tree
        let mut root_children = HashMap::new();

        // Add a simple file
        root_children.insert(
            "hello.txt".to_string(),
            Node::File { content: "Welcome to Uclid".to_string() }
        );

        // Add a simulations directory
        let mut sim_children = HashMap::new()
        sim_children.insert(
            "pendulum.dim".to_string(),
            Node::File { content: "Physics data goes here.".to_string() }
        );

        root_children.insert(
            "simulations",to_string(),
            Node::Directory { children: sim_children }
        );

        VirtualFileSystem {
            root: Node::Directory { children: sim_children },
            current_path: Vec::new(), // Starts at root
        }
    }
}