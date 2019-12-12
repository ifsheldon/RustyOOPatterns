//!
//! This mod contains example codes of Composite Pattern in Rust
//!
//! Examples from *Head First Design Pattern*
//!
//! Thanks to jethrogb on the Rust forum(see https://users.rust-lang.org/t/a-weird-type-mismatch-error-occurred-when-practising-composite-pattern/35117), these codes finally compile.
//!
//! # Technique Focus
//! Although codes are for learning Design Patterns initially, there are many small technical points(all in peripheries.rs) related and only related to Rust (orz)
//! * Lifetime issues
//! * Iter of HashSet returns references instead owned objects
//! * Mutability issues
//! * Implementing traits for a trait
//! * Error implementation
//! * Extra difficulty because the `Iterator` trait dose not have conventional `hasNext()` as other languages
//! # More
//! see https://raphlinus.github.io/personal/2018/05/08/ecs-ui.html
pub mod backbone;
pub mod peripheries;


#[cfg(test)]
pub mod composite_pattern_test
{
    //!
    //! A mod for test
    //!
    use super::*;
    use crate::composite_pattern::backbone::{Menu, MenuComponent, MenuItem};
    use crate::composite_pattern::peripheries::CompositeIterator;

    ///
    /// Test run
    ///
    /// # Extra notes
    /// Because `add_component()` actually takes the ownership of the arguments, please mind the order of multiple calls of `add_component()` when creating the component tree
    ///
    #[test]
    pub fn test()
    {
        let pancake_house_menu = Menu::new(String::from("PANCAKE HOUSE MENU"), String::from("Breakfast"));
        let diner_menu = Menu::new(String::from("DINER MENU"), String::from("Lunch"));
        let cafe_menu = Menu::new(String::from("CAFE MENU"), String::from("Dinner"));
        let dessert_menu = Menu::new(String::from("DESSERT MENU"), String::from("Dessert!"));
        let special_pancake_menu = Menu::new(String::from("Special Pancake Menu"), String::from("Special"));

        let mut pancake_house_menu = Box::new(pancake_house_menu);
        let mut diner_menu = Box::new(diner_menu);
        let mut cafe_menu = Box::new(cafe_menu);
        let mut dessert_menu = Box::new(dessert_menu);
        let mut special_pancake_menu = Box::new(special_pancake_menu);

        let mut all_menu = Menu::new(String::from("ALL MENUS"), String::from("all menus"));

        dessert_menu.add_component(Box::new(MenuItem::new(String::from("Banana pie"), String::from("Banana Pie"), true, 1.59)));
        dessert_menu.add_component(Box::new(MenuItem::new(String::from("Apple pie"), String::from("Apple Pie"), true, 1.59)));
        diner_menu.add_component(Box::new(MenuItem::new(String::from("pasta"), String::from("Spaghetti with Marinara Source"), true, 3.89)));
        diner_menu.add_component(dessert_menu);

        special_pancake_menu.add_component(Box::new(MenuItem::new(String::from("Special Pancake"), String::from("Egg, Tea, Ice Cream"), false, 3.00)));
        pancake_house_menu.add_component(Box::new(MenuItem::new(String::from("Special Breakfast"), String::from("Egg, Tea, Ice Cream"), false, 3.00)));
        pancake_house_menu.add_component(special_pancake_menu);
        cafe_menu.add_component(Box::new(MenuItem::new(String::from("Burger"), String::from("Hamburger"), false, 0.89)));

        all_menu.add_component(pancake_house_menu);
        all_menu.add_component(diner_menu);
        all_menu.add_component(cafe_menu);
        all_menu.print();
        println!();
        for component in CompositeIterator::new(all_menu.get_iter().unwrap())
            {
                println!("{}", component.get_name());
            }
    }
}