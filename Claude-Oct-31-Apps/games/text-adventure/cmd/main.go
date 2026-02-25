package main

import (
	"bufio"
	"flag"
	"fmt"
	"os"
	"strings"
	"textadventure/engine"
	"textadventure/gameformat"
)

var (
	Version   = "1.0.0"
	BuildTime = "unknown"
	GitCommit = "unknown"
)

func main() {
	debugMode := flag.Bool("debug", false, "Enable debug mode")
	versionFlag := flag.Bool("version", false, "Show version")
	helpFlag := flag.Bool("help", false, "Show help")

	flag.Parse()

	if *versionFlag {
		printVersion()
		return
	}

	if *helpFlag {
		printHelp()
		return
	}

	args := flag.Args()
	if len(args) == 0 {
		printUsage()
		return
	}

	gameFile := args[0]

	// Load game
	game, err := gameformat.LoadGameFromFile(gameFile)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error loading game: %v\n", err)
		os.Exit(1)
	}

	// Start game
	if err := game.Start(); err != nil {
		fmt.Fprintf(os.Stderr, "Error starting game: %v\n", err)
		os.Exit(1)
	}

	// Print opening message
	printGameHeader()

	// Main game loop
	runGameLoop(game, *debugMode)
}

func runGameLoop(game *engine.Game, debugMode bool) {
	reader := bufio.NewReader(os.Stdin)

	for !game.IsGameOver() {
		// Print room description and current state
		printRoomInfo(game, debugMode)

		// Get user input
		fmt.Print("\n> ")
		input, err := reader.ReadString('\n')
		if err != nil {
			break
		}

		input = strings.TrimSpace(input)

		// Handle special commands
		if input == "" {
			continue
		}

		if strings.EqualFold(input, "quit") || strings.EqualFold(input, "exit") {
			fmt.Println("\nThanks for playing!")
			break
		}

		if strings.EqualFold(input, "help") {
			printGameHelp()
			continue
		}

		if strings.EqualFold(input, "save") {
			fmt.Println("Save functionality not yet implemented.")
			continue
		}

		if strings.EqualFold(input, "load") {
			fmt.Println("Load functionality not yet implemented.")
			continue
		}

		// Execute command
		err = game.ExecuteCommand(input)
		if err != nil {
			// Check if it's a direction command with no exit
			if strings.Contains(err.Error(), "exit") || strings.Contains(err.Error(), "Exit") {
				fmt.Println("You can't go that way.")
			} else if strings.Contains(err.Error(), "not found") || strings.Contains(err.Error(), "Not found") {
				fmt.Println("You don't see that here.")
			} else if strings.Contains(err.Error(), "unknown command") {
				fmt.Printf("I don't understand '%s'. Type 'help' for commands.\n", input)
			} else {
				fmt.Printf("Error: %v\n", err)
			}
		} else {
			// Handle command-specific output
			handleCommandOutput(game, input)
		}
	}
}

func printRoomInfo(game *engine.Game, debugMode bool) {
	// Print room name
	fmt.Printf("\n%s\n", strings.ToUpper(game.GetCurrentRoomName()))
	fmt.Println(strings.Repeat("=", 50))

	// Print room description
	desc := game.GetCurrentRoomDescription()
	fmt.Println(desc)

	// Print items in room
	items := game.GetCurrentRoomItems()
	if len(items) > 0 {
		fmt.Println("\nYou can see:")
		for _, item := range items {
			if !item.Hidden {
				fmt.Printf("  - %s\n", item.Name)
			}
		}
	}

	// Print NPCs in room
	npcs := game.GetCurrentRoomNPCs()
	if len(npcs) > 0 {
		fmt.Println("\nPeople here:")
		for _, npc := range npcs {
			fmt.Printf("  - %s\n", npc.Name)
		}
	}

	// Print exits
	exits := game.GetCurrentRoomExits()
	if len(exits) > 0 {
		fmt.Print("\nExits: ")
		fmt.Println(strings.Join(exits, ", "))
	}

	// Debug info
	if debugMode {
		fmt.Printf("\n[DEBUG] Room: %s, Turn: %d, Score: %d\n",
			game.State.CurrentRoom, game.TurnCounter, game.State.GetScore())
	}
}

func handleCommandOutput(game *engine.Game, input string) {
	lower := strings.ToLower(strings.TrimSpace(input))

	switch {
	case strings.HasPrefix(lower, "look"):
		// Already printed
	case strings.HasPrefix(lower, "inventory") || lower == "i":
		fmt.Println(game.GetInventoryDisplay())
	case strings.HasPrefix(lower, "examine") || lower == "x":
		// Could print item description here
		parts := strings.Fields(input)
		if len(parts) > 1 {
			itemName := strings.Join(parts[1:], " ")
			fmt.Printf("You examine %s carefully.\n", itemName)
		}
	case strings.HasPrefix(lower, "take") || strings.HasPrefix(lower, "get"):
		parts := strings.Fields(input)
		if len(parts) > 1 {
			itemName := strings.Join(parts[1:], " ")
			fmt.Printf("You take %s.\n", itemName)
		}
	case strings.HasPrefix(lower, "drop"):
		parts := strings.Fields(input)
		if len(parts) > 1 {
			itemName := strings.Join(parts[1:], " ")
			fmt.Printf("You drop %s.\n", itemName)
		}
	case strings.HasPrefix(lower, "wait") || lower == "z":
		fmt.Println("Time passes...")
	}
}

func printGameHeader() {
	fmt.Println("\n" + strings.Repeat("=", 50))
	fmt.Println("    DUNGEON ESCAPE - A Text Adventure")
	fmt.Println(strings.Repeat("=", 50))
	fmt.Println("\nYou awaken in a cold, stone dungeon with no memory")
	fmt.Println("of how you got here. Your only way out is to solve")
	fmt.Println("the mysteries of this place and escape!")
	fmt.Println("\nType 'help' for a list of commands.")
}

func printGameHelp() {
	fmt.Println("\n" + strings.Repeat("-", 50))
	fmt.Println("COMMAND HELP")
	fmt.Println(strings.Repeat("-", 50))
	fmt.Println("\nMovement:")
	fmt.Println("  north, south, east, west (or n, s, e, w)")
	fmt.Println("  up, down (or u, d)")
	fmt.Println("\nActions:")
	fmt.Println("  look / l              - Look around the room")
	fmt.Println("  examine <item> / x    - Examine something closely")
	fmt.Println("  take <item>           - Pick up an item")
	fmt.Println("  drop <item>           - Drop an item")
	fmt.Println("  use <item> on <obj>   - Use one item on another")
	fmt.Println("  open <container>      - Open a container")
	fmt.Println("  close <container>     - Close a container")
	fmt.Println("  talk to <npc>         - Talk to someone")
	fmt.Println("\nInventory:")
	fmt.Println("  inventory / i         - List your items")
	fmt.Println("  put <item> in <obj>   - Put item into container")
	fmt.Println("\nSystem:")
	fmt.Println("  wait / z              - Wait a turn")
	fmt.Println("  save <name>           - Save your progress")
	fmt.Println("  load <name>           - Load a saved game")
	fmt.Println("  help                  - Show this help")
	fmt.Println("  quit / exit           - Quit the game")
	fmt.Println(strings.Repeat("-", 50) + "\n")
}

func printUsage() {
	fmt.Fprintf(os.Stderr, "Usage: %s [options] <game-file>\n", os.Args[0])
	fmt.Fprintf(os.Stderr, "\nOptions:\n")
	fmt.Fprintf(os.Stderr, "  -debug     Enable debug mode\n")
	fmt.Fprintf(os.Stderr, "  -version   Show version\n")
	fmt.Fprintf(os.Stderr, "  -help      Show this help\n")
	fmt.Fprintf(os.Stderr, "\nExample: %s games/dungeon-escape/dungeon-escape.json\n", os.Args[0])
}

func printVersion() {
	fmt.Printf("Text Adventure Engine v%s\n", Version)
	fmt.Printf("Built: %s\n", BuildTime)
	fmt.Printf("Commit: %s\n", GitCommit)
}
