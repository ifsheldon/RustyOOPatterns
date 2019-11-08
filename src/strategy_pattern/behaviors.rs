//!
//! This mod contains structs of behaviors
//!
//!
///
/// The struct of behavior
///
/// Including behaviors/methods and needed information/fields
///
pub struct AppleEater {
    apple: u32
}

impl AppleEater {
    pub fn new() -> Self
    {
        AppleEater { apple: 0 }
    }
    pub fn report(&self) {
        println!("I ate {} apple!", self.apple);
    }

    pub fn eat_apple(&mut self) { self.apple += 1; }
}

///
/// trait for achieving pseudo-inheritance in Rust
///
/// for methods not changing fields
pub trait AsAppleEater {
    fn as_apple_eater(&self) -> &AppleEater;
    fn report(&self)
    { self.as_apple_eater().report() }
}

///
/// trait for achieving pseudo-inheritance in Rust
///
/// for methods changing fields
pub trait AsMutAppleEater: AsAppleEater {
    fn as_mut_apple_eater(&mut self) -> &mut AppleEater;

    fn eat_apple(&mut self)
    {
        self.as_mut_apple_eater().eat_apple();
        self.report();
    }
}