use crate::graph::{UndirectedGraph, Vertex};

mod error;
mod gext;
mod graph;

fn print_eccentricity(graph: &UndirectedGraph, sorted_vertices: &[&Vertex]) -> anyhow::Result<()> {
    println!(
        "Calculando excentricidade do grafo {}.",
        graph.name().unwrap_or("não-nomeado")
    );

    for vertex in sorted_vertices {
        let eccentricity = graph.eccentricity_of_vertex(&vertex.key)?;
        println!(
            "Vértice {} (ID {}): {eccentricity}",
            &vertex.value, &vertex.key
        );
    }

    Ok(())
}

fn print_closeness_centrality(
    graph: &UndirectedGraph,
    sorted_vertices: &[&Vertex],
) -> anyhow::Result<()> {
    println!(
        "Calculando a centralidade da proximidade dos vértices do grafo {}.",
        graph.name().unwrap_or("não-nomeado")
    );

    for vertex in sorted_vertices {
        let closeness = graph.closeness_centrality_of_vertex(&vertex.key)?;
        println!(
            "Vértice {} (ID {}): {closeness:.6}",
            &vertex.value, &vertex.key
        );
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let graph = gext::read("LesMiserables.gexf")?;

    let mut vertices = graph.get_vertices();
    vertices.sort_by_key(|v| (v.key.parse::<u64>().unwrap_or(u64::MAX), v.key.to_string()));

    print_eccentricity(&graph, &vertices)?;
    println!("\n");
    print_closeness_centrality(&graph, &vertices)?;
    Ok(())
}
