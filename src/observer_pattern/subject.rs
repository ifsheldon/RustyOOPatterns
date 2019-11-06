use crate::observer_pattern::observer::Observer;

pub trait Subject<T>
{
    fn register(&mut self, o: Box<dyn Observer<T>>);
    fn notify_all(&mut self);
}