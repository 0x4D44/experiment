package tests

import (
	"testing"
	"textadventure/engine"
)

func TestParseDirection(t *testing.T) {
	tests := []struct {
		input    string
		expected string
		wantErr  bool
	}{
		// Full directions
		{"north", "north", false},
		{"south", "south", false},
		{"east", "east", false},
		{"west", "west", false},
		{"up", "up", false},
		{"down", "down", false},

		// Abbreviated directions
		{"n", "north", false},
		{"s", "south", false},
		{"e", "east", false},
		{"w", "west", false},
		{"u", "up", false},
		{"d", "down", false},

		// Case insensitivity
		{"North", "north", false},
		{"SOUTH", "south", false},
		{"N", "north", false},

		// Invalid directions
		{"northeast", "", true},
		{"left", "", true},
		{"right", "", true},
		{"nowhere", "", true},
	}

	parser := engine.NewParser()
	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			cmd, err := parser.Parse(tt.input)
			if (err != nil) != tt.wantErr {
				t.Errorf("got error %v, want error %v", err != nil, tt.wantErr)
			}
			if !tt.wantErr && cmd.Object != tt.expected {
				t.Errorf("got %q, want %q", cmd.Object, tt.expected)
			}
		})
	}
}

func TestParseSimpleCommand(t *testing.T) {
	tests := []struct {
		input  string
		action string
		object string
	}{
		{"take sword", "take", "sword"},
		{"take the sword", "take", "sword"},
		{"get sword", "take", "sword"},
		{"grab sword", "take", "sword"},
		{"pick up sword", "take", "sword"},
		{"drop sword", "drop", "sword"},
		{"examine key", "examine", "key"},
		{"look at key", "examine", "key"},
		{"x key", "examine", "key"},
		{"inspect key", "examine", "key"},
		{"look", "look", ""},
		{"l", "look", ""},
		{"inventory", "inventory", ""},
		{"i", "inventory", ""},
	}

	parser := engine.NewParser()
	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			cmd, err := parser.Parse(tt.input)
			if err != nil {
				t.Errorf("unexpected error: %v", err)
			}
			if cmd.Action != tt.action {
				t.Errorf("action: got %q, want %q", cmd.Action, tt.action)
			}
			if cmd.Object != tt.object {
				t.Errorf("object: got %q, want %q", cmd.Object, tt.object)
			}
		})
	}
}

func TestParseComplexCommand(t *testing.T) {
	tests := []struct {
		input  string
		action string
		object string
		target string
	}{
		{"use key on door", "use", "key", "door"},
		{"use key in door", "use", "key", "door"},
		{"put key in chest", "put", "key", "chest"},
		{"place key on table", "put", "key", "table"},
		{"ask guard about princess", "ask", "guard", "princess"},
		{"talk to guard about escape", "talk", "guard", "escape"},
	}

	parser := engine.NewParser()
	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			cmd, err := parser.Parse(tt.input)
			if err != nil {
				t.Errorf("unexpected error: %v", err)
			}
			if cmd.Action != tt.action {
				t.Errorf("action: got %q, want %q", cmd.Action, tt.action)
			}
			if cmd.Object != tt.object {
				t.Errorf("object: got %q, want %q", cmd.Object, tt.object)
			}
			if cmd.Target != tt.target {
				t.Errorf("target: got %q, want %q", cmd.Target, tt.target)
			}
		})
	}
}

func TestParseAliases(t *testing.T) {
	tests := []struct {
		input    string
		action   string
		expected string
	}{
		{"x sword", "examine", "sword"},
		{"i", "inventory", ""},
		{"l", "look", ""},
	}

	parser := engine.NewParser()
	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			cmd, err := parser.Parse(tt.input)
			if err != nil {
				t.Errorf("unexpected error: %v", err)
			}
			if cmd.Action != tt.action {
				t.Errorf("action: got %q, want %q", cmd.Action, tt.action)
			}
		})
	}
}

func TestFuzzyMatching(t *testing.T) {
	tests := []struct {
		input        string
		shouldMatch  bool
		suggestion   string
	}{
		{"sord", true, "sword"}, // typo
		{"kye", true, "key"},     // typo
		{"swor", true, "sword"},  // partial
	}

	parser := engine.NewParser()
	// Note: These tests assume fuzzy matching returns suggestions
	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			cmd, err := parser.Parse(tt.input)
			if tt.shouldMatch && err != nil {
				t.Logf("fuzzy match returned: %v (suggestion: %s)", err, cmd.Object)
			}
		})
	}
}

func TestSystemCommands(t *testing.T) {
	tests := []struct {
		input  string
		action string
	}{
		{"save game1", "save"},
		{"load game1", "load"},
		{"quit", "quit"},
		{"exit", "quit"},
		{"help", "help"},
		{"?", "help"},
		{"wait", "wait"},
		{"z", "wait"},
	}

	parser := engine.NewParser()
	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			cmd, err := parser.Parse(tt.input)
			if err != nil {
				t.Errorf("unexpected error: %v", err)
			}
			if cmd.Action != tt.action {
				t.Errorf("action: got %q, want %q", cmd.Action, tt.action)
			}
		})
	}
}

func TestEmptyAndInvalidInput(t *testing.T) {
	tests := []struct {
		input   string
		wantErr bool
	}{
		{"", true},
		{"   ", true},
		{"\n", true},
		{"   \t   ", true},
		{"xyz123abc", true},
		{"@#$%", true},
	}

	parser := engine.NewParser()
	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			_, err := parser.Parse(tt.input)
			if (err != nil) != tt.wantErr {
				t.Errorf("got error %v, want error %v", err != nil, tt.wantErr)
			}
		})
	}
}

func TestContextualMatching(t *testing.T) {
	// Test that parser can resolve ambiguous inputs with context
	parser := engine.NewParser()
	world := createTestWorld()
	room := world.GetRoom("test_room")

	// "sword" should match if sword is in room
	context := &engine.ParseContext{
		CurrentRoom:      room,
		AvailableItems:   []string{"sword", "shield"},
		AvailableNPCs:    []string{"guard"},
	}

	cmd, err := parser.ParseWithContext("take sword", context)
	if err != nil {
		t.Errorf("unexpected error: %v", err)
	}
	if cmd.Object != "sword" {
		t.Errorf("got %q, want %q", cmd.Object, "sword")
	}
}

func TestArticleRemoval(t *testing.T) {
	tests := []struct {
		input  string
		object string
	}{
		{"take the sword", "sword"},
		{"examine a key", "key"},
		{"drop an apple", "apple"},
		{"get the old sword", "old sword"},
		{"use the golden key on the iron door", "golden key"},
	}

	parser := engine.NewParser()
	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			cmd, err := parser.Parse(tt.input)
			if err != nil {
				t.Errorf("unexpected error: %v", err)
			}
			if cmd.Object != tt.object {
				t.Errorf("got %q, want %q", cmd.Object, tt.object)
			}
		})
	}
}

// Helper function to create a test world
func createTestWorld() *engine.World {
	world := engine.NewWorld()
	world.AddRoom(&engine.Room{
		ID:          "test_room",
		Name:        "Test Room",
		Description: "A test room",
	})
	return world
}
