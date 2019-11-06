pub struct Light
{
    on: bool,
    order: u8
}

impl Light
{
    pub fn new(light_on: bool, order: u8) -> Self
    {
        Light { on: light_on, order }
    }
    pub fn switch(&mut self)
    {
        if self.on
        {
            println!("Light {} turned off", self.order);
        } else {
            println!("Light {} turned on", self.order);
        }
        self.on = !self.on;
    }
}

