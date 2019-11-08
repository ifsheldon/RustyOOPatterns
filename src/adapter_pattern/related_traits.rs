//!
//! This mod contains `trait`s used in the practice
//!

///
/// The Duck `trait`
/// The interface that the adapter adapts to
pub trait Duck
{
    fn quack(&self);
    fn fly(&self);
}

///
/// The Turkey `trait`
///
/// The adaptee interface
///
pub trait Turkey
{
    fn gobble(&self);
    fn fly(&self);
}

///
/// Derived trait of Turkey
///
/// For illustrating the benefit of Object Adapter
///
pub trait FancyTurkey : Turkey
{
    fn gobble_and_fly(&self)
    {
        self.gobble();
        self.fly();
    }
}