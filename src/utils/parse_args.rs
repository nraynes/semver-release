pub fn parse_args(args: Vec<String>) -> String {
    if args.len() < 2 {
        return String::from("config.semver.json");
    }
    args[1].clone()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_args_default() {
        let mock_args = vec![String::from("script/path")];
        let parse_args = parse_args(mock_args);
        assert_eq!(parse_args, "config.semver.json");
    }

    #[test]
    fn test_parse_args_one() {
        let mock_args = vec![
            String::from("script/path"),
            String::from("conf.release.json"),
        ];
        let parse_args = parse_args(mock_args);
        assert_eq!(&parse_args, "conf.release.json");
    }

    #[test]
    fn test_parse_args_multiple() {
        let mock_args = vec![
            String::from("script/path"),
            String::from("conf.release.json"),
            String::from("additional_arg"),
        ];
        let parse_args = parse_args(mock_args);
        assert_eq!(&parse_args, "conf.release.json");
    }
}
