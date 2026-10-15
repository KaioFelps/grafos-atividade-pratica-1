use std::path::Path;

use anyhow::anyhow;
use quick_xml::{
    Reader,
    events::{BytesStart, Event},
};

use crate::{
    error::GraphError,
    graph::{UndirectedGraph, Vertex},
};

pub fn read<P: AsRef<Path>>(path: P) -> anyhow::Result<UndirectedGraph> {
    // code sample from https://github.com/tafia/quick-xml
    let xml = std::fs::read_to_string(path)?;

    let mut reader = Reader::from_str(xml.as_ref());
    reader.config_mut().trim_text(true);

    let mut expected_vertices_count: Option<usize> = None;
    let mut expected_edges_count: Option<usize> = None;

    let mut graph = UndirectedGraph::default();

    loop {
        match reader.read_event() {
            Err(e) => {
                return Err(anyhow!(
                    "Erro ao ler o grafo na posição {}: {:?}",
                    reader.error_position(),
                    e
                ));
            }

            Ok(Event::Eof) => break,

            Ok(Event::Start(e) | Event::Empty(e)) => match e.name().as_ref() {
                "nodes" => {
                    if let Ok(Some(attribute)) = e.try_get_attribute("count") {
                        let count = attribute.value.parse::<usize>()?;
                        graph.reserve_space_for_vertices(count);
                        expected_vertices_count = Some(count);
                    }
                }
                "edges" => {
                    if let Ok(Some(attribute)) = e.try_get_attribute("count") {
                        let count = attribute.value.parse::<usize>()?;
                        graph.reserve_space_for_edges(count);
                        expected_edges_count = Some(count);
                    }
                }
                "graph" => {
                    if let Ok(Some(attribute)) = e.try_get_attribute("id") {
                        graph.rename(attribute.value.to_string());
                    }
                }
                "node" => {
                    let key = get_attribute(&e, "id")?;
                    let value = get_attribute(&e, "label")?;
                    let vertex = Vertex::new(key, value);
                    graph.add_vertex(vertex);
                }
                "edge" => {
                    let id = get_attribute(&e, "id")?;
                    let source = get_attribute(&e, "source")?;
                    let target = get_attribute(&e, "target")?;

                    graph.add_edge(id, &source, &target)?;
                }
                _ => (),
            },
            _ => (),
        }
    }

    let is_missing_vertices =
        expected_vertices_count.is_some_and(|expect| expect != graph.vertices_len());

    let is_missing_edges = expected_edges_count.is_some_and(|expect| expect != graph.edges_len());

    if is_missing_vertices || is_missing_edges {
        return Err(anyhow!(GraphError::malformed_graph()));
    }

    Ok(graph)
}

fn get_attribute(e: &BytesStart<'_>, key: &'static str) -> Result<String, GraphError> {
    e.try_get_attribute(key)
        .ok()
        .flatten()
        .ok_or(GraphError::malformed_vertex())
        .map(|attribute| attribute.value.to_string())
}
