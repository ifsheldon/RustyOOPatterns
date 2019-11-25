//!
//! This mod contains demo codes of Template Method Pattern
//!
//! Examples from *Head First Design Pattern*
//!
//! included two versions(relaxed/strict) of Rust implementation
//!
//! # Notice
//! Hook methods mentioned in the book are not implemented as they have no difference from methods in Rust from Rust's perspective,
//! which is not true for the Java examples given in the book.
//!
pub mod relaxed_version;
pub mod strict_version;

#[cfg(test)]
pub mod template_method_pattern_test
{
    use super::*;
    use crate::template_method_pattern::strict_version::AsCaffeineBeverage;


    #[test]
    fn test_relaxed_version()
    {
        use crate::template_method_pattern::relaxed_version::AsCaffeineBeverage;
        let tea = relaxed_version::Tea{};
        let coffee = relaxed_version::Coffee{};
        tea.prepare_recipe();
        coffee.prepare_recipe();
    }

    #[test]
    fn test_strict_version()
    {
        use crate::template_method_pattern::strict_version::AsCaffeineBeverage;
        let tea = strict_version::Tea::new();
        let coffee = strict_version::Coffee::new();
        tea.prepare_recipe();
        coffee.prepare_recipe();
    }
}