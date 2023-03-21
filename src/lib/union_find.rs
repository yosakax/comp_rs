use std::io::*;

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}
impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            return x;
        }
        self.parent[x] = self.root(self.parent[x]);
        return self.parent[x];
    }
    fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
    fn union(&mut self, mut parent: usize, mut child: usize) -> bool {
        parent = self.root(parent);
        child = self.root(child);
        if parent == child {
            return false;
        }
        if self.size[parent] < self.size[child] {
            std::mem::swap(&mut parent, &mut child);
        }
        self.parent[child] = parent;
        self.size[parent] += self.size[child];
        return true;
    }
    fn size(&mut self, x: usize) -> usize {
        let root: usize = self.root(x);
        return self.size[root];
    }
    fn roots(&mut self) -> Vec<usize> {
        let mut roots = Vec::<usize>::new();
        for (i, v) in self.parent.iter().enumerate() {
            if v.to_owned() == i {
                roots.push(i);
            }
        }
        return roots;
    }
}
