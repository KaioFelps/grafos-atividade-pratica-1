use std::{collections::HashMap, rc::Rc};

use crate::error::GraphError;

#[derive(Debug)]
pub struct Edge(String, Rc<str>, Rc<str>);

impl Edge {
    pub fn new(id: String, origin: Rc<str>, target: Rc<str>) -> Self {
        Self(id, origin, target)
    }

    pub fn id(&self) -> &str {
        self.0.as_str()
    }

    pub fn origin(&self) -> &str {
        self.1.as_ref()
    }

    pub fn target(&self) -> &str {
        self.2.as_ref()
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Vertex {
    pub key: Rc<str>,
    pub value: String,
}

impl Vertex {
    pub fn new(key: String, value: String) -> Self {
        Self {
            key: Rc::from(key),
            value,
        }
    }
}

#[derive(Debug)]
pub struct UndirectedGraph {
    name: String,
    vertices: HashMap<Rc<str>, Vertex>,
    connections: Vec<Edge>,
}

impl Default for UndirectedGraph {
    fn default() -> Self {
        Self {
            name: "yet_unnamed".into(),
            connections: Vec::new(),
            vertices: HashMap::new(),
        }
    }
}

impl UndirectedGraph {
    pub fn new<Name: ToString>(name: Name) -> Self {
        Self {
            name: name.to_string(),
            connections: Vec::new(),
            vertices: HashMap::new(),
        }
    }

    pub(super) fn rename(&mut self, name: String) {
        self.name = name;
    }

    pub fn with_capacity<Name: ToString>(name: Name, vertices_capacity: usize) -> Self {
        Self {
            name: name.to_string(),
            connections: Vec::new(),
            vertices: HashMap::with_capacity(vertices_capacity),
        }
    }

    pub fn get_vertex(&self, key: &str) -> Option<&Vertex> {
        self.vertices.get(key)
    }

    pub fn add_vertex(&mut self, vertex: Vertex) {
        self.vertices.insert(vertex.key.clone(), vertex);
    }

    pub fn add_edge(&mut self, id: String, from: &str, to: &str) -> Result<(), GraphError> {
        if let Some(from) = self.vertices.get(from)
            && let Some(to) = self.vertices.get(to)
        {
            self.connections
                .push(Edge::new(id, from.key.clone(), to.key.clone()));

            return Ok(());
        }

        return Err(GraphError::foreign_vertex());
    }

    pub fn vertices_len(&self) -> usize {
        self.vertices.len()
    }

    pub fn edges_len(&self) -> usize {
        self.connections.len()
    }
}
