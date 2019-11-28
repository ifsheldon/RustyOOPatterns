pub mod backbone;
pub mod peripheries;

#[cfg(test)]
pub mod composite_pattern_test
{
    use super::*;
    use crate::composite_pattern::backbone::{Menu, MenuComponent, MenuItem};
    use std::rc::Rc;
    use std::borrow::BorrowMut;

    #[test]
    pub fn test()
    {
        let mut pancake_house_menu = Menu::new(String::from("PANCAKE HOUSE MENU"), String::from("Breakfast"));
        let mut diner_menu = Menu::new(String::from("DINER MENU"), String::from("Lunch"));
        let mut cafe_menu = Menu::new(String::from("CAFE MENU"), String::from("Dinner"));
        let mut dessert_menu = Menu::new(String::from("DESSERT MENU"), String::from("Dessert!"));

        let mut pancake_house_menu = Box::new(pancake_house_menu);
        let mut diner_menu = Box::new(diner_menu);
        let mut cafe_menu = Box::new(cafe_menu);
        let mut dessert_menu = Box::new(dessert_menu);

        let mut all_menu = Menu::new(String::from("ALL MENUS"), String::from("all menus"));

        dessert_menu.add_component(Box::new(MenuItem::new(String::from("apple pie"), String::from("Apple Pie"), true, 1.59)));
        diner_menu.add_component(Box::new(MenuItem::new(String::from("pasta"), String::from("Spaghetti with Marinara Source"), true, 3.89)));
        diner_menu.add_component(dessert_menu);

        pancake_house_menu.add_component(Box::new(MenuItem::new(String::from("Special Breakfast"), String::from("Egg, Tea, Ice Cream"), false, 3.00)));
        cafe_menu.add_component(Box::new(MenuItem::new(String::from("Burger"), String::from("Hamburger"), false, 0.89)));

        all_menu.add_component(pancake_house_menu);
        all_menu.add_component(diner_menu);
        all_menu.add_component(cafe_menu);
//        all_menu.print();
        for component in all_menu.get_iter()
            {

            }
    }
}