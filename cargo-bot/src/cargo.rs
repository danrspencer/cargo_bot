use std::env;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::thread;

pub fn fmt() -> (String, Result<(), String>) {
    let args = ["fmt"];
    (format!("cargo {}", args.join(" ")), command(&args))
}

pub fn command<T: AsRef<str> + AsRef<std::ffi::OsStr>>(args: &[T]) -> Result<(), String> {
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

    let _stdout_output = stdout_handle.join().unwrap();
    let stderr_output = stderr_handle.join().unwrap();

    let stripped = strip_ansi_escapes::strip(stderr_output).unwrap();

    match result {
        Ok(status) if status.success() => Ok(()),
        _ => Err(String::from_utf8(stripped).unwrap()),
    }
}
