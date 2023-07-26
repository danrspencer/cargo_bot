# Cargo Exo (Alpha)

*Strap in and say 'Express elevator to hell, going down!' to your coding challenges. Squash bugs and streamline your cargo like a pro with the new feature of automated error fixing[^1]*
<p align="center">
 <img src="./resources/images/loader.jpeg" style="width: 40%; height: auto;">
</p>

---

![Rust Workflow](https://github.com/danrspencer/cargo_exo/actions/workflows/rust.yml/badge.svg)

Cargo Exo is a Rust ecosystem plugin that enhances your development experience. It uses RustFix to attempt automatic fixes for any errors it encounters. In case of build failures with no automatic fixes available, it utilizes ChatGPT to suggest solutions, thus making your codebase more efficient and bug-free.

## Table of Contents
- [Introduction](#introduction)
- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Introduction
Cargo Exo combines the power of GPT from OpenAI and RustFix to process the warnings and errors from your Rust code, generating intelligent, context-aware suggestions for automatic fixes or manual modifications. With interactive confirmation prompts, you retain full control over the applied changes.

## Installation

To install Cargo Exo, you need Rust on your machine. If Rust isn't installed yet, follow the instructions on the official [Rust website](https://www.rust-lang.org/tools/install).

With Rust and cargo installed, install the Cargo Exo plugin using the command:

```bash
$ cargo install --git http://github.com/danrspencer/cargo_exo
```

## Usage

Use Cargo Exo alongside standard cargo commands like this:

```bash
$ cargo exo -x "clippy -- -D warnings"
```

This command runs `cargo clippy` with the `-D warnings` flag, and any warnings or errors are processed by Cargo Exo. If there are fixable issues, Cargo Exo displays the suggested changes in a diff format, asking if you'd like to apply them.

Example output:

```bash
🤖 cargo clippy -- -D warnings
error: unused variable: `t`
 --> cargo-exo-functions/src/explain/mod.rs:7:13
  |
7 |         let t = "test";
  |             ^ help: if this is intentional, prefix it with an underscore: `_t`
  |
  = note: `-D unused-variables` implied by `-D warnings`

✔ Phone a friend? 📞🤖 · yes
⠙ 🤖 thinking ... (gpt-3.5-turbo-0613)
  🤖 done!

error: unused variable
 --> cargo-exo-functions/src/explain/mod.rs:7
7 | -        let t = "test";
7 | +        let _t = "test";
✔ Do you want to apply these changes? · yes
```

As depicted, Cargo Exo takes the unused variable warning, suggests a fix, and asks for your approval to apply changes.

### Usage with Cargo Watch

To enhance your development experience, you can also pair Cargo Exo with Cargo Watch for real-time error fixing. Ensure you have Cargo Watch installed, then run the following command:

```bash
$ cargo watch --watch-when-idle -x exo
```

This command will automatically apply Cargo Exo each time your source code changes, offering suggestions for fixes as you code.

## Contributing

Contributions are most welcome! Feel free to submit a Pull Request.

## License

Cargo Exo is under MIT license. For further information, refer to the [LICENSE](LICENSE) file.

[^1]: This tagline is AI-generated and may or may not fully capture the experience of using an exosuit to handle cargo and squash bugs in your Rust code. Your actual results may be less dramatic, but hopefully still effective![^2]

[^2]: This explanatory note is also AI-generated. The AI hopes you find the humor in this situation.
