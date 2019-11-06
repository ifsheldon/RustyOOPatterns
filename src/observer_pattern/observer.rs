//!
//!# this is Observer
//!
//!

pub trait Observer<T>
{
    fn update(&mut self,msg:&T);
}