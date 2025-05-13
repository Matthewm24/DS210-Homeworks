use std::collections::VecDeque;
use rand::Rng;
use crate::node::Node;
use crate::tree_ops::calculate_diameter;

pub struct TreeNode {
    pub id: usize,
    pub children: Vec<usize>,
}

pub struct BinaryTree {
    pub nodes: Vec<TreeNode>,
    pub root: usize,
}

impl BinaryTree {
    pub fn new(size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut nodes = Vec::with_capacity(size);
        
        nodes.push(TreeNode {
            id: 0,
            children: Vec::new(),
        });

        for i in 1..size {
            nodes.push(TreeNode {
                id: i,
                children: Vec::new(),
            });
        }

        let mut available_nodes = VecDeque::new();
        available_nodes.push_back(0);

        for i in 1..size {
            if let Some(parent_idx) = available_nodes.pop_front() {
                nodes[parent_idx].children.push(i);
                available_nodes.push_back(i);
                
                if rng.gen::<f64>() < 0.5 && i + 1 < size {
                    nodes[parent_idx].children.push(i + 1);
                    available_nodes.push_back(i + 1);
                }
            }
        }

        BinaryTree {
            nodes,
            root: 0,
        }
    }

    pub fn compute_diameter(&self) -> usize {
        let (farthest_node, _) = self.bfs_farthest(self.root);
        let (_, diameter) = self.bfs_farthest(farthest_node);
        diameter
    }

    fn bfs_farthest(&self, start: usize) -> (usize, usize) {
        let mut visited = vec![false; self.nodes.len()];
        let mut queue = VecDeque::new();
        let mut distances = vec![0; self.nodes.len()];
        
        queue.push_back(start);
        visited[start] = true;
        
        while let Some(current) = queue.pop_front() {
            if let Some(parent) = self.find_parent(current) {
                if !visited[parent] {
                    visited[parent] = true;
                    distances[parent] = distances[current] + 1;
                    queue.push_back(parent);
                }
            }
            
            for &child in &self.nodes[current].children {
                if !visited[child] {
                    visited[child] = true;
                    distances[child] = distances[current] + 1;
                    queue.push_back(child);
                }
            }
        }
        
        let max_distance = distances.iter().max().unwrap();
        let farthest_node = distances.iter().position(|&d| d == *max_distance).unwrap();
        
        (farthest_node, *max_distance)
    }

    fn find_parent(&self, node_id: usize) -> Option<usize> {
        for (i, node) in self.nodes.iter().enumerate() {
            if node.children.contains(&node_id) {
                return Some(i);
            }
        }
        None
    }

    pub fn find_diameter(&self) -> i32 {
        calculate_diameter(&self.root)
    }

    pub fn get_root_value(&self) -> Option<i32> {
        self.root.as_ref().map(|node| node.get_value())
    }
} 