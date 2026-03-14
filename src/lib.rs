#[derive(Debug)]
struct Node<T: Ord + Clone> {
    element: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord + Clone> Node<T> {
    fn insert(node: &mut Option<Box<Node<T>>>, element: T) {
        if let Some(node) = node {
            match element.cmp(&node.element) {
                std::cmp::Ordering::Equal => (),
                std::cmp::Ordering::Greater => Self::insert(&mut node.right, element),
                std::cmp::Ordering::Less => Self::insert(&mut node.left, element),
            }
        } else {
            *node = Some(Box::new(Node {
                element,
                left: None,
                right: None,
            }))
        }
    }

    fn find_min(node: &Option<Box<Node<T>>>) -> Option<T> {
        let node = node.as_ref()?;

        match &node.left {
            Some(_) => Self::find_min(&node.left),
            None => Some(node.element.clone()),
        }
    }

    fn find_max(node: &Option<Box<Node<T>>>) -> Option<T> {
        let node = node.as_ref()?;

        match &node.right {
            Some(_) => Self::find_max(&node.right),
            None => Some(node.element.clone()),
        }
    }
    
    fn remove(node: &mut Option<Box<Node<T>>>, value: T) -> Option<T> {
        let node_val = node.as_mut()?;
        let removed = node_val.element.clone();
        
        match value.cmp(&node_val.element) {
            std::cmp::Ordering::Greater => Self::remove(&mut node_val.right, value),
            std::cmp::Ordering::Less => Self::remove(&mut node_val.left, value), 
            std::cmp::Ordering::Equal => {
                let right = node_val.right.is_some();
                let left  = node_val.left.is_some();
                
                match (left, right) {
                    (true, true) => {
                        let sucessor = Self::find_min(&node_val.right).unwrap();
                        
                        node_val.element = sucessor.clone();
                        Self::remove(&mut node_val.right, sucessor);
                    },
                    (false, true) => *node = node_val.right.take(),
                    _ => *node = node_val.left.take(),
                }
                
                Some(removed)
            },
        }
    }
}

#[derive(Debug)]
pub struct BST<T: Ord + Clone> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord + Clone> BST<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, element: T) {
        Node::insert(&mut self.root, element);
    }

    pub fn find_min(&self) -> Option<T> {
        Node::find_min(&self.root)
    }
    
    pub fn find_max(&self) -> Option<T> {
        Node::find_max(&self.root)
    }
    
    pub fn remove(&mut self, value: T) -> Option<T> {
        Node::remove(&mut self.root, value)
    }
}
