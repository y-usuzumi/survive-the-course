automod::dir!(pub "src/playground");

#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

impl<T> List<T> {
    fn from_arr(arr: Vec<T>) -> Self {
        if arr.len() == 0 {
            return List::Nil;
        }
        let mut iter = arr.into_iter();
        let mut l = List::Cons(iter.next().unwrap(), Box::new(List::Nil));
        let mut p = &mut l;
        while let Some(item) = iter.next() {
            if let List::Cons(_, ref mut next) = p {
                let q = List::Cons(item, Box::new(List::Nil));
                *next = Box::new(q);
                p = next;
            }
        }
        return l;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_arr() {
        println!("{:?}", List::from_arr(vec![1, 2, 3, 4, 5]));
    }
}
