use rand::Rng;

pub struct Graph {
    pub vertices: usize,
    pub adjacency_list: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new(vertices: usize) -> Self {
        Graph {
            vertices,
            adjacency_list: vec![Vec::new(); vertices],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.adjacency_list[from].push(to);
    }

    pub fn walk_simulation(&self, start: usize, steps: usize) -> usize {
        let mut current = start;
        let mut rng = rand::thread_rng();

        for _ in 0..steps {
            if self.adjacency_list[current].is_empty() {
                current = rng.gen_range(0..self.vertices);
            } else {
                let random = rng.gen::<f64>();
                if random < 0.8 {
                    let edges = &self.adjacency_list[current];
                    current = edges[rng.gen_range(0..edges.len())];
                } else {
                    current = rng.gen_range(0..self.vertices);
                }
            }
        }
        current
    }

    pub fn compute_pagerank(&self, num_walks: usize, steps_per_walk: usize) -> Vec<(usize, f64)> {
        let mut visit_counts = vec![0; self.vertices];

        for start in 0..self.vertices {
            for _ in 0..num_walks {
                let end = self.walk_simulation(start, steps_per_walk);
                visit_counts[end] += 1;
            }
        }

        let total_walks = num_walks as f64 * self.vertices as f64;
        let mut pageranks: Vec<(usize, f64)> = visit_counts
            .iter()
            .enumerate()
            .map(|(vertex, &count)| (vertex, count as f64 / total_walks))
            .collect();
        pageranks.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        pageranks
    }
} 