use ipnet::Ipv4Net;
use ptree::*;

fn lookup_assert<D>(top: &Ptree<D>, addr: &str, route: &str) {
    let n = top.route_ipv4_lookup(addr);
    let p: Ipv4Net = route.parse().unwrap();
    assert_eq!(n.unwrap().prefix, p);
}

fn lookup_assert_none<D>(top: &Ptree<D>, addr: &str) {
    let n = top.route_ipv4_lookup(addr);
    assert!(n.is_none());
}

fn lookup_test<D>(top: &Ptree<D>) {
    lookup_assert(top, "10.0.0.0/32", "10.0.0.0/32");
    lookup_assert(top, "10.0.0.1/32", "10.0.0.0/31");
    lookup_assert(top, "10.0.0.2/32", "10.0.0.0/30");
    lookup_assert(top, "10.0.0.3/32", "10.0.0.0/30");

    lookup_assert(top, "10.0.0.4/32", "10.0.0.0/29");
    lookup_assert(top, "10.0.0.7/32", "10.0.0.0/29");
    lookup_assert(top, "10.0.0.8/32", "10.0.0.0/28");
    lookup_assert(top, "10.0.0.15/32", "10.0.0.0/28");
    lookup_assert(top, "10.0.0.0/28", "10.0.0.0/28");

    lookup_assert_none(top, "10.0.0.16/32");
    lookup_assert_none(top, "10.0.0.255/32");
    lookup_assert_none(top, "0.0.0.0/0");
}

#[test]
fn ipv4_lookup_reverse_test() {
    let mut top = Ptree::new();

    // 10.0.0.0/{28..32}
    top.route_ipv4_add("10.0.0.0/32", 32);
    top.route_ipv4_add("10.0.0.0/31", 31);
    top.route_ipv4_add("10.0.0.0/30", 30);
    top.route_ipv4_add("10.0.0.0/29", 29);
    top.route_ipv4_add("10.0.0.0/28", 28);

    lookup_test(&top);
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

    assert_eq!(top.iter().count(), 0);
}

#[test]
fn ipv4_delete_default() {
    let mut top = Ptree::new();

    top.route_ipv4_add("0.0.0.0/0", 0);
    assert_eq!(top.iter().count(), 1);

    top.route_ipv4_delete("0.0.0.0/0");
    assert_eq!(top.iter().count(), 0);
}

#[test]
fn ipv4_delete_table_default() {
    let mut top = Ptree::new();

    top.route_ipv4_add("0.0.0.0/4", 4);
    assert_eq!(top.iter().count(), 1);

    top.route_ipv4_add("0.0.0.0/5", 5);
    assert_eq!(top.iter().count(), 2);

    top.route_ipv4_delete("0.0.0.0/4");
    assert_eq!(top.iter().count(), 1);
}
