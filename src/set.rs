use std::fmt;

#[derive(Debug)]
pub struct Set<T> where T: PartialEq {
     pub elements: Vec<T>
}

impl<T> Set<T> where T: PartialEq {
    pub fn new() -> Set<T> {
        Set{elements: Vec::new()}
    }

    pub fn add(&mut self, elem: T) {
        if !self.elements.contains(&elem) {
            self.elements.push(elem);
        }
    } 
}

impl<T> fmt::Display for Set<T> where T: fmt::Display + fmt::Debug + PartialEq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.elements)
    }
}