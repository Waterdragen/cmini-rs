
#[cfg(test)]
mod tests {
    use cmini_rs::util::parser::get_kwargs;
    use cmini_rs::util::parser::KwargType as KT;
    use fxhash::FxHashMap;

    macro_rules! str {
        ($expr:expr) => {
            String::from($expr)
        };
    }

    #[test]
    fn parse_kwargs() {
        let cmd_kwargs = FxHashMap::from_iter([
            ("vec".to_owned(), KT::Vec),
            ("bool".to_owned(), KT::Bool),
            ("str".to_owned(), KT::Str),
        ]);
        let kwargs = get_kwargs("", &cmd_kwargs).unwrap();
        assert_eq!(kwargs.arg, "");
        let kwargs = get_kwargs("hello vec --vec 1 2 3", &cmd_kwargs).unwrap();
        assert_eq!(kwargs.arg, "hello vec");
        assert_eq!(kwargs["vec"].unwrap_vec(), Some(&*vec![str!("1"), str!("2"), str!("3")]));
        let kwargs = get_kwargs("hello str --str bogos binted", &cmd_kwargs).unwrap();
        assert_eq!(kwargs.arg, "hello str");
        assert_eq!(kwargs["str"].unwrap_str(), Some("bogos binted"));
        let kwargs = get_kwargs("hello bool --bool", &cmd_kwargs).unwrap();
        assert_eq!(kwargs.arg, "hello bool");
        assert!(kwargs["bool"].unwrap_bool());
        let kwargs = get_kwargs("hello all --vec a b --str c d --bool", &cmd_kwargs).unwrap();
        assert_eq!(kwargs.arg, "hello all");
        assert_eq!(kwargs["vec"].unwrap_vec(), Some(&*vec![str!("a"), str!("b")]));
        assert_eq!(kwargs["str"].unwrap_str(), Some("c d"));
        assert!(kwargs["bool"].unwrap_bool());
        let not_kwargs = get_kwargs("hello all --other --flag", &cmd_kwargs);
        assert!(not_kwargs.is_err());
    }
}