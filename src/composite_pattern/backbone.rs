use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use crate::composite_pattern::peripheries::{OpNotSupportedError, DynMenuIter, CompositeIterator};


pub trait MenuComponent
{
    fn get_name(&self) -> &String;
    fn get_description(&self) -> &String;
    fn get_price(&self) -> Option<f32>;
    fn is_vegetarian(&self) -> Option<bool>;
    fn print(&self);
    fn get_component_num(&self) -> usize;
    fn add_component(&mut self, component: Box<dyn MenuComponent>) -> Result<(), OpNotSupportedError>;
    fn remove_component(&mut self, component: Box<dyn MenuComponent>) -> Result<bool, OpNotSupportedError>;
    fn get_iter(&mut self) -> Option<DynMenuIter>;
}

impl Hash for dyn MenuComponent
{
    fn hash<H: Hasher>(&self, state: &mut H)
    {
        self.get_name().hash(state);
        self.get_description().hash(state);
        match self.get_price() {
            Some(price) => format!("{:.*}", 2, price).hash(state),
            None => {}
        }
        match self.is_vegetarian() {
            Some(isveg) => isveg.hash(state),
            None => {}
        }
        self.get_component_num().hash(state)
    }
}

impl PartialEq for dyn MenuComponent
{
    fn eq(&self, other: &Self) -> bool
    {
        self.get_name() == other.get_name() &&
            self.get_description() == other.get_description() &&
            self.get_price() == other.get_price() &&
            self.is_vegetarian() == other.is_vegetarian() &&
            self.get_component_num() == other.get_component_num()
    }
}

impl Eq for dyn MenuComponent
{}

pub struct Menu
{
    name: String,
    description: String,
    menu_components: HashSet<Box<dyn MenuComponent>>
}

impl Menu
{
    pub fn new(menu_name: String, menu_description: String) -> Self
    {
        Menu {
            name: menu_name,
            description: menu_description,
            menu_components: HashSet::new()
        }
    }
}

impl MenuComponent for Menu
{
    fn get_name(&self) -> &String
    {
        &self.name
    }
    fn get_description(&self) -> &String {
        &self.description
    }
    fn get_price(&self) -> Option<f32> {
        None
    }

    fn is_vegetarian(&self) -> Option<bool> {
        None
    }

    fn print(&self) {
        println!("\n{} , {}\n--------------", self.get_name(), self.get_description());
        let iter = self.menu_components.iter();
        for component in iter
            {
                component.print();
            }
    }

    fn get_component_num(&self) -> usize
    {
        self.menu_components.len()
    }

    fn add_component(&mut self, component: Box<dyn MenuComponent>) -> Result<(), OpNotSupportedError>
    {
        self.menu_components.insert(component);
        Ok(())
    }

    fn remove_component(&mut self, component: Box<dyn MenuComponent>) -> Result<bool, OpNotSupportedError> {
        Ok(self.menu_components.remove(component.as_ref()))
    }
    fn get_iter(&mut self) -> Option<DynMenuIter>
    {
        let composite_iter = CompositeIterator::new(Box::new(self.menu_components.iter()));
        Some(Box::new(composite_iter))
    }
}

pub struct MenuItem
{
    name: String,
    description: String,
    vegetarian: bool,
    price: f32
}

impl MenuItem
{
    pub fn new(name: String, description: String, vegetarian: bool, price: f32) -> Self
    {
        Self {
            name,
            description,
            vegetarian,
            price
        }
    }
}

impl MenuComponent for MenuItem
{
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_description(&self) -> &String {
        &self.description
    }

    fn get_price(&self) -> Option<f32> {
        Some(self.price)
    }

    fn is_vegetarian(&self) -> Option<bool> {
        Some(self.vegetarian)
    }

    fn print(&self) {
        println!(" {}", self.get_name());
        if self.vegetarian
        {
            print!("(v)")
        }
        println!(", {}     -- {}", self.price, self.description);
    }

    fn get_component_num(&self) -> usize {
        0
    }

    fn add_component(&mut self, _component: Box<dyn MenuComponent>) -> Result<(), OpNotSupportedError> {
        Err(OpNotSupportedError {})
    }

    fn remove_component(&mut self, _component: Box<dyn MenuComponent>) -> Result<bool, OpNotSupportedError> {
        Err(OpNotSupportedError {})
    }
    fn get_iter(&mut self) -> Option<DynMenuIter>
    {
        None
    }
}