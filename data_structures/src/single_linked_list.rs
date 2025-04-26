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
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.head.is_none() {
            write!(f, "empty")
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
}
