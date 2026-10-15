use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum GraphErrorKind {
    MalformedGraph,
    ForeignVertex,
    MalformedVertex,
    MalformedEdge,
}

#[derive(Debug)]
pub struct GraphError {
    kind: GraphErrorKind,
}

impl GraphError {
    pub fn foreign_vertex() -> Self {
        Self {
            kind: GraphErrorKind::ForeignVertex,
        }
    }

    pub fn malformed_graph() -> Self {
        Self {
            kind: GraphErrorKind::MalformedGraph,
        }
    }

    pub fn malformed_vertex() -> Self {
        Self {
            kind: GraphErrorKind::MalformedVertex,
        }
    }

    pub fn malformed_edge() -> Self {
        Self {
            kind: GraphErrorKind::MalformedEdge,
        }
    }
}

impl Display for GraphError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_msg = match self.kind {
            GraphErrorKind::MalformedGraph => "O grafo final está mal formado.",
            GraphErrorKind::ForeignVertex => "O vértice fornecido não pertence ao grafo.",
            GraphErrorKind::MalformedEdge => "A aresta fornecida está mal formada.",
            GraphErrorKind::MalformedVertex => "O vértice fornecido está mal formado.",
        };

        write!(f, "{err_msg}")?;
        Ok(())
    }
}

impl Error for GraphError {}
