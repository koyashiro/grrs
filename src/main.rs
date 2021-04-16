use anyhow::{Context, Result};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::from_args();
    let content =
        std::fs::read_to_string(&args.path).with_context(|| format!("could not read file"))?;

    find_matches(&content, &args.pattern, std::io::stdout())?;

    Ok(())
}

fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) -> Result<()> {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line).with_context(|| format!("could not write"))?;
        }
    }

    Ok(())
}

#[test]
fn find_a_match() {
    let mut writer = Vec::new();
    let result = find_matches("lorem ipsum\ndolar sit  amet", "lorem", &mut writer);
    assert!(result.is_ok());
    assert_eq!(writer, b"lorem ipsum\n");
}
