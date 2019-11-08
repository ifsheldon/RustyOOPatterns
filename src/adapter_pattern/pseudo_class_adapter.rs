//!
//! The mod contains the implementation of 'Class Adapter'
//!
//! **Notice**:
//! Since actual class adapters are implemented via multiple inheritance
//! this is just a pseudo-implementation
//!
//!
use crate::adapter_pattern::related_traits::{Duck, Turkey};

///
/// The implementation of `class adapter`
///
/// **Notice:**
///
/// `TurkeyPseudoClassAdapter` is an implementation of both `Duck` and `Turkey`
///
/// So the logic of Turkey is in it, which is not as flexible as object adapters, in which the logic of Turkey is inside the dyn Turkey object they holds
pub struct TurkeyPseudoClassAdapter;

impl Turkey for TurkeyPseudoClassAdapter
{
    fn gobble(&self)
    {
        println!("Gobble Gobble");
    }
    fn fly(&self)
    {
        println!("Fly a short time");
    }
}

impl Duck for TurkeyPseudoClassAdapter
{
    fn quack(&self)
    {
        self.gobble();
    }
    fn fly(&self)
    {
        Turkey::fly(self); // avoid name ambiguity
    }
}