use strum::EnumIter;
use strum::IntoEnumIterator;

#[derive(EnumIter)]
enum Test {
    Left,
    Right,
}
fn main() {
    let x = Test::iter();
    println!("Hello, world!");
}
