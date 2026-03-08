#[derive(Debug)]
struct BinarySearchTree {
    value: i32,
    left: Option<Box<BinarySearchTree>>,
    right: Option<Box<BinarySearchTree>>,
}

impl BinarySearchTree {
    pub fn with_root(root: i32) -> Self {
        Self {
            value: root,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, value: i32) {
        if value <= self.value {
            if let Some(node) = &mut self.left {
                node.insert(value)
            } else { 
                self.left = Some(Box::new(BinarySearchTree {
                    left: None,
                    right: None,   
                    value: value
                }))
            }
        } else {
            if let Some(node) = &mut self.right {
                node.insert(value)
            } else {
                self.right = Some(Box::new(BinarySearchTree {
                    left: None,
                    right: None,   
                    value: value
                }))
            }
            
        }
    }
}

fn main() {
    let mut tree = BinarySearchTree::with_root(10);
    
    tree.insert(5);
    tree.insert(12);
    tree.insert(12);
    tree.insert(3);
    tree.insert(100);
    tree.insert(6);
    tree.insert(4);
    
    dbg!(tree);
}
