//!
//! The root of the project
//!
//! use `cargo doc --open` in cmd to see the documentation
//!
//! TODO: implement Singleton Pattern somehow
//!
use std::rc::Rc;
use PatternsImpl::*;
use std::ops::{Add, AddAssign};
use std::borrow::{BorrowMut, Borrow};
use std::sync::Arc;
use std::cell::RefCell;



#[derive(Copy, Clone)]
pub enum Enu
{
    A = 1,
    B = 2
}

impl Enu
{
    fn to_usize(&self) -> usize
    {
        *self as usize
    }
    fn gggg(&self)
    {
        match self {
            Enu::A => println!("A"),
            Enu::B => println!("B")
        }
    }
}

fn main()
{
    let i = 1;
    let refcell = RefCell::new(i);
    let rc1 = Rc::new(refcell);
    let rc2 = rc1.clone();
    let borr: &RefCell<i32> = rc2.borrow();
//    let borrmut = borr.borrow_mut();
//    borrmut.add_assign(1);
    borr.borrow_mut().add_assign(1);
    println!("{}", borr.borrow());
    println!("{}", Enu::A.to_usize());
    Enu::B.gggg();
}

