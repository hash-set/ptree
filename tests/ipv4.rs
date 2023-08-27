use ipnet::Ipv4Net;
use ptree::*;
use std::rc::Rc;

fn iter(ptree: &Ptree<u32>) {
    for i in ptree.iter() {
        if i.data.borrow().is_some() {
            println!("Iter: {} [{}]", i.prefix, i.data.borrow().unwrap());
        } else {
            println!("Iter: {} [N/A]", i.prefix);
        }
    }
}

#[test]
#[ignore]
fn test_top() {
    println!("--top--");
    let mut ptree = Ptree::<u32>::new();
    let net0: Ipv4Net = "0.0.0.0/8".parse().unwrap();
    ptree.add(&net0, 1);

    iter(&ptree);

    ptree.delete(&net0);

    iter(&ptree);
}

#[test]
#[ignore]
fn test_mask() {
    println!("--mask--");
    let mut ptree = Ptree::<u32>::new();
    let net0: Ipv4Net = "0.0.0.0/32".parse().unwrap();
    ptree.add(&net0, 1);

    let net128: Ipv4Net = "128.0.0.0/32".parse().unwrap();
    ptree.add(&net128, 128);

    iter(&ptree);

    ptree.delete(&net128);
    ptree.delete(&net0);

    iter(&ptree);
}

#[test]
#[ignore]
fn test_sub() {
    let mut ptree = Ptree::<u32>::new();
    println!("--sub--");
    let net0: Ipv4Net = "0.0.0.0/8".parse().unwrap();
    ptree.add(&net0, 1);

    let net64: Ipv4Net = "64.0.0.0/8".parse().unwrap();
    ptree.add(&net64, 64);
    ptree.add(&net64, 64);

    ptree.delete(&net64);
    ptree.delete(&net0);
}

#[test]
#[ignore]
fn test_data() {
    println!("--data--");
    let mut ptree = Ptree::new();
    let net0: Ipv4Net = "0.0.0.0/8".parse().unwrap();
    ptree.add(&net0, 1);

    let it = ptree.lookup_exact(&net0);
    if let Some(node) = it.node {
        println!("node is found");
        node.set_data(100);
    }
}

#[test]
fn test_drop() {
    let mut ptree = Ptree::new();
    println!("--drop--");
    let net128: Ipv4Net = "128.0.0.0/8".parse().unwrap();
    let node = Rc::new(Node::<u32>::new(&net128));
    ptree.add(&net128, 128);
    println!("{:?}", node);
}
