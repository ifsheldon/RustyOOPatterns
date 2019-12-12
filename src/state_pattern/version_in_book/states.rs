//!
//! This mod contains the state part of the translation of Java example given by the book
//! # Logic
//! * To avoid reference cycle, the states hold `Weak` reference to the context
//! * Using RefCell's interior mutability to resolve the issue of not being able to mutate the content via Rc
use crate::state_pattern::version_in_book::context::GumballMachine;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::borrow::Borrow;


pub trait State
{
    fn set_gumball_machine(&mut self, gumball_machine_weak_ref: Weak<RefCell<GumballMachine>>);
    fn get_gumball_machine(&self) -> Option<Rc<RefCell<GumballMachine>>>;
    fn insert_quarter(&self);
    fn eject_quarter(&self);
    fn turn_crank(&self);
    fn dispense(&self);
}

pub struct NoQuarterState
{
    gumball_machine: Weak<RefCell<GumballMachine>>
}

impl NoQuarterState
{
    pub fn new(gumball_machine: Weak<RefCell<GumballMachine>>) -> Self
    {
        NoQuarterState
            {
                gumball_machine
            }
    }
}

impl State for NoQuarterState
{
    fn set_gumball_machine(&mut self, gumball_machine_weak_ref: Weak<RefCell<GumballMachine>>) {
        self.gumball_machine = gumball_machine_weak_ref;
    }

    fn get_gumball_machine(&self) -> Option<Rc<RefCell<GumballMachine>>> {
        self.gumball_machine.upgrade()
    }

    fn insert_quarter(&self) {
        match self.get_gumball_machine() {
            None => {},
            Some(gumball_machine_rc) =>
                {
                    println!("You inserted a quarter");
                    let gumball_machine_ref: &RefCell<GumballMachine> = gumball_machine_rc.borrow();
                    gumball_machine_ref.borrow_mut().set_state(gumball_machine_ref.borrow().get_has_quarter_state());
                }
        }
    }

    fn eject_quarter(&self) {
        println!("Haven't got a quarter");
    }

    fn turn_crank(&self) {
        println!("You turned, but there's no quarter");
    }

    fn dispense(&self) {
        println!("You need to pay first");
    }
}

pub struct HasQuarterState {
    gumball_machine: Weak<RefCell<GumballMachine>>
}

impl HasQuarterState
{
    pub fn new(gumball_machine: Weak<RefCell<GumballMachine>>) -> Self
    {
        HasQuarterState
            {
                gumball_machine
            }
    }
}

impl State for HasQuarterState
{
    fn set_gumball_machine(&mut self, gumball_machine_weak_ref: Weak<RefCell<GumballMachine>>) {
        self.gumball_machine = gumball_machine_weak_ref;
    }

    fn get_gumball_machine(&self) -> Option<Rc<RefCell<GumballMachine>>> {
        self.gumball_machine.upgrade()
    }
    fn insert_quarter(&self) {
        println!("You can't insert another quarter, for you've inserted a quarter");
    }

    fn eject_quarter(&self) {
        match self.get_gumball_machine() {
            None => {},
            Some(gumball_machine_rc) =>
                {
                    println!("Returned a quarter");
                    let gumball_machine_ref: &RefCell<GumballMachine> = gumball_machine_rc.borrow();
                    gumball_machine_ref.borrow_mut().set_state(gumball_machine_ref.borrow().get_no_quarter_state());
                }
        }
    }

    fn turn_crank(&self) {
        match self.get_gumball_machine() {
            None => {},
            Some(gumball_machine_rc) =>
                {
                    println!("You turned...");
                    let gumball_machine_ref: &RefCell<GumballMachine> = gumball_machine_rc.borrow();
                    gumball_machine_ref.borrow_mut().set_state(gumball_machine_ref.borrow().get_sold_state());
                }
        }
    }

    fn dispense(&self) {
        println!("No gumball dispensed");
    }
}

pub struct SoldState {
    gumball_machine: Weak<RefCell<GumballMachine>>
}

impl SoldState
{
    pub fn new(gumball_machine: Weak<RefCell<GumballMachine>>) -> Self
    {
        SoldState
            {
                gumball_machine
            }
    }
}

impl State for SoldState
{
    fn set_gumball_machine(&mut self, gumball_machine_weak_ref: Weak<RefCell<GumballMachine>>) {
        self.gumball_machine = gumball_machine_weak_ref;
    }

    fn get_gumball_machine(&self) -> Option<Rc<RefCell<GumballMachine>>> {
        self.gumball_machine.upgrade()
    }
    fn insert_quarter(&self) {
        println!("Please wait, we're giving you a gumball");
    }

    fn eject_quarter(&self) {
        println!("No refund");
    }

    fn turn_crank(&self) {
        println!("Turning crank twice won't give you one more");
    }

    fn dispense(&self) {
        match self.get_gumball_machine() {
            None => {},
            Some(gumball_machine_rc) =>
                {
                    let gumball_machine_ref: &RefCell<GumballMachine> = gumball_machine_rc.borrow();
                    gumball_machine_ref.borrow_mut().release_ball();
                    let gumball_count = gumball_machine_ref.borrow().get_count();
                    let gumball_machine = gumball_machine_ref.borrow();
                    if gumball_count > 0 {
                        gumball_machine_ref.borrow_mut().set_state(gumball_machine.get_no_quarter_state());
                    } else {
                        println!("Oops, out of gumballs");
                        gumball_machine_ref.borrow_mut().set_state(gumball_machine.get_sold_out_state());
                    }
                }
        }
    }
}

pub struct SoldOutState {
    gumball_machine: Weak<RefCell<GumballMachine>>
}

impl SoldOutState
{
    pub fn new(gumball_machine: Weak<RefCell<GumballMachine>>) -> Self
    {
        SoldOutState
            {
                gumball_machine
            }
    }
}

impl State for SoldOutState
{
    fn set_gumball_machine(&mut self, gumball_machine_weak_ref: Weak<RefCell<GumballMachine>>) {
        self.gumball_machine = gumball_machine_weak_ref;
    }

    fn get_gumball_machine(&self) -> Option<Rc<RefCell<GumballMachine>>> {
        self.gumball_machine.upgrade()
    }

    fn insert_quarter(&self) {
        println!("Sold out. cannot accept quarters");
    }

    fn eject_quarter(&self) {
        println!("You can’t eject, you haven’t inserted a quarter yet");
    }

    fn turn_crank(&self) {
        println!("You turned, but there are no gumballs");
    }

    fn dispense(&self) {
        println!("No gumball dispensed");
    }
}