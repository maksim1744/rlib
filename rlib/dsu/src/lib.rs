#[derive(Clone, Debug)]
pub struct DSU {
    p: Vec<usize>,
    sz: Vec<usize>,
}

impl DSU {
    pub fn new(n: usize) -> Self {
        Self {
            p: (0..n).collect(),
            sz: vec![1; n],
        }
    }

    pub fn reset(&mut self, n: usize) {
        self.p.resize(n, 0);
        for i in 0..n {
            self.p[i] = i;
        }
        self.sz.resize(n, 0);
        for i in 0..n {
            self.sz[i] = 1;
        }
    }

    pub fn par(&mut self, v: usize) -> usize {
        if self.p[v] != v {
            self.p[v] = self.par(self.p[v]);
        }
        self.p[v]
    }

    pub fn un(&mut self, mut u: usize, mut v: usize) -> bool {
        u = self.par(u);
        v = self.par(v);
        if u == v {
            return false;
        }
        if self.sz[u] > self.sz[v] {
            std::mem::swap(&mut u, &mut v);
        }
        self.sz[v] += self.sz[u];
        self.p[u] = v;
        true
    }

    pub fn check(&mut self, u: usize, v: usize) -> bool {
        self.par(u) == self.par(v)
    }

    pub fn size(&mut self, v: usize) -> usize {
        let v = self.par(v);
        self.sz[v]
    }
}
