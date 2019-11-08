//!
//! This mod is for practicing Adapter Pattern in Rust
//!
//! Examples are from *Head First Design*
//!
//! **Notice** the differences between Object Adapter and Class Adapter
//!
use crate::adapter_pattern::related_traits::{Turkey, FancyTurkey, Duck};
use crate::adapter_pattern::object_adapter::TurkeyObjectAdapter;
use crate::adapter_pattern::pseudo_class_adapter::TurkeyPseudoClassAdapter;

pub mod object_adapter;
pub mod pseudo_class_adapter;
pub mod related_traits;

///
/// A concrete implementation of `Turkey`
///
/// For testing the object adapter
///
pub struct ConcreteTurkey;

impl Turkey for ConcreteTurkey
{
    fn gobble(&self)
    {
        println!("Concrete Gobble");
    }
    fn fly(&self)
    {
        println!("Concrete turkey fly");
    }
}

///
/// A concrete implementation realizing both `Turkey` and `FancyTurkey`
///
/// For illustrating the benefit of the object adapter
///
pub struct FancyConcreteTurkey;

impl Turkey for FancyConcreteTurkey
{
    fn gobble(&self)
    {
        println!("Concrete Fancy Gobble");
    }
    fn fly(&self)
    {
        println!("Concrete Fancy Turkey Fly");
    }
}

impl FancyTurkey for FancyConcreteTurkey {}

///
/// A test function
///
pub fn test_run()
{
    let turkey = ConcreteTurkey;
    let fancy_turkey = FancyConcreteTurkey;
    let duck_box = Box::new(turkey);
    let duck_fancy_box = Box::new(fancy_turkey);
    let duck = TurkeyObjectAdapter::from(duck_box);
    let fancy_duck = TurkeyObjectAdapter::from(duck_fancy_box);
    println!("-------------------");
    quack_and_fly(duck);
    println!("-------------------");
    quack_and_fly(fancy_duck);
    println!("-------------------");
    let pseudo_duck = TurkeyPseudoClassAdapter;
    let pseudo_duck = Box::new(pseudo_duck);
    quack_and_fly(pseudo_duck);
}

///
/// use Duck::fly and Duck::quack
///
/// For the use of dynamic dispatch
///
fn quack_and_fly(duck: Box<dyn Duck>)
{
    duck.quack();
    duck.fly();
}

///
/// A mod for test run
///
#[cfg(test)]
pub mod adapter_pattern_test
{
    use super::*;
    ///
    /// A test run
    ///
    #[test]
    fn run()
    {
        test_run();
    }
}