//!
//! This mod contains a stricter version of the implementation of Template Method Pattern in Rust
//!
//! However, it is still not as strict as the Java example given by the book.
//!

///
/// The trait contains common processes(methods) of making caffeine beverages that can be overridden.
///
pub trait CommonCaffeineProcess
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
}

///
/// The behavior struct that contains un-overridable process/algorithm(method)
///
pub struct RecipePreparation;

impl RecipePreparation
{
    ///
    /// The method contains un-overridable process/algorithm
    ///
    fn prepare_recipe(&self, common_caffeine_process: &Box<dyn CommonCaffeineProcess>)
    {
        common_caffeine_process.boil_water();
        common_caffeine_process.brew();
        common_caffeine_process.pour_in_cup();
        common_caffeine_process.add_condiments();
    }
}

///
/// The trait for realizing stricter Template Method Pattern
///
pub trait AsCaffeineBeverage
{
    fn get_common_process(&self) -> &Box<dyn CommonCaffeineProcess>;
    fn get_recipe_preparation_process(&self) -> &RecipePreparation;
    ///
    /// The method contains "un-overridable" process/algorithm
    ///
    /// # Notice
    /// Although the method `prepare_recipe` of RecipePreparation is not changable,
    /// the `prepare_recipe` in `AsCaffeineBeverage` can be overridden.
    ///
    /// Therefore, it is still not as strict as the final method in the Java example given by the book.
    fn prepare_recipe(&self)
    {
        let common_process = self.get_common_process();
        let recipe_preparation_process = self.get_recipe_preparation_process();
        recipe_preparation_process.prepare_recipe(common_process);
    }
}

///
/// A behavior struct for the use in Coffee
///
pub struct CoffeeProcess;

impl CommonCaffeineProcess for CoffeeProcess
{
    fn brew(&self)
    {
        println!("Brew Coffee");
    }
    fn add_condiments(&self)
    {
        println!("Add Milk");
    }
}

///
/// An example of caffeine beverage
///
pub struct Coffee
{
    coffee_process: Box<dyn CommonCaffeineProcess>,
    recipe_preparation: RecipePreparation
}

impl AsCaffeineBeverage for Coffee
{
    fn get_common_process(&self) -> &Box<dyn CommonCaffeineProcess>
    {
        &self.coffee_process
    }
    fn get_recipe_preparation_process(&self) -> &RecipePreparation
    {
        &self.recipe_preparation
    }
}

impl Coffee
{
    pub fn new() -> Self
    {
        Coffee {
            coffee_process: Box::new(CoffeeProcess {}),
            recipe_preparation: RecipePreparation {}
        }
    }
}

///
/// A behavior struct for the use in Tea
///
pub struct TeaProcess;

impl CommonCaffeineProcess for TeaProcess
{
    fn brew(&self) {
        println!("Brew Tea");
    }

    fn add_condiments(&self) {
        println!("Add Lemon")
    }
}

///
/// An example of caffeine beverage
///
pub struct Tea
{
    tea_process: Box<dyn CommonCaffeineProcess>,
    recipe_preparation: RecipePreparation
}

impl AsCaffeineBeverage for Tea
{
    fn get_common_process(&self) -> &Box<dyn CommonCaffeineProcess>
    {
        &self.tea_process
    }
    fn get_recipe_preparation_process(&self) -> &RecipePreparation
    {
        &self.recipe_preparation
    }
}

impl Tea
{
    pub fn new() -> Self
    {
        Tea {
            tea_process: Box::new(TeaProcess {}),
            recipe_preparation: RecipePreparation {}
        }
    }
}

