# rust.sandbox
A Sandbox to play around with Rust: https://www.rust-lang.org/learn/get-started

Trying out some basic stuff:
- Building the out of the box sample
  ```
  $ cargo new hello-rust
  $ cp -r hello-rust/* .
  $ rm -r hello-rust/
  ```

  - basic commands
    ```
    $ cargo build
    $ cargo run
    $ cargo test
    $ cargo doc
    ```
- Run some tests and publish to Testspace
- Generate some documentation and deploy to GitHub pages

## References

- Testspace: https://munderseth.testspace.com/projects/munderseth:rust.sandbox
- GitHub Pages: https://munderseth.github.io/rust.sandbox/
   - using a redirect to the sub-folder called `hello_rust`
- There are two GitHub workflows
   - CI for running tests, publishing
   - DOC for building & deploying docs to GitHub pages. Note, using the `workflow_dispatch` event (UI)

## Installation 
Info

- https://www.rust-lang.org/tools/install
  - https://visualstudio.microsoft.com/visual-cpp-build-tools/ 
