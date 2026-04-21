pub fn parse_args(args: Vec<String>) -> String {
    if args.len() < 2 {
        return String::from("config.semver.json");
    }
    args[1].clone()
}
