//!
//! # This is a mod for learning implementation of Command Pattern in Rust
//!
//! related trouble shooting discussion at https://users.rust-lang.org/t/command-pattern-in-rust-and-lifetime-issues/34368/3
//!
//! Examples from *Head First Design Pattern*
//!
//!**Pay extra attention to lifetime when references are used**
//!
//! Reference for lifetime: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
//!
//! Further Improvement: implement Command Pattern with Null Object Pattern to eliminate `match`
pub mod receivers;
pub mod commands;

use crate::command_pattern::receivers::Light;
use crate::command_pattern::commands::{SwitchLightCommand, MutCommand};

///
/// # A control struct
/// ## lifetime issue
/// <'a> is needed to indicate *MutCommand* has a lifetime,
/// otherwise Box<T> will take the instance of T as that having 'static lifetime
///
pub struct RemoteControl<'a>
{
    on_commands: [Option<Box<dyn MutCommand + 'a>>; 7],
    off_commands: [Option<Box<dyn MutCommand + 'a>>; 7]
}

impl<'a> RemoteControl<'a>
{
    ///
    /// Constructor of RemoteControl
    ///
    /// # Notice
    /// 1. the return type is recommended to be Self no matter when lifetime is in play
    /// 2. array construction [value; length] needs the value to be implemented Copy trait
    /// 3. Option<T> is not implemented Copy trait, so cannot use [value; length] to construct an Option array
    //or equivalently pub fn new() -> Self // which is recommended
    pub fn new() -> RemoteControl<'a>
    {
        RemoteControl {
            on_commands:[None, None, None, None, None, None, None],
            off_commands:[None, None, None, None, None, None, None]
        }
    }

    ///
    /// For setting a command
    ///
    /// # Notice
    ///
    /// array index must to be *usize*
    ///
    pub fn set_command(&mut self, slot: usize, on_command: Option<Box<dyn MutCommand + 'a>>, off_command: Option<Box<dyn MutCommand + 'a>>)
    {
        self.on_commands[slot] = on_command;
        self.off_commands[slot] = off_command;
    }


    ///
    /// For on button press
    ///
    /// # Notice
    /// match can use & and mut&, just as initializing a variable
    ///
    pub fn on_button_pressed(&mut self, slot: usize)
    {
        match &mut self.on_commands[slot] {
            None => {},
            Some(cmd) => cmd.mut_execute()
        };
    }
    pub fn off_button_pressed(&mut self, slot: usize)
    {
        match &mut self.on_commands[slot] {
            None => {},
            Some(cmd) => cmd.mut_execute()
        };
    }
}


/////
///// A simple test with simple situation
///// where remote has only one slot and one button
/////
//pub fn test()
//{
//    let mut light1: Light = Light::new(false, 1);
//    let mut light2: Light = Light::new(false, 2);
//    let switch_light1_command: SwitchLightCommand = SwitchLightCommand::new(&mut light1);
//    let mut remote: RemoteControl = RemoteControl::new(Box::new(switch_light1_command));
//    remote.button_pressed();
//    {
//        // no compile error due to life time
//        // because RemoteControl owns the box,
//        // and the box owns the command that has a reference to light
//        let switch_light2_command = SwitchLightCommand::new(&mut light2);
//        remote.set_command(Box::new(switch_light2_command));
//    }
//    remote.button_pressed();
//}

///
/// A simple test
///
/// # notice
///
/// &mut here is not so appropriate as no other way can reference the value after mut&
///
/// can use Rc, RefCell, Cell, Arc, Mutex
///
pub fn test()
{
    let mut light1: Light = Light::new(false, 1);
    let mut light2: Light = Light::new(false, 2);
    let l1switch = SwitchLightCommand::new(&mut light1);
    let l2switch = SwitchLightCommand::new(&mut light2);
    let mut remote = RemoteControl::new();
    remote.set_command(0, Some(Box::new(l1switch)), None);
    remote.set_command(1, Some(Box::new(l2switch)), None);
    remote.on_button_pressed(0);
    remote.on_button_pressed(1);
}

///
/// A test mod
///
#[cfg(test)]
mod command_pattern_test
{
    use super::*;
    //cargo test -- --nocapture
    #[test]
    fn cptest()
    {
        test();
    }
}