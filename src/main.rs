#[macro_use]
pub mod set;

#[allow(unused_macros)]
#[macro_export]
macro_rules! set [
    ($($e: expr),*) => ({
        let mut s = set::Set::new();
        $(s.add($e);)*
        s
    })
];

fn main() {
    let mut s = set::Set::<f32>::new(); // float set

    let mut s2 = set::Set::<String>::new(); // string set

    s.add(1.23);
    s.add(18.45);
    s.add(-1.74);

    s.remove(1.23);

    s2.add(String::from("Name 1"));
    s2.add(String::from("Lion"));
    s2.add(String::from("Todo list"));

    println!("{}", s.len());

    println!("{}", s);

    println!("{}", s2.len());

    println!("{}", s2);

    let s3 = set![1,2,3,4,5]; // using macro

    println!("{}", s3.len());

    println!("{}", s3);

}

#[test]
fn set_macro() {
    let mut s = set![1,2,3];
    s.add(7);
    s.add(7);

    assert_eq!(4, s.len());
}

