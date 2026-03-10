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
    
    pub fn find_max(&self) -> Option<i32> {
        if let Some(right) = &self.right {
            right.find_max() 
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

    dbg!(tree);
}
