use std::vec;

#[derive(Clone)]
pub struct Edge {
    pub node: usize,
    pub weight: i32,
}

impl Edge {
    pub fn new(node: usize, weight: i32) -> Self {
        Self { node, weight } //Teoricamente só será necessário 1 byte para o peso, mas otimizações vem depois
    }
}

pub struct ListaAdjacencia {
    grafo: Vec<Vec<Edge>>,
}

impl ListaAdjacencia {
    fn create_list(n: usize) -> Self {
        Self {
            grafo: vec![vec![]; n],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.grafo.is_empty()
    }

    pub fn add_edge(&mut self, u: usize, v: usize, weight: i32) {
        self.grafo[u].push(Edge::new(v, weight));
        self.grafo[v].push(Edge::new(v, weight));
    }

    pub fn new(size: usize, edges: &[(usize, usize, i32)]) -> Self {
        let mut novo = Self::create_list(size);
        for &(a, b, c) in edges.iter() {
            novo.add_edge(a, b, c);
        }
        novo
    }

    pub fn edges(&self) -> impl Iterator<Item = (usize, usize, i32)> + '_ {
        self.grafo
            .iter()
            .enumerate()
            .flat_map(|(a, edges)| edges.iter().map(move |b| (a, b.node, b.weight)))
    }

    pub fn num_edges(&self) -> usize {
        self.edges().count()
    }

    pub fn size(&self) -> usize {
        self.grafo.len()
    }

    pub fn nodes(&self) -> impl Iterator<Item = (usize, &Vec<Edge>)> {
        self.grafo.iter().enumerate()
    }
}
