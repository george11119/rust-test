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

        match &mut self.head {
            None => {
                self.head = Some(new_node);
            }
            Some(current) => {
                current.next = Some(new_node);

                while let Some(current) = &current.next {
                    println!("from add_node: {:?}", current);
                }
            }
        }
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "empty")
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
        println!("{:?}", list);
        list.add_node(2);
        println!("{:?}", list);
        println!("{}", list);
    }
}
