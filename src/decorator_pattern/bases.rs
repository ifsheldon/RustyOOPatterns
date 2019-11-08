//!
//! This mod contains base-related traits and a sample base struct
//!
pub trait Beverage
{
    ///
    /// get the total description of a beverage
    ///
    fn get_description(&self) -> String
    {
        String::from("unknown beverage")
    }
    ///
    /// get the total cost
    ///
    fn get_total_cost_rmb(&self) -> f32;
    ///
    /// set one base/condiment's cost
    ///
    /// this is for factory pattern
    ///
    fn set_cost_rmb(&mut self, cost: f32);
}

///
/// A sample base
///
pub struct Espresso
{
    description: String,
    cost: f32
}

impl Espresso
{
    pub fn new() -> Self
    {
        Espresso { description: String::from("Espresso"), cost: 15.0 }
    }
}

impl Beverage for Espresso
{
    fn get_description(&self) -> String {
        self.description.clone()
    }

    fn get_total_cost_rmb(&self) -> f32 {
        self.cost
    }

    fn set_cost_rmb(&mut self, cost: f32) {
        self.cost = cost;
    }
}