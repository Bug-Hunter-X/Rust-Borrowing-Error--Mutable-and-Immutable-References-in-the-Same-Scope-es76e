# Rust Borrowing Error: Mutable and Immutable References

This repository demonstrates a common error in Rust programming related to borrowing rules.  The `bug.rs` file contains code that attempts to have both mutable and immutable references to the same variable within the same scope, leading to a compile-time error. The `bugSolution.rs` file provides a corrected version.

## The Problem
Rust's borrow checker prevents data races and ensures memory safety by enforcing strict rules about how references can be used.  One key rule is that you cannot have both mutable and immutable references to the same data in the same scope.

## How to Reproduce
1. Clone this repository.
2. Navigate to the repository's directory.
3. Compile the `bug.rs` file using `rustc bug.rs`. This will result in a compile-time error.
4. Compile the `bugSolution.rs` file using `rustc bugSolution.rs`. This will compile and run successfully.