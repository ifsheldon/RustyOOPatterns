//!
//! This mod contains the state of relaxed version 1 of implementations
//!
//! # Logic
//! Via returning different enum values. the state can "inform" the context to change its states
//!

pub enum StateEnum
{
    HasQuarter,
    NoQuarter,
    Sold,
    SoldOut,
    NoChange
}

pub trait State
{
    fn insert_quarter(&self) -> StateEnum;
    fn eject_quarter(&self) -> StateEnum;
    fn turn_crank(&self) -> StateEnum;
    fn dispense(&self, gumball_count: usize) -> StateEnum;
}

pub struct NoQuarterState;

impl State for NoQuarterState
{
    fn insert_quarter(&self) -> StateEnum {
        println!("You inserted a quarter");
        StateEnum::HasQuarter
    }

    fn eject_quarter(&self) -> StateEnum {
        println!("Haven't got a quarter");
        StateEnum::NoChange
    }

    fn turn_crank(&self) -> StateEnum {
        println!("You turned, but there's no quarter");
        StateEnum::NoChange
    }

    fn dispense(&self, _: usize) -> StateEnum {
        println!("You need to pay first");
        StateEnum::NoChange
    }
}

pub struct HasQuarterState;

impl State for HasQuarterState
{
    fn insert_quarter(&self) -> StateEnum {
        println!("You can't insert another quarter, for you've inserted a quarter");
        StateEnum::NoChange
    }

    fn eject_quarter(&self) -> StateEnum {
        println!("Returned a quarter");
        StateEnum::NoQuarter
    }

    fn turn_crank(&self) -> StateEnum {
        println!("You turned...");
        StateEnum::Sold
    }

    fn dispense(&self, _: usize) -> StateEnum {
        println!("No gumball dispensed");
        StateEnum::NoChange
    }
}

pub struct SoldState;

impl State for SoldState
{
    fn insert_quarter(&self) -> StateEnum {
        println!("Please wait, we're giving you a gumball");
        StateEnum::NoChange
    }

    fn eject_quarter(&self) -> StateEnum {
        println!("No refund");
        StateEnum::NoChange
    }

    fn turn_crank(&self) -> StateEnum {
        println!("Turning crank twice won't give you one more");
        StateEnum::NoChange
    }

    fn dispense(&self, gumball_count: usize) -> StateEnum {
        if gumball_count > 0
        {
            println!("One ball");
            StateEnum::NoQuarter
        } else {
            println!("Oops, out of gumballs");
            StateEnum::SoldOut
        }
    }
}

pub struct SoldOutState;

impl State for SoldOutState
{
    fn insert_quarter(&self) -> StateEnum {
        println!("Sold out. cannot accept quarters");
        StateEnum::NoChange
    }

    fn eject_quarter(&self) -> StateEnum {
        println!("You can’t eject, you haven’t inserted a quarter yet");
        StateEnum::NoChange
    }

    fn turn_crank(&self) -> StateEnum {
        println!("You turned, but there are no gumballs");
        StateEnum::NoChange
    }

    fn dispense(&self, _: usize) -> StateEnum {
        println!("No gumball dispensed");
        StateEnum::NoChange
    }
}