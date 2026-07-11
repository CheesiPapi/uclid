# UCLID  PHYSICS SIMULATOR
This is a Log on things learned while building the UCLID physics engine. My goal for this physics engine is that it will be super light-weight and once compiled, the code runs on almost no cpu power. Again, that is the goal. 

#### 07/09/26
## Phase (1)

### Initial Naming
This whole thing started by choosing the name of what the project would be.
This was pretty fun and I wanted to go with a nod to mathematics or physics. 
I decided to go with Euclid because Euclid wrote [*The Elements*](https://share.google/8a95Soy4MwDvJnjkU) which has stood the test of time.
Now there is an ESA Euclid space telescope that was launched in 2023 to map the dark universe.
I wanted to avoid confusion so I opted to drop the "E" from the front of Euclid, and just go with uclid.
### Cargo.toml File
Rust uses Cargo to pull down crates and organize the architecture of the build.
```
[package]
name = "uclid"
version = "0.1.0"
edition = "2021"
description = "A terminal-driven, hardware-accelerated physics simulation engine."
# Replace with your actual name/email when you open-source it
authors = ["Your Name <your.email@example.com>"] 

[dependencies]
# Graphics and Windowing (Week 1 & 4)
# raylib-rs is the safest, most maintained Rust binding for Raylib
raylib = "5.0" 

# Serialization & Parsing (Week 2 & 3)
# These will allow you to read your scenario.toml files effortlessly
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"
```
### The src/main.rs File
For this project I intend to utilize raylib to draw the visuals.
Raylib is light-weight and can do 2D and 3D graphics.
I want this project to be as light-weight as possible after the initial compile is completed. 
If there are updates after the initial release (which there will be), then it will have to be re-compiled.
This is a chore that I can live with. 
The physics engine will get commands from the user through TOML files, and run pre-defined scenarios from those TOML files.<br>
<br>
This is the main.rs file and this will run a loop that keeps the running the program until it is finished or the user closes the program.

```
use raylib::prelude::*;

// We use an Enum to strictly manage what the engine is allowed to do.
// Deriving PartialEq allows us to easily compare current state with ==
#[derive(PartialEq)]
enum AppState {
    Simulating,
    ConsoleOpen,
}

fn main() {
    // 1. Initialize the Raylib window and input thread
    let (mut rl, thread) = raylib::init()
        .size(1024, 768)
        .title("uclid - Kinematics Engine")
        .build();

    // Lock the frame rate so physics don't run wild on your Ryzen 9
    rl.set_target_fps(60);

    // 2. Initialize Engine State
    let mut current_state = AppState::Simulating;
    let mut console_input = String::new();

    // 3. The Core Execution Loop
    while !rl.window_should_close() {
        
        // --- INPUT & UPDATE PHASE ---
        
        // Toggle console with the Tilde/Grave key (`)
        if rl.is_key_pressed(KeyboardKey::KEY_GRAVE) {
            current_state = match current_state {
                AppState::Simulating => AppState::ConsoleOpen,
                AppState::ConsoleOpen => AppState::Simulating,
            };
        }

        // Handle typing if the console is open
        if current_state == AppState::ConsoleOpen {
            // Capture typed characters
            if let Some(char_pressed) = rl.get_char_pressed() {
                // Basic check to ensure it's a printable ASCII character
                if char_pressed as u32 >= 32 && char_pressed as u32 <= 126 {
                    console_input.push(char_pressed);
                }
            }

            // Handle backspace
            if rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE) || rl.is_key_down(KeyboardKey::KEY_BACKSPACE) {
                console_input.pop();
            }
            
            // Handle execution (Enter key) - For now, just clears it
            if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                println!("Command Sent: {}", console_input); // Prints to standard terminal
                console_input.clear();
            }
        }

        // --- DRAW PHASE ---
        // rl.begin_drawing creates a "RaylibDrawHandle". 
        // This takes a mutable reference to `rl`. The borrow checker loves this 
        // because it ensures nothing else can mutate `rl` while drawing is happening.
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::new(15, 15, 18, 255)); // Deep charcoal background

        // Draw the simulation elements
        d.draw_text(
            "uclid physics engine : running", 
            20, 20, 20, 
            Color::DARKGRAY
        );

        // Draw the drop-down console over the simulation if active
        if current_state == AppState::ConsoleOpen {
            // Draw a semi-transparent console window
            d.draw_rectangle(0, 0, 1024, 300, Color::new(20, 20, 20, 230));
            d.draw_line(0, 300, 1024, 300, Color::GREEN);
            
            // Draw the prompt path and the user's input
            let prompt = "uclid/engine>";
            d.draw_text(prompt, 20, 260, 20, Color::GREEN);
            
            // Offset the typing cursor by the length of the prompt (roughly 140 pixels)
            d.draw_text(&console_input, 160, 260, 20, Color::RAYWHITE);
        }
    }
}
```

To build the file, you utilize *"cargo build --release"*.
Then *"./uclid"* to run the file.

### Error produced
```
esmos@CheesiBeasti MINGW64 ~/local_Documents/coding/uclid (master)
$ cargo run
bash: cargo: command not found

esmos@CheesiBeasti MINGW64 ~/local_Documents/coding/uclid (master)
$
```

Gemini said that when i installed rust via rustup, it placed cargo and rustc executables into a hidden folder in the directory (~/.cargo/bin). 
I just needed to refresh the current session by either *"source $HOME/.cargo/env"* or by simply closing the window and opening it back up again.
I ran the build but it kicked back an error after a lot of compiliing.

```
running: "cmake" "--build" "C:\\Users\\esmos\\local_Documents\\coding\\uclid\\target\\debug\\build\\raylib-sys-84f1d04ca8b5f052\\out\\build" "--target" "install" "--config" "Debug" "--parallel" "24"



  thread 'main' (35004) panicked at C:\Users\esmos\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\bindgen-0.70.1\lib.rs:622:27:

  Unable to find libclang: "couldn't find any valid shared libraries matching: ['clang.dll', 'libclang.dll'], set the `LIBCLANG_PATH` environment variable to a path where one of these files can be found (invalid: [])"

  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace 
```
Gemini said this is something to do with C-interop on Windows. 
The C-side of the compilation process was throwing a fit.
Raylib was written in C and for my Rust code to talk with Raylib, a crate called *raylib-sys* uses a tool called *bindgen*.
*bindgen* acts as an automatic translator from Rust to C and C to Rust.
Although *bindgen* needs the LLVM compiler infrastructure (specifically the library called *libclang*).

#### Git/GitHub Knowledge 1
Was instructed to go to [LLVM GitHub Releases](https://github.com/llvm/llvm-project/releases), and download **LLVM-<version>-win64.exe**. I specifically used **LLVM-22.1.8-win64.exe**.
Once the executable is ran, you follow the install wizard. 
It is important to select **"Add LLVM to the system PATH for all users"**
Then you restart the Forge.
The Rust Forge is the official Rust ecosystem (*forge.rust-lang.org*).

After that
```
cargo clean
cargo run
```

Then it started working.
A black screen pulled up and it says "uclid physics engine : running" on a black background.
Beautiful.

### Saving
Apparently I was at the end of Phase 1.
This took about 3-4 hours. 
If that. 
I initiated the git and made the first and second commits.

#### 07/11/26
#### Git/GitHub Knowledge 2
I took the next day off and on the saturday to follow, I made the GitHub and first branch. 
The location of the git on GitHub is https://github.com/CheesiPapi/uclid.git.
Of course I can just get the info from my own computer, but I might send this log to different computers and I dont want to forget. 
What am I saying. 
This is a log, I dont have to explain myself.

#### Git/GitHub knowledge 3
Once main was made, I branched off to **week2-parser** branch. 
All I have done so far on this branch is make this log. 
I think i'm going to change the name and start making issues and making those branches associated with the issues being addressed like Dan taught me.
I commited the log changes i made before and then pushed and merged it with main. 
This was because the only changes i made that were different, were the log changes.
Now that everything is nice and clean, I made a new branch called *1-issue-create-configuration-file*.
