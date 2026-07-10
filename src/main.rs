// This is the main.rs file

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
