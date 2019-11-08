//!
//! A typical Factory Pattern implementation
//!
pub trait ITStaff
{
    fn working(&self) -> String;
    fn get_salary(&self) -> u32;
}

pub struct ITManager
{
    year_bonus: u32
}

impl ITStaff for ITManager
{
    fn working(&self) -> String
    {
        String::from("IT Manager")
    }
    fn get_salary(&self) -> u32
    {
        30000
    }
}

impl ITManager
{
    fn new(year_bonus: u32) -> Self
    {
        ITManager { year_bonus }
    }
}

pub struct Developer
{
    level: u8
}

impl ITStaff for Developer
{
    fn working(&self) -> String {
        String::from("Developer")
    }

    fn get_salary(&self) -> u32 {
        10000 + (self.level) as u32 * 2000
    }
}

impl Developer
{
    pub fn new(level: u8) -> Self
    {
        Developer { level }
    }
}

pub struct Tester
{
    level: u8
}

impl ITStaff for Tester
{
    fn working(&self) -> String {
        String::from("Tester")
    }

    fn get_salary(&self) -> u32 {
        8000 + (self.level) as u32 * 1500
    }
}

impl Tester
{
    pub fn new(level: u8) -> Self
    {
        Tester { level }
    }
}

///
/// enum used for the factory
///
pub enum ITStaffKind
{
    ITManager,
    Developer,
    Tester
}

///
/// The factory
///
/// May be implemented as 'static' factory
///
pub struct ITStaffFactory;

impl ITStaffFactory
{
    pub fn create_IT_staff(kind: ITStaffKind) -> Box<dyn ITStaff>
    {
        match kind
            {
                ITStaffKind::Developer => Box::new(Developer::new(0)),
                ITStaffKind::ITManager => Box::new(ITManager::new(0)),
                ITStaffKind::Tester => Box::new(Tester::new(0))
            }
    }
}
