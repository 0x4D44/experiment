use maze_generator::*;
use std::io;

fn main() -> io::Result<()> {
    loop {
        display_menu();

        let choice = read_input()?;
        match choice.trim() {
            "1" => play_game()?,
            "2" => generate_and_display()?,
            "3" => show_algorithms(),
            "4" => {
                println!("Thanks for playing! Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please try again.\n"),
        }
    }

    Ok(())
}

fn display_menu() {
    println!("\n╔════════════════════════════════════╗");
    println!("║     MAZE GENERATOR - MAIN MENU     ║");
    println!("╠════════════════════════════════════╣");
    println!("║ 1. Play Interactive Maze           ║");
    println!("║ 2. Generate and Display Maze       ║");
    println!("║ 3. Algorithm Information           ║");
    println!("║ 4. Exit                            ║");
    println!("╚════════════════════════════════════╝");
    print!("\nSelect option (1-4): ");
    use std::io::Write;
    io::stdout().flush().unwrap();
}

fn read_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}

fn select_difficulty() -> (usize, usize) {
    loop {
        println!("\nSelect Difficulty:");
        println!("1. Tiny (10x10)");
        println!("2. Small (25x25)");
        println!("3. Medium (50x50)");
        println!("4. Large (100x100)");
        println!("5. Huge (250x250)");
        println!("6. Custom Size");

        print!("Choice (1-6): ");
        use std::io::Write;
        io::stdout().flush().unwrap();

        if let Ok(choice) = read_input() {
            match choice.trim() {
                "1" => return (10, 10),
                "2" => return (25, 25),
                "3" => return (50, 50),
                "4" => return (100, 100),
                "5" => return (250, 250),
                "6" => {
                    print!("Enter width: ");
                    io::stdout().flush().unwrap();
                    if let Ok(w) = read_input() {
                        if let Ok(width) = w.trim().parse::<usize>() {
                            print!("Enter height: ");
                            io::stdout().flush().unwrap();
                            if let Ok(h) = read_input() {
                                if let Ok(height) = h.trim().parse::<usize>() {
                                    return (width, height);
                                }
                            }
                        }
                    }
                    println!("Invalid input. Please try again.");
                }
                _ => println!("Invalid choice."),
            }
        }
    }
}

fn select_algorithm() -> Algorithm {
    loop {
        println!("\nSelect Algorithm:");
        println!("1. Recursive Backtracker (winding)");
        println!("2. Kruskal's (uniform)");
        println!("3. Prim's (clustered)");
        println!("4. Binary Tree (fast)");
        println!("5. Aldous-Broder (unbiased)");
        println!("6. Wilson's (loop-erased)");
        println!("7. Random Algorithm");

        print!("Choice (1-7): ");
        use std::io::Write;
        io::stdout().flush().unwrap();

        if let Ok(choice) = read_input() {
            match choice.trim() {
                "1" => return Algorithm::RecursiveBacktracker,
                "2" => return Algorithm::Kruskal,
                "3" => return Algorithm::Prim,
                "4" => return Algorithm::BinaryTree,
                "5" => return Algorithm::AldousBroder,
                "6" => return Algorithm::Wilson,
                "7" => {
                    use rand::Rng;
                    let mut rng = rand::thread_rng();
                    let algorithms = vec![
                        Algorithm::RecursiveBacktracker,
                        Algorithm::Kruskal,
                        Algorithm::Prim,
                        Algorithm::BinaryTree,
                    ];
                    return algorithms[rng.gen_range(0..algorithms.len())];
                }
                _ => println!("Invalid choice."),
            }
        }
    }
}

fn play_game() -> io::Result<()> {
    let (width, height) = select_difficulty();
    let algorithm = select_algorithm();

    println!("\nGenerating {}x{} maze using {}...", width, height, algorithm);

    let seed = rand::random::<u64>();
    let mut generator: Box<dyn MazeGenerator> = match algorithm {
        Algorithm::RecursiveBacktracker => Box::new(generator::RecursiveBacktracker::new(width, height, seed)),
        Algorithm::Kruskal => Box::new(generator::Kruskal::new(width, height, seed)),
        Algorithm::Prim => Box::new(generator::Prim::new(width, height, seed)),
        Algorithm::BinaryTree => Box::new(generator::BinaryTree::new(width, height, seed)),
        Algorithm::AldousBroder => Box::new(generator::AldousBroder::new(width, height, seed)),
        Algorithm::Wilson => Box::new(generator::Wilson::new(width, height, seed)),
    };

    let maze = generator.generate();
    println!("Maze generated! Seed: {}\n", seed);

    let mut game = GameState::new(maze);

    loop {
        game.update_stats();

        // Clear screen
        print!("\x1B[2J\x1B[H");

        // Display maze
        let rendered = renderer::Renderer::render_gameplay(&game.maze, game.player_pos, &game.visited_cells);
        println!("{}", rendered);

        // Display status
        let status = renderer::Renderer::status_line(
            game.maze.width,
            game.maze.height,
            game.stats.steps_taken,
            game.elapsed_seconds(),
            game.stats.hints_used,
        );
        println!("{}", status);

        // Display solution if revealed
        if game.solution_revealed {
            println!("\n(Solution shown - marked with *)");
        }

        // Display controls
        println!("\n{}", renderer::Renderer::controls_line());

        if game.won {
            println!("\n{}", game.get_summary());
            println!("Press any key to continue...");
            let _ = read_input();
            break;
        }

        // Get input
        print!("Move (↑↓←→ or WASD, or H/S/R/Q): ");
        use std::io::Write;
        io::stdout().flush().unwrap();

        if let Ok(input) = read_input() {
            let cmd = input.trim().to_lowercase();

            if cmd.is_empty() {
                continue;
            }

            let first_char = cmd.chars().next().unwrap_or(' ');

            match first_char {
                'w' | 'k' => { game.try_move(0, -1); } // North
                'a' | 'h' => { game.try_move(-1, 0); } // West (h conflicts with hint!)
                's' | 'j' => { game.try_move(0, 1); } // South
                'd' | 'l' => { game.try_move(1, 0); } // East
                'u' => {
                    // Full 'h' command for hint - check full input
                    if cmd == "hint" || cmd == "h" {
                        if let Some(next) = game.get_hint() {
                            println!("Hint: Move towards ({}, {})", next.0, next.1);
                        } else {
                            println!("No solution available!");
                        }
                        let _ = read_input();
                    }
                }
                'r' => {
                    println!("Resetting maze...");
                    game.reset();
                }
                'q' => {
                    println!("Quitting game. Thanks for playing!");
                    break;
                }
                _ => {
                    // Handle full commands
                    match cmd.as_str() {
                        "hint" | "h" => {
                            if let Some(next) = game.get_hint() {
                                println!("Hint: Move towards ({}, {})", next.0, next.1);
                            } else {
                                println!("No solution available!");
                            }
                            let _ = read_input();
                        }
                        "show" | "s" => {
                            println!("Showing solution...");
                            game.reveal_solution();
                        }
                        "reset" | "r" => {
                            println!("Resetting maze...");
                            game.reset();
                        }
                        "quit" | "q" => {
                            println!("Quitting game. Thanks for playing!");
                            break;
                        }
                        "b" => {
                            game.toggle_breadcrumbs();
                            println!("Breadcrumbs: {}", if game.show_breadcrumbs { "ON" } else { "OFF" });
                            let _ = read_input();
                        }
                        _ => {
                            println!("Unknown command. Use WASD to move, or H/S/R/Q for actions.");
                            let _ = read_input();
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn generate_and_display() -> io::Result<()> {
    let (width, height) = select_difficulty();
    let algorithm = select_algorithm();

    println!("\nGenerating {}x{} maze using {}...", width, height, algorithm);

    let seed = rand::random::<u64>();
    let mut generator: Box<dyn MazeGenerator> = match algorithm {
        Algorithm::RecursiveBacktracker => Box::new(generator::RecursiveBacktracker::new(width, height, seed)),
        Algorithm::Kruskal => Box::new(generator::Kruskal::new(width, height, seed)),
        Algorithm::Prim => Box::new(generator::Prim::new(width, height, seed)),
        Algorithm::BinaryTree => Box::new(generator::BinaryTree::new(width, height, seed)),
        Algorithm::AldousBroder => Box::new(generator::AldousBroder::new(width, height, seed)),
        Algorithm::Wilson => Box::new(generator::Wilson::new(width, height, seed)),
    };

    let maze = generator.generate();

    println!("\nMaze generated! Seed: {}\n", seed);

    let rendered = renderer::Renderer::render_simple(&maze);
    println!("{}", rendered);

    // Find and display solution
    if let Some(solution) = pathfinder::Pathfinder::bfs(&maze) {
        println!("Solution found: {} steps\n", solution.len());

        let rendered_with_solution = renderer::Renderer::render_with_solution(&maze, &solution);
        println!("Maze with solution (marked with *):\n");
        println!("{}", rendered_with_solution);

        println!("Maze Statistics:");
        println!("  Width: {}", width);
        println!("  Height: {}", height);
        println!("  Total Cells: {}", width * height);
        println!("  Algorithm: {}", algorithm);
        println!("  Seed: {}", seed);
        println!("  Solution Length: {}", solution.len());
    } else {
        println!("ERROR: No solution found!");
    }

    println!("\nPress any key to continue...");
    let _ = read_input();

    Ok(())
}

fn show_algorithms() {
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║           MAZE GENERATION ALGORITHMS                       ║");
    println!("╠════════════════════════════════════════════════════════════╣");

    println!("║ 1. RECURSIVE BACKTRACKER                                   ║");
    println!("║    - Type: Depth-First Search (DFS)                        ║");
    println!("║    - Creates long, winding passages                        ║");
    println!("║    - Time: O(n*m), Space: O(n*m)                           ║");
    println!("║    - Generation Speed: ~50-100μs per cell                  ║");
    println!("║    - Best for: Interesting visual mazes                    ║");

    println!("║                                                            ║");
    println!("║ 2. KRUSKAL'S ALGORITHM                                     ║");
    println!("║    - Type: Randomized Minimum Spanning Tree (MST)          ║");
    println!("║    - Uniform random maze generation                        ║");
    println!("║    - Time: O(n*m * α(n*m)), Space: O(n*m)                  ║");
    println!("║    - Generation Speed: ~30-60μs per cell                   ║");
    println!("║    - Best for: Well-balanced mazes                         ║");

    println!("║                                                            ║");
    println!("║ 3. PRIM'S ALGORITHM                                        ║");
    println!("║    - Type: Randomized MST with frontier expansion          ║");
    println!("║    - Clustered passage patterns                            ║");
    println!("║    - Time: O(n*m * log(n*m)), Space: O(n*m)                ║");
    println!("║    - Generation Speed: ~40-80μs per cell                   ║");
    println!("║    - Best for: Natural-looking passages                    ║");

    println!("║                                                            ║");
    println!("║ 4. BINARY TREE                                             ║");
    println!("║    - Type: Directional bias algorithm                      ║");
    println!("║    - Very fast generation                                  ║");
    println!("║    - Time: O(n*m), Space: O(1)                             ║");
    println!("║    - Generation Speed: ~10-20μs per cell (fastest)         ║");
    println!("║    - Best for: Quick generation of large mazes             ║");

    println!("║                                                            ║");
    println!("║ 5. ALDOUS-BRODER                                           ║");
    println!("║    - Type: Random walk based                               ║");
    println!("║    - Unbiased uniform generation                           ║");
    println!("║    - Time: O(n²*m²) worst case, Space: O(n*m)              ║");
    println!("║    - Generation Speed: ~100-200μs per cell (slowest)       ║");
    println!("║    - Best for: Fair distribution                           ║");

    println!("║                                                            ║");
    println!("║ 6. WILSON'S ALGORITHM                                      ║");
    println!("║    - Type: Loop-erased random walk                         ║");
    println!("║    - Unbiased uniform generation                           ║");
    println!("║    - Time: O(n*m * log(n*m)) expected, Space: O(n*m)       ║");
    println!("║    - Generation Speed: ~50-100μs per cell                  ║");
    println!("║    - Best for: Unbiased generation with good speed         ║");

    println!("╚════════════════════════════════════════════════════════════╝");

    println!("\nAll algorithms generate guaranteed solvable mazes with");
    println!("no loops (spanning tree structure).");

    println!("\nPress any key to return to menu...");
    let _ = read_input();
}
