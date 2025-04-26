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

    pub fn test(&self) -> i32 {
        5
    }

    pub fn add_node(&mut self, value: i32) {
        let new_node = Box::new(Node { value, next: None });

        let mut next = &mut self.head;
        while let Some(node) = next {
            next = &mut node.next;
        }

        *next = Some(new_node);
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
    fn it_works() {
        let list = List::new();
        let x = list.test();
        assert_eq!(x, 5);
    }

    #[test]
    fn add_node() {
        let mut list = List::new();
        list.add_node(1);
        list.add_node(2);
        println!("{}", list);
    }
}
