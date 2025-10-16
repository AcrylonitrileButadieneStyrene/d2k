#[macro_export]
macro_rules! export {
    ($a:ident, $b:ident) => {
        mod $a;
        pub use $a::$b as $a;
    };
    ($a:ident) => {
        mod $a;
        pub use $a::$a;
    };
}

#[macro_export]
macro_rules! switch {
    ($token:expr, $( $pat:pat => $result:expr ),+ $(,)?) => {{
        match $token {
            $( $pat => $result, )+
            _ => {
                return Err($crate::Expected::Multiple(
                    vec![$(
                        (stringify!($pat)
                            .rsplit("::").next().unwrap()
                            .split('(').next().unwrap()
                            .to_owned()),
                    )+],
                ).into());
            }
        }
    }};
}
