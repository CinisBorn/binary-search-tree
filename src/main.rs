use std::cmp;

/// A binary search implementation where the`value` is `i32`
/// by design. It could also be a generic type. 
#[derive(Debug)]
struct BinarySearchTree {
    value: i32,
    left: Option<Box<BinarySearchTree>>,
    right: Option<Box<BinarySearchTree>>,
}

impl BinarySearchTree {
    /// Creates a new binary search tree with the specified root. 
    pub fn with_root(root: i32) -> Self {
        Self {
            value: root,
            left: None,
            right: None,
        }
    }
    /// Inserts a new node in the tree. The exact position depends on the value
    /// itself. See more in: 

    /// [Binary Search Tree](https://en.wikipedia.org/wiki/Binary_search_tree)
    pub fn insert(&mut self, value: i32) {
        match value.cmp(&self.value) {
            cmp::Ordering::Equal => (),
            cmp::Ordering::Greater => {
                self.right
                    .get_or_insert_with(|| {
                        Box::new(Self {
                            left: None,
                            right: None,
                            value,
                        })
                    })
                    .insert(value);
            }
            cmp::Ordering::Less => {
                self.left
                    .get_or_insert_with(|| {
                        Box::new(Self {
                            left: None,
                            right: None,
                            value,
                        })
                    })
                    .insert(value);
            }
        }
    }
    
    /// Checks if a value exists. 
    pub fn contains(&self, value: i32) -> bool {
        match value.cmp(&self.value) {
            cmp::Ordering::Equal => true,
            cmp::Ordering::Greater => match &self.right {
                Some(right) => right.contains(value),
                None => false,
            },
            cmp::Ordering::Less => match &self.left {
                Some(left) => left.contains(value),
                None => false,
            },
        }
    }
    
    /// Gets the maximum value in the tree. It has a complexity of *O(h)*
    pub fn find_max(&self) -> Option<i32> {
        if let Some(right) = &self.right {
            right.find_max() 
        } else {
            Some(self.value)
        }
    }
    /// Gets the minimum value in the tree. It has a complexity of *O(h)*
    pub fn find_min(&self) -> Option<i32> {
        if let Some(left) = &self.left {
            left.find_min() 
        } else {
            Some(self.value)
        }
    }
}

fn main() {
    let mut tree = BinarySearchTree::with_root(10);

    tree.insert(5);
    tree.insert(12);
    tree.insert(12); // ignored
    tree.insert(13);

    println!("{}", tree.contains(5));
    println!("{}", tree.contains(30_000));
    println!("{}", tree.contains(100));
    
    println!("Max: {}", tree.find_max().unwrap());
    println!("Min: {}", tree.find_min().unwrap());
    
    dbg!(tree);
}
