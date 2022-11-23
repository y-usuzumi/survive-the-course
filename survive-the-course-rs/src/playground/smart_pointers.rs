use std::rc::Rc;

// This cannot be used as an indirection because it contains the data in itself.
struct Dummy<T> {
    val: T,
}

// Rc<T> acts as an indirection i.e. it holds a pointer to data allocated on the heap.
// Therefore it can work around the infinite size issue on recursive data structures.
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropped.");
    }
}

impl Drop for List {
    fn drop(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drop_trait() {
        let p = CustomSmartPointer {
            data: "Hello".to_string(),
        };
        // Will call Drop::drop on the smart pointer.
    }
}
