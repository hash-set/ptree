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
    fn bit_at(&self, index: u8) -> u8;
    fn from_common(prefix1: &Self, prefix2: &Self) -> Self;
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

    fn from_common(prefix1: &Self, prefix2: &Self) -> Self {
        let octets1: [u8; 4] = prefix1.addr().octets();
        let octets2: [u8; 4] = prefix2.addr().octets();
        let mut octets: [u8; 4] = [0; 4];

        let mut i: usize = 0;
        while i < prefix1.prefix_len() as usize / 8 {
            if octets1[i] == octets2[i] {
                octets[i] = octets1[i];
            } else {
                break;
            }
            i += 1;
        }

        let mut prefixlen = (i * 8) as u8;

        if prefixlen != prefix1.prefix_len() {
            let diff = octets1[i] ^ octets2[i];
            let mut mask = 0x80u8;
            while prefixlen < prefix1.prefix_len() && (mask & diff) == 0 {
                mask >>= 1;
                prefixlen += 1;
            }
            octets[i] = octets1[i] & MASK_BITS[prefixlen as usize % 8];
        }

        Ipv4Net::new(
            Ipv4Addr::new(octets[0], octets[1], octets[1], octets[3]),
            prefixlen,
        )
        .unwrap()
    }

    fn bit_at(&self, index: u8) -> u8 {
        let offset = index / 8;
        let shift = 7 - (index % 8);

        let octets = self.addr().octets();

        (octets[offset as usize] >> shift) & 0x1
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
    children: [RefCell<Option<Rc<Node>>>; 2],
    // left: RefCell<Option<Rc<Node>>>,
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

fn set_child(parent: Rc<Node>, child: Rc<Node>) {
    let bit = child.prefix.bit_at(parent.prefix.prefix_len());
    println!("bit: {}", bit);
    parent.set_child_at(child.clone(), bit);
    child.set_parent(parent.clone());
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
            Some(node) => {
                println!("Some");
                new_node = Rc::new(Node::from_common(&node.prefix, prefix));
                println!("new_node: {:?}", new_node);
                set_child(new_node.clone(), node);
            }
            None => {
                println!("None");
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
            children: [RefCell::new(None), RefCell::new(None)],
            // left: RefCell::new(None),
            right: RefCell::new(None),
        }
    }

    fn from_common(prefix1: &Ipv4Net, prefix2: &Ipv4Net) -> Self {
        let common = Ipv4Net::from_common(prefix1, prefix2);
        println!("common {}", common);
        Self::new(&common)
    }

    fn set_parent(&self, parent: Rc<Node>) {
        self.parent.replace(Some(parent));
    }

    fn set_child_at(&self, child: Rc<Node>, bit: u8) {
        self.children[bit as usize].borrow_mut().replace(child);
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
            if let Some(node) = node.children[0].borrow().clone() {
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
    let net0: Ipv4Net = "0.0.0.0/8".parse().unwrap();
    ptree.insert(&net0);

    let net128: Ipv4Net = "64.0.0.0/8".parse().unwrap();
    ptree.insert(&net128);
}

fn main() {
    let mut ptree = Ptree { top: None };
    sub(&mut ptree);
    // println!("{:?}", ptree);

    for i in ptree.iter() {
        println!("Iter: {}", i.prefix);
    }
    // if let Some(node) = ptree.top {
    //     println!("count {}", Rc::strong_count(&node));
    // }
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
