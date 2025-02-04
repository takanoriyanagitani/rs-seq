use std::process::ExitCode;

use std::io;

use rs_seq::wtr::std::ints2stdout;
use rs_seq::MAX_INT_DEFAULT;

fn env_val_by_key(key: &'static str) -> Result<String, io::Error> {
    std::env::var(key).map_err(io::Error::other)
}

fn args() -> impl Iterator<Item = String> {
    std::env::args().skip(1)
}

fn max_int() -> Result<i64, io::Error> {
    env_val_by_key("ENV_MAX_INT")
        .and_then(|s: String| str::parse(s.as_str()).map_err(io::Error::other))
        .or_else(|_| {
            args()
                .next()
                .ok_or_else(|| io::Error::other("no arg"))
                .and_then(|s| str::parse(s.as_str()).map_err(io::Error::other))
        })
}

fn cnt2integers(cnt: i64) -> impl Iterator<Item = i64> {
    1..=cnt
}

fn integers() -> impl Iterator<Item = i64> {
    let max: i64 = max_int().unwrap_or(MAX_INT_DEFAULT);
    cnt2integers(max)
}

fn integers2stdout() -> Result<(), io::Error> {
    let ints = integers();
    ints2stdout(ints)
}

fn main() -> ExitCode {
    integers2stdout()
        .map(|_| ExitCode::SUCCESS)
        .unwrap_or_else(|e| {
            eprintln!("{e}");
            ExitCode::FAILURE
        })
}
