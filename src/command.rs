use log::warn;
use serde::{Deserialize, Serialize};
use std::process::Command;
use toml_edit::InlineTable;

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Cmd {
    pub script: String,
    pub added: String,
    pub timeout: i64,
    pub last_runs_successful: Option<bool>,
    pub last_runs_output: Option<String>,
}

impl Cmd {
    pub fn from_toml(item: &toml_edit::Item) -> Self {
        let last_runs_successful = match item.get("last_runs_successful") {
            Some(val) => val.as_bool(),
            None => None,
        };

        let last_runs_output = item.get("last_runs_output").map(|val| val.to_string());

        Cmd {
            script: item.get("script").unwrap().as_str().unwrap().to_string(),
            added: item.get("added").unwrap().as_str().unwrap().to_string(),
            timeout: item.get("timeout").unwrap().as_integer().unwrap(),
            last_runs_successful,
            last_runs_output,
        }
    }

    pub fn to_toml(&self) -> toml_edit::Item {
        let table: toml_edit::Item = {
            let mut table = InlineTable::default();
            table.insert("script", self.script.as_str().into());
            table.insert("added", self.added.as_str().into());
            table.insert("timeout", self.timeout.into());
            if let Some(last_runs_successful) = &self.last_runs_successful {
                table.insert(
                    "last_runs_successful",
                    last_runs_successful.to_owned().into(),
                );
            }

            if let Some(last_runs_output) = &self.last_runs_output {
                table.insert("last_runs_output", last_runs_output.into());
            }
            toml_edit::value(toml_edit::Value::InlineTable(table))
        };
        table
    }

    pub fn run_cmd(&self) {
        let str: Vec<&str> = self.script.split_whitespace().collect();
        match &str[..] {
            [first, tail @ ..] => cmd(&first, tail),
            _ => println!("somethng else"),
        }
    }
}

fn cmd(first: &str, args: &[&str]) {
    let out = Command::new(first)
        .args(args)
        .output()
        // .spawn()
        .expect("command failed");

    // io::stdout().(&out.stdout).unwrap();
    // io::stderr().write_all(&out.stderr).unwrap();

    warn!("{}", std::str::from_utf8(&out.stdout).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    const CMD_STR: &str = "{ script = \"curl https://localhost:5001\", added = \"2022-07-04T16:05:32.032Z\", timeout = 0 }";

    #[test]
    fn cmd_parses_from_toml() {
        let toml = CMD_STR.parse::<toml_edit::Item>().unwrap();
        let cmd = Cmd::from_toml(&toml);

        assert_eq!(cmd.script, "curl https://localhost:5001".to_string());
        assert_eq!(cmd.added, "2022-07-04T16:05:32.032Z".to_string());
        assert_eq!(cmd.timeout, 0);
        assert_eq!(cmd.last_runs_successful, None);
        assert_eq!(cmd.last_runs_output, None);
    }

    #[test]
    fn cmd_parses_to_tome() {
        let toml = CMD_STR.parse::<toml_edit::Item>().unwrap();
        let cmd = Cmd::from_toml(&toml);

        assert_eq!(cmd.to_toml().to_string(), CMD_STR);
    }
}
