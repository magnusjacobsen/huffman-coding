use std::cmp::Ordering;
use std::collections::{HashMap, BinaryHeap};
use bit_vec::BitVec;
use std::io::prelude::*;
use std::fs::{self, File};

#[derive(Debug)]
struct Node {
    freq: u32,
    ch: Option<char>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

/* needed for min-heap/priority queue */
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

fn new_node(freq: u32, ch: Option<char>) -> Node {
    Node {freq: freq, ch: ch, left: None, right: None}
}

fn new_box(n: Node) -> Box<Node> {
    Box::new(n)
}

fn frequency_map(s: &str) -> HashMap<char, u32> {
    let mut h = HashMap::new();
    for ch in s.chars() {
        let counter = h.entry(ch).or_insert(0);
        *counter += 1;
    }

    // add the end of transmission character
    h.insert('\u{0004}', 0);

    h
}

// returns the root of the tree
fn huffman_tree_from_frequencies(freq: HashMap<char, u32>) -> Box<Node> {
    let nodes: Vec<Box<Node>> = 
        freq.iter()
            .map(|x| new_box(new_node(*(x.1), Some(*(x.0)))))
            .collect();
    let mut heap = BinaryHeap::<Box<Node>>::from(nodes);

    while heap.len() > 1 {
        let a = heap.pop().unwrap();
        let b = heap.pop().unwrap();
        let mut c = new_box(new_node(a.freq + b.freq, None));
        c.left = Some(a);
        c.right = Some(b);
        heap.push(c);
    }
    heap.pop().unwrap()
}

// returns a hashmap of char -> bitvec
fn prefix_map_from_tree(opt: &Box<Node>, mut map: HashMap<char, BitVec>, prefix: BitVec) -> HashMap<char, BitVec> {
    if let Some(ch) = opt.ch {
        map.insert(ch, prefix);
    } else {
        if let Some(ref left) = opt.left {
            let mut new_prefix = prefix.clone();
            new_prefix.push(false); 
            map = prefix_map_from_tree(left, map, new_prefix);
        }
        if let Some(ref right) = opt.right {
            let mut new_prefix = prefix.clone();
            new_prefix.push(true); 
            map = prefix_map_from_tree(right, map, new_prefix);
        }
    }
    map
}

fn encode(data: &str, prefix_map: &HashMap<char, BitVec>) -> BitVec {
    let mut nbits = 0;
    for c in data.chars() {
        nbits += prefix_map.get(&c).unwrap().len();
    }

    let mut res = BitVec::with_capacity(nbits);
    for c in data.chars() {
        let bits = prefix_map.get(&c).unwrap();
        for bit in bits {
            res.push(bit);
        }
    }
    res
}

fn decode(bits: &BitVec, tree: &Box<Node>) -> String {
    let mut res = String::new();
    let mut nodeptr = tree;

    for bit in bits {
        if bit {
            if let Some(ref right) = nodeptr.right {
                nodeptr = right;
            }
        } else {
            if let Some(ref  left) = nodeptr.left {
                nodeptr = left;
            }  
        }
        if let Some(ch) = nodeptr.ch {
            if ch == '\u{0004}' {
                break;
            }
            res.push(ch);
            nodeptr = tree;
        }
    }
    res
}

fn write_encoding_to_file(data: &BitVec, filename: &str) -> std::io::Result<()> {
    let bytes = data.to_bytes();
    println!("converted to bytes");
    let mut buffer = File::create(filename)?;
    println!("created buffer");
    buffer.write_all(&bytes)?;
    println!("buffer written!");

    Ok(())
}

fn read_encoded_file(filename: &str) -> std::io::Result<BitVec> {
    let mut file = File::open(filename).expect("Could not open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Could not read file content");
    
    Ok(BitVec::from_bytes(&buffer))
}

fn read_text_file(filename: &str) -> std::io::Result<String> {
    let mut message = fs::read_to_string(filename).expect("Could not read file");
    message.push('\u{0004}');
    Ok(message)
}

fn main() -> std::io::Result<()> {
    let message = read_text_file("lorem.txt")?;

    let freq = frequency_map(&message);

    let tree = huffman_tree_from_frequencies(freq);
    
    let prefixes = prefix_map_from_tree(&tree, HashMap::new(), BitVec::new());
 
    let encoded = encode(&message, &prefixes);

    write_encoding_to_file(&encoded, "encoded_lorem")?;

    let encoding_from_file = read_encoded_file("encoded_lorem")?;

    let decoded = decode(&encoding_from_file, &tree);
    println!("decoded: {}", &decoded);

    Ok(())
}
