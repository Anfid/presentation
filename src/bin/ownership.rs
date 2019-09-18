fn main() {
    // x владеет строкой Hello
    let x = String::from("Hello");

    // строка передана во владение y
    let y = x;

    println!("{}", y);
}
