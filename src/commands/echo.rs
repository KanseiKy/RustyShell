pub fn echo(message: std::vec::IntoIter<String>) -> () {
    for i in message {
        print!("{} ", i.replace("\"", "").replace("\'", ""));
    }

    print!("\n");
}