#[allow(unused)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::Quit;

    match msg {
        Message::Quit => return,
        _ => unreachable!(),
    }
}