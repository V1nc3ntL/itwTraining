#[derive (Debug)]

mod node; 

struct Tree {
    l : Node,
    r : Node,
    }

pub fn build_tree(l : Node,r : Node) -> Tree{
    Tree{
        l,
        r
    }
}

pub fn print_tree(){

}