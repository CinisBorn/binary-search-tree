use std::cmp;

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
            match &mut self.left {
                Some(left) => left.insert(value),
                None => {
                    self.left = Some(Box::new(BinarySearchTree {
                        left: None,
                        right: None,   
                        value: value
                    }))
                }
            }
        } else {
            match &mut self.right {
                Some(right) => right.insert(value),
                None => {
                    self.right = Some(Box::new(BinarySearchTree {
                        left: None,
                        right: None,   
                        value: value
                    }))
                }
            }
        }
    }
    
    pub fn contains(&self, value: i32) -> bool {
        match value.cmp(&self.value) {
            cmp::Ordering::Equal => true,
            cmp::Ordering::Greater => match &self.right {
                Some(right) => right.contains(value),
                None => return false,
            },
            cmp::Ordering::Less => match &self.left {
                Some(left) => left.contains(value),
                None => return false,
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
    
    println!("{}", tree.contains(5));
    println!("{}", tree.contains(30_000));
    println!("{}", tree.contains(100));
    
    dbg!(tree);
}
