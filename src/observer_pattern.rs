//!
//! This mod is for practicing observer pattern in Rust
//!
//! TODO:
//! Improve code, maybe remove <dyn Observer<>>
//!
pub mod observer;
pub mod subject;

use crate::observer_pattern::observer::Observer;
use crate::observer_pattern::subject::Subject;


///
/// A concrete observer
///
struct Obs(u8);

impl Observer<u8> for Obs {
    fn update(&mut self, num: &u8)
    {
        self.0 += *num;
        println!("after update num={}", self.0);
    }
}

///
/// A concrete subject
///
struct Subj
{
    subscribers: Vec<Box<dyn Observer<u8>>>
}

impl Subject<u8> for Subj
{
    fn register(&mut self, o: Box<dyn Observer<u8>>)
    {
        self.subscribers.push(o);
    }
    fn notify_all(&mut self)
    {
        let one: u8 = 1;
        for i in self.subscribers.iter_mut()
            {
                i.update(&one);
            }
    }
}

///
/// A test function
///
pub fn test_observer_pattern()
{
    let  ob1 = Obs(1);
    let  ob2 = Obs(2);
    let mut sub = Subj { subscribers: Vec::<Box<dyn Observer<u8>>>::new() };
    sub.register(Box::new(ob1));
    sub.register(Box::new(ob2));
    sub.notify_all();
}

#[cfg(test)]
mod observer_pattern_test
{
    use super::*;
    #[test]
    fn test()
    {
        test_observer_pattern();
    }
}