use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let strict = args.iter().any(|a| a == "--strict");
    let files: Vec<&String> = args.iter().filter(|a| !a.starts_with("--")).collect();
    let (example, actual) = match files.as_slice() {
        [] => (".env.example".to_string(), ".env".to_string()),
        [a, b] => (a.to_string(), b.to_string()),
        _ => {
            eprintln!("usage: envkeydiff [--strict] [EXAMPLE_FILE ACTUAL_FILE]");
            process::exit(2);
        }
    };
    let read = |p: &str| {
        fs::read_to_string(p).unwrap_or_else(|e| {
            eprintln!("cannot read {p}: {e}");
            process::exit(2);
        })
    };
    let d = envkeydiff::diff(&envkeydiff::keys(&read(&example)), &envkeydiff::keys(&read(&actual)));
    for k in &d.missing {
        println!("missing: {k}");
    }
    for k in &d.extra {
        println!("extra:   {k}");
    }
    if !d.missing.is_empty() || (strict && !d.extra.is_empty()) {
        process::exit(1);
    }
    if d.is_clean() {
        println!("ok: {actual} matches {example}");
    }
}
