use std::fmt;

#[derive(Debug)]
pub struct List {
    head: Link,
}

type Link = Option<Box<Node>>;

#[derive(Debug)]
pub struct Node {
    value: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn add_first(&mut self, value: i32) {
        let mut new_node = Box::new(Node { value, next: None });
        let tmp = self.head.take();
        new_node.next = tmp;
        self.head = Some(new_node);
    }

    pub fn add_last(&mut self, value: i32) {
        let new_node = Box::new(Node { value, next: None });

        let mut next = &mut self.head;
        while let Some(node) = next {
            next = &mut node.next;
        }

        *next = Some(new_node);
    }

    pub fn remove_first(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    pub fn remove_last(&mut self) -> Option<i32> {
        if self.head.is_none() {
            return None;
        }

        let mut current = &mut self.head;
        while current.as_ref().unwrap().next.is_some() {
            current = &mut current.as_mut().unwrap().next;
        }
        Some(current.take().unwrap().value)
    }

    pub fn peek_first(&self) -> Option<i32> {
        // match &self.head {
        //     Some(node) => Some(node.value),
        //     None => None,
        // }
        self.head.as_ref().map(|node| node.value)
    }

    pub fn peek_last(&self) -> Option<i32> {
        if self.head.is_none() {
            return None;
        }

        let mut current = &self.head;
        while current.as_ref().unwrap().next.is_some() {
            current = &current.as_ref().unwrap().next;
        }
        Some(current.as_ref().unwrap().value)
    }

    pub fn into_iter(self) -> IntoIter {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

pub struct IntoIter(List);

impl Iterator for IntoIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.remove_first()
    }
}

pub struct Iter<'a> {
    next: Option<&'a Node>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.value
        })
    }
}

pub struct IterMut<'a> {
    next: Option<&'a mut Node>,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.value
        })
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.head.is_none() {
            writeln!(f, "empty")
        } else {
            let mut print_result = String::from("");
            let mut next = &self.head;
            while let Some(node) = next {
                print_result.push_str(&format!("{} -> ", node.value));
                next = &node.next;
            }
            print_result.push_str("None");
            writeln!(f, "{}", print_result)
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basic() {
        let mut list = List::new();
        list.add_first(3);
        list.add_first(2);
        list.add_first(1);
        assert_eq!(list.remove_first(), Some(1));
        assert_eq!(list.remove_first(), Some(2));
        assert_eq!(list.remove_first(), Some(3));
        assert_eq!(list.remove_first(), None);

        list.add_last(1);
        list.add_last(2);
        list.add_last(3);
        assert_eq!(list.remove_last(), Some(3));
        assert_eq!(list.remove_last(), Some(2));
        assert_eq!(list.remove_last(), Some(1));
        assert_eq!(list.remove_last(), None);
    }

    #[test]
    fn peek_first() {
        let mut list = List::new();
        assert_eq!(list.peek_first(), None);

        list.add_last(1);
        list.add_last(2);
        list.add_last(3);

        assert_eq!(list.peek_first(), Some(1));
        list.remove_first();
        assert_eq!(list.peek_first(), Some(2));
        list.remove_first();
        assert_eq!(list.peek_first(), Some(3));
        list.remove_first();
        assert_eq!(list.peek_first(), None);
    }

    #[test]
    fn peek_last() {
        let mut list = List::new();

        list.add_last(1);
        list.add_last(2);
        list.add_last(3);

        assert_eq!(list.peek_last(), Some(3));
        list.remove_last();
        assert_eq!(list.peek_last(), Some(2));
        list.remove_last();
        assert_eq!(list.peek_last(), Some(1));
        list.remove_last();
        assert_eq!(list.peek_last(), None);
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.add_last(1);
        list.add_last(2);
        list.add_last(3);

        let mut list_iter = list.into_iter();
        assert_eq!(list_iter.next(), Some(1));
        assert_eq!(list_iter.next(), Some(2));
        assert_eq!(list_iter.next(), Some(3));
        assert_eq!(list_iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.add_last(1);
        list.add_last(2);
        list.add_last(3);

        let mut list_iter = list.iter();
        assert_eq!(list_iter.next(), Some(&1));
        assert_eq!(list_iter.next(), Some(&2));
        assert_eq!(list_iter.next(), Some(&3));
        assert_eq!(list_iter.next(), None);

        // list ownership should not have changed
        assert_eq!(list.remove_last(), Some(3));
        assert_eq!(list.remove_last(), Some(2));
        assert_eq!(list.remove_last(), Some(1));
        assert_eq!(list.remove_last(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.add_last(1);
        list.add_last(2);
        list.add_last(3);

        let mut list_iter = list.iter_mut();
        assert_eq!(list_iter.next(), Some(&mut 1));
        assert_eq!(list_iter.next(), Some(&mut 2));
        assert_eq!(list_iter.next(), Some(&mut 3));
        assert_eq!(list_iter.next(), None);

        for val in list.iter_mut() {
            *val *= 2;
        }

        // list ownership should not have changed
        assert_eq!(list.remove_last(), Some(6));
        assert_eq!(list.remove_last(), Some(4));
        assert_eq!(list.remove_last(), Some(2));
        assert_eq!(list.remove_last(), None);
    }

    #[test]
    fn display() {
        let mut list = List::new();
        assert_eq!("empty\n", list.to_string());
        list.add_last(1);
        list.add_last(2);
        list.add_last(3);
        assert_eq!("1 -> 2 -> 3 -> None\n", list.to_string());
    }
}
