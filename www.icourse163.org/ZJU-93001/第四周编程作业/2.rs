type BalanceFactor = i32;

enum AVL<T: Ord> {
    Nil,
    AVL {
        data: T,
        bf: BalanceFactor,
        left: Box<AVL<T>>,
        right: Box<AVL<T>>
    }
}

fn insert<T: Ord>(tree: AVL<T>, insert_data: T) -> AVL<T> {
    match tree {
        AVL::Nil => AVL::AVL { data: insert_data,
                               bf: 0,
                               left: Box::new(AVL::Nil),
                               right: Box::new(AVL::Nil) },
        AVL::AVL { data, bf, left, right } => if insert_data < data {
            AVL::AVL { left: Box::new(insert(*left, insert_data)),
                       .. tree }
        }
    }
}

fn main() {
    if let k @ AVL::AVL { .. } = (AVL::AVL { data: 123, bf: 0, left: Box::new(AVL::Nil), right: Box::new(AVL::Nil) }) {
        println!("GG");
    }
    
}
