pub mod make_readme;

use std::fs::OpenOptions;

use bpaf::{Bpaf, Parser};
use make_readme::{Test, TestCmd};
use owo_colors::OwoColorize;
use xshell::Shell;

use crate::make_readme::create_readme;

// bpaf docs: https://docs.rs/bpaf/latest/bpaf/index.html
// xshell docs: https://docs.rs/xshell/latest/xshell/index.html

/// Automatically runs the various required binaries, and then updates README.md
/// based on the results.
#[derive(Bpaf, Debug, Clone)]
struct AutoRun {
    /// Description of difference from main.
    #[bpaf(positional("DIFFERENCE_FROM_MAIN"))]
    diff_descr: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let opts = auto_run().run();
    let sh = Shell::new()?;

    let packages = ["user", "macro_inside"];
    let features = [vec![], vec!["wrapper"]];
    let test_cmds: Vec<TestCmd> = packages
        .iter()
        .flat_map(|pkg| features.iter().map(|features| TestCmd::new(pkg, features)))
        .collect();
    println!(
        "{}\n\t{}",
        "will run the following commands:".bold(),
        test_cmds
            .iter()
            .map(|tc| tc.to_string())
            .collect::<Vec<String>>()
            .join(",\n\t")
    );
    println!();
    let tests: Vec<Test> = test_cmds
        .into_iter()
        .map(|t| Test::execute(&sh, t))
        .collect::<anyhow::Result<Vec<Test>>>()?;
    println!("{}", "Summary:".bold());
    for test in tests.iter() {
        println!(
            "    {}: {}",
            if test.success {
                "success".bright_green().bold().to_string()
            } else {
                "fail".bright_red().bold().to_string()
            },
            test.cmd.purple()
        )
    }
    let mut f = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(sh.current_dir().join("README.md"))?;
    create_readme(&mut f, opts.diff_descr, tests)?;
    Ok(())
}
