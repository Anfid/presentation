fn sub_10(n: u32) -> Result<u32, String> {
    if n < 10 {
        Err(String::from("Invalid value"))
    } else {
        Ok(n - 10)
    }
}

fn main() {
    let x = sub_10(25);
    let y = sub_10(5);

    println!("{:?}\n{:?}", x, y);
}
