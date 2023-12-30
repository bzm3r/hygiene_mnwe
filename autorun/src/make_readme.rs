use core::fmt;
use std::{
    borrow::Cow,
    fmt::Display,
    io::{stdout, Write},
    iter::once,
};

use owo_colors::OwoColorize;
use strip_ansi_escapes::strip;
use xshell::{cmd, Shell};

#[derive(Debug, Clone)]
pub struct TestCmd(pub Vec<&'static str>);

impl TestCmd {
    pub fn new(krate: &'static str, features: &[&'static str]) -> Self {
        let base_args = vec!["run", "-p", krate];
        let args = if !features.is_empty() {
            base_args
                .into_iter()
                .chain(once("--features"))
                .chain(features.iter().copied())
                .collect()
        } else {
            base_args
        };
        Self(args)
    }
}

impl Display for TestCmd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            once("cargo")
                .chain(self.0.iter().copied())
                .collect::<Vec<&'static str>>()
                .join(" ")
        )?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Test {
    pub cmd: TestCmd,
    pub output: String,
    pub success: bool,
}

impl Test {
    pub fn execute(sh: &Shell, cmd: TestCmd) -> anyhow::Result<Self> {
        let sh_cmd = cmd!(sh, "cargo")
            .args(cmd.0.iter())
            .args(["--color", "always"].iter());
        print!("{}...", &sh_cmd.white());
        let cmd_out = sh_cmd.ignore_status().output()?;
        let output = if cmd_out.status.success() {
            println!("{}", "success!".bright_green().bold());
            println!();
            String::from_utf8_lossy(&strip(&cmd_out.stdout)).to_string()
        } else {
            println!("{}", "failed.".bright_red().bold());
            stdout().write_all(&cmd_out.stderr)?;
            println!();
            String::from_utf8_lossy(&strip(&cmd_out.stderr))
                .to_string()
                .split_once('\n')
                .map(|(_, second)| second)
                .unwrap_or("nothing to split on.")
                .to_string()
        };
        Ok(Test {
            cmd,
            output,
            success: cmd_out.status.success(),
        })
    }
}

impl Display for Test {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "#### `{}`\n\nWill produce:\n\n```txt\n{}```",
            self.cmd, self.output
        )
    }
}

fn what_should_happen<F: Write>(f: &mut F) -> anyhow::Result<()> {
    writeln!(
        f,
        "# What should happen\n\nAll cases should produce:\n\n```txt\nhello world!\n```\n"
    )?;
    Ok(())
}

pub fn difference<F: Write>(f: &mut F, diff: Option<Cow<'static, str>>) -> anyhow::Result<()> {
    if let Some(diff) = diff {
        writeln!(f, "# Difference from `main`\n\n{}\n", diff)?;
    }
    Ok(())
}

fn what_will_happen<F: Write>(f: &mut F, tests: &[Test]) -> anyhow::Result<()> {
    writeln!(f, "# What will happen\n")?;
    writeln!(f, "Overall:")?;
    for test in tests {
        writeln!(
            f,
            "    - {}: `{}`",
            if test.success { "**SUCCESS**" } else { "FAIL" },
            test.cmd
        )?;
    }
    writeln!(f)?;
    for test in tests {
        writeln!(f, "{}", test)?;
    }
    Ok(())
}

pub fn create_readme<F: Write>(
    f: &mut F,
    diff_from_main: Option<String>,
    tests: Vec<Test>,
) -> anyhow::Result<()> {
    what_should_happen(f)?;
    difference(f, diff_from_main.map(|s| s.into()))?;
    what_will_happen(f, &tests)
}
