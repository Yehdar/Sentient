use std::collections::HashMap;
use std:fmt;

struct Node {
    children: Vec<Node>,
    node_type: NodeType,
}

struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}

type AttrMap = HashMap<String, String>;

enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(String),
}

impl Node {
    fn new(node_type: NodeType, children: Vec<Node>) -> Node {
        Node {
            node_type,
            children,
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.node_type),
    }    
}
