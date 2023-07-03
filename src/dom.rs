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
