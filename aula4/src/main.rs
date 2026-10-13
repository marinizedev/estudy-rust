// aula4/src/main.rs

struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Box<Node> {
        Box::new(Node {
            value,
            next: None
        })
    }

    fn append(node: Box<Node>, value: i32) -> Box<Node> {
        Box::new(Node {
            value,
            next: Some(node),
        })
    }
}

fn main() {
    // Cria o primeiro nó.
    let first = Node::new(10);
    // Adiciona mais um nó à lista.
    let second = Node::append(first, 20);

    println!("Primeiro nó: {}", second.next.as_ref().unwrap().value);
    println!("Segundo nó: {}", second.value);
}
