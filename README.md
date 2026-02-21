<p align="center">
  <img src="https://img.shields.io/badge/License-MIT-yellow.svg" />
  <img src="https://img.shields.io/badge/Language-Rust-orange.svg" />
  <img src="https://img.shields.io/badge/Status-Stable-brightgreen.svg" /> 
  <img src="https://img.shields.io/badge/Library-Serde-blue.svg" />
  <img src="https://img.shields.io/badge/Library-Chrono-red.svg" />
</p>

<h1 align="center">🌌 Nebula Logistics Hub - Galactic Fleet Management</h1>

<p align="center">
  A high-performance Command Line Interface (CLI) designed for interstellar logistics, featuring persistent JSON storage, real-time inventory tracking, and strict memory safety.
</p>

---

## 🎓 Educational Disclaimer
This repository is a cornerstone of my **Personal Apprenticeship** in Rust. 
* **Purpose**: This project focuses on mastering complex data relations (Nested Structs) and their transformation into persistent storage.
* **Evolution**: Building upon basic logic, Nebula Logistics introduces "Relational Management"—handling a collection of objects (Ships) each containing their own dynamic data (Cargo).
* **Focus**: Deep dive into **Serde serialization**, **Nested Iterators**, and the **Ownership/Borrowing** system to manage shared data.

## 🌟 Features
* **Fleet Orchestration**: Manages multiple `Spaceship` instances, each with unique IDs, names, and operational statuses (`Active`, `Maintenance`, `Docked`).
* **Dynamic Inventory**: Implements a nested `Cargo` system, allowing each ship to carry an unlimited list of materials.
* **Global Time Consistency**: Leverages the `Chrono` library to timestamp every cargo entry with UTC precision for synchronized galactic logs.
* **Data Integrity**: Implements ID validation to prevent duplicate entries and uses `retain` logic for safe, non-destructive ship removal.

## 🛠️ Technical Deep Dive
* **Relational Serialization**: Converts complex nested Rust Structs (Fleet > Ships > Cargo) into structured JSON using `serde_json` with pretty-print formatting.
* **Advanced Iteration**: Utilizes `.iter()`, `.iter_mut()`, and `.any()` to traverse and modify the fleet without violating Rust's strict borrowing rules.
* **Efficient Memory Mapping**: Demonstrates the transition from `String` to `&str` references to optimize performance during command-line interaction and data searching.

---

## 🚀 How to Run
1. Clone the repository:
   ```bash
   git clone [https://github.com/tuo-username/nebula-logistics-hub-rust.git](https://github.com/tuo-username/nebula-logistics-hub-rust.git)

2. Build and run:
    ```bash
    cargo run

## ⚖️ License & Copyright

Copyright © 2026 *[dandiest]*

This project is licensed under the MIT License.

*You are free to use, study, and modify this code for educational purposes. Professionalism starts with sharing knowledge.*
