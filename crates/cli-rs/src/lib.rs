use std::fmt::{Display, Formatter, Result as FmtResult};

pub use cli_rs_macro::{parse, Arg, Flag, FlagArg, Group};

pub struct LongFlag(String);

impl LongFlag {
    pub fn new(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl Display for LongFlag {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "--{}", self.0)
    }
}

pub struct ShortFlag(char);

impl ShortFlag {
    pub fn new(c: char) -> Self {
        Self(c)
    }
}

impl Display for ShortFlag {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "-{}", self.0)
    }
}

pub trait AsArg: Sized {
    fn name() -> String;

    fn description() -> String;
}

pub trait AsFlag: Sized {
    fn long() -> LongFlag;

    fn short() -> Option<ShortFlag>;

    fn description() -> String;
}

pub trait AsFlagArg: Sized {
    fn long() -> LongFlag;

    fn short() -> Option<ShortFlag>;

    fn description() -> String;

    fn parse(s: &str) -> Option<Self>;
}

pub trait AsGroup: Sized {
    fn name() -> String;

    fn description() -> String;
}

#[derive(Debug)]
pub enum Token {
    Long(String),
    Short(char),
    Value(String),
}

pub fn parse_into_tokens(args: impl Iterator<Item = String>) -> impl Iterator<Item = Token> {
    args.skip(1).flat_map(|arg| {
        if let Some(flag) = arg.strip_prefix("--") {
            return vec![Token::Long(flag.to_owned())];
        }
        if let Some(cs) = arg.strip_prefix('-') {
            return cs.chars().map(Token::Short).collect::<Vec<_>>();
        }
        vec![Token::Value(arg)]
    })
}

// Experimental
mod hygienic_macro {
    #[macro_export]
    macro_rules! parse2 {
        ( $args:expr, ) => {};

        ( $args:expr, arg { $( $p:pat = $ty:ty ),* $(,)? } $( $rest:tt )* ) => {
            println!("[Arguments]");
            $( println!("    {}", stringify!($ty)); )*
            cli_rs::parse2!($args, $( $rest )*);
        };

        ( $args:expr, flag { $( $p:pat = $ty:ty ),* $(,)? } $( $rest:tt )* ) => {
            println!("[Flags]");
            $( println!("    {}", stringify!($ty)); )*
            cli_rs::parse2!($args, $( $rest )*);
        };

        ( $args:expr, flag_arg { $( $p:pat = $ty:ty ),* $(,)? } $( $rest:tt )* ) => {
            println!("[Flag arguments]");
            $( println!("    {}", stringify!($ty)); )*
            cli_rs::parse2!($args, $( $rest )*);
        };

        ( $args:expr, group { $( $p:pat = $ty:ty ),* $(,)? } $( $rest:tt )* ) => {
            println!("[Groups]");
            $( println!("    {}", stringify!($ty)); )*
            cli_rs::parse2!($args, $( $rest )*);
        };
    }
}
