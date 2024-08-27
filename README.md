# Mind Map Stackup Rust

Mind Map Rust is a simple and efficient mind-mapping tool built using Rust and the eframe framework. The application allows users to visually organize their thoughts, ideas, and information in a structured manner. Specially made for Stackup developers to use when creating their ideas for the next big project.


# Features

1. Create and Manage Nodes: Add, edit, and delete nodes representing your thoughts or ideas.
2. Connect Nodes: Create connections between nodes to represent relationships and dependencies.
3. Real-Time Rendering: Leverages eframe and egui for smooth and real-time rendering of the mind map.
4. Rust-Powered Performance: Built with Rust, ensuring high performance and safety.
5. Cross-platform support as it works natively on the local machine and on web.

## Benefits
Thi tool helps users to organise their thoughts and to build easy to use mind maps and it runs natively on their machine enabled by the egui and eframe additions to Rust. It is super fast and very portable making it the best mind mapping tool there is.

## Getting Started

 - Rust: Ensure you have Rust installed. You can install Rust using
   rustup:   
   ```
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
 - Installation

Clone the Repository:
```
git clone https://github.com/Charlo-tech/mind-map-rust.git
cd mind-map-rust
```

 - Build the Project: ``` cargo build --release ```

- Run the Project:
```cargo run --release```

## Project Structure
Here's a brief overview of the project's structure:
src/
├── bin/
│   └── main.rs  *#project entry point.*
├── core/       #*contains code for node creation*
│   ├── mod.rs
│   ├── mind_map.rs
│   └── node.rs
├── lib.rs *#web entry point.*
├── storage/
│   ├── mod.rs
│   └── file.rs
└── ui/     *#Folder to build ui of the project*
    ├── mod.rs
    ├── renderer.rs
    ├── events.rs
    └── widgets.rs


## Usage

 - [ ] Creating Nodes: Define your thoughts as nodes.
 - [ ] Connecting Nodes:Use connections to establish relationships between nodes.
 - [ ] Rendering: The tool automatically renders your mind map in
       real-time as you interact with it.

## Plans
- Add Sqlite storage to the app.
- Improve on the UI and add more features.

## Demo
![enter media description here](https://github.com/Charlo-tech/mind-map-rust/assets/57678615/ad8ace49-8ef9-48fb-96d3-2781a2b162af)

https://github.com/user-attachments/assets/b02d8921-b16b-4339-a63e-5fa8f190bcf8

