//!
//! This mod contains the context part of the translation of Java example given by the book
//! # Logic
//! * The context does not have to pass itself to states' methods, because states hold a ref to the context, and the states actively change the state of the context
//! * To avoid reference cycle, the states hold `Weak` reference to the context
//! * Using RefCell's interior mutability to resolve the issue of not being able to mutate the content via Rc
//! * Using getters to enable states to get and change the context's state
//!
use crate::state_pattern::version_in_book::states::{NoQuarterState, HasQuarterState, SoldState, SoldOutState, State};
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::borrow::Borrow;


pub struct GumballMachine
{
    no_quarter_state: Rc<RefCell<dyn State>>,
    has_quarter_state: Rc<RefCell<dyn State>>,
    sold_state: Rc<RefCell<dyn State>>,
    sold_out_state: Rc<RefCell<dyn State>>,
    current_state: Rc<RefCell<dyn State>>,
    count: usize
}

impl GumballMachine
{
    pub fn new(count: usize) -> Rc<RefCell<Self>>
    {
        let no_quarter_state = Rc::new(RefCell::new(NoQuarterState::new(Weak::new())));
        let has_quarter_state = Rc::new(RefCell::new(HasQuarterState::new(Weak::new())));
        let sold_state = Rc::new(RefCell::new(SoldState::new(Weak::new())));
        let sold_out_state = Rc::new(RefCell::new(SoldOutState::new(Weak::new())));
        let gumball_machine = GumballMachine
            {
                has_quarter_state,
                sold_state,
                current_state: match count {
                    0 => sold_out_state.clone(),
                    _ => no_quarter_state.clone()
                },
                no_quarter_state: no_quarter_state.clone(),
                sold_out_state: sold_out_state.clone(),
                count
            };
        let r = Rc::new(RefCell::new(gumball_machine));
        let gumball_machine_ref: &RefCell<GumballMachine> = r.borrow();
        gumball_machine_ref.borrow_mut().init(r.clone());
        r
    }

    ///
    /// Initialize the weak references  to the context hold by states
    ///
    fn init(&mut self, gumball_machine_ref: Rc<RefCell<GumballMachine>>)
    {
        let mut refcell: &RefCell<dyn State> = self.has_quarter_state.borrow();
        refcell.borrow_mut().set_gumball_machine(Rc::downgrade(&gumball_machine_ref));
        refcell = self.no_quarter_state.borrow();
        refcell.borrow_mut().set_gumball_machine(Rc::downgrade(&gumball_machine_ref));
        refcell = self.sold_state.borrow();
        refcell.borrow_mut().set_gumball_machine(Rc::downgrade(&gumball_machine_ref));
        refcell = self.sold_out_state.borrow();
        refcell.borrow_mut().set_gumball_machine(Rc::downgrade(&gumball_machine_ref));
    }

    pub fn release_ball(&mut self)
    {
        if self.count > 0 {
            self.count -= 1;
            println!("One ball released");
        }
    }

    pub fn get_count(&self) -> usize
    {
        self.count
    }

    pub fn set_state(&mut self, state: Rc<RefCell<dyn State>>)
    {
        self.current_state = state;
    }

    pub fn get_has_quarter_state(&self) -> Rc<RefCell<dyn State>>
    {
        self.has_quarter_state.clone()
    }

    pub fn get_sold_state(&self) -> Rc<RefCell<dyn State>>
    {
        self.sold_state.clone()
    }

    pub fn get_sold_out_state(&self) -> Rc<RefCell<dyn State>>
    {
        self.sold_out_state.clone()
    }

    pub fn get_no_quarter_state(&self) -> Rc<RefCell<dyn State>>
    {
        self.no_quarter_state.clone()
    }
    pub fn insert_quarter(&self)
    {
        let refcell: &RefCell<dyn State> = self.current_state.borrow();
        refcell.borrow_mut().insert_quarter();
    }
    pub fn eject_quarter(&self)
    {
        let refcell: &RefCell<dyn State> = self.current_state.borrow();
        refcell.borrow_mut().eject_quarter();
    }
    pub fn turn_crank(&self)
    {
        let refcell: &RefCell<dyn State> = self.current_state.borrow();
        refcell.borrow_mut().turn_crank();
        refcell.borrow_mut().dispense();
    }
}