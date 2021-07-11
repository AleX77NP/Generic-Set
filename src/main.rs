pub mod set;

fn main() {
    let mut s = set::Set::<f32>::new();

    s.add(1.23);
    s.add(18.45);
    s.add(-1.74);

    println!("{}", s);
}
