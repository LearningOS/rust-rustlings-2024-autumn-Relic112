// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.



#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
    Move{x:i32,y:i32},
    Echo{S:String},
    ChangeColor{a:i32,b:i32,c:i32}, 
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo{S:String::from("hello world")},
        Message::ChangeColor{a:200, b:255, c:255},
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
