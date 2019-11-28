//!
//! The root of the project
//!
//! use `cargo doc --open` in cmd to see the documentation
//!
//! TODO: implement Singleton Pattern somehow
//!
use std::rc::Rc;
use PatternsImpl::*;

fn main()
{
    let i = 1;
    let rc1 = Rc::new(i);
    let rc2 = rc1.clone();
    println!("{}", rc1.eq(&rc2));
}

