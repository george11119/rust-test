#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(val: u64) -> u64 {
    val + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal 4"))
        }
    }

    // #[test]
    // fn another() {
    //     panic!("hahah");
    // }

    #[test]
    fn add_two_works() {
        let result = add_two(5);
        assert_eq!(result, 7);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 4,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 4,
        };

        assert!(!smaller.can_hold(&larger));
    }
}
