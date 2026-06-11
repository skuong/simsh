pub fn run(input: &str) {
    let args: Vec<&str> = input.splitn(2, " ").collect();
    if args[0] == "-p" {
        println!("complete: {}: no completion specification", args[1]);
    }
}
