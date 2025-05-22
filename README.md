> 🚧 **This project is currently a work in progress.** Expect incomplete features and active development.

# 🐟 Toyfish (Rust Edition)

A Rust implementation of the [Toyfish](https://www.chessprogramming.org/Toy_Fish) chess engine — a minimal, educational chess engine designed for simplicity and clarity. This project aims to bring Toyfish to Rust with a clean architecture and modern Rust best practices.

## 📌 About

Toyfish is a small UCI-compatible chess engine originally written in Python. This Rust port is primarily for educational purposes, ideal for learning how chess engines work under the hood: move generation, evaluation, and search.

This project is **not meant to be competitive** with modern engines like Stockfish, but it will help you understand the building blocks of a functioning chess engine.

## 🚀 Getting Started

### Prerequisites

Rust (latest stable)  
You can install it via [rustup.rs](https://rustup.rs)

### Build and Run

```bash
git clone https://github.com/anthonycabreralara/toyfish-rs.git
cd toyfish-rs
cargo build --release
