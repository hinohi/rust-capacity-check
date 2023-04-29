use std::collections::HashSet;

fn chack_hash() {
    let mut col = HashSet::new();
    let mut cap = col.capacity();
    loop {
        for i in col.len()..=cap {
            col.insert(i);
        }
        cap = col.capacity();
        println!("{}", cap);
    }
}

fn chack_vec() {
    let mut col = Vec::new();
    let mut cap = col.capacity();
    loop {
        for i in col.len()..=cap {
            col.push(i);
        }
        cap = col.capacity();
        println!("{}", cap);
    }
}

fn main() {
    chack_hash();
}
