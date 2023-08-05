use std::cell::RefCell;
use std::net::Ipv4Addr;
use std::rc::Rc;

use ipnet::Ipv4Net;

const MASK_BITS: [u8; 9] = [0x00, 0x80, 0xc0, 0xe0, 0xf0, 0xf8, 0xfc, 0xfe, 0xff];

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
struct Node {
    #[allow(dead_code)]
    val: u32,
    prefix: Ipv4Net,
    parent: RefCell<Option<Rc<Node>>>,
    children: [RefCell<Option<Rc<Node>>>; 2],
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
    parent.set_child_at(child.clone(), bit);
    child.set_parent(parent.clone());
}

impl Ptree {
    fn insert(&mut self, prefix: &Ipv4Net) {
        let mut cursor = self.top.clone();
        let mut matched: Option<Rc<Node>> = None;
        let mut new_node: Rc<Node>;

        while node_match_prefix(cursor.clone(), prefix) {
            let node = cursor.clone().unwrap();
            if node.prefix.prefix_len() == prefix.prefix_len() {
                println!("Same prefix already exists");
                return;
            }
            matched = Some(node.clone());
            cursor = node.child_with(prefix.bit_at(node.prefix.prefix_len()));
        }

        match cursor {
            Some(node) => {
                new_node = Rc::new(Node::from_common(&node.prefix, prefix));
                set_child(new_node.clone(), node);

                match matched {
                    Some(node) => {
                        set_child(node, new_node.clone());
                    }
                    None => {
                        self.top.replace(new_node.clone());
                    }
                }

                if new_node.prefix.prefix_len() != prefix.prefix_len() {
                    matched = Some(new_node.clone());
                    new_node = Rc::new(Node::new(prefix));
                    set_child(matched.unwrap().clone(), new_node.clone());
                }
            }
            None => {
                new_node = Rc::new(Node::new(prefix));
                match matched {
                    Some(node) => {
                        set_child(node, new_node.clone());
                    }
                    None => {
                        self.top.replace(new_node.clone());
                    }
                }
            }
        }
    }

    fn lookup_exact(&self, prefix: &Ipv4Net) -> NodeIter {
        let mut cursor = self.top.clone();

        while node_match_prefix(cursor.clone(), prefix) {
            let node = cursor.clone().unwrap();

            if node.prefix.prefix_len() == prefix.prefix_len() {
                return NodeIter::from_node(node);
            }
            cursor = node.child_with(prefix.bit_at(node.prefix.prefix_len()));
        }
        NodeIter { node: None }
    }

    fn delete(&mut self, prefix: &Ipv4Net) {
        let iter = self.lookup_exact(prefix);
        if let Some(node) = iter.node {
            let has_left = node.child(NodeChild::Left).is_some();
            let has_right = node.child(NodeChild::Right).is_some();

            if has_left && has_right {
                return;
            }

            // NodeChild
            let child = if has_left {
                node.child(NodeChild::Left)
            } else {
                node.child(NodeChild::Right)
            };

            // Parent
            let parent = node.parent();

            // Replace child's parent.
            if let Some(child) = child.clone() {
                child.parent.replace(parent.clone());
            }

            if let Some(parent) = parent {
                if let Some(left) = parent.child(NodeChild::Left) {
                    if Node::eq(left.as_ref(), node.as_ref()) {
                        parent.children[NodeChild::Left as usize].replace(child.clone());
                    }
                }
                if let Some(right) = parent.child(NodeChild::Right) {
                    if Node::eq(right.as_ref(), node.as_ref()) {
                        parent.children[NodeChild::Right as usize].replace(child.clone());
                    }
                }
            } else {
                self.top = child.clone();
            }
        } else {
            println!("Delete: node not found");
        }
    }

    fn iter(&self) -> NodeIter {
        NodeIter {
            node: self.top.clone(),
        }
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping: {}", self.prefix);
    }
}

enum NodeChild {
    Left = 0,
    Right = 1,
}

impl Node {
    pub fn new(prefix: &Ipv4Net) -> Self {
        Node {
            val: 1,
            prefix: prefix.clone(),
            parent: RefCell::new(None),
            children: [RefCell::new(None), RefCell::new(None)],
        }
    }

    pub fn parent(&self) -> Option<Rc<Node>> {
        self.parent.borrow().clone()
    }

    fn child(&self, bit: NodeChild) -> Option<Rc<Node>> {
        self.children[bit as usize].borrow().clone()
    }

    fn from_common(prefix1: &Ipv4Net, prefix2: &Ipv4Net) -> Self {
        let common = Ipv4Net::from_common(prefix1, prefix2);
        Self::new(&common)
    }

    fn child_with(&self, bit: u8) -> Option<Rc<Node>> {
        self.children[bit as usize].borrow().clone()
    }

    fn set_parent(&self, parent: Rc<Node>) {
        self.parent.replace(Some(parent));
    }

    fn set_child_at(&self, child: Rc<Node>, bit: u8) {
        self.children[bit as usize].borrow_mut().replace(child);
    }

    fn eq(lhs: &Self, rhs: &Self) -> bool {
        lhs as *const _ == rhs as *const _
    }

    fn next(&self) -> Option<Rc<Node>> {
        if let Some(node) = self.child(NodeChild::Left) {
            return Some(node.clone());
        } else if let Some(node) = self.child(NodeChild::Right) {
            return Some(node.clone());
        } else {
            if let Some(parent) = self.parent() {
                if let Some(left) = parent.child(NodeChild::Left) {
                    if Node::eq(left.as_ref(), self) {
                        if let Some(right) = parent.child(NodeChild::Right) {
                            return Some(right.clone());
                        }
                    }
                }
                let mut cursor = parent;
                while let Some(parent) = cursor.parent() {
                    if let Some(left) = parent.child(NodeChild::Left) {
                        if Node::eq(left.as_ref(), cursor.as_ref()) {
                            if let Some(right) = parent.child(NodeChild::Right) {
                                return Some(right.clone());
                            }
                        }
                    }
                    cursor = parent;
                }
            }
        }
        None
    }
}

struct NodeIter {
    node: Option<Rc<Node>>,
}

impl NodeIter {
    fn from_node(node: Rc<Node>) -> Self {
        NodeIter {
            node: Some(node.clone()),
        }
    }
}

impl Iterator for NodeIter {
    type Item = Rc<Node>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node.clone();

        if let Some(node) = node {
            self.node = node.next().clone();
            Some(node)
        } else {
            None
        }
    }
}

fn top() {
    println!("--top--");
    let mut ptree = Ptree { top: None };
    let net0: Ipv4Net = "0.0.0.0/8".parse().unwrap();
    ptree.insert(&net0);

    for i in ptree.iter() {
        println!("Iter: {}", i.prefix);
    }

    ptree.delete(&net0);

    for i in ptree.iter() {
        println!("Iter: {}", i.prefix);
    }
}

fn mask() {
    println!("--mask--");
    let mut ptree = Ptree { top: None };
    let net0: Ipv4Net = "0.0.0.0/32".parse().unwrap();
    ptree.insert(&net0);

    let net128: Ipv4Net = "128.0.0.0/32".parse().unwrap();
    ptree.insert(&net128);

    for i in ptree.iter() {
        println!("Iter: {}", i.prefix);
    }

    ptree.delete(&net128);
    ptree.delete(&net0);

    for i in ptree.iter() {
        println!("Iter: {}", i.prefix);
    }
}

fn sub(ptree: &mut Ptree) {
    println!("--sub--");
    let net0: Ipv4Net = "0.0.0.0/8".parse().unwrap();
    ptree.insert(&net0);

    let net64: Ipv4Net = "64.0.0.0/8".parse().unwrap();
    ptree.insert(&net64);
    ptree.insert(&net64);

    for i in ptree.iter() {
        println!("Iter: {}", i.prefix);
    }

    ptree.delete(&net64);

    for i in ptree.iter() {
        println!("Iter: {}", i.prefix);
    }

    ptree.delete(&net0);

    for i in ptree.iter() {
        println!("Iter: {}", i.prefix);
    }
}

fn main() {
    top();
    mask();

    let mut ptree = Ptree { top: None };
    sub(&mut ptree);
    {
        println!("--drop--");
        let net128: Ipv4Net = "128.0.0.0/8".parse().unwrap();
        let node = Rc::new(Node::new(&net128));
        println!("{:?}", node);
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
