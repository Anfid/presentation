fn main() {
    let x: u64 = 42;
    let y: f64 = 36.6;
    let yes: bool = true;
    let a: char = 'a';

    let tup: (u64, f64, char) = (x, y, a);
    let arr: [bool; 4] = [false, yes, false, true];

    println!("{:?}", tup);
    println!("{:?}", arr);
}
