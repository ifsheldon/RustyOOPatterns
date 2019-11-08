//!
//! This mod contains structs of actors and a little demonstration of pseudo-inheritance in Rust
//!
use crate::strategy_pattern::behaviors::{AppleEater, AsAppleEater, AsMutAppleEater};

///
/// the actor Student
///
pub struct Student
{
    apple_eater: AppleEater
}

impl Student
{
    pub fn new() -> Self
    {
        Student { apple_eater: AppleEater::new() }
    }
}

impl AsAppleEater for Student {
    fn as_apple_eater(&self) -> &AppleEater
    { &self.apple_eater }
}

impl AsMutAppleEater for Student {
    fn as_mut_apple_eater(&mut self) -> &mut AppleEater
    { &mut self.apple_eater }
}

///
/// the actor Teacher
///
/// who has common behavior via pseudo-inheritance
///
pub struct Teacher
{
    apple_eater: AppleEater
}

impl AsAppleEater for Teacher {
    fn as_apple_eater(&self) -> &AppleEater
    { &self.apple_eater }
}

impl AsMutAppleEater for Teacher {
    fn as_mut_apple_eater(&mut self) -> &mut AppleEater
    { &mut self.apple_eater }
}

impl Teacher
{
    pub fn new() -> Self
    {
        Teacher { apple_eater: AppleEater::new() }
    }
}