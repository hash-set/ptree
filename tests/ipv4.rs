use ptree::*;

fn iter(ptree: &Ptree<u32>) {
    for i in ptree.iter() {
        if i.data.borrow().is_some() {
            println!("Iter: {} [{}]", i.prefix, i.data.borrow().unwrap());
        } else {
            println!("Iter: {} [N/A]", i.prefix);
        }
    }
}

fn top() {
    println!("--top--");
    let mut ptree = Ptree::<u32> { top: None };
    let net0: Ipv4Net = "0.0.0.0/8".parse().unwrap();
    ptree.add(&net0, 1);

    iter(&ptree);

    ptree.delete(&net0);

    iter(&ptree);
}

fn mask() {
    println!("--mask--");
    let mut ptree = Ptree::<u32> { top: None };
    let net0: Ipv4Net = "0.0.0.0/32".parse().unwrap();
    ptree.add(&net0, 1);

    let net128: Ipv4Net = "128.0.0.0/32".parse().unwrap();
    ptree.add(&net128, 128);

    iter(&ptree);

    ptree.delete(&net128);
    ptree.delete(&net0);

    iter(&ptree);
}

fn sub(ptree: &mut Ptree<u32>) {
    println!("--sub--");
    let net0: Ipv4Net = "0.0.0.0/8".parse().unwrap();
    ptree.add(&net0, 1);

    let net64: Ipv4Net = "64.0.0.0/8".parse().unwrap();
    ptree.add(&net64, 64);
    ptree.add(&net64, 64);

    iter(ptree);

    ptree.delete(&net64);

    iter(ptree);

    ptree.delete(&net0);

    iter(ptree);
}

fn data() {
    println!("--data--");
    let mut ptree = Ptree { top: None };
    let net0: Ipv4Net = "0.0.0.0/8".parse().unwrap();
    ptree.add(&net0, 1);

    let it = ptree.lookup_exact(&net0);
    if let Some(node) = it.node {
        println!("node is found");
        node.set_data(100);
    }
}

fn main() {
    top();
    mask();
    data();

    let mut ptree = Ptree { top: None };
    sub(&mut ptree);
    {
        println!("--drop--");
        let net128: Ipv4Net = "128.0.0.0/8".parse().unwrap();
        let node = Rc::new(Node::<u32>::new(&net128));
        println!("{:?}", node);
    }
}
