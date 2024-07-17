pub struct CombinedStruct<T1, T2, T3, const N: usize> {
    pub named_tuple: (T1, T2),
    pub named_array: [T3; N],
}


pub fn test() {
    let combined_struct = CombinedStruct {
        named_tuple: ("Rust", 2024),
        named_array: [0.1, 0.2, 0.3, 0.4, 0.5],
    };

    println!("Tuple: {:?}", combined_struct.named_tuple);
    println!("Array: {:?}", combined_struct.named_array);
}