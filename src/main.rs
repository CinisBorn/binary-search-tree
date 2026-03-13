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
    println!("Remove {:?}", tree.remove(1310));
   
    dbg!(tree);    
}