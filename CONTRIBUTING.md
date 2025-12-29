# Contributing to Aura OS

Thank you for your interest in contributing to **Aura OS**! We are building a post-modern operating system, and we welcome contributions from the community.

## Code of Conduct
Please note that this project is released with a [Code of Conduct](CODE_OF_CONDUCT.md). By participating in this project you agree to abide by its terms. (Note: Code of Conduct file to be added).

## Development Workflow

1.  **Fork** the repository.
2.  **Clone** your fork.
3.  Create a **Feature Branch** (`git checkout -b feature/amazing-feature`).
4.  **Commit** your changes (see below for commit standards).
5.  **Push** to the branch.
6.  Open a **Pull Request**.

## Coding Standards

### Rust (Kernel & Userspace)
- **Formatting**: All code must be formatted with `rustfmt`.
- **Linting**: Code must pass `clippy` checks without warnings.
- **Style**: Follow standard Rust idioms (Idomatic Rust).

### C++ (Visual Engine & Cognitive Services)
- **Standard**: C++20.
- **Formatting**: Use `clang-format` with the Google style, but with 4-space indentation.
- **Safety**: Prefer smart pointers (`std::unique_ptr`, `std::shared_ptr`) over raw pointers.

## Commit Messages
We follow the **Conventional Commits** specification:
- `feat: add chronometric light engine`
- `fix: resolve crash in window manager`
- `docs: update architecture blueprint`
