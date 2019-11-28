use std::fmt;
use std::error::Error;
use crate::composite_pattern::backbone::MenuComponent;

#[derive(Debug, Clone)]
pub struct OpNotSupportedError;

impl fmt::Display for OpNotSupportedError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "Op Not Supported By this Struct")
    }
}

impl Error for OpNotSupportedError
{
    fn source(&self) -> Option<&(dyn Error + 'static)>
    {
        None
    }
}

pub type DynMenuIter = Box<dyn Iterator<Item=Box<dyn MenuComponent>>>;

pub struct CompositeIterator
{
    stack: Vec<DynMenuIter>
}

impl CompositeIterator
{
    pub fn new(iter: DynMenuIter) -> Self
    {
        let mut stack = Vec::new();
        stack.push(iter);
        CompositeIterator { stack }
    }
}

impl Iterator for CompositeIterator
{
    type Item = Box<dyn MenuComponent>;

    fn next(&mut self) -> Option<Self::Item> {
        // need to debug
        match self.stack.last_mut() {
            None => { return None },
            Some(top) =>
                {
                    match top.next() {
                        None => {
                            self.stack.pop();
                            return self.next()
                        },
                        Some(mut next_one) => {
                            match next_one.get_iter() {
                                None => { return Some(next_one); },
                                Some(iter) => {
                                    self.stack.push(iter);
                                    return Some(next_one);
                                }
                            }
                        }
                    }
                }
        }
    }
}