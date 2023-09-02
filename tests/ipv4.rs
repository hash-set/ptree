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

#[test]
fn ipv4_iter_count() {
    let mut top = Ptree::new();

    top.route_ipv4_add("0.0.0.0/0", 0);
    top.route_ipv4_add("0.0.0.0/1", 1);
    top.route_ipv4_add("128.0.0.0/1", 1);

    top.route_ipv4_add("0.0.0.0/2", 2);
    top.route_ipv4_add("64.0.0.0/2", 2);
    top.route_ipv4_add("128.0.0.0/2", 2);
    top.route_ipv4_add("192.0.0.0/2", 2);

    top.route_ipv4_add("0.0.0.0/3", 3);
    top.route_ipv4_add("32.0.0.0/3", 3);
    top.route_ipv4_add("64.0.0.0/3", 3);
    top.route_ipv4_add("96.0.0.0/3", 3);
    top.route_ipv4_add("128.0.0.0/3", 3);
    top.route_ipv4_add("160.0.0.0/3", 3);
    top.route_ipv4_add("192.0.0.0/3", 3);
    top.route_ipv4_add("224.0.0.0/3", 3);

    top.route_ipv4_add("0.0.0.0/4", 4);
    top.route_ipv4_add("32.0.0.0/4", 4);
    top.route_ipv4_add("64.0.0.0/4", 4);
    top.route_ipv4_add("96.0.0.0/4", 4);
    top.route_ipv4_add("128.0.0.0/4", 4);
    top.route_ipv4_add("160.0.0.0/4", 4);
    top.route_ipv4_add("192.0.0.0/4", 4);
    top.route_ipv4_add("224.0.0.0/4", 4);
    top.route_ipv4_add("16.0.0.0/4", 4);
    top.route_ipv4_add("48.0.0.0/4", 4);
    top.route_ipv4_add("89.0.0.0/4", 4);
    top.route_ipv4_add("112.0.0.0/4", 4);
    top.route_ipv4_add("144.0.0.0/4", 4);
    top.route_ipv4_add("176.0.0.0/4", 4);
    top.route_ipv4_add("208.0.0.0/4", 4);
    top.route_ipv4_add("240.0.0.0/4", 4);

    assert_eq!(top.iter().count(), 31);
}

#[test]
#[ignore]
fn ipv4_iter_count_delete() {
    let mut top = Ptree::new();

    top.route_ipv4_add("0.0.0.0/0", 0);
    top.route_ipv4_add("0.0.0.0/1", 1);
    top.route_ipv4_add("128.0.0.0/1", 1);

    top.route_ipv4_add("0.0.0.0/2", 2);
    top.route_ipv4_add("64.0.0.0/2", 2);
    top.route_ipv4_add("128.0.0.0/2", 2);
    top.route_ipv4_add("192.0.0.0/2", 2);

    top.route_ipv4_add("0.0.0.0/3", 3);
    top.route_ipv4_add("32.0.0.0/3", 3);
    top.route_ipv4_add("64.0.0.0/3", 3);
    top.route_ipv4_add("96.0.0.0/3", 3);
    top.route_ipv4_add("128.0.0.0/3", 3);
    top.route_ipv4_add("160.0.0.0/3", 3);
    top.route_ipv4_add("192.0.0.0/3", 3);
    top.route_ipv4_add("224.0.0.0/3", 3);

    top.route_ipv4_add("0.0.0.0/4", 4);
    top.route_ipv4_add("32.0.0.0/4", 4);
    top.route_ipv4_add("64.0.0.0/4", 4);
    top.route_ipv4_add("96.0.0.0/4", 4);
    top.route_ipv4_add("128.0.0.0/4", 4);
    top.route_ipv4_add("160.0.0.0/4", 4);
    top.route_ipv4_add("192.0.0.0/4", 4);
    top.route_ipv4_add("224.0.0.0/4", 4);
    top.route_ipv4_add("16.0.0.0/4", 4);
    top.route_ipv4_add("48.0.0.0/4", 4);
    top.route_ipv4_add("89.0.0.0/4", 4);
    top.route_ipv4_add("112.0.0.0/4", 4);
    top.route_ipv4_add("144.0.0.0/4", 4);
    top.route_ipv4_add("176.0.0.0/4", 4);
    top.route_ipv4_add("208.0.0.0/4", 4);
    top.route_ipv4_add("240.0.0.0/4", 4);

    top.route_ipv4_delete("0.0.0.0/0");
    top.route_ipv4_delete("0.0.0.0/1");
    top.route_ipv4_delete("128.0.0.0/1");

    top.route_ipv4_delete("0.0.0.0/2");
    top.route_ipv4_delete("64.0.0.0/2");
    top.route_ipv4_delete("128.0.0.0/2");
    top.route_ipv4_delete("192.0.0.0/2");

    top.route_ipv4_delete("0.0.0.0/3");
    top.route_ipv4_delete("32.0.0.0/3");
    top.route_ipv4_delete("64.0.0.0/3");
    top.route_ipv4_delete("96.0.0.0/3");
    top.route_ipv4_delete("128.0.0.0/3");
    top.route_ipv4_delete("160.0.0.0/3");
    top.route_ipv4_delete("192.0.0.0/3");
    top.route_ipv4_delete("224.0.0.0/3");

    top.route_ipv4_delete("0.0.0.0/4");
    top.route_ipv4_delete("32.0.0.0/4");
    top.route_ipv4_delete("64.0.0.0/4");
    top.route_ipv4_delete("96.0.0.0/4");
    top.route_ipv4_delete("128.0.0.0/4");
    top.route_ipv4_delete("160.0.0.0/4");
    top.route_ipv4_delete("192.0.0.0/4");
    top.route_ipv4_delete("224.0.0.0/4");
    top.route_ipv4_delete("16.0.0.0/4");
    top.route_ipv4_delete("48.0.0.0/4");
    top.route_ipv4_delete("89.0.0.0/4");
    top.route_ipv4_delete("112.0.0.0/4");
    top.route_ipv4_delete("144.0.0.0/4");
    top.route_ipv4_delete("176.0.0.0/4");
    top.route_ipv4_delete("208.0.0.0/4");
    top.route_ipv4_delete("240.0.0.0/4");

    for i in top.iter() {
        println!("{}", i.prefix);
    }

    assert_eq!(top.iter().count(), 0);
}

#[test]
fn ipv4_tree_test() {
    let mut top = Ptree::new();

    top.route_ipv4_add("0.0.0.0/0", 0);
    top.route_ipv4_add("0.0.0.0/1", 1);
    top.route_ipv4_add("128.0.0.0/1", 1);

    // top.route_ipv4_add("0.0.0.0/2", 2);
    // top.route_ipv4_add("64.0.0.0/2", 2);
    // top.route_ipv4_add("128.0.0.0/2", 2);
    // top.route_ipv4_add("192.0.0.0/2", 2);

    top.route_ipv4_delete("0.0.0.0/0");
    top.route_ipv4_delete("0.0.0.0/1");
    top.route_ipv4_delete("128.0.0.0/1");

    println!("--");
    for i in top.node_iter() {
        println!("{}", i.prefix);
    }
    println!("--");
}
