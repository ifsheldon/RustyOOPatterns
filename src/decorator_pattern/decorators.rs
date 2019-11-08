//!
//! This mod contains decorator-related traits and sample structs
//!
//!
use crate::decorator_pattern::bases::Beverage;

///
/// Only for batter structure, not so useful
///
pub trait Condiment: Beverage
{
    ///
    /// get the condiment description
    ///
    fn get_condiment_description(&self) -> String;
}

///
/// A sample Decorator struct
///
pub struct Milk
{
    beverage: Box<dyn Beverage>,
    description: String,
    cost_rmb: f32
}

impl Milk
{
    pub fn new(beverage: Box<dyn Beverage>) -> Self
    {
        Milk { beverage, description: String::from("Milk"), cost_rmb: 2.0 }
    }
}

impl Beverage for Milk
{
    fn get_description(&self) -> String
    {
        format!("{} {}", self.beverage.get_description(), self.get_condiment_description())
    }
    fn get_total_cost_rmb(&self) -> f32 {
        self.beverage.get_total_cost_rmb() + self.cost_rmb
    }

    fn set_cost_rmb(&mut self, cost_rmb: f32) {
        self.cost_rmb = cost_rmb;
    }
}

impl Condiment for Milk
{
    fn get_condiment_description(&self) -> String
    {
        self.description.clone()
    }
}

///
/// A sample Decorator struct
///
pub struct Mocha
{
    beverage: Box<dyn Beverage>,
    description: String,
    cost_rmb: f32
}

impl Mocha
{
    pub fn new(beverage: Box<dyn Beverage>) -> Self
    {
        Mocha { beverage, description: String::from("Mocha"), cost_rmb: 1.0 }
    }
}

impl Beverage for Mocha
{
    fn get_description(&self) -> String
    {
        format!("{} {}", self.beverage.get_description(), self.get_condiment_description())
    }
    fn get_total_cost_rmb(&self) -> f32 {
        self.beverage.get_total_cost_rmb() + self.cost_rmb
    }

    fn set_cost_rmb(&mut self, cost_rmb: f32) {
        self.cost_rmb = cost_rmb;
    }
}

impl Condiment for Mocha
{
    fn get_condiment_description(&self) -> String
    {
        self.description.clone()
    }
}