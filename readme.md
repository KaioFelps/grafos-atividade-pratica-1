# Algoritmos em Grafos (Prática 1)

- [x] Implemente um código que faça a leitura de um grafo no formato GEXF e o
    armazene um estrutura de dados para um **grafo não dirigido**. Considere que o
    grafo de entrada de teste **é conexo**.
- [x] Implemente um código que calcule a excentricidade de cada vértice.
    - [x] Escreva nos comentários do código o que essa estatística representa.
    - [x] O código deve imprimir como saída de texto a excentricidade de cada vértice.
- [x] Pesquise sobre a estatística closeness centrality e implemente um código que calcule a
    versão normalizada da closeness centrality de cada vértice.
    - [x] Escreva nos comentários do código o que essa estatística representa.
    - [x] O código também deve imprimir como saída de texto a closeness centrality normalizado de cada vértice.

## Rodando
Para executar, é necessário ter o Rust (e o Cargo) instalados. Com isso feito, basta
utilizar o comando `cargo run -- <CAMINHO DO ARQUIVO .gexf`. (Os `--` são necessários para repassar os argumentos
pro programa ao invés de simplesmente passar pro Cargo.)

Se preferir, utilize `cargo build --release` e, então, `./target/release/atividade-pratica-1 <CAMINHO DO ARQUIVO.gexf>`.
