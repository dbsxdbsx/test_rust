use std::collections::{HashMap, HashSet};

fn main() {
    let mut my_map = HashMap::<usize, HashSet<String>>::new();

    for i in 0..10 {
        my_map
            .entry(i)
            .or_insert_with(|| HashSet::from(["a".to_string()]));
    }

    println!("{:?}", my_map);

    for key in 0..10 {
        my_map
            .entry(key)
            .and_modify(|v| {
                v.insert("b".to_string());
            })
            .or_insert_with(|| HashSet::from(["a".to_string()]));
    }

    println!("{:?}", my_map);
}
