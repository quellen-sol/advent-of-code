use std::{
    path::{Path, PathBuf},
    process::Command,
};

use clap::ValueEnum;

#[derive(Clone, Copy, ValueEnum)]
pub enum Lang {
    #[value(name = "rust", alias = "rs")]
    Rust,
    #[value(name = "ts", alias = "typescript")]
    TypeScript,
    #[value(name = "go")]
    Go,
    #[value(name = "py", alias = "python")]
    Python,
}

impl Lang {
    /// The name to type on the command line.
    pub fn name(self) -> &'static str {
        match self {
            Lang::Rust => "rust",
            Lang::TypeScript => "ts",
            Lang::Go => "go",
            Lang::Python => "py",
        }
    }

    fn dir(self) -> &'static str {
        match self {
            Lang::Rust => "rust",
            Lang::TypeScript => "typescript",
            Lang::Go => "go",
            Lang::Python => "py",
        }
    }

    pub fn template_path(self, root: &Path) -> PathBuf {
        let file = match self {
            Lang::Rust => "template.rs",
            Lang::TypeScript => "template.ts.tmpl",
            Lang::Go => "template.gofile",
            Lang::Python => "template.py",
        };
        root.join(self.dir()).join(file)
    }

    pub fn solution_path(self, root: &Path, year: u16, day: u8) -> PathBuf {
        let relative = match self {
            Lang::Rust => format!("src/years/y{year}/solutions/day_{day}.rs"),
            Lang::TypeScript => format!("src/years/{year}/day_{day}.ts"),
            Lang::Go => format!("years/y{year}/day_{day}.go"),
            Lang::Python => format!("src/py/years/y{year}/day_{day}.py"),
        };
        root.join(self.dir()).join(relative)
    }

    /// Builds the runner invocation. Every language takes the same three flags.
    pub fn command(self, root: &Path, year: u16, day: u8, input: &Path) -> Command {
        let (program, args): (&str, &[&str]) = match self {
            Lang::Rust => ("cargo", &["run", "--release", "--quiet", "--"]),
            Lang::TypeScript => ("pnpm", &["--silent", "start"]),
            Lang::Go => ("go", &["run", "."]),
            Lang::Python => ("uv", &["run", "--quiet", "py"]),
        };

        let mut command = Command::new(program);
        command
            .current_dir(root.join(self.dir()))
            .args(args)
            .args(["--year", &year.to_string()])
            .args(["--day", &day.to_string()])
            .arg("--input")
            .arg(input);
        command
    }
}

pub fn render_template(template: &str, year: u16, day: u8) -> String {
    template
        .replace("__YEAR__", &year.to_string())
        .replace("__DAY__", &day.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fills_adjacent_placeholders() {
        assert_eq!(
            render_template("class Day__DAY__Solution__YEAR__ {}", 2026, 5),
            "class Day5Solution2026 {}"
        );
    }
}
