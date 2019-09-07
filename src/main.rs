fn main() {
    let errorlevel = match std::env::args().nth(1) {
        Some(n) => n,
        None => "0".to_string(),
    };
    let errorlevel = match errorlevel.parse::<i32>() {
        Ok(n) => n,
        Err(_) => 0,
    };
    std::process::exit(errorlevel);
}
