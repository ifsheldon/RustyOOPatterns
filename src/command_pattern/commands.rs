use crate::command_pattern::receivers::Light;

pub trait Command
{
    fn execute(&self);
}

pub trait MutCommand
{
    fn mut_execute(&mut self);
}

pub struct SwitchLightCommand<'a>
{
    light: &'a mut Light
}

impl<'a> SwitchLightCommand<'a>
{
    pub fn new(light: &'a mut Light) -> Self
    {
        SwitchLightCommand { light }
    }
}

impl<'a> MutCommand for SwitchLightCommand<'a>
{
    fn mut_execute(&mut self)
    {
        self.light.switch();
    }
}