fn error_level(args: Vec<String>) -> i32 {
    if args.is_empty() {
        return 0;
    }
    match &args[0].parse::<i32>() {
        Ok(n) => *n,
        Err(_) => 0,
    }
}

fn main() {
    let args = std::env::args().skip(1).collect();
    let error_level = error_level(args);
    std::process::exit(error_level);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_level() {
        assert_eq!(error_level(vec!["1".to_owned()]), 1);
        assert_eq!(error_level(vec!["2".to_owned()]), 2);
        assert_eq!(error_level(vec!["-1".to_owned()]), -1);
        assert_eq!(error_level(vec!["10".to_owned()]), 10);
        assert_eq!(error_level(vec!["1".to_owned(), "2".to_owned()]), 1);
        assert_eq!(error_level(vec!["abc".to_owned()]), 0);
        assert_eq!(error_level(vec![]), 0);
    }
}
