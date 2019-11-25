//!
//! This mod contains relaxed version of Template Method Pattern in Rust
//!

///
/// The trait for realizing relaxed Template Method Pattern
///
pub trait AsCaffeineBeverage
{
    fn boil_water(&self)
    {
        println!("Boil Water");
    }
    fn brew(&self);
    fn pour_in_cup(&self)
    {
        println!("Pour in Cup");
    }
    fn add_condiments(&self);
    ///
    /// the method containing the key algorithm
    ///
    /// Here, this can be overridden, so it is not as strict as the final method in the Java example the book gives
    ///
    fn prepare_recipe(&self)
    {
        self.boil_water();
        self.brew();
        self.pour_in_cup();
        self.add_condiments();
    }
}

///
/// One example struct of caffeine beverage
///
pub struct Coffee;
///
/// Another example of caffeine beverage
///
pub struct Tea;
impl AsCaffeineBeverage for Coffee
{
    fn brew(&self)
    {
        println!("brew coffee");
    }
    fn add_condiments(&self)
    {
        println!("add sugar and milk");
    }
}
impl AsCaffeineBeverage for Tea
{
    fn brew(&self)
    {
        println!("brew tea");
    }
    fn add_condiments(&self)
    {
        println!("add lemon");
    }
}