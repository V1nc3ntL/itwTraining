
#[derive (Debug)]

enum Op<T>{
    Go,
    Stop,
    Val(T)
}

struct Node<T>{
    l : Option<Box<Node<T>>>,
    r : Option<Box<Node<T>>>,
    op : Op<T>,
}

impl Node<bool>{
   pub fn new(op: Op<bool>, l: Node<bool>,r: Node<bool> )-> Self{
    Node::<bool>{
        op,
        l : Some(Box::new(l)),
        r : Some(Box::new(r)),
    }
   } 
}