use clap::ArgMatches;

const DEFAULT_FLAGS: &[&str; 2] = &["--quiet", "--color=always"];

const ARG_EXEC: &str = "arg:exec";

pub struct Args {
    pub cmd: String,
}

impl Args {
    pub fn new(args: ArgMatches) -> Self {
        let cmd = args
            .get_one::<String>(ARG_EXEC)
            .cloned()
            .unwrap_or_else(|| "clippy -- -D warnings".to_string());

        Self { cmd }
    }

    pub fn get_cmd_vec(&self) -> Vec<&str> {
        to_cmd_vec(&self.cmd)
    }
}

fn to_cmd_vec(cmd: &str) -> Vec<&str> {
    let mut cmd = cmd.split(' ').collect::<Vec<_>>();
    cmd.splice(1..1, DEFAULT_FLAGS.iter().copied());
    cmd
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_add_the_default_flags() {
        let cmd = "clippy";
        let cmd_slice = to_cmd_vec(cmd);
        assert_eq!(cmd_slice, &["clippy", "--quiet", "--color=always"]);
    }

    #[test]
    fn it_adds_the_default_flags_before_dashdash() {
        let cmd = "clippy -- -D warnings";
        let cmd_slice = to_cmd_vec(cmd);
        assert_eq!(
            cmd_slice,
            &[
                "clippy",
                "--quiet",
                "--color=always",
                "--",
                "-D",
                "warnings"
            ]
        );
    }
}
