use std::env;
use std::fmt::{Display, Formatter};
use std::io;
use std::io::Read;
use std::io::{BufRead, BufReader};
use std::process::{Command, ExitStatus, Stdio};
use std::thread;

pub struct CargoCommand {
    args: Vec<String>,
}

impl Display for CargoCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "cargo {}", self.args.join(" "))
    }
}

impl CargoCommand {
    pub fn new(command: &str) -> Self {
        Self {
            args: command.split(' ').map(String::from).collect::<Vec<_>>(),
        }
    }

    pub fn quiet(mut self) -> Self {
        self.args.insert(1, "--quiet".to_string());
        self
    }

    pub fn color_always(mut self) -> Self {
        self.args.insert(1, "--color=always".to_string());
        self
    }

    pub fn message_format_json(mut self) -> Self {
        self.args.insert(1, "--message-format=json".to_string());
        self
    }

    pub fn run(&self, show_stdout: bool, show_stderr: bool) -> CargoCommandResult {
        let current_dir = env::current_dir().expect("failed to get current directory");

        let mut child = Command::new("cargo")
            .args(&self.args)
            .current_dir(current_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("failed to execute process");

        let mut stdout = child.stdout.take().unwrap();
        let mut stderr = child.stderr.take().unwrap();

        let stdout_output = if show_stdout {
            // Spawn a thread to handle stdout
            thread::spawn(move || {
                let reader = BufReader::new(stdout);
                let mut output = String::new();

                for line in reader.lines() {
                    let line = line.unwrap();
                    println!("{}", line);
                    output.push_str(&line);
                    output.push('\n');
                }

                output
            })
            .join()
            .unwrap()
        } else {
            let mut stdout_output = String::new();
            stdout.read_to_string(&mut stdout_output).unwrap();
            stdout_output
        };

        let stderr_output = if show_stderr {
            // Spawn another thread to handle stderr
            thread::spawn(move || {
                let reader = BufReader::new(stderr);
                let mut output = String::new();

                for line in reader.lines() {
                    let line = line.unwrap();
                    eprintln!("{}", line);
                    output.push_str(&line);
                    output.push('\n');
                }

                output
            })
            .join()
            .unwrap()
        } else {
            let mut stderr_output = String::new();
            stderr.read_to_string(&mut stderr_output).unwrap();
            stderr_output
        };

        let result = child.wait();

        let stdout_stripped = strip_ansi_escapes::strip(stdout_output).unwrap();
        let stderr_stripped = strip_ansi_escapes::strip(stderr_output).unwrap();

        CargoCommandResult {
            stdout: String::from_utf8(stdout_stripped).unwrap(),
            stderr: String::from_utf8(stderr_stripped).unwrap(),
            result,
        }
    }
}

#[derive(Debug)]
pub struct CargoCommandResult {
    pub stdout: String,
    pub stderr: String,
    pub result: io::Result<ExitStatus>,
}

impl CargoCommandResult {
    pub fn was_success(&self) -> bool {
        match self.result {
            Ok(status) => status.success(),
            Err(_) => false,
        }
    }
}
