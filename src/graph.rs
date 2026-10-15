use std::{
    collections::{HashMap, VecDeque},
    rc::Rc,
};

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
    name: Option<String>,
    vertices: HashMap<Rc<str>, Vertex>,
    connections: Vec<Edge>,
}

impl Default for UndirectedGraph {
    fn default() -> Self {
        Self {
            name: None,
            connections: Vec::new(),
            vertices: HashMap::new(),
        }
    }
}

impl UndirectedGraph {
    pub fn new<Name: ToString>(name: Name) -> Self {
        Self {
            name: Some(name.to_string()),
            connections: Vec::new(),
            vertices: HashMap::new(),
        }
    }

    pub(super) fn rename(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn reserve_space_for_vertices(&mut self, count: usize) {
        self.vertices.reserve(count);
    }

    pub fn reserve_space_for_edges(&mut self, count: usize) {
        self.connections.reserve(count);
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn get_vertices(&self) -> Vec<&Vertex> {
        self.vertices.values().collect()
    }

    pub fn with_capacity<Name: ToString>(name: Name, vertices_capacity: usize) -> Self {
        Self {
            name: Some(name.to_string()),
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

    /// A excentricidade de um vértice é a maior distância que ele pode ter em relação a algum vértice
    /// dentro do grafo, isto é, é a distância entre ele e o vértice mais distante dele.
    pub fn eccentricity_of_vertex(&self, vertex_key: &str) -> Result<usize, GraphError> {
        // assim é garantido que o vértice existe e ao mesmo tempo já pegamos a chave
        // dentro de um Rc
        let Some((vertex_key, _)) = self.vertices.get_key_value(vertex_key) else {
            return Err(GraphError::foreign_vertex());
        };

        let vertex_key = vertex_key.as_ref();

        // a lista de adjacência mapeia um vértice para um vetor de todos os vértices que estão ligados a ele
        let mut adjacency_map = HashMap::<&str, Vec<&str>>::with_capacity(self.vertices.len());
        for edge in &self.connections {
            // isso praticamente pega o vetor mapeado pelo vértice de origem e já insere o destino no seu vetor de adjacência...
            // o mesmo acontece pro target
            adjacency_map
                .entry(edge.origin())
                .or_default()
                .push(edge.target());

            adjacency_map
                .entry(edge.target())
                .or_default()
                .push(edge.origin());
        }

        let distances = bfs_distances(vertex_key, &adjacency_map);
        // esse unwrap nunca vai ser chamado na realidade
        let eccentricity = distances.into_values().max().unwrap_or(0);

        Ok(eccentricity)
    }
}

fn bfs_distances<'a>(
    start: &'a str,
    adjacency: &HashMap<&'a str, Vec<&'a str>>,
) -> HashMap<&'a str, usize> {
    let mut distances = HashMap::new();
    let mut queue = VecDeque::new();

    distances.insert(start, 0);
    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
        let distance = distances[current];

        for &neighbor in adjacency.get(current).into_iter().flatten() {
            if !distances.contains_key(neighbor) {
                distances.insert(neighbor, distance + 1);
                queue.push_back(neighbor);
            }
        }
    }

    distances
}
