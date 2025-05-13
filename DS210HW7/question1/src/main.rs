mod graph;
mod file_ops;

use std::io;
use file_ops::read_graph_from_file;

fn main() -> io::Result<()> {
    let graph = read_graph_from_file("pagerank_data.txt")?;
    let pageranks = graph.compute_pagerank(90, 90);

    for (vertex, rank) in pageranks.iter().take(5) {
        println!("vertex {}: approximate PageRank {:.3}", vertex, rank);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use graph::Graph;

    #[test]
    fn test_sum_of_pagerank_is_1() {
        let mut g = Graph::new(3);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 0);

        let pageranks = g.compute_pagerank(10, 10);
        assert_eq!(pageranks.len(), 3);
        let total: f64 = pageranks.iter().map(|(_, pr)| pr).sum();
        assert!((total - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_graph_creation() {
        let mut g = Graph::new(4);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        g.add_edge(3, 0);
        
        assert_eq!(g.vertices, 4);
        assert_eq!(g.adjacency_list[0], vec![1]);
        assert_eq!(g.adjacency_list[1], vec![2]);
        assert_eq!(g.adjacency_list[2], vec![3]);
        assert_eq!(g.adjacency_list[3], vec![0]);
    }
}
