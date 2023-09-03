use ptree::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[test]
#[ignore]
fn ipv4_route_random1() {
    let mut top = Ptree::<u32>::new();

    let file = File::open("tests/data/v4routes-random1.txt").unwrap();
    let bufferd = BufReader::new(file);

    for line in bufferd.lines() {
        let line = line.unwrap();
        top.route_ipv4_add(&line, 0);
    }
    println!("count start");
    // assert_eq!(top.iter().count(), 569770);
    let n = top.route_ipv4_lookup_exact("200.23.150.0/24");
    println!("n prefix {}", n.unwrap().prefix);
    let p = top.route_ipv4_lookup("200.23.143.0/24");
    let p = p.unwrap();
    println!("XXX prefix {}", p.prefix);
    if p.parent.borrow().is_some() {
        println!("XXX parent {}", p.parent.borrow().as_ref().unwrap().prefix);
    }
    println!("count end");

    // let file = File::open("tests/data/v4routes-random1.txt").unwrap();
    // let bufferd = BufReader::new(file);

    // for line in bufferd.lines() {
    //     let line = line.unwrap();
    //     top.route_ipv4_delete(&line);
    // }

    // assert_eq!(top.iter().count(), 0);
}

//#[test]
// fn ipv4_route_random1_lookup_exact() {
//     let mut top = Ptree::<u32>::new();

//     let file = File::open("tests/data/v4routes-random1.txt").unwrap();
//     let bufferd = BufReader::new(file);

//     for line in bufferd.lines() {
//         let line = line.unwrap();
//         top.route_ipv4_add(&line, 0);
//     }
//     assert_eq!(top.iter().count(), 569770);

//     let file = File::open("tests/data/v4routes-random1.txt").unwrap();
//     let bufferd = BufReader::new(file);

//     for line in bufferd.lines() {
//         let line = line.unwrap();
//         let result = top.route_ipv4_lookup_exact(&line);
//         assert!(result.is_some());
//     }
// }
