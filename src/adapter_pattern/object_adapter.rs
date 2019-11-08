//!
//! The mod contains the implementation of object adapter
//!
//!
use crate::adapter_pattern::related_traits::{Duck, Turkey};

///
/// THE object adapter
///
/// The logic of Turkey is inside the dyn Turkey object it owns, which is more flexible
pub struct TurkeyObjectAdapter
{
    turkey: Box<dyn Turkey>
}

impl TurkeyObjectAdapter
{
    pub fn from(turkey_duck: Box<dyn Turkey>) -> Box<dyn Duck>
    {
        Box::new(TurkeyObjectAdapter { turkey: turkey_duck })
    }
}

impl Duck for TurkeyObjectAdapter
{
    fn quack(&self)
    {
        self.turkey.gobble();
    }
    fn fly(&self)
    {
        self.turkey.fly();
    }
}