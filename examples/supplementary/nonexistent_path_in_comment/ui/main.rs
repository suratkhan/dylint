// This path does not exist
// See ../nonexistent/path/file.rs

// This path should exist
// See ../src/lib.rs

/* This is a block comment with a nonexistent path
   See ../another/nonexistent/path/file.go
*/

// Single dot path that does exist
// ./main.rs

// Single dot path that does not exist
// The span ./ido/not/exist.rs only points to the path

// Workspace root path that does exist
// nonexistent_path_in_comment/Cargo.toml

// Negative test: urls
// This is a url: https://github.com/trailofbits/dylint

// Nonexistent path with line and column reference
// See ../nonexistent/path/file.rs:1:1

// Don't strip back to a colon before the last slash
// See ../nonexistent:directory:with:colons/file.rs:1:1

// Negative test: existing path with line and column reference
// See ../src/lib.rs:1:1

fn main() {}
