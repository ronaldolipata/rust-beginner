use std::io;
use rand::Rng;

fn send_bitcoin() {
    println!("\nWe're going to send some Bitcoin!\n");

    let clients: Vec<&str> = vec!["Homer", "Marge", "Bart", "Lisa"];

    println!("Who do you want to send Bitcoin to?\n");

    for client in &clients { print!("{} ", client) };

    println!("\n");

    let mut recipient: String = String::new();
    io::stdin().read_line(&mut recipient);

    if clients.contains(&recipient.trim()) {
        println!("How many Bitcoin do you want to send?\n");

        let mut amount = String::new();
        io::stdin().read_line(&mut amount);

        return println!("\nYou sent {} Bitcoin to {}!\n", amount.trim(), recipient.trim());
    }

    println!("{} is not in your contacts!", recipient.trim());
}

fn receive_bitcoin() {
    println!("\nWe're going to recieve some Bitcoin!\n");
    let _amount: i32 = rand::thread_rng().gen_range(1, 10);
    println!("You just recieved {} Bitcoin!\n", _amount)
}

fn exit_console() {
    println!("Invalid option, must be (s) or (r)")
}

fn console() {
    println!("\nLet's have fun with Bitcoin!\n");
    println!("Do you want to send (s) or recieve (r) Bitcoin?\n");

    let mut command: String = String::new();
    io::stdin().read_line(&mut command);

    if command.trim() == "s" { return send_bitcoin(); }
    if command.trim() == "r" { return receive_bitcoin(); }
    exit_console();
}

fn main() {
    console();
}
