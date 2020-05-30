
struct Node {
    value: i32,
    next: Option<Box<Node>>
}

struct TransactionLog {
    head: Option<Box<Node>>,
    tail: Option<Box<Node>>,
    pub length: u64
}


fn main() {
    let n = Node{
        value: 1,
        next: None
    };

    let n_head = Node{
        value: 2,
        next: Some(Box::new(n))
    };

    let t = TransactionLog{
        head: Some(Box::new(n_head)),
        tail: Some(Box::new(n)),
        length: 2
    };

    let mut head = t.head.expect("No head");

    head.next = Some(Box::new(Node{
        value: 100,
        next: None
    }));
}
