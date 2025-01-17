use std::io;
use std::process::ExitCode;

use rs_string2fixed::str2fixed::stdin2fixed2stdout_default_type_str;

fn sub() -> Result<(), io::Error> {
    let s: String = std::env::var("ENV_CONVERSION_TYPE")
        .ok()
        .unwrap_or_default();
    stdin2fixed2stdout_default_type_str(s.as_str())?;
    Ok(())
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
