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

impl Ptree {
    fn set_top(&mut self, node: Rc<Node>) {
        self.top = Some(node);
    }

    #[allow(dead_code)]
    fn insert(&mut self, node: Node) {
        if self.top.is_none() {
            self.top = Some(Rc::new(node));
        }
    }

    fn iter(&self) -> NodeIter {
        NodeIter {
            node: self.top.clone(),
        }
    }
}

impl Node {
    pub fn new(prefix: Ipv4Net) -> Self {
        Node {
            val: 1,
            prefix,
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
    let net10: Ipv4Net = "10.0.0.0/8".parse().unwrap();
    let node10 = Rc::new(Node::new(net10));
    println!("{:?}", node10);

    let net11: Ipv4Net = "10.128.0.0/16".parse().unwrap();
    let node11 = Rc::new(Node::new(net11));
    println!("{:?}", node11);

    *node10.left.borrow_mut() = Some(node11.clone());

    let net12: Ipv4Net = "10.255.0.0/16".parse().unwrap();
    let node12 = Rc::new(Node::new(net12));
    println!("{:?}", node12);

    *node10.right.borrow_mut() = Some(node12.clone());

    if net10.contains(&net11) {
        println!("XXX Contain");
    } else {
        println!("XXX Dose not Contain");
    }

    ptree.set_top(node10.clone());
    println!("count {}", Rc::strong_count(&node10));
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
