use std::fmt;
use std::error::Error;
use crate::composite_pattern::backbone::MenuComponent;

///
/// A customized Error indicating OpNotSupported
///
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

pub type DynMenuIter<'a> = Box<dyn Iterator<Item=&'a Box<dyn MenuComponent>> + 'a>;

///
/// An iterator recursively iterates over the whole component tree
///
/// A lifetime indicator is needed as iterators give references
///
pub struct CompositeIterator<'a>
{
    stack: Vec<DynMenuIter<'a>>
}

impl<'a> CompositeIterator<'a>
{
    pub fn new(iter: DynMenuIter<'a>) -> Self
    {
        let mut stack = Vec::new();
        stack.push(iter);
        CompositeIterator { stack }
    }
}

impl<'a> Iterator for CompositeIterator<'a>
{
    type Item = &'a Box<dyn MenuComponent>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.last_mut() { // check whether the stack is empty
            None => None,
            Some(top) => // not empty then take the top of stack
                {
                    match top.next() { // check whether a sub tree has been iterated over
                        None => { // fully iterated
                            self.stack.pop();
                            self.next()
                        },
                        Some(next_one) => { // not benn fully iterated
                            match next_one.get_iter() { // check if has children
                                None => Some(&next_one), //is leaf
                                Some(iter) => { // is not leaf
                                    self.stack.push(iter);
                                    Some(&next_one)
                                }
                            }
                        }
                    }
                }
        }
    }
}