package engine

import (
	"errors"
	"regexp"
	"strings"
	"unicode/utf8"
)

// NewParser creates a new command parser
func NewParser() *Parser {
	p := &Parser{
		Aliases: make(map[string]string),
	}
	p.registerDefaultAliases()
	return p
}

// registerDefaultAliases sets up built-in command aliases
func (p *Parser) registerDefaultAliases() {
	// Examine aliases
	p.Aliases["x"] = "examine"
	p.Aliases["inspect"] = "examine"
	p.Aliases["look at"] = "examine"
	p.Aliases["check"] = "examine"

	// Inventory aliases
	p.Aliases["i"] = "inventory"
	p.Aliases["inv"] = "inventory"

	// Look aliases
	p.Aliases["l"] = "look"

	// Take aliases
	p.Aliases["get"] = "take"
	p.Aliases["pick up"] = "take"
	p.Aliases["grab"] = "take"
	p.Aliases["pick"] = "take"

	// Drop aliases
	p.Aliases["place"] = "drop"
	p.Aliases["put down"] = "drop"

	// Direction aliases
	p.Aliases["n"] = "north"
	p.Aliases["s"] = "south"
	p.Aliases["e"] = "east"
	p.Aliases["w"] = "west"
	p.Aliases["u"] = "up"
	p.Aliases["d"] = "down"

	// System commands
	p.Aliases["quit"] = "quit"
	p.Aliases["exit"] = "quit"
	p.Aliases["help"] = "help"
	p.Aliases["?"] = "help"
	p.Aliases["wait"] = "wait"
	p.Aliases["z"] = "wait"

	// Talk aliases
	p.Aliases["talk"] = "talk"
	p.Aliases["converse"] = "talk"
	p.Aliases["say"] = "talk"

	// Use aliases
	p.Aliases["use"] = "use"
	p.Aliases["apply"] = "use"

	// Open/Close
	p.Aliases["open"] = "open"
	p.Aliases["close"] = "close"
}

// Parse converts a string input into a Command
func (p *Parser) Parse(input string) (*Command, error) {
	// Trim and validate input
	input = strings.TrimSpace(input)
	if input == "" {
		return nil, errors.New("empty command")
	}

	// Convert to lowercase for matching
	lower := strings.ToLower(input)

	// Parse the command
	cmd := &Command{Raw: input}

	// Try direction patterns first
	if direction := p.parseDirection(lower); direction != "" {
		cmd.Action = direction
		cmd.Object = direction
		return cmd, nil
	}

	// Split into words
	words := strings.Fields(lower)

	// Single word commands
	if len(words) == 1 {
		word := words[0]

		// Check for aliases
		if alias, ok := p.Aliases[word]; ok {
			cmd.Action = alias
			return cmd, nil
		}

		// Check for system commands
		switch word {
		case "look", "l":
			cmd.Action = "look"
			return cmd, nil
		case "inventory", "i", "inv":
			cmd.Action = "inventory"
			return cmd, nil
		case "help", "?":
			cmd.Action = "help"
			return cmd, nil
		case "wait", "z":
			cmd.Action = "wait"
			return cmd, nil
		}

		return nil, errors.New("unknown command: " + input)
	}

	// Multi-word commands
	verb := words[0]

	// Expand aliases
	if alias, ok := p.Aliases[verb]; ok {
		verb = alias
	}

	cmd.Action = verb

	switch verb {
	case "north", "south", "east", "west", "up", "down":
		cmd.Action = verb
		cmd.Object = verb
		return cmd, nil

	case "take", "get", "grab", "pick":
		cmd.Action = "take"
		cmd.Object = p.extractNoun(words[1:])
		return cmd, nil

	case "drop", "place", "put":
		cmd.Action = "drop"
		cmd.Object = p.extractNoun(words[1:])
		return cmd, nil

	case "examine", "x", "inspect", "look":
		cmd.Action = "examine"
		if len(words) > 1 {
			cmd.Object = p.extractNoun(words[1:])
		}
		return cmd, nil

	case "use", "apply":
		cmd.Action = "use"
		if len(words) >= 3 {
			// "use key on door" or "use key in lock"
			preposition := ""
			objEndIdx := 1
			for i := 1; i < len(words); i++ {
				if words[i] == "on" || words[i] == "in" || words[i] == "with" {
					preposition = words[i]
					objEndIdx = i
					break
				}
			}
			cmd.Object = p.extractNoun(words[1:objEndIdx])
			if preposition != "" {
				cmd.Target = p.extractNoun(words[objEndIdx+1:])
			}
		} else {
			cmd.Object = p.extractNoun(words[1:])
		}
		return cmd, nil

	case "put":
		cmd.Action = "put"
		if len(words) >= 3 {
			// "put key in chest"
			objEndIdx := 1
			for i := 1; i < len(words); i++ {
				if words[i] == "in" || words[i] == "on" {
					objEndIdx = i
					break
				}
			}
			cmd.Object = p.extractNoun(words[1:objEndIdx])
			cmd.Target = p.extractNoun(words[objEndIdx+1:])
		}
		return cmd, nil

	case "talk", "converse", "ask":
		cmd.Action = "talk"
		// "talk to guard" or "ask guard about princess"
		if len(words) >= 2 {
			if words[1] == "to" && len(words) >= 3 {
				cmd.Object = words[2]
				if len(words) > 3 && words[3] == "about" {
					cmd.Target = p.extractNoun(words[4:])
				}
			} else {
				cmd.Object = words[1]
				if len(words) > 2 && words[2] == "about" {
					cmd.Target = p.extractNoun(words[3:])
				}
			}
		}
		return cmd, nil

	case "open", "close":
		cmd.Action = verb
		cmd.Object = p.extractNoun(words[1:])
		return cmd, nil

	case "save", "load":
		cmd.Action = verb
		if len(words) > 1 {
			cmd.Object = strings.Join(words[1:], " ")
		}
		return cmd, nil

	case "quit", "exit":
		cmd.Action = "quit"
		return cmd, nil

	default:
		return nil, errors.New("unknown command: " + input)
	}
}

// ParseWithContext parses input with additional game context
func (p *Parser) ParseWithContext(input string, ctx *ParseContext) (*Command, error) {
	cmd, err := p.Parse(input)
	if err != nil {
		// Could implement fuzzy matching here
		return nil, err
	}
	return cmd, nil
}

// parseDirection extracts direction from input
func (p *Parser) parseDirection(input string) string {
	directions := map[string]string{
		"north": "north", "n": "north",
		"south": "south", "s": "south",
		"east": "east", "e": "east",
		"west": "west", "w": "west",
		"up": "up", "u": "up",
		"down": "down", "d": "down",
	}

	if dir, ok := directions[input]; ok {
		return dir
	}
	return ""
}

// extractNoun extracts the main noun from a word list, removing articles
func (p *Parser) extractNoun(words []string) string {
	articles := map[string]bool{
		"a": true, "an": true, "the": true,
	}

	// Remove leading articles
	start := 0
	for start < len(words) && articles[words[start]] {
		start++
	}

	if start >= len(words) {
		return ""
	}

	// Join remaining words
	noun := strings.Join(words[start:], " ")
	return strings.TrimSpace(noun)
}

// LevenshteinDistance calculates edit distance between two strings
func LevenshteinDistance(a, b string) int {
	if len(a) == 0 {
		return len(b)
	}
	if len(b) == 0 {
		return len(a)
	}

	if len(a) > len(b) {
		a, b = b, a
	}

	previous := make([]int, len(a)+1)
	for i := 0; i <= len(a); i++ {
		previous[i] = i
	}

	for j := 1; j <= len(b); j++ {
		current := make([]int, len(a)+1)
		current[0] = j

		for i := 1; i <= len(a); i++ {
			insertions := previous[i] + 1
			deletions := current[i-1] + 1
			substitutions := previous[i-1]
			if a[i-1] != b[j-1] {
				substitutions++
			}

			current[i] = min(insertions, min(deletions, substitutions))
		}
		previous = current
	}

	return previous[len(a)]
}

// FuzzyMatch finds best matching string from candidates
func FuzzyMatch(input string, candidates []string, maxDistance int) (string, int) {
	bestMatch := ""
	bestDistance := maxDistance + 1

	for _, candidate := range candidates {
		distance := LevenshteinDistance(input, candidate)
		if distance < bestDistance {
			bestDistance = distance
			bestMatch = candidate
		}
	}

	if bestDistance > maxDistance {
		return "", -1
	}
	return bestMatch, bestDistance
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

// NormalizeInput removes extra whitespace and normalizes text
func NormalizeInput(input string) string {
	// Replace multiple spaces with single space
	re := regexp.MustCompile(`\s+`)
	normalized := re.ReplaceAllString(strings.TrimSpace(input), " ")
	return strings.ToLower(normalized)
}

// ValidateCommand checks if command structure is valid
func (p *Parser) ValidateCommand(cmd *Command) error {
	if cmd == nil {
		return errors.New("command is nil")
	}
	if cmd.Action == "" {
		return errors.New("command action is empty")
	}
	return nil
}

// GetCommandSuggestions returns command suggestions for partial input
func (p *Parser) GetCommandSuggestions(input string) []string {
	suggestions := []string{}
	commands := []string{
		"take", "drop", "examine", "look", "inventory",
		"north", "south", "east", "west", "up", "down",
		"use", "talk", "open", "close", "save", "load",
		"help", "quit", "wait",
	}

	for _, cmd := range commands {
		if strings.HasPrefix(cmd, input) {
			suggestions = append(suggestions, cmd)
		}
	}

	return suggestions
}

// IsValidCommandCharacter checks if a character is valid in commands
func IsValidCommandCharacter(r rune) bool {
	return (r >= 'a' && r <= 'z') ||
		(r >= 'A' && r <= 'Z') ||
		(r >= '0' && r <= '9') ||
		r == ' ' || r == '-' || r == '\''
}

// ValidateCommandInput checks if input contains only valid characters
func ValidateCommandInput(input string) bool {
	for _, r := range input {
		if !IsValidCommandCharacter(r) {
			return false
		}
	}
	return utf8.ValidateString(input)
}
