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

    pub fn remove(&mut self, elem: T) where T: Copy {
        self.elements.retain(|&x| x != elem);
    } 
    
    pub fn len(&self) -> usize {
        self.elements.len()
    }
}

impl<T> fmt::Display for Set<T> where T: fmt::Display + fmt::Debug + PartialEq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.elements)
    }
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! set [
    ($($e: expr),*) => ({
        let mut s = set::Set::new();
        $(s.add($e);)*
        s
    })
];