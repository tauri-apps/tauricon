macro_rules! exit {
    ($code:expr) => {
        std::process::exit($code);
    };

    () => {
        std::process::exit(1);
    };
}
