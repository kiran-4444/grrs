#![allow(unused)]

use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
struct CLI {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = CLI::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    grrs_ck::find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    grrs_ck::find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}
