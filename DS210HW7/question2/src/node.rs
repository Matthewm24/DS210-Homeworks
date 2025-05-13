#[derive(Debug)]
pub struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn new(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    pub fn add_child(&mut self, value: i32) -> bool {
        if self.left.is_none() {
            self.left = Some(Box::new(Node::new(value)));
            true
        } else if self.right.is_none() {
            self.right = Some(Box::new(Node::new(value)));
            true
        } else {
            false
        }
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }

    pub fn get_left(&self) -> &Option<Box<Node>> {
        &self.left
    }

    pub fn get_right(&self) -> &Option<Box<Node>> {
        &self.right
    }
} 