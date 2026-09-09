$env:CARGO_HOME = "Z:\rust\.cargo"
$env:RUSTUP_HOME = "Z:\rust\.rustup"
$env:PATH = "Z:\rust\.cargo\bin;$env:PATH"

rustc --version
cargo --version
