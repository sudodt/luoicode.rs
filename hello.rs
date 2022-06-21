hello.rsmod foo;

fn main() {
    foo::hello();
}

pub fn hello() {
    println!("Hello, world!");
}