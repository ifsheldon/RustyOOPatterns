//!
//! This mod is for practicing Factory Pattern in Rust
//!
//!
pub mod simple_factory;


///
/// A simple mod for test run
///
#[cfg(test)]
mod factory_pattern_test
{
    use super::*;
    use crate::factory_pattern::simple_factory::ITStaffKind;

    ///
    /// A test
    ///
    #[test]
    fn test_run()
    {
        let staff = simple_factory::ITStaffFactory::create_IT_staff(ITStaffKind::ITManager);
        println!("{}",staff.working());
    }
}