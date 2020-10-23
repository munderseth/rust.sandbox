/// foo is a function
pub fn foo() {
    String::from("Hello!");
}

/// greeting is a function that was copied from Learn Rust
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}