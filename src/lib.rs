use std::cell::RefCell;
use std::rc::{Rc, Weak};
/**
* This is a default BST implementation
* This implementation by default behaves like a BST but unless the cursor is used
* to insert into the tree. Then it becomes a Binary Tree
*/

type StrongLink = Rc<RefCell<Node>>;
type WeakLink = Weak<RefCell<Node>>;

/// A node is the fundamental unit of BST.
/// It has references to
///  - itself
///  - parent
///  - left child
///  - right child
/// This implementation of Node contains only i32 as data.
#[derive(Debug)]
pub struct Node{
    node: Option<WeakLink>,
    parent: Option<WeakLink>,

    right: Option<StrongLink>,
    pub data: i32,
    left: Option<StrongLink>,
}

/// Node implementation
impl Node{
    /// Create a StrongLink an Rc<RefCell<Node>> out of data d
    fn new(d: i32)-> StrongLink {
        let link = Rc::new(
            RefCell::new(
                Node{node: None, parent:None, left: None, data: d, right: None}
            )
        );
        link.borrow_mut().node = Some( Rc::downgrade(&link));
        link
    }

    /// Inserts data d under the current node
    fn insert(&mut self, d: i32){
        let data = self.data.clone();
        if d > data {
            let right = self.right.take();
            match right{
                None => {
                    let new_node = Node::new(d);
                    new_node.borrow_mut().parent = self.node
                        .as_ref()
                        .map(|c| c.clone());
                    self.right = Some(new_node);

                },
                Some(right_node) => {
                    right_node
                        .borrow_mut()
                        .insert(d);

                    self.right = Some(right_node);
                }
            }
        }else{
            let left = self.left.take();
            match left{
                None => self.left = Some(Node::new(d)),
                Some(left_node) => {
                    left_node
                        .borrow_mut()
                        .insert(d);

                    self.left = Some(left_node);
                }
            }
        }
    }

    /// Finds a node with data d and returns a result
    fn find(&self, d: i32) -> Result<WeakLink, String>{
        let error_string = format!("{}: Not found", d);
        if self.data == d{
            match self.node.as_ref(){
                Some(n) =>{
                    Ok(n.clone())
                },
                _ => Err(error_string)
            }
        }else if d > self.data {
            match self.right.as_ref(){
                Some(node) =>{
                    node
                        .as_ref()
                        .borrow()
                        .find(d)
                },
                None => Err(error_string)
            }
        }else{
            match self.left.as_ref(){
                Some(node) =>{
                    node
                        .as_ref()
                        .borrow()
                        .find(d)
                },
                None => Err(error_string)
            }
        }
    }

    /// Performs inline printout
    fn inline_display(&self){
        self.left.as_ref().map(|n| n.borrow().inline_display());
        print!(" {} ", self.data);
        self.right.as_ref().map(|n| n.borrow().inline_display());
    }

    /// Performs prefix printout
    fn prefix_display(&self){
        match self.left.as_ref().or(self.right.as_ref()){
            None => print!("{} ", self.data),
            Some(_) =>{
                print!("{}[", self.data);

                match self.left.as_ref(){
                    Some(l) => l.borrow().prefix_display(),
                    None => print!("*")
                }
                match self.right.as_ref(){
                    Some(r) => r.borrow().prefix_display(),
                    None => print!("*")
                }

                print!("] ");
            }
        }
    }
}

#[cfg(Debug)]
impl Drop for Node{
    fn drop(&mut self) {
        self.node = None;
        println!("Node containing: {} is going out of memory!", self.data);
    }
}

/// This is like an iterator over the binary tree
#[derive(Clone)]
pub struct BSTNodeCursor{
    node: Option<WeakLink>
}

impl BSTNodeCursor{
    /// Get the data under the cursor
    pub fn data(&mut self)->Option<i32>{
        match self.node.as_ref(){
            Some(n) => n
                .upgrade()
                .map(|x| {
                    x
                        .as_ref()
                        .borrow()
                        .data
                }),
            None => None
        }
    }

    /// Get the data under the cursor
    pub fn insert(&mut self, d: i32){
        match self.node.as_ref(){
            Some(weak_ref) => {
                match weak_ref.upgrade(){
                    Some(ref strong_ref) => {
                        strong_ref
                            .borrow_mut()
                            .insert(d);
                    },
                    None =>{
                        panic!("WeakRef to StrongRef resolution failure");
                    }
                }
            },
            None => {
                panic!("Empty Cursor!");
            }
        }
    }
}

enum _LRP {
    Parent,
    Left,
    Right
}

/// LRNodeCursor performs a generic left/right traversal.
trait LRNodeCursor{
    fn _left_and_right(&mut self, lr: _LRP);
}

/// NodeCursor implements traversals.
pub trait NodeCursor{
    fn parent(&mut self);

    fn left(&mut self);
    fn right(&mut self);

    fn find(&mut self, d: i32);
}

impl LRNodeCursor for BSTNodeCursor{
    fn _left_and_right(&mut self, lr: _LRP) {
        match self.node.take(){
            Some(weak_ref) => {
                match weak_ref.upgrade() {
                    None => (),
                    Some(strong) => {
                        match lr{
                            _LRP::Left => {
                                self.node = strong
                                    .borrow()
                                    .left
                                    .as_ref()
                                    .map(|n| Rc::downgrade(n))
                            },
                            _LRP::Right =>{
                                self.node = strong
                                    .borrow()
                                    .right
                                    .as_ref()
                                    .map(|n| Rc::downgrade(n))
                            },
                            _LRP::Parent =>{
                                self.node = strong
                                    .borrow()
                                    .parent
                                    .as_ref()
                                    .map(|n| n.clone())
                            }
                        }
                    }
                }
            },
            _ => self.node = None
        }
    }
}
impl NodeCursor for BSTNodeCursor{
    /// Traverse to parent
    fn parent(&mut self){
        self._left_and_right(_LRP::Parent);
    }

    /// Traverse left
    fn left(&mut self){
        self._left_and_right(_LRP::Left);
    }

    /// Traverse right
    fn right(&mut self){
        self._left_and_right(_LRP::Right);
    }

    /// Traverse to the target from the current position - downwards only
    fn find(&mut self, d: i32) {
        match self.data(){
            Some(i) => {
                if d != i{
                    match self.node.as_ref(){
                        Some(weak_node) => {
                            match weak_node.upgrade(){
                                Some(strong_node) =>{
                                    let result = strong_node
                                        .as_ref()
                                        .borrow()
                                        .find(d);
                                    match result{
                                        Ok(r) => {
                                            self.node = Some(r);
                                        },
                                        Err(_)=>{
                                            self.node = None;
                                        }
                                    }
                                },
                                _ => panic!("Weak to strong Node upgrade failed!")
                            }
                        },
                        _ => panic!("This is an impossible case!")
                    }
                }
            },
            _ => ()
        }
    }
}

/// Container for BinarySearchTree
pub struct BinarySearchTree{
    root: Option<StrongLink>,
    size: u64
}

impl BinarySearchTree{
    pub fn new()->Self{
        BinarySearchTree{root: None, size: 0}
    }

    /// Insertion into BST
    pub fn insert(&mut self, d: i32){
        let root_node = self.root.clone();
        match root_node {
            None => self.root = Some(Node::new(d)),
            Some(ref node) => node.borrow_mut().insert(d)
        }

        self.size += 1;
    }

    /// Perform inline display call over the containing tree
    pub fn inline_display(&self){
        let root_node = self.root.clone();
        match root_node {
            None => println!("<Empty>"),
            Some(ref node) => {
                node.borrow().inline_display();
                println!();
            }
        }
    }

    /// Perform prefix display call over the containing tree
    pub fn prefix_display(&self){
        let root_node = self.root.clone();
        match root_node {
            None => println!("<Empty>"),
            Some(ref node) => {
                node.borrow().prefix_display();
                println!();
            }
        }
    }

    /// Get length of the BST
    pub fn length(&self) -> u64{
        self.size
    }

    /// Get cursor for the root node
    pub fn cursor(&self)->BSTNodeCursor{
        BSTNodeCursor{node: self.root.as_ref().map(|n| Rc::downgrade(n))}
    }
}
