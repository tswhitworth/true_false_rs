# True and False Unix Commands in Rust

>This project is a simple implementation of OpenBSD's ***true*** and ***false*** commands in Rust.
The ***true*** command does nothing and exits with a successful status code (0), while the false command does nothing and exits with a failure status code (1). The code also includes a basic set of tests to ensure the correct behavior of both commands.

## File Structure

>- ***true_rs*** - The implementation of the true command in Rust.

>- ***false_rs*** - The implementation of the false command in Rust.

>- ***cli.rs*** - The test suite for the true and false commands.

## Prerequisites
To build and test this project, you will need the following:

>Rust (version 1.54 or later)

## Add To Path
To add the true and false commands to your path:

```bash
cargo build --release
sudo cp target/release/true_rs /usr/local/bin/true_rs
sudo cp target/release/false_rs /usr/local/bin/false_rs
```

## Example usage:

#### Manually Check - true_rs:
```bash
$ cargo run --quiet --bin true_rs
$ echo $?
0
```

#### Manually Check - false_rs:
```bash
$ cargo run --quiet --bin false
$ echo $?
1
```

#### Test Suite:

```bash
$ cargo test
...
running 2 tests
test false_not_ok ... ok
test true_ok ... ok
```

#### Exit Values Make Programs More Composable:
The exit value is crucial as it determines the success or failure of a process when combined with another process.
In bash, the ***&&*** operator can chain commands like ***true*** and ***ls***, where the second command only runs if the first one succeeds.

```bash
$ true_rs && ls
Cargo.lock  Cargo.toml  README.md  src  target  tests
```

When running ***false*** and ***ls***, the first process fails, preventing ls from executing.
The entire command's exit status is nonzero due to the initial failure.

```bash
$ false_rs && ls
$ echo $?
1
```
