use std::collections::HashMap;

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
