mod day00;
use day00::a;
fn main() {
    println!("Hello, world!");
    a::day_00();
}

#[cfg(test)]
mod days {
    use super::*;
    #[test]
    fn day_00() {
        a::day_00();
    }
}
