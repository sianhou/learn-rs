/// lesson 9

struct A {
    a: u8,
    b: u32,
    c: u16,
}

#[repr(C)]
struct B {
    a: u8,
    b: u32,
    c: u16,
}

enum C {
    One,
    Two,
}

enum D {
    N,
    H(u32),
    M(Box<u32>),
}

union E {
    u: u32,
    v: u64,
}

fn main() {
    println!("A: {:?}", std::mem::size_of::<A>());
    println!("B: {:?}", std::mem::size_of::<B>());
    println!("C: {:?}", std::mem::size_of::<C>());
    println!("D: {:?}", std::mem::size_of::<D>());
    println!("E: {:?}", std::mem::size_of::<E>());
}