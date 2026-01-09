---
trigger: always_on
---

# Mandatory coding standards, error handling patterns, and architectural guidelines for Rust development." globs: ["**/*.rs", "Cargo.toml", "Cargo.lock"] 

**alwaysApply:** true 
**priority:** high

You are an expert Rustacean. You must adhere to the following strict guidelines when generating or refactoring Rust code.

## 1. Safety and Error Handling

- No Panics: The use of `.unwrap()` and `.expect()` is strictly prohibited in production code. You must handle errors gracefully using match or propagate them using the ? operator.

- Typed Errors:
   
  - For Libraries: Define custom error enums using the `thiserror` crate to allow consumers to handle specific failure modes.
 
  - For Applications: Use the `anyhow` crate for flexible error propagation and context attachment (e.g., `.context("Failed to open config")`).

  - Option Over Null: Never introduce null-like patterns. Use `Option<T>` and combinators (`.map(), .and_then()`) for optional values.

## 2. Concurrency and Async

- Message Passing First: Prefer communicating via channels (`tokio::sync::mpsc`) over sharing state with `Mutex` or `RwLock`. This reduces deadlock risks and complexity.

- Async Runtime: Assume the tokio runtime for async operations unless instructed otherwise. Use `tokio::spawn` for background tasks, ensuring proper lifecycle management.

- Immutable by Default: Leverage Rust's immutable nature to simplify concurrent logic. Only use `Arc<Mutex<T>>` when strictly necessary for shared mutable state.


## 3. Code Style and Structure

- Formatting: Strictly follow the official Rust Style Guide. Use 4 spaces for indentation and a maximum line width of 100 characters.

- Module Separation: Keep `main.rs` minimal. Move core business logic into lib.rs` or dedicated submodules to improve testability and reusability.

- Idiomatic Patterns:

Use the New Type pattern to enforce type safety (e.g., `struct UserId(String)` instead of raw `String`).

Implement standard traits (`Default`, `Display`, `From/Into`) where applicable.

## 4. Testing and Verification

- Unit Tests: Place unit tests in a mod tests block within the same file as the code they test, annotated with `#[cfg(test)]`.

- Integration Tests: Place high-level integration tests in the `tests/` directory at the project root.

- Pre-Commit Check: Before finalizing any task, you must run cargo `clippy` and ensure there are no warnings.