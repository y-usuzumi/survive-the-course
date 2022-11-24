// Check list when implementing linked list:
// 1. Uniform allocation
// 2. No junk allocation
// 3. Make use of null-pointer optimization

pub mod bad_list {
    // This implementation has a layout that looks like this:
    // [] = Stack
    // () = Heap
    //
    // [Elem A, ptr] -> (Elem B, ptr) -> (Empty, *junk*)
    //
    // This layout has several problems:
    // 1. Allocation is not uniform (i.e. first element is allocated on stack)
    // 2. Last element allocates junk -- waste of memory
    //
    // This layout does take advantage of null-pointer optimization

    #[derive(Debug)]
    pub enum List<T> {
        Cons(T, Box<List<T>>),
        Nil,
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_1() {
            let l = List::Cons(
                1,
                Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
            );
            println!("{:?}", l);
        }
    }
}

pub mod worse_list {
    // Check list:
    // 1. ✗ Uniform allocation
    // 2. ✓ No junk allocation
    // 3. ✗ Make use of null-pointer optimization
    #[derive(Debug)]
    pub enum List<T> {
        Empty,
        ConsEmpty(T),
        ConsNonEmpty(T, Box<List<T>>),
    }
}

pub mod better_list {
    // Check list:
    // 1. ✓ Uniform allocation
    // 2. ✓ No junk allocation
    // 3. ✓ Make use of null-pointer optimization

    // List may be allocated on the stack, so we need an indirection if the value
    // is not empty.
    // #[derive(Debug)]
    // pub enum List<T> {
    //     Empty,
    //     // The line below will cause a compile error "can't leak private type".
    //     // Check the other implementation below on how to fix it.
    //     More(Box<Node<T>>),
    // }

    // Node is always allocated on the stack. Therefore we don't need an extra
    // indirection.
    // #[derive(Debug)]
    // struct Node<T> {
    //     val: T,
    //     next: List<T>,
    // }
}

pub mod better_list2 {
    use std::mem;

    #[derive(Debug)]
    pub struct List<T> {
        head: Link<T>,
    }

    #[derive(Debug)]
    enum Link<T> {
        Empty,
        More(Box<Node<T>>),
    }

    #[derive(Debug)]
    struct Node<T> {
        val: T,
        next: Link<T>,
    }

    impl<T> List<T> {
        pub fn new() -> Self {
            List { head: Link::Empty }
        }

        pub fn pushleft(&mut self, val: T) {
            let new_head = Link::More(Box::new(Node {
                val,
                // mem::replace replaces the value referenced by mut-ref `dest` with
                // `src`, and returns the owned original value
                next: mem::replace(&mut self.head, Link::Empty),
            }));
            self.head = new_head;
        }

        pub fn popleft(&mut self) -> Option<T> {
            match mem::replace(&mut self.head, Link::Empty) {
                Link::Empty => None,
                Link::More(next) => {
                    self.head = next.next;
                    Some(next.val)
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_pop_empty() {
            assert_eq!(List::<i32>::new().popleft(), None);
        }

        #[test]
        fn test_push_pop() {
            let mut list = List::new();
            let val = 3;
            list.pushleft(val);
            assert_eq!(list.popleft(), Some(val));
            assert_eq!(list.popleft(), None);
        }
    }
}
