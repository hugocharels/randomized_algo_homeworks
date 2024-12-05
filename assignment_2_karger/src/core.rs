#[derive(Debug, Clone)]
pub struct AdjacencMatrix {
    pub matrix: Vec<Vec<usize>>,
}

impl AdjacencMatrix {
    pub fn new() -> Self {
        Self { matrix: Vec::new() }
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.add_vertex_if_not_exists(u);
        self.add_vertex_if_not_exists(v);
        self.matrix[u.max(v)][v.min(u)] += 1;
    }

    pub fn len(&self) -> usize {
        self.matrix.len()
    }

    fn add_vertex_if_not_exists(&mut self, v: usize) {
        for i in 0..v+1 {
            if self.matrix.len() <= i {
                self.matrix.push(vec![0; i+1]);
            } else if self.matrix[i].len() <= i {
                self.matrix[i].resize(v+1, 0);
            }
        }
    }

}

#[derive(Debug, Clone)]
pub struct Graph {
    pub adj_matrix: AdjacencMatrix,
}

impl Graph {
    pub fn new() -> Self {
        Self { adj_matrix: AdjacencMatrix::new() }
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.adj_matrix.add_edge(u, v);
    }

    pub fn contract_edge(&mut self, u: usize, v: usize) {
        if self.adj_matrix.len() <= u || self.adj_matrix.len() <= v {
            panic!("Vertex {} or {} does not exist", u, v);
        } else {
            todo!();
        }
    }
}
