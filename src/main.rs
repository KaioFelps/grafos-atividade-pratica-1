use std::println;

mod error;
mod gext;
mod graph;

fn main() -> anyhow::Result<()> {
    let graph = gext::read("LesMiserables.gexf")?;
    println!("{graph:?}");
    Ok(())
}
