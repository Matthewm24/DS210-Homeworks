mod node;
mod tree;
mod tree_ops;
mod tests;

use tree::BinaryTree;

fn main() {

    let tree = BinaryTree::new(120);
    

    let diameter = tree.compute_diameter();
    println!("The diameter of the binary tree is: {}", diameter);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_tree_diameter() {
        let mut tree = BinaryTree::new(5);

        tree.nodes[0].children = vec![1, 2];
        tree.nodes[1].children = vec![3, 4];
        
        assert_eq!(tree.compute_diameter(), 3);
    }

    #[test]
    fn test_tree_creation() {
        let tree = BinaryTree::new(120);
        assert_eq!(tree.nodes.len(), 120);
        assert_eq!(tree.root, 0);
    }
}