use crate::grafo::ListaAdjacencia;

enum NodeContent {
    Vertice(usize),
    Aresta(usize, usize, i32),
}

pub struct Node<NodeContent> {
    pub valor: NodeContent,
    dir: Option<Box<Node<NodeContent>>>,
    esq: Option<Box<Node<NodeContent>>>,
    pai: Option<Box<Node<NodeContent>>>,
}

pub struct Leaf {
    pub valor: usize,
    pai: Option<Box<Node<NodeContent>>>,
}

impl Node<NodeContent> {
    pub fn new(conteudo: NodeContent) -> Self {
        Node {
            valor: conteudo,
            dir: None,
            esq: None,
            pai: None,
        }
    }

    pub fn direita(&self) -> Option<&Node<NodeContent>> {
        self.dir.as_deref()
    }

    pub fn esquerda(&self) -> Option<&Node<NodeContent>> {
        self.esq.as_deref()
    }

    pub fn pai(&self) -> Option<&Node<NodeContent>> {
        self.pai.as_deref()
    }

    pub fn criar_dir(&mut self, valor: NodeContent) -> Result<&Node<NodeContent>, String> {
        if self.dir.is_none() {
            self.dir = Some(Box::new(Node::new(valor)));
            Ok(self.dir.as_deref().unwrap())
        } else {
            Err(String::from("Erro ao inserir, está cheio"))
        }
    }

    pub fn criar_esq(&mut self, valor: NodeContent) -> Result<&Node<NodeContent>, String> {
        if self.esq.is_none() {
            self.esq = Some(Box::new(Node::new(valor)));
            Ok(self.esq.as_deref().unwrap())
        } else {
            Err(String::from("Erro ao inserir, está cheio"))
        }
    }

    pub fn set_dir(&mut self, valor: Node<NodeContent>) -> Option<Node<NodeContent>> {
        self.dir.replace(Box::new(valor)).map(|bx| *bx)
    }

    pub fn set_esq(&mut self, valor: Node<NodeContent>) -> Option<Node<NodeContent>> {
        self.esq.replace(Box::new(valor)).map(|bx| *bx)
    }

    pub fn set_pai(&mut self, valor: Node<NodeContent>) -> Option<Node<NodeContent>> {
        self.pai.replace(Box::new(valor)).map(|bx| *bx)
    }
}

pub struct BPT {
    pub root: Node<NodeContent>,
}

impl BPT {}

// impl ListaAdjacencia {
//     pub fn criar_apb(&self) -> ListaAdjacencia {
//         let n = self.size();
//         let mut edges = self.edges().collect::<Vec<_>>();
//         edges.sort_by_key(|(_f, _t, cost)| *cost);
//         let mut msf_edges = Vec::new();
//     }
// }
