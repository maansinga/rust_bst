use libbst::{BinarySearchTree, NodeCursor};

fn main() {
    let mut bst: BinarySearchTree = BinarySearchTree::new();
    {
        bst.insert(7);
        bst.insert(5);
        bst.insert(9);
    }
    bst.insert(4);
    bst.insert(6);
    bst.insert(8);
    bst.insert(10);

    bst.prefix_display();

    println!("BST size:{}", bst.length());

    let mut bst_cursor = bst.cursor();
    let mut root_bst_cursor = bst_cursor.clone();

    bst_cursor.find(10);
    println!("{:?}", bst_cursor.data());
    // Bad practice!
    bst_cursor.insert(1);

    root_bst_cursor.find(11);
    println!("{:?}", root_bst_cursor.data());

    bst.inline_display();


    println!("Hello, world!");
}
