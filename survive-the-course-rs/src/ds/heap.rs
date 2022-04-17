pub struct Heap<T: PartialOrd> {
    heap: Vec<T>,
}

impl<T: PartialOrd> Heap<T> {
    pub fn new() -> Self {
        Heap { heap: Vec::new() }
    }

    pub fn from_list(elems: &[T])
    where
        T: Clone,
    {
        let mut heap = Vec::from(elems);
        Self::heapify(&mut heap);
    }

    fn heapify(heap: &mut Heap<T>) {
        let len = heap.heap.len();
        let count_need_adjust = len.log2();
        for idx in rev(0..1<<count_need_adjust) {
            if idx > len
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, 1)
    }
}
