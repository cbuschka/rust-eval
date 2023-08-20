pub struct Sayer {
    message: String
}

impl Sayer {
    pub fn new(message: String) -> Self {
        Sayer { message }
    }
    pub fn say(&self) -> String {
        return self.message.clone();
    }

}

fn main() {
    let sayer = Sayer::new("Hello World!".to_string());
    let message = sayer.say();

    println!("{}", message);
}
