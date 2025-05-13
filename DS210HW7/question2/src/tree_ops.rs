use crate::node::Node;

pub fn calculate_diameter(root: &Option<Box<Node>>) -> i32 {
    if let Some(root) = root {
        let (_, diameter) = dfs_diameter(root);
        diameter
    } else {
        0
    }
}

fn dfs_diameter(node: &Box<Node>) -> (i32, i32) {
    let mut max_diameter = 0;
    let mut depths = Vec::new();

    if let Some(left) = node.get_left() {
        let (depth, diameter) = dfs_diameter(left);
        depths.push(depth);
        max_diameter = max_diameter.max(diameter);
    }

    if let Some(right) = node.get_right() {
        let (depth, diameter) = dfs_diameter(right);
        depths.push(depth);
        max_diameter = max_diameter.max(diameter);
    }

    depths.sort_unstable();
    let current_diameter = depths.iter().sum::<i32>();
    max_diameter = max_diameter.max(current_diameter);
    let max_depth = depths.last().unwrap_or(&0) + 1;

    (max_depth, max_diameter)
} 