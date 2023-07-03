struct Node {
    children: Vec<Node>,
    node_type: NodeType,
}
<<<<<<< HEAD

struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}

enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(HashMap),
}

=======
>>>>>>> 0daa1407a47058a100e34a3bc7301bae8e21dd18
