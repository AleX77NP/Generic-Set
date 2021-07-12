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

#[test]
fn init_set() {
    let s = Set::<i32>::new();
    assert_eq!(0, s.elements.len());
}

#[test]
fn add_to_set() {
    let mut s = Set::<&str>::new();
    s.add("Lion");
    s.add("Dog");
    assert_eq!(true, s.elements.contains(&"Lion"));
}

#[test]
fn remove_from_set() {
    let mut s = Set::<&str>::new();
    s.add("Lion");
    s.add("Dog");
    s.remove("Dog");
    assert_ne!(true, s.elements.contains(&"Dog"));
}

#[test]
fn set_len() {
    let mut s = Set::<&str>::new();
    s.add("Lion");
    s.add("Lion");
    s.add("Lion");
    s.add("Dog");
    let l = s.len();
    assert!(2 == l, "{}", l);
}

