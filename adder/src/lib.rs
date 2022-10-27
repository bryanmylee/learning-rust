pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            height: 8,
            width: 7,
        };
        let smaller = Rectangle {
            height: 5,
            width: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            height: 8,
            width: 7,
        };
        let smaller = Rectangle {
            height: 5,
            width: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
}
