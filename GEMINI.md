# GEMINI.md

This file provides a comprehensive overview of the "Flappy Bird" clone project, intended to be used as a context for future AI-assisted development.

## Project Overview

This project is a clone of the popular game "Flappy Bird," built using the [Bevy game engine](https://bevyengine.org/) in Rust. The game features a bird that the player controls by making it "jump" to navigate through a series of pipes.

## Building and Running

To build and run the project, use the following `cargo` commands:

```bash
# Build the project
cargo build

# Run the project
cargo run
```

## Project Structure

The project is organized into several modules, each with a specific responsibility:

-   `src/main.rs`: The main entry point of the application. It initializes the Bevy app, sets up the game window, and adds the main `GamePlugin`.
-   `src/states`: Defines the different game states, which control the flow of the game (`Menu`, `Playing`, `Paused`, `GameOver`).
-   `src/core`: The core module of the game. It contains the `GamePlugin`, which brings together all the other plugins and systems. It also defines core resources and systems that are used throughout the game.
-   `src/plugins`: This module contains the core gameplay logic, separated into plugins for different aspects of the game:
    -   `asset_loader.rs`: Responsible for loading game assets like images and fonts.
    -   `bird.rs`: Handles the bird's movement, jumping, and spawning/despawning.
    -   `pipes.rs`: Manages the spawning, movement, and collision detection of the pipes, as well as the scoring system.
-   `assets`: This directory contains all the game's assets, such as images and fonts.

## Development Conventions

The project follows the standard conventions of a Bevy application, which is based on the Entity-Component-System (ECS) architecture. The game logic is organized into plugins, systems, and states, which allows for a modular and scalable codebase.

-   **Plugins:** The game is divided into several plugins, each responsible for a specific feature (e.g., `BirdPlugin`, `PipesPlugin`).
-   **Systems:** Systems are functions that run on entities with specific components. They contain the game's logic (e.g., `bird_movement`, `check_collisions`).
-   **States:** The game's flow is managed by a state machine, which transitions between different `GameStates` (e.g., from `Menu` to `Playing`).
