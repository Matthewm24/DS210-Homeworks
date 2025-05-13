use super::tree::BinaryTree;

#[cfg(test)]
mod tree_tests {
    use super::*;

    #[test]
    fn test_empty_tree() {
        let tree = BinaryTree::new();
        assert_eq!(tree.find_diameter(), 0);
        assert_eq!(tree.get_root_value(), None);
    }

    #[test]
    fn test_single_node_tree() {
        let mut tree = BinaryTree::new();
        tree.build_random_tree(1);
        assert_eq!(tree.find_diameter(), 0);
        assert_eq!(tree.get_root_value(), Some(0));
    }

    #[test]
    fn test_two_node_tree() {
        let mut tree = BinaryTree::new();
        tree.build_random_tree(2);
        let diameter = tree.find_diameter();
        assert_eq!(diameter, 1);
        assert_eq!(tree.get_root_value(), Some(0));
    }

    #[test]
    fn test_three_node_tree() {
        let mut tree = BinaryTree::new();
        tree.build_random_tree(3);
        let diameter = tree.find_diameter();
        assert!(diameter >= 1 && diameter <= 2);
        assert_eq!(tree.get_root_value(), Some(0));
    }

    #[test]
    fn test_large_tree() {
        let mut tree = BinaryTree::new();
        tree.build_random_tree(120);
        let diameter = tree.find_diameter();
        assert!(diameter > 0);
        assert!(diameter <= 119); // Maximum possible diameter for 120 nodes
        assert_eq!(tree.get_root_value(), Some(0));
    }

    #[test]
    fn test_tree_construction() {
        let mut tree = BinaryTree::new();
        tree.build_random_tree(5);
        assert_eq!(tree.get_root_value(), Some(0));
        let diameter = tree.find_diameter();
        assert!(diameter >= 0 && diameter <= 4);
    }
} 