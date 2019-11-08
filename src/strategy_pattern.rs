//!
//! This mod is for demonstrating the Strategy Pattern in Rust
//!
//! More discussions and details in https://blog.csdn.net/qq_41905133/article/details/99675944
//!
//!
use crate::strategy_pattern::actors::Student;
use crate::strategy_pattern::behaviors::{AsMutAppleEater, AsAppleEater};

pub mod behaviors;
pub mod actors;

///
/// A test function
///
pub fn test_run()
{
    let mut student = Student::new();
    student.eat_apple();
    student.eat_apple();
    student.report();
}

///
/// A test mod
///
#[cfg(test)]
pub mod strategy_pattern_tests
{
    use super::*;
    #[test]
    fn test()
    {
        test_run();
    }
}