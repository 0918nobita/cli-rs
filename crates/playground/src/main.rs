use cli_rs::{Arg, Flag, FlagArg, Group};

/// ソースファイルのパス
#[derive(Debug, Arg)]
struct Input(String);

/// ソースコードを標準入力から読み込む
#[derive(Debug, Flag)]
#[flag(long = "stdin")]
struct StdinFlag;

#[allow(dead_code)]
#[derive(Debug, Group)]
enum InputGroup {
    File(Input),
    Stdin(StdinFlag),
}

/// ソースファイルの形式
#[derive(Debug, FlagArg)]
#[flag_arg(default)]
enum InputFormat {
    Json,

    #[allow(dead_code)]
    Yaml,
}

impl Default for InputFormat {
    fn default() -> Self {
        InputFormat::Json
    }
}

impl std::str::FromStr for InputFormat {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str {
            "json" => Ok(InputFormat::Json),
            "yaml" => Ok(InputFormat::Yaml),
            _ => Err(()),
        }
    }
}

/// 出力するファイルのパス
#[derive(Debug, FlagArg)]
#[flag_arg(short = 'o')]
struct Output(String);

/// 標準出力に出力する
#[derive(Debug, Flag)]
#[flag(long = "stdout")]
struct StdoutFlag;

#[allow(dead_code)]
#[derive(Debug, Group)]
enum OutputGroup {
    File(Output),
    Stdout(StdoutFlag),
}

#[derive(Flag)]
struct Verbose;

fn main() {
    cli_rs::parse!(
        std::env::args(),
        arg {
            input = Input,
        }
        flag_arg {
            input_format = InputFormat,
            output = Output,
        }
        flag {
            verbose = Verbose,
        }
    );

    /*
    println!();

    // 健全マクロを使うバージョン
    cli_rs::parse2!(
        std::env::args(),
        arg {
            input = Input,
        }
        flag_arg {
            input_format = InputFormat,
            output = Output,
        }
        flag {
            verbose = Verbose,
        }
    );
    */
}
