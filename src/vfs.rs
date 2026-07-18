use std::collections::HashMap;

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