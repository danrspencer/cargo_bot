# Cargo Exo

![Loader Image](./resources/images/loader.jpeg)

Cargo Exo: Strap in and say 'Express elevator to hell, going down!' to your coding challenges. Squash bugs and streamline your cargo like a pro. [^1]

---

Cargo Exo is a plugin for the Rust ecosystem that enhances your Rust development experience. It utilizes the output of other Rust commands to suggest changes and improve your codebase. 

## Table of Contents
- [Introduction](#introduction)
- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Introduction
Cargo Exo uses the power of GPT from OpenAI to process the warnings and errors from your Rust code, generating intelligent, context-aware suggestions to fix them. With interactive confirmation prompts, you have complete control over which changes to apply.

## Installation

To install Cargo Exo, you need to have Rust installed on your machine. If you haven't installed Rust, please follow the instructions on the official [Rust website](https://www.rust-lang.org/tools/install).

With Rust and cargo installed, you can install the Cargo Exo plugin using the following command:

```bash
$ cargo install --git http://github.com/danrspencer/cargo_exo
```

## Usage

You can use Cargo Exo alongside standard cargo commands like so:

```bash
$ cargo exo clippy -- -D warnings
```

This command runs `cargo clippy` with the `-D warnings` flag, then processes any warnings or errors using Cargo Exo. If there are any fixable issues, Cargo Exo will present the suggested changes in a diff format, and ask if you'd like to apply the changes.

Example output:

```bash
🤖 cargo exo clippy -- -D warnings
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

As shown, Cargo Exo takes the unused variable warning, suggests a fix and asks for your confirmation to apply the changes.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

Cargo Exo is licensed under MIT. For more information, see the [LICENSE](LICENSE) file.

[^1]: This tagline is AI-generated and may or may not fully capture the experience of using an exosuit to handle cargo and squash bugs in your Rust code. Your actual results may be less dramatic, but hopefully still effective![^2]

[^2]: This explanatory note is also AI-generated. The AI hopes you find the humor in this situation.