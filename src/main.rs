use binary_search_tree::BST;

fn main() {
    let mut tree: BST<i32> = BST::new();
    
    tree.insert(10);
    tree.insert(15);
    tree.insert(16);
    tree.insert(5);
    tree.insert(3);
    
    println!("Min {}", tree.find_min().unwrap());
    println!("Max {}", tree.find_max().unwrap());
    
    tree.remove(5);
    
    println!("Contain: {}", tree.contains(10));
    println!("Contains: {}", tree.contains(5));
    println!("Contains: {}", tree.contains(10_000));
   
    dbg!(tree);    
}