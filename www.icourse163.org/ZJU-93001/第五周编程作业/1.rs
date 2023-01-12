use std::io;

struct Heap<T: Ord> {
    vector: Vec<T>
}

impl<T> Heap<T> where T: Ord + Clone + Default {
    fn new(size: usize) -> Heap<T> {
        let v = vec![Default::default(); size];
        Heap {
            vector: v
        }
    }
}

fn split_i32(s: &String) -> Vec<i32> {
    let slices = s.split_whitespace();
    let mut vec = Vec::new();
    for slice in slices {
        vec.push(slice.parse().unwrap());
    }
    vec
}

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    let i32s = split_i32(&input);
    let (n, m) = (i32s[0], i32s[1]);
    input.clear();
    let _ = io::stdin().read_line(&mut input);
    let nums = split_i32(&input);
    
}
