//!
//! This mod is for practicing decorator pattern in Rust
//!
//! Examples from *Head First Design Pattern*
//!
pub mod bases;
pub mod decorators;

///
/// A test mod
///
#[cfg(test)]
pub mod decorator_pattern_test
{
    use super::*;
    use crate::decorator_pattern::bases::{Espresso, Beverage};
    use crate::decorator_pattern::decorators::{Milk, Mocha};

    #[test]
    fn test()
    {
        let beverage = Box::new(Espresso::new());
        let beverage = Box::new(Milk::new(beverage));
        let beverage = Box::new(Milk::new(beverage)); //double milk
        let beverage = Box::new(Mocha::new(beverage));
        println!("{}\nCost: {}", beverage.get_description(), beverage.get_total_cost_rmb());
        assert_eq!(beverage.get_total_cost_rmb(), 20.0);
    }
}