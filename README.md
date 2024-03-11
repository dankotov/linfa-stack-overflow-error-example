# linfa-stack-overflow-error-example

To reproduce the issue:

1. Run `cargo run --release > output.json` as is - verify that the initial dataset doesnt produce a stack overflow error.
2. Comment line 8 and uncomment line 10 to use the dataset that produces a stack overflow error.
3. Run `cargo run --release > output.json` - verify that the second dataset produces a stack overflow error, despite being smaller than the initial dataset.
