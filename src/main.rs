#[derive(Debug)]
struct Node<'a> {
    val: i32,
    next: Option<&'a Box<Node<'a>>>,
}

impl Node<'_>  {
    fn new(val: i32) -> Node<'static>  {
        Node { val, next: None }
    }

    fn add_front(nodes: &mut Vec<Box<Node>>, val: i32) {
        let mut new_node = Box::new(Node::new(val));
        new_node.next = nodes.get(0);

    //treba ubaciti sad novi node(new_node) u nodes Vec
    }
}

fn main() {
    let mut nodes: Vec<Box<Node>> = Vec::new();
    nodes.insert(0, Box::new(Node::new(1)));
   

    for i in 2..4  {
        Node::add_front(&mut nodes,i);
    }

}
