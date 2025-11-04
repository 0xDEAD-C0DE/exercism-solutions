pub mod graph {
    use graph_items::edge::Edge;
    use graph_items::node::Node;
    use std::collections::HashMap;

    #[derive(PartialEq, Debug)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        // pub attrs: Vec<&'a [(String, String)]>,
        pub attrs: HashMap<String, String>,
    }
    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Debug, PartialEq, Clone)]
            pub struct Edge {
                tail: String,
                head: String,
                // attrs: Vec<&'a [(String, String)]>,
                attrs: HashMap<String, String>,
            }
            impl Edge {
                pub fn new(tail: &str, head: &str) -> Self {
                    Self {
                        tail: tail.into(),
                        head: head.into(),
                        attrs: HashMap::new(),
                    }
                }
                pub fn with_attrs(mut self, a: &[(&str, &str)]) -> Self {
                    for attr in a {
                        self.attrs.insert(attr.0.into(), attr.1.into());
                    }
                    self
                }
                pub fn attr(&self, id: &str) -> Option<&str> {
                    self.attrs.get(id).map(|v| v.as_str())
                }
            }
        }
        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, PartialEq, Clone)]
            pub struct Node {
                pub name: String,
                attrs: HashMap<String, String>,
            }
            impl Node {
                pub fn new(name: &str) -> Self {
                    Self {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    }
                }
                pub fn with_attrs(mut self, a: &[(&str, &str)]) -> Self {
                    for attr in a {
                        self.attrs.insert(attr.0.into(), attr.1.into());
                    }
                    self
                }
                pub fn attr(&self, id: &str) -> Option<&str> {
                    self.attrs.get(id).map(|v| v.as_str())
                }
            }
        }
    }
    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }
        pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Self {
            for node in nodes {
                self.nodes.push(node.to_owned());
            }
            self
        }
        pub fn with_attrs(mut self, a: &[(&str, &str)]) -> Self {
            for attr in a {
                self.attrs.insert(attr.0.into(), attr.1.into());
            }
            self
        }
        pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
            for edge in edges {
                self.edges.push(edge.to_owned());
            }
            self
        }
        pub fn node(&self, id: &str) -> Option<&Node> {
            self.nodes.iter().find(|node| node.name == id)
        }
    }
    // pub trait Attrs {
    //     fn with_attrs(mut self, a: &[(String, String)]) -> Self {
    //         self
    //     }
    // }
}
