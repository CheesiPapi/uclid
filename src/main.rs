use raylib::prelude::*;
use serde::Deserialize;
use std::fs;

// The top-level struct that holds the entire file
#[derive(Deserialize, Debug)]
struct Scenario {
    environment: Environment,
    entity: Entity,
}

#[derive(Deserialize, Debug)]
struct Environment {
    gravity: f32,
}

#[derive(Deserialize, Debug)]
struct Entity {
    mass: f32,
    radius: f32,
    pos_x: f32,
    pos_y: f32,
}

// We use an Enum to strictly manage what the engine is allowed to do.
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

    rl.set_target_fps(60);

    // 2. Load the Configuration File
    // Read the file directly from the disk
    let config_raw = fs::read_to_string("scenario.toml")
        .expect("Failed to find scenario.toml file in the root directory");
    
    // Let serde and the toml crate map the text into our Rust structs
    let mut current_scenario: Scenario = toml::from_str(&config_raw)
        .expect("Failed to parse the TOML file. Check for typos!");

    // 3. Initialize Engine State
    let mut current_state = AppState::Simulating;
    let mut console_input = String::new();
    let mut velocity_y: f32 = 0.0; // <--- ADD THIS LINE

    // 4. The Core Execution Loop
    while !rl.window_should_close() {
        
        // --- INPUT & UPDATE PHASE ---
        
        // Toggle console with the Tilde/Grave key (`)
        if rl.is_key_pressed(KeyboardKey::KEY_GRAVE) {
            current_state = match current_state {
                AppState::Simulating => AppState::ConsoleOpen,
                AppState::ConsoleOpen => AppState::Simulating,
            };
        }

        // --- PHYSICS UPDATE ---
        // Only run physics if the console is closed
        if current_state == AppState::Simulating {
            // Calculate a fixed delta-time for 60 FPS
            let dt = 1.0 / 60.0;
            
            // v = v0 + a*t
            velocity_y += current_scenario.environment.gravity * dt;
            
            // p = p0 + v
            current_scenario.entity.pos_y += velocity_y;

            // Basic floor collision
            let floor = 768.0 - current_scenario.entity.radius;
            if current_scenario.entity.pos_y >= floor {
                current_scenario.entity.pos_y = floor;
                velocity_y *= -0.8; // Reverse velocity and lose 20% energy
            }
        }

        // --- PHYSICS UPDATE ---
        // Only run physics if the console is closed
        if current_state == AppState::Simulating {
            // Calculate a fixed delta-time for 60 FPS
            let dt = 1.0 / 60.0;
            
            // v = v0 + a*t
            velocity_y += current_scenario.environment.gravity * dt;
            
            // p = p0 + v
            current_scenario.entity.pos_y += velocity_y;

            // Basic floor collision
            let floor = 768.0 - current_scenario.entity.radius;
            if current_scenario.entity.pos_y >= floor {
                current_scenario.entity.pos_y = floor;
                velocity_y *= -0.8; // Reverse velocity and lose 20% energy
            }
        }

        // Handle typing if the console is open
        if current_state == AppState::ConsoleOpen {
            if let Some(char_pressed) = rl.get_char_pressed() {
                if char_pressed as u32 >= 32 && char_pressed as u32 <= 126 {
                    console_input.push(char_pressed);
                }
            }

            if rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE) || rl.is_key_down(KeyboardKey::KEY_BACKSPACE) {
                console_input.pop();
            }
            
            // The Parser Execution Block
            if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                // Split the typed command into words
                let mut parts = console_input.trim().split_whitespace();
                let command = parts.next().unwrap_or("");
                let target = parts.next().unwrap_or("");
                let value = parts.next().unwrap_or("");

// Advanced Parser Logic
                if command == "set" {
                    // First, try to parse the value into a float
                    if let Ok(num_value) = value.parse::<f32>() {
                        // Match the target string to the correct struct field
                        match target {
                            "gravity" => {
                                current_scenario.environment.gravity = num_value;
                                println!("Gravity updated to: {}", num_value);
                            }
                            "radius" => {
                                current_scenario.entity.radius = num_value;
                                println!("Radius updated to: {}", num_value);
                            }
                            "x" => {
                                current_scenario.entity.pos_x = num_value;
                                println!("X position updated to: {}", num_value);
                            }
                            "y" => {
                                current_scenario.entity.pos_y = num_value;
                                velocity_y = 0.0; // Reset velocity so it drops from a standstill
                                println!("Y position updated to: {}", num_value);
                            }
                            _ => {
                                // The '_' acts as a catch-all for any unknown words
                                println!("Error: Unknown target '{}'. Try 'gravity', 'radius', 'x', or 'y'.", target);
                            }
                        }
                    } else {
                        println!("Error: '{}' is not a valid number.", value);
                    }
                } else if command == "reset" {
                    // A quick command to reload the TOML file from scratch
                    let fresh_config = fs::read_to_string("scenario.toml").unwrap_or_default();
                    if let Ok(fresh_scenario) = toml::from_str::<Scenario>(&fresh_config) {
                        current_scenario = fresh_scenario;
                        velocity_y = 0.0;
                        println!("Scenario reset to TOML defaults.");
                    }
                } else {
                    println!("Error: Unknown command '{}'", command);
                }

        // --- DRAW PHASE ---
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::new(15, 15, 18, 255));

        // Draw the simulation elements using our dynamic scenario data
        d.draw_text(
            &format!("uclid physics engine : running | Gravity: {:.2}", current_scenario.environment.gravity), 
            20, 20, 20, 
            Color::DARKGRAY
        );

        // Draw the physical entity using the parsed TOML variables
        d.draw_circle(
            current_scenario.entity.pos_x as i32, 
            current_scenario.entity.pos_y as i32, 
            current_scenario.entity.radius, 
            Color::RAYWHITE
        );

        // Draw the drop-down console over the simulation if active
        if current_state == AppState::ConsoleOpen {
            d.draw_rectangle(0, 0, 1024, 300, Color::new(20, 20, 20, 230));
            d.draw_line(0, 300, 1024, 300, Color::GREEN);
            
            let prompt = "uclid/engine>";
            d.draw_text(prompt, 20, 260, 20, Color::GREEN);
            d.draw_text(&console_input, 160, 260, 20, Color::RAYWHITE);
        }
    }
}
