use std::cell::Cell;

struct Foo {
    x: u32,
    y: Cell<u32>,
}

fn main() {
    let foo = Foo { x: 1, y: Cell::new(3) };
    assert_eq!(1, foo.x);
    assert_eq!(3, foo.y.get());
    foo.y.set(5);
    assert_eq!(5, foo.y.get());

    let s = "Hello".to_string();
    let bar = Cell::new(s);
    let x = bar.into_inner();
    assert_eq!("Hello".to_string(), x);
    //assert_eq!("Hello".to_string(), s);
    //assert_eq!(5, bar.get());
}