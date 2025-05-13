use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use crate::graph::Graph;

pub fn read_graph_from_file<P>(filename: P) -> io::Result<Graph>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();
    
    let mut graph = None;
    let mut line_count = 0;

    for line in lines {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        if line_count == 0 {
            let vertices = parts[0].parse::<usize>().unwrap();
            graph = Some(Graph::new(vertices));
        } else {
            let from = parts[0].parse::<usize>().unwrap();
            let to = parts[1].parse::<usize>().unwrap();
            if let Some(ref mut g) = graph {
                g.add_edge(from, to);
            }
        }
        line_count += 1;
    }

    Ok(graph.unwrap())
} 