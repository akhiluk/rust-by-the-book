pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    return internal_adder(a, 2);
}

fn internal_adder(a: i32, b: i32) -> i32 {
    return a+b;
}

#[derive(Debug, PartialEq)]
pub struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }

    pub fn area(&self) -> u32 {
        return self.height * self.width;
    }
}

pub struct Guess {
    value: i32
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        return Guess {value};
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
    
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle{
            width: 8,
            height: 7
        };

        let smaller = Rectangle {
            width: 6,
            height: 6
        };

        assert!(larger.can_hold(&smaller));
    }
    
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7
        };

        let smaller = Rectangle {
            width: 6,
            height: 6
        };

        assert!(!smaller.can_hold(&larger));
    }
    
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn it_changes_the_number() {
        assert_ne!(2, add_two(2), "The number provided did not change.");
    }

    #[test]
    fn check_area() {
        let small = Rectangle {
            height: 8,
            width: 5
        };

        assert_eq!(40, small.area());
    }

    #[test]
    #[should_panic]
    fn greater_than_hundred() {
        Guess::new(200);
    }
}
