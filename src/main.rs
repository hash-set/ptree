use std::cell::RefCell;
use std::net::Ipv4Addr;
use std::rc::Rc;

use ipnet::Ipv4Net;

const IPV4_MASK: [[u8; 4]; 33] = [
    [0x00, 0x00, 0x00, 0x00],
    [0x80, 0x00, 0x00, 0x00],
    [0xc0, 0x00, 0x00, 0x00],
    [0xe0, 0x00, 0x00, 0x00],
    [0xf0, 0x00, 0x00, 0x00],
    [0xf8, 0x00, 0x00, 0x00],
    [0xfc, 0x00, 0x00, 0x00],
    [0xfe, 0x00, 0x00, 0x00],
    [0xff, 0x00, 0x00, 0x00],
    [0xff, 0x80, 0x00, 0x00],
    [0xff, 0xc0, 0x00, 0x00],
    [0xff, 0xe0, 0x00, 0x00],
    [0xff, 0xf0, 0x00, 0x00],
    [0xff, 0xf8, 0x00, 0x00],
    [0xff, 0xfc, 0x00, 0x00],
    [0xff, 0xfe, 0x00, 0x00],
    [0xff, 0xff, 0x00, 0x00],
    [0xff, 0xff, 0x80, 0x00],
    [0xff, 0xff, 0xc0, 0x00],
    [0xff, 0xff, 0xe0, 0x00],
    [0xff, 0xff, 0xf0, 0x00],
    [0xff, 0xff, 0xf8, 0x00],
    [0xff, 0xff, 0xfc, 0x00],
    [0xff, 0xff, 0xfe, 0x00],
    [0xff, 0xff, 0xff, 0x00],
    [0xff, 0xff, 0xff, 0x80],
    [0xff, 0xff, 0xff, 0xc0],
    [0xff, 0xff, 0xff, 0xe0],
    [0xff, 0xff, 0xff, 0xf0],
    [0xff, 0xff, 0xff, 0xf8],
    [0xff, 0xff, 0xff, 0xfc],
    [0xff, 0xff, 0xff, 0xfe],
    [0xff, 0xff, 0xff, 0xff],
];

const MASK_BITS: [u8; 9] = [0x00, 0x80, 0xc0, 0xe0, 0xf0, 0xf8, 0xfc, 0xfe, 0xff];

trait Prefix {
    fn to_masked(&self) -> Self;
    fn contains(&self, prefix: &Self) -> bool;
}

impl Prefix for Ipv4Net {
    fn to_masked(&self) -> Self {
        let octets: [u8; 4] = self.addr().octets();
        let mask = &IPV4_MASK[self.prefix_len() as usize];
        let addr = Ipv4Addr::new(
            octets[0] & mask[0],
            octets[1] & mask[1],
            octets[2] & mask[2],
            octets[3] & mask[3],
        );
        Ipv4Net::new(addr, self.prefix_len()).unwrap()
    }

    fn contains(&self, prefix: &Self) -> bool {
        if self.prefix_len() > prefix.prefix_len() {
            return false;
        }

        let lp = self.addr().octets();
        let rp = prefix.addr().octets();

        let shift = self.prefix_len() as usize % 8;
        let mut offset = self.prefix_len() as usize / 8;

        if shift > 0 && (MASK_BITS[shift] & (lp[offset] ^ rp[offset])) > 0 {
            return false;
        }

        while offset > 0 {
            offset -= 1;
            if lp[offset] != rp[offset] {
                return false;
            }
        }

        true
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Node {
    val: u32,
    prefix: Ipv4Net,
    parent: RefCell<Option<Rc<Node>>>,
    left: RefCell<Option<Rc<Node>>>,
    right: RefCell<Option<Rc<Node>>>,
}

#[derive(Debug)]
struct Ptree {
    top: Option<Rc<Node>>,
}

fn node_match_prefix(node: Option<Rc<Node>>, prefix: &Ipv4Net) -> bool {
    match node {
        None => false,
        Some(node) => {
            node.prefix.prefix_len() <= prefix.prefix_len() && node.prefix.contains(prefix)
        }
    }
}

impl Ptree {
    fn insert(&mut self, prefix: &Ipv4Net) {
        let cursor = self.top.clone();
        let matched: Option<Rc<Node>> = None;
        let new_node: Rc<Node>;

        while node_match_prefix(cursor.clone(), prefix) {
            //
        }

        match cursor {
            Some(_) => {}
            None => {
                new_node = Rc::new(Node::new(prefix));
                match matched {
                    Some(_) => {}
                    None => {
                        self.top.replace(new_node);
                    }
                }
            }
        }
    }

    #[allow(dead_code)]
    fn lookup_exact(&self, _prefix: Ipv4Net) -> Option<Rc<Node>> {
        None
    }

    fn iter(&self) -> NodeIter {
        NodeIter {
            node: self.top.clone(),
        }
    }
}

impl Node {
    pub fn new(prefix: &Ipv4Net) -> Self {
        Node {
            val: 1,
            prefix: prefix.clone(),
            parent: RefCell::new(None),
            left: RefCell::new(None),
            right: RefCell::new(None),
        }
    }
}

struct NodeIter {
    node: Option<Rc<Node>>,
}

impl Iterator for NodeIter {
    type Item = Rc<Node>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node.clone();
        if let Some(node) = node {
            if let Some(node) = node.left.borrow().clone() {
                self.node = Some(node);
            } else {
                self.node = None;
            }
            Some(node)
        } else {
            None
        }
    }
}

fn sub(ptree: &mut Ptree) {
    let net0: Ipv4Net = "0.0.0.0/32".parse().unwrap();
    let node0 = Rc::new(Node::new(&net0));
    // println!("{:?}", node0);

    let net11: Ipv4Net = "10.128.0.0/16".parse().unwrap();
    let node11 = Rc::new(Node::new(&net11));
    // println!("{:?}", node11);

    *node0.left.borrow_mut() = Some(node11.clone());

    let net12: Ipv4Net = "10.255.0.0/16".parse().unwrap();
    let node12 = Rc::new(Node::new(&net12));
    // println!("{:?}", node12);

    *node0.right.borrow_mut() = Some(node12.clone());

    ptree.insert(&net0);

    let net128: Ipv4Net = "128.0.0.0/32".parse().unwrap();
    ptree.insert(&net128);
}

fn main() {
    let mut ptree = Ptree { top: None };
    sub(&mut ptree);
    println!("{:?}", ptree);

    for i in ptree.iter() {
        println!("Iter: {:?}", i);
    }
    for i in ptree.iter() {
        println!("Iter: {:?}", i);
    }
    if let Some(node) = ptree.top {
        println!("count {}", Rc::strong_count(&node));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_to_masked() {
        let net10: Ipv4Net = "10.1.1.1/8".parse().unwrap();
        assert_eq!(
            net10.to_masked(),
            Ipv4Net::new(Ipv4Addr::new(10, 0, 0, 0), 8).unwrap()
        );

        let net10: Ipv4Net = "10.1.1.1/16".parse().unwrap();
        assert_eq!(
            net10.to_masked(),
            Ipv4Net::new(Ipv4Addr::new(10, 1, 0, 0), 16).unwrap()
        );

        let net10: Ipv4Net = "10.255.255.255/31".parse().unwrap();
        assert_eq!(
            net10.to_masked(),
            Ipv4Net::new(Ipv4Addr::new(10, 255, 255, 254), 31).unwrap()
        );

        let net10: Ipv4Net = "10.255.255.255/0".parse().unwrap();
        assert_eq!(
            net10.to_masked(),
            Ipv4Net::new(Ipv4Addr::new(0, 0, 0, 0), 0).unwrap()
        );

        let net10: Ipv4Net = "10.255.255.255/32".parse().unwrap();
        assert_eq!(
            net10.to_masked(),
            Ipv4Net::new(Ipv4Addr::new(10, 255, 255, 255), 32).unwrap()
        );
    }

    #[test]
    pub fn test_contains() {
        let net10_8: Ipv4Net = "10.0.0.0/8".parse().unwrap();
        let net10_16: Ipv4Net = "10.0.0.0/16".parse().unwrap();
        let net127_8: Ipv4Net = "127.0.0.0/8".parse().unwrap();
        assert_eq!(net10_8.contains(&net10_16), true);
        assert_eq!(net10_8.contains(&net127_8), false);
    }
}
