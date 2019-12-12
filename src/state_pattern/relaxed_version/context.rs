//!
//! This mod contains the context part of the relaxed version 1 of implementations
//!
//! # Logic
//! The context, based on the inform returned by states, accordingly change its states
//! # Drawback
//! More logic coupling between context and states, because the context must know more about states to change its state accordingly(see `fn turn_crank()`)
//!
//!
use crate::state_pattern::relaxed_version::states::{State, SoldOutState, NoQuarterState, HasQuarterState, SoldState, StateEnum};


pub struct GumballMachine
{
    current_state: Box<dyn State>,
    count: usize
}

impl GumballMachine
{
    pub fn new(count: usize) -> Self
    {
        GumballMachine
            {
                current_state: match count {
                    0 => Box::new(SoldOutState),
                    _ => Box::new(NoQuarterState)
                },
                count
            }
    }

    fn change_state(&mut self, next: StateEnum)
    {
        match next {
            StateEnum::NoChange => {},
            StateEnum::NoQuarter => self.current_state = Box::new(NoQuarterState),
            StateEnum::HasQuarter => self.current_state = Box::new(HasQuarterState),
            StateEnum::Sold => self.current_state = Box::new(SoldState),
            StateEnum::SoldOut => self.current_state = Box::new(SoldOutState)
        }
    }

    pub fn insert_quarter(&mut self)
    {
        let next = self.current_state.insert_quarter();
        self.change_state(next);
    }
    pub fn eject_quarter(&mut self)
    {
        let next = self.current_state.eject_quarter();
        self.change_state(next);
    }
    pub fn turn_crank(&mut self)
    {
        let next = self.current_state.turn_crank();
        self.change_state(next);
        let next = self.current_state.dispense(self.count);
        self.change_state(next);
        match self.count {
            0 => self.change_state(StateEnum::SoldOut),
            _ => self.count -= 1
        }
    }
}