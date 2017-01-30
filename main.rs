fn main() {
    for arg in std::env::args() {
        println!("{}", arg);
    }
    std::process::exit(1);
}
