use std::cmp::PartialEq;

fn main() {}

struct Set<'a, T> {
    size: usize,
    elements: Vec<&'a T>,
}

impl<'a, T: PartialEq> Set<'a, T> {
    fn new() -> Self {
        Self {
            size: 0,
            elements: Vec::with_capacity(100),
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn add(&mut self, item: &'a T) {
        if self.contains(item) {
            return;
        }
        
        self.elements.insert(self.size, item);
        self.size += 1;
    }

    fn size(&self) -> usize {
        self.size
    }

    fn contains(&mut self, item: &T) -> bool {
        if let Some(_) = self.index_of(item) {
            return true;
        }

        false
    }

    fn remove(&mut self, item: &T) {
        if let Some(index) = self.index_of(item) {
            self.elements[index] = self.elements[self.size - 1];
            self.elements.remove(self.size - 1);
            self.size -= 1;
        }
    }

    fn index_of(&self, item: &T) -> Option<usize> {
        for i in 0..self.size {
            if self.elements[i].eq(&item) {
                return Some(i);
            }
        }
        
        None
    }
}

mod tests {
    use super::*;

    #[test]
    fn ignores_duplicates() {
        let mut one = Set::new();

        one.add(&"1");
        one.add(&"1");

        assert_eq!(1, one.size());
    }

    #[test]
    fn remove() {
        let mut set = Set::new();
        
        set.add(&"1");
        set.add(&"2");
        set.add(&"3");
        set.add(&"4");

        set.remove(&"2");
        set.remove(&"does not exist");

        assert_eq!(3, set.size());
        assert_eq!(false, set.contains(&"2"));
        assert_eq!(true, set.contains(&"1"));
        assert_eq!(true, set.contains(&"3"));
        assert_eq!(true, set.contains(&"4"));
    }

    #[test]
    fn contains() {
        let mut empty = Set::new();
        let mut one = Set::new();
        let mut many = Set::new();

        one.add(&"1");
        many.add(&"1");
        many.add(&"2");

        assert_eq!(false, empty.contains(&"1"));
        assert_eq!(false, empty.contains(&"2"));
        assert_eq!(true, one.contains(&"1"));
        assert_eq!(false, one.contains(&"2"));
        assert_eq!(true, many.contains(&"1"));
        assert_eq!(true, many.contains(&"2"));
    }

    #[test]
    fn emptiness() {
        let empty: Set<&str> = Set::new();
        let mut one = Set::new();
        let mut many = Set::new();

        one.add(&"1");
        many.add(&"1");
        many.add(&"2");

        assert_eq!(true, empty.is_empty());
        assert_eq!(false, one.is_empty());
        assert_eq!(false, many.is_empty());
    }

    #[test]
    fn size() {
        let empty: Set<&str> = Set::new();
        let mut one = Set::new();
        let mut many = Set::new();

        one.add(&"1");
        many.add(&"1");
        many.add(&"2");

        assert_eq!(0, empty.size());
        assert_eq!(1, one.size());
        assert_eq!(true, many.size() > 1);
    }
}
