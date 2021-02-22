use std::collections::HashMap;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

fn frequency(s: &str) -> HashMap<char, u32> {
    let mut h = HashMap::new();
    for ch in s.chars() {
        let counter = h.entry(ch).or_insert(0);
        *counter += 1;
    }
    h
}

#[derive(Debug)]
struct Node {
    freq: u32,
    ch: Option<char>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn new_node(freq: u32, ch: Option<char>) -> Node {
    Node {freq: freq, ch: ch, left: None, right: None}
}

fn new_box(n: Node) -> Box<Node> {
    Box::new(n)
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.freq.partial_cmp(&self.freq)
    }
}

fn assign_codes(p: &Box<Node>, mut h: HashMap<char, String>, s: String) -> HashMap<char, String> {
    if let Some(ch) = p.ch {
        h.insert(ch, s);
    } else {
        if let Some(ref left) = p.left {
            h = assign_codes(left, h, s.clone() + "0");
        }
        if let Some(ref right) = p.right {
            h = assign_codes(right, h, s.clone() + "1");
        }
    }
    h
}

fn encode(s: &str, h: &HashMap<char, String>) -> String {
    let mut r = String::new();
    let mut t: Option<&String>;

    for ch in s.chars() {
        t = h.get(&ch);
        r.push_str(t.unwrap())
    }
    r
}

fn decode(s: &str, root: &Box<Node>) -> String {
    let mut retval = String::new();
    let mut nodeptr = root;

    for x in s.chars() {
        if x == '0' {
            if let Some(ref  left) = nodeptr.left {
                nodeptr = left;
            }
        } else {
            if let Some(ref right) = nodeptr.right {
                nodeptr = right;
            }
        }
        if let Some(ch) = nodeptr.ch {
            retval.push(ch);
            nodeptr = root;
        }
    }
    retval
}

fn main() {
    let message = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".to_string();

    let freq = frequency(&message);

    let p: Vec<Box<Node>> = 
        freq.iter()
            .map(|x| new_box(new_node(*(x.1), Some(*(x.0)))))//new_box(new_node(*(x.1), Some(*(x.0)))))
            .collect();
    let mut h = BinaryHeap::<Box<Node>>::from(p);

    while h.len() > 1 {
        let a = h.pop().unwrap();
        let b = h.pop().unwrap();
        let mut c = new_box(new_node(a.freq + b.freq, None));
        c.left = Some(a);
        c.right = Some(b);
        h.push(c);
    }

    let root = h.pop().unwrap();
    let codes = assign_codes(&root, HashMap::new(), String::new());

    // encode example
    println!("message: {}", &message);

    let mut binary = "".to_string();
    for ch in message.clone().into_bytes() {
        binary += &format!("0{:b}", ch);
    }
    println!("binary: {}", binary);
 
    let encoded = encode(&message, &codes);
    println!("huffman coded: {}", &encoded);
    let decoded = decode(&encoded, &root);
    println!("decoded: {}", &decoded)
}
