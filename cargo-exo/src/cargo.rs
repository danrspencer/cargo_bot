use std::io::{BufRead, BufReader};
use std::process::{Command, ExitStatus, Stdio};
use std::thread;
use std::{env, io};

pub struct CargoCommand {
    pub stdout: String,
    pub stderr: String,
    pub result: io::Result<ExitStatus>,
}

impl CargoCommand {
    pub fn run<T: AsRef<str> + AsRef<std::ffi::OsStr>>(args: &[T]) -> Self {
        let current_dir = env::current_dir().expect("failed to get current directory");

        let mut child = Command::new("cargo")
            .args(args)
            .current_dir(current_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("failed to execute process");

        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        // Spawn a thread to handle stdout
        let stdout_handle = thread::spawn(move || {
            let reader = BufReader::new(stdout);
            let mut output = String::new();

            for line in reader.lines() {
                let line = line.unwrap();
                println!("{}", line);
                output.push_str(&line);
                output.push('\n');
            }

            output
        });

        // Spawn another thread to handle stderr
        let stderr_handle = thread::spawn(move || {
            let reader = BufReader::new(stderr);
            let mut output = String::new();

            for line in reader.lines() {
                let line = line.unwrap();
                eprintln!("{}", line);
                output.push_str(&line);
                output.push('\n');
            }

            output
        });

        let result = child.wait();

        let stdout_output = stdout_handle.join().unwrap();
        let stderr_output = stderr_handle.join().unwrap();

        let stdout_stripped = strip_ansi_escapes::strip(stdout_output).unwrap();
        let stderr_stripped = strip_ansi_escapes::strip(stderr_output).unwrap();

        Self {
            stdout: String::from_utf8(stdout_stripped).unwrap(),
            stderr: String::from_utf8(stderr_stripped).unwrap(),
            result,
        }
    }

    pub fn fmt() -> Self {
        let args = ["fmt"];
        Self::run(&args)
    }

    pub fn is_ok(&self) -> bool {
        self.result.is_ok()
    }
}
