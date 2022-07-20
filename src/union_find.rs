/// Union-Find processes the following queries for an edgeless graph in `O(α(n))` amortized time.
/// * Add an undirected edge.
/// * Deciding whether given two vertices are in the same connected component
pub struct UnionFind {
    /// For each element, one of the following is stored.
    /// * Size of the connected component to which it belongs (if it is representative of a connected component)
    /// * Index of the element that is its own parent (otherwise)
    parent_or_size: Vec<i32>,

    /// If it is not representative, the index of its parent is stored.
    group_num: usize,
}

#[allow(dead_code)]
impl UnionFind {
    /// It creates an undirected graph with `n` vertices and 0 edges.
    pub fn new(n: usize) -> Self {
        UnionFind {
            parent_or_size: vec![-1; n],
            group_num: n,
        }
    }

    /// It returns the representative of the connected component that contains the vertex `a`.
    pub fn leader(&mut self, a: usize) -> usize {
        if self.parent_or_size[a] < 0 {
            return a;
        }
        self.parent_or_size[a] = self.leader(self.parent_or_size[a] as usize) as i32;
        self.parent_or_size[a] as usize
    }

    /// It returns whether the vertices `a` and `b` are in the same connected component.
    pub fn same(&mut self, a: usize, b: usize) -> bool {
        self.leader(a) == self.leader(b)
    }

    /// It adds an edge between vertex `a` and vertex `b`.
    pub fn merge(&mut self, a: usize, b: usize) -> bool {
        let mut leader_a = self.leader(a);
        let mut leader_b = self.leader(b);
        if leader_a == leader_b {
            return false;
        }
        if self.parent_or_size[leader_a] > self.parent_or_size[leader_b] {
            std::mem::swap(&mut leader_a, &mut leader_b);
        }
        self.parent_or_size[leader_a] += self.parent_or_size[leader_b];
        self.parent_or_size[leader_b] = leader_a as i32;
        self.group_num -= 1;
        true
    }

    /// It returns the size of the connected component that contains the vertex `a`.
    pub fn size(&mut self, a: usize) -> usize {
        let leader = self.leader(a);
        -self.parent_or_size[leader] as usize
    }

    /// It adds a new vertex.
    pub fn add(&mut self) {
        self.parent_or_size.push(-1);
        self.group_num += 1;
    }

    /// It returns the number of connected components.
    pub fn get_group_num(&self) -> usize {
        self.group_num
    }

    /// It returns the number of elements.
    pub fn get_elem_num(&self) -> usize {
        self.parent_or_size.len()
    }
}
