use std::println;

use crate::graph::UndirectedGraph;

mod error;
mod gext;
mod graph;

fn print_eccentricity(graph: &UndirectedGraph) -> anyhow::Result<()> {
    println!(
        "Calculando excentricidade do grafo {}.",
        graph.name().unwrap_or("não-nomeado")
    );

    for vertex in graph.get_vertices() {
        let eccentricity = graph.eccentricity_of_vertex(&vertex.key)?;
        println!(
            "Vértice {} (ID {}): {eccentricity}",
            &vertex.value, &vertex.key
        );
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let graph = gext::read("LesMiserables.gexf")?;
    print_eccentricity(&graph)?;
    Ok(())
}
