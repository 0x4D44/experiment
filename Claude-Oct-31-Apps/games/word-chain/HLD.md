# Word Chain Game - High Level Design (HLD)

## 1. Game Architecture Overview

### 1.1 Core Concept
Word Chain is a turn-based word game where players alternate adding words to a chain. Each new word must start with the last letter of the previous word. The game ends when a player cannot provide a valid word.

### 1.2 System Architecture

```
┌─────────────────────────────────────────────────────────┐
│                   Game Controller                        │
│  (Orchestrates game flow, turn management, validation)   │
└──────────┬────────────────────────────────────┬──────────┘
           │                                    │
      ┌────▼────┐                    ┌─────────▼────────┐
      │Dictionary│                    │AI Engine (4 Lvls)│
      │Manager   │                    │  - Decision Tree  │
      │(50k+ words)                   │  - Strategy Eval  │
      └────┬────┘                    │  - Difficulty     │
           │                        │    Scaling        │
      ┌────▼──────────┐             └─────────┬────────┘
      │Validation Layer│                       │
      │ - Trie/HashMap │         ┌─────────────▼───────┐
      │ - Lookup <1ms  │         │ Scoring System       │
      │ - Suggestions  │         │ - Base Points        │
      └────────────────┘         │ - Rarity Bonus       │
                                 │ - Combo Multiplier   │
                                 │ - Difficulty Bonus   │
                                 └──────────────────────┘

      ┌──────────────────┐        ┌──────────────────┐
      │Game State        │        │UI/Renderer       │
      │ - Chain          │        │ - Chain Display  │
      │ - Used Words     │        │ - Statistics     │
      │ - Scores         │        │ - Hints System   │
      │ - Round Num      │        │ - Timer Display  │
      └──────────────────┘        └──────────────────┘
```

---

## 2. Data Structures

### 2.1 Dictionary Management System

#### Dictionary Loading Strategy
- **Format**: Compressed binary format + fallback to text format
- **Indexing**: Hybrid approach - Trie for prefix matching + HashMap for O(1) lookup
- **Memory Optimization**: Pre-computed letter-to-words mapping for AI decisions
- **Load Time Target**: <500ms for 50,000 words

```zig
// Core dictionary structure
const Dictionary = struct {
    words: std.ArrayList([]const u8),              // All words
    word_set: std.StringHashMap(WordMetadata),     // Fast lookup
    by_starting_letter: [26]std.ArrayList(usize),  // Letter index
    by_ending_letter: [26]std.ArrayList(usize),    // Letter index
    by_length: [50]std.ArrayList(usize),           // Length index
    categories: std.StringHashMap([]usize),        // Category index
    difficulty_map: std.ArrayList(u8),             // Difficulty per word

    allocator: std.mem.Allocator,
};

const WordMetadata = struct {
    index: usize,
    length: u8,
    difficulty: u8,        // 1-5 scale
    category: ?[]const u8,
    definition: ?[]const u8,
    frequency_rank: u32,   // Lower = more common
};
```

#### Dictionary File Format
```
HEADER (8 bytes):
  - Magic: "WORDCH01" (8 bytes)

METADATA (variable):
  - Word count: u32
  - Compressed size: u32
  - Last updated: u64

COMPRESSED WORD DATA:
  - Using zstd or similar for 60-70% compression
  - Each word: length byte + utf8 string

INDEX SECTION:
  - Starting letter indices (26 * 4 bytes)
  - Ending letter indices (26 * 4 bytes)
  - Length indices (50 * 4 bytes)
  - Difficulty values (word_count bytes)
```

### 2.2 Game State Structure

```zig
const GameState = struct {
    current_chain: std.ArrayList(WordEntry),
    used_words: std.StringHashMap(void),           // O(1) lookup
    current_round: u32,
    current_player_index: u8,
    scores: [4]u32,                                // Up to 4 players
    turn_time_limit_ms: u32,
    chain_start_letter: u8,                        // For current turn
    game_status: GameStatus,
    mode: GameMode,
    difficulty: Difficulty,
    elapsed_time_ms: u64,

    allocator: std.mem.Allocator,
};

const WordEntry = struct {
    word: []const u8,
    player_index: u8,
    added_at_ms: u64,
    points_earned: u32,
    definition: ?[]const u8,
};

const GameStatus = enum {
    waiting_for_input,
    validating,
    processing_ai,
    round_complete,
    game_over,
};

const GameMode = enum {
    classic,
    speed_chain,
    theme_chain,
    longest_chain,
    battle_royale,
    endless,
};

const Difficulty = enum {
    novice,
    intermediate,
    expert,
    master,
};
```

### 2.3 Player Structure

```zig
const Player = struct {
    name: []const u8,
    is_ai: bool,
    difficulty: Difficulty,
    personality: AIPersonality,
    current_score: u32,
    words_played: std.ArrayList([]const u8),
    win_count: u32,
    loss_count: u32,

    allocator: std.mem.Allocator,
};

const AIPersonality = enum {
    greedy,            // Longest word possible
    defensive,         // Words ending in difficult letters
    offensive,         // Force opponent into hard positions
    balanced,          // Mix of strategies
    learning,          // Adapt to player patterns
};
```

---

## 3. Dictionary Management System

### 3.1 Word Validation Algorithm

**Validation Pipeline**:
1. Input normalization (lowercase, trim whitespace)
2. Length check (1-50 characters)
3. Character validation (A-Z, apostrophes)
4. Trie prefix lookup (optional early rejection)
5. HashMap exact match (O(1))
6. Used word check (StringHashMap lookup)
7. Letter matching validation (first letter = previous last letter)

**Performance Target**: <1ms per validation

```zig
fn validateWord(
    self: *Dictionary,
    word: []const u8,
    expected_start_letter: u8,
    used_words: *const std.StringHashMap(void),
) !ValidationResult {
    // Step 1: Normalize
    var normalized: [50]u8 = undefined;
    var normalized_len = try normalizeWord(word, &normalized);

    // Step 2: Check used
    if (used_words.contains(normalized[0..normalized_len])) {
        return ValidationResult { .valid = false, .reason = "already_used" };
    }

    // Step 3: Check starting letter
    if (normalized[0] != expected_start_letter) {
        return ValidationResult { .valid = false, .reason = "wrong_letter" };
    }

    // Step 4: Lookup
    if (self.word_set.get(normalized[0..normalized_len])) |metadata| {
        return ValidationResult { .valid = true, .metadata = metadata };
    }

    return ValidationResult { .valid = false, .reason = "not_in_dictionary" };
}

const ValidationResult = struct {
    valid: bool,
    metadata: ?WordMetadata = null,
    reason: ?[]const u8 = null,
};
```

### 3.2 Dictionary Optimization Strategies

**Strategy 1: Prefix Indexing (Trie)**
- Used for: Letter frequency analysis, possible word suggestions
- Memory: ~2-3MB for 50k words
- Lookup: O(word_length)
- Benefit: Fast prefix matching for hints system

**Strategy 2: Letter-based Indexing**
```zig
// For quick "words ending in X" lookups
by_ending_letter: [26]std.ArrayList(usize),  // indices to words array

// Use case: When player plays "TIGER", instantly know all words starting with 'R'
// Complexity: O(number of words starting with R)
```

**Strategy 3: Frequency-based Caching**
- Top 500 most common words cached separately
- Used for: Novice AI decisions, faster lookup
- Memory: Negligible
- Benefit: 90% of common queries hit cache

**Strategy 4: Memory-mapped Loading (Optional)**
- For systems with large RAM
- Allows loading without decompression time
- Fallback to in-memory for safety

### 3.3 Category Filtering

```zig
const WordCategory = enum {
    animals,
    foods,
    colors,
    countries,
    professions,
    actions,
    emotions,
    objects,
    nature,
    science,
};

// Theme Chain mode uses category filtering
fn filterByCategory(
    self: *Dictionary,
    category: WordCategory,
) !std.ArrayList([]const u8) {
    if (self.categories.get(@tagName(category))) |word_indices| {
        var result = std.ArrayList([]const u8).init(self.allocator);
        for (word_indices) |idx| {
            try result.append(self.words.items[idx]);
        }
        return result;
    }
    return error.CategoryNotFound;
}
```

---

## 4. Word Validation & Chain Rules

### 4.1 Chain Rule Engine

```zig
const ChainRuleEngine = struct {
    dictionary: *Dictionary,
    used_words: std.StringHashMap(void),

    fn isValidMove(
        self: *ChainRuleEngine,
        new_word: []const u8,
        previous_word: []const u8,
    ) !bool {
        // Rule 1: Word exists in dictionary
        const metadata = self.dictionary.word_set.get(new_word) orelse
            return false;

        // Rule 2: First letter matches previous last letter
        if (new_word[0] != previous_word[previous_word.len - 1])
            return false;

        // Rule 3: Word not already used
        if (self.used_words.contains(new_word))
            return false;

        // Rule 4: Valid characters only
        for (new_word) |char| {
            if (!isValidChar(char)) return false;
        }

        return true;
    }

    fn getPossibleWords(
        self: *ChainRuleEngine,
        starting_letter: u8,
    ) !std.ArrayList([]const u8) {
        var result = std.ArrayList([]const u8).init(self.allocator);

        // Get all words starting with this letter
        var word_indices = self.dictionary.by_starting_letter[starting_letter - 'A'];

        for (word_indices.items) |idx| {
            const word = self.dictionary.words.items[idx];
            // Filter out used words
            if (!self.used_words.contains(word)) {
                try result.append(word);
            }
        }

        return result;
    }
};
```

### 4.2 Letter Frequency Analysis

```zig
const LetterStats = struct {
    frequency: [26]u32,
    most_common: u8,
    least_common: u8,
    entropy: f32,
    difficulty_letter: u8,  // Hardest letter to continue from

    fn analyze(words: []const []const u8) LetterStats {
        var stats: LetterStats = undefined;
        @memset(&stats.frequency, 0);

        for (words) |word| {
            if (word.len > 0) {
                const last_char = word[word.len - 1];
                if (last_char >= 'a' and last_char <= 'z') {
                    stats.frequency[last_char - 'a'] += 1;
                }
            }
        }

        // Find difficulty letter (fewest options)
        var min_count: u32 = std.math.maxInt(u32);
        for (stats.frequency, 0..) |count, i| {
            if (count > 0 and count < min_count) {
                min_count = count;
                stats.difficulty_letter = @intCast(i + 'a');
            }
        }

        return stats;
    }
};
```

---

## 5. AI Opponent Implementation

### 5.1 AI Architecture

```
┌──────────────────────────────────┐
│      AI Decision Engine          │
├──────────────────────────────────┤
│  1. Get possible words           │
│  2. Score each word (multi-factor)
│  3. Apply personality modifiers  │
│  4. Consider game state          │
│  5. Select optimal move          │
└──────────────────────────────────┘
```

### 5.2 Word Scoring Algorithm

```zig
const AIScorer = struct {
    dictionary: *Dictionary,
    game_state: *const GameState,
    difficulty: Difficulty,
    personality: AIPersonality,

    // Master scoring function
    fn scoreWord(
        self: *AIScorer,
        word: []const u8,
        possible_continuations: u32,
    ) f32 {
        var score: f32 = 0.0;

        // Factor 1: Word length (longer = higher difficulty for opponent)
        const length_score = @as(f32, @floatFromInt(word.len)) * 1.5;
        score += length_score;

        // Factor 2: Rarity (uncommon words are harder to counter)
        const metadata = self.dictionary.word_set.get(word).?;
        const rarity_score = rarity(metadata.frequency_rank) * 2.0;
        score += rarity_score;

        // Factor 3: Opponent difficulty (fewer continuations = harder)
        const continuation_score = @as(f32, @floatFromInt(possible_continuations));
        const difficulty_multiplier = 100.0 / continuation_score;
        score += difficulty_multiplier;

        // Factor 4: Personality modifier
        const personality_score = self.personalityModifier(word);
        score += personality_score;

        // Factor 5: Look-ahead (if expert/master)
        if (self.difficulty == .expert or self.difficulty == .master) {
            const lookahead_score = self.evaluateLookAhead(word);
            score += lookahead_score * 0.5;  // Weight is 50%
        }

        return score;
    }

    fn rarity(frequency_rank: u32) f32 {
        // Convert rank to rarity score (1-10)
        // Lower rank = more common = lower rarity
        if (frequency_rank < 1000) return 1.0;
        if (frequency_rank < 5000) return 2.0;
        if (frequency_rank < 10000) return 4.0;
        return 6.0;
    }

    fn personalityModifier(self: *AIScorer, word: []const u8) f32 {
        return switch (self.personality) {
            .greedy => self.greedyScore(word),
            .defensive => self.defensiveScore(word),
            .offensive => self.offensiveScore(word),
            .balanced => self.balancedScore(word),
            .learning => self.learningScore(word),
        };
    }
};
```

### 5.3 AI Difficulty Levels

**Level 1: Novice (1000 word subset)**
```zig
fn selectWord_Novice(ai: *AIScorer, possible_words: []const []const u8) []const u8 {
    var best_score: f32 = -std.math.inf(f32);
    var best_word_idx: usize = 0;

    // 90% pick best available, 10% random
    const should_random = (random.int(u32) % 100) < 10;

    for (possible_words, 0..) |word, idx| {
        // Only consider top 1000 common words
        const metadata = ai.dictionary.word_set.get(word).?;
        if (metadata.frequency_rank > 1000) continue;

        const score = ai.scoreWord(word, 0);  // No lookahead
        if (score > best_score or should_random) {
            best_score = score;
            best_word_idx = idx;
        }
    }

    return possible_words[best_word_idx];
}
```

**Level 2: Intermediate (5000 words, basic strategy)**
```zig
fn selectWord_Intermediate(ai: *AIScorer, possible_words: []const []const u8) []const u8 {
    // 70% strategic selection, 30% randomness
    const should_be_strategic = (random.int(u32) % 100) < 70;

    var best_score: f32 = -std.math.inf(f32);
    var best_word_idx: usize = 0;

    for (possible_words, 0..) |word, idx| {
        const metadata = ai.dictionary.word_set.get(word).?;
        if (metadata.frequency_rank > 5000) continue;

        const continuation_count = countPossibleContinuations(word);
        let score = if (should_be_strategic)
            ai.scoreWord(word, continuation_count)
        else
            random.float(f32);

        if (score > best_score) {
            best_score = score;
            best_word_idx = idx;
        }
    }

    return possible_words[best_word_idx];
}
```

**Level 3: Expert (20000 words, 1-move lookahead)**
```zig
fn selectWord_Expert(ai: *AIScorer, possible_words: []const []const u8) []const u8 {
    // 95% strategic, 5% randomness
    var best_score: f32 = -std.math.inf(f32);
    var best_word_idx: usize = 0;

    for (possible_words, 0..) |word, idx| {
        // Use full word set (20k words)
        const metadata = ai.dictionary.word_set.get(word).?;

        // Count exact continuations
        var continuation_count: u32 = 0;
        const last_letter = word[word.len - 1];
        for (ai.dictionary.by_starting_letter[last_letter - 'a'].items) |_| {
            continuation_count += 1;
        }

        var score = ai.scoreWord(word, continuation_count);

        // 1-move lookahead: evaluate best opponent response
        const opponent_best = ai.findBestOpponentResponse(word);
        score -= opponent_best * 0.3;  // Penalize if leads to good opponent move

        if (score > best_score) {
            best_score = score;
            best_word_idx = idx;
        }
    }

    return possible_words[best_word_idx];
}
```

**Level 4: Master (Full dictionary, optimal play)**
```zig
fn selectWord_Master(ai: *AIScorer, possible_words: []const []const u8) []const u8 {
    // 100% optimal, minimax-like algorithm
    var best_score: f32 = -std.math.inf(f32);
    var best_word_idx: usize = 0;

    for (possible_words, 0..) |word, idx| {
        // Minimax with depth 2
        var score = ai.scoreWord(word, 0);

        // 2-move lookahead
        const opponent_response = ai.findBestOpponentResponse(word);
        score -= opponent_response * 0.4;

        // Our best response to opponent
        const our_followup = ai.findBestFollowup(opponent_response);
        score += our_followup * 0.2;

        if (score > best_score) {
            best_score = score;
            best_word_idx = idx;
        }
    }

    return possible_words[best_word_idx];
}
```

### 5.4 AI Personality Implementations

**Greedy: Maximize word length**
```zig
fn greedyScore(self: *AIScorer, word: []const u8) f32 {
    return @as(f32, @floatFromInt(word.len)) * 5.0;
}
```

**Defensive: Avoid difficult letters for opponent**
```zig
fn defensiveScore(self: *AIScorer, word: []const u8) f32 {
    const last_letter = word[word.len - 1];
    const continuation_count = self.dictionary.by_starting_letter[last_letter - 'a'].items.len;

    // Score based on number of continuations (fewer = better for defense)
    return 100.0 / (@as(f32, @floatFromInt(continuation_count)) + 1.0);
}
```

**Offensive: Force opponent into hard positions**
```zig
fn offensiveScore(self: *AIScorer, word: []const u8) f32 {
    let last_letter = word[word.len - 1];
    let continuation_count = self.dictionary.by_starting_letter[last_letter - 'a'].items.len;

    if (continuation_count < 5) return 50.0;  // Very few options
    if (continuation_count < 20) return 25.0;  // Limited options
    return 1.0;  // Opponent has many options
}
```

**Balanced: Mix of strategies**
```zig
fn balancedScore(self: *AIScorer, word: []const u8) f32 {
    const greedy = self.greedyScore(word);
    const defensive = self.defensiveScore(word);
    const offensive = self.offensiveScore(word);

    return (greedy * 0.3) + (defensive * 0.3) + (offensive * 0.4);
}
```

**Learning: Adapt to player patterns**
```zig
const LearningAI = struct {
    player_patterns: std.StringHashMap(f32),  // word -> preference
    learning_rate: f32 = 0.1,

    fn learningScore(self: *AIScorer, word: []const u8) f32 {
        if (self.player_patterns.get(word)) |pattern_value| {
            // Avoid words player prefers
            return -pattern_value;
        }
        return 0.0;
    }

    fn observePlayerWord(self: *LearningAI, word: []const u8) void {
        if (self.player_patterns.get(word)) |existing| {
            self.player_patterns.put(word, existing + self.learning_rate) catch {};
        } else {
            self.player_patterns.put(word, self.learning_rate) catch {};
        }
    }
};
```

---

## 6. Scoring System

### 6.1 Point Calculation

```zig
const ScoringEngine = struct {
    fn calculateScore(
        word: []const u8,
        metadata: WordMetadata,
        difficulty: Difficulty,
        combo_multiplier: f32,
    ) u32 {
        // Base score: 10 points per character
        var base = word.len * 10;

        // Length bonus: +5 points per character over 5
        if (word.len > 5) {
            base += (word.len - 5) * 5;
        }

        // Rarity bonus: uncommon words earn more
        var rarity_bonus: u32 = 0;
        if (metadata.frequency_rank < 1000) rarity_bonus = 5;      // Common
        else if (metadata.frequency_rank < 5000) rarity_bonus = 15; // Uncommon
        else if (metadata.frequency_rank < 15000) rarity_bonus = 30; // Rare
        else rarity_bonus = 50;                                      // Very rare

        // Difficulty bonus: harder difficulties earn more
        var difficulty_bonus: u32 = switch (difficulty) {
            .novice => 1,
            .intermediate => 2,
            .expert => 3,
            .master => 5,
        };

        // Combo multiplier (for consecutive quick plays)
        var total = @as(u32, @intFromFloat(
            (@as(f32, @floatFromInt(base + rarity_bonus)) * combo_multiplier) +
            @as(f32, @floatFromInt(difficulty_bonus))
        ));

        return total;
    }
};
```

### 6.2 Combo System

```zig
const ComboTracker = struct {
    consecutive_fast_plays: u32 = 0,
    last_play_time_ms: u64 = 0,
    threshold_ms: u32 = 5000,  // 5 second threshold

    fn updateCombo(self: *ComboTracker, current_time_ms: u64) f32 {
        if (current_time_ms - self.last_play_time_ms < self.threshold_ms) {
            self.consecutive_fast_plays += 1;
        } else {
            self.consecutive_fast_plays = 1;
        }
        self.last_play_time_ms = current_time_ms;

        // Multiplier increases with combo
        return 1.0 + (@as(f32, @floatFromInt(self.consecutive_fast_plays)) * 0.1);
    }
};
```

---

## 7. Chain Visualization Design

### 7.1 Display Format

```
═══════════════════════════════════════════════════════════
                   WORD CHAIN - Round 5
═══════════════════════════════════════════════════════════
Score: You: 245 | AI (Expert): 210 | Time: 2:15

Current Chain (12 words):
START → TIGER → RABBIT → TURTLE → ENGINE →
ENERGY → YELLOW → WINDOW → WALRUS → SATURN →
NIGHT → THUNDER → ?

Last word: THUNDER (ends with 'R')
Your turn! Enter a word starting with 'R' (~245 options):

> river

Good choice! RIVER added to chain
Points: +25 (length: +10, rarity: +10, combo x1.2: +5)
Running combo: 2x

Letter Analysis:
Most used: E(8) R(6) T(5) N(4) A(3)
Least used: Q(0) X(0) Z(0) J(1) K(1)

Game Stats:
- Longest chain in history: 24 words
- Your average word length: 5.2 chars
- AI average word length: 6.1 chars
```

### 7.2 Visual Components

**Component 1: Chain Display**
- Shows last 8 words inline with arrows
- Older words shown as "... (4 more)"
- Color coding: Player words (green), AI words (red)
- Bold/underline for recently played words

**Component 2: Statistics Panel**
- Real-time letter frequency
- Remaining word count estimate
- Time elapsed
- Current scores with gap indicator

**Component 3: Hint System Display**
- Show when player types "?" to get hint
- Options: Count remaining words, first letter, strategic suggestion

---

## 8. Performance Optimization Strategy

### 8.1 Hot Path Optimization

**Hot Path 1: Word Validation (<1ms requirement)**
- Use HashMap for O(1) lookup
- Pre-compute all letter indices at startup
- Cache frequently validated words
- Avoid allocations in validation path

**Hot Path 2: AI Move Selection (<100ms for Master)**
- Pre-compute possible words during opponent's turn
- Use incremental scoring (don't recalculate everything)
- Limit lookahead depth for real-time responsiveness
- Cache frequently evaluated positions

**Hot Path 3: Render Update (<10ms)**
- Minimal terminal redraws (only changed lines)
- Use buffered output
- Avoid color code recalculation

### 8.2 Memory Management

```zig
// Two allocator strategy:
// 1. Arena allocator for game state (reset each round)
// 2. Static allocator for dictionary (persists)

const game_allocator = try allocator.create(std.heap.ArenaAllocator);
defer game_allocator.deinit();

const dict_allocator = std.heap.page_allocator;
var dictionary = try loadDictionary(dict_allocator, "dictionary.wc");
defer dictionary.deinit();
```

### 8.3 Dictionary Compression

**Format**: Zstandard compression
- **Compression ratio**: 60-70%
- **Decompression speed**: <300ms for 50k words
- **On-disk size**: ~800KB (vs 2MB uncompressed)

### 8.4 Incremental Loading

For large dictionaries (100k+ words):
- Load common 10k words immediately
- Load rest on first use
- Memory-mapped file for very large sets

---

## 9. Multiplayer Architecture

### 9.1 Game Modes

**Mode 1: Classic (1-4 players)**
- Turn-based alternation
- First unable to play loses
- All players visible to each other

**Mode 2: Speed Chain (1-2 players)**
- Time limit per turn (default 30 seconds)
- Faster play = higher points
- Combo multipliers for quick plays

**Mode 3: Theme Chain (1-4 players)**
- Words must be from specific category
- Difficulty adjusts based on category
- Educational variant

**Mode 4: Longest Chain (Solo/AI)**
- Build chain as long as possible
- AI tries to block
- High score mode

**Mode 5: Battle Royale (2-4 players)**
- Elimination mode
- Last player standing wins
- Progressive word difficulty increase

**Mode 6: Endless (Solo)**
- No AI opponent
- Infinite chain
- Personal best tracking

### 9.2 Hot-Seat Multiplayer

```zig
const HotSeatGame = struct {
    players: std.ArrayList(Player),
    current_player_index: u8,

    fn switchToNextPlayer(self: *HotSeatGame) void {
        self.current_player_index =
            (self.current_player_index + 1) % @intCast(self.players.items.len);
    }

    fn endGame(self: *HotSeatGame) ?usize {
        // Find player with highest score
        var highest_score: u32 = 0;
        var winner_idx: usize = 0;

        for (self.players.items, 0..) |player, idx| {
            if (player.current_score > highest_score) {
                highest_score = player.current_score;
                winner_idx = idx;
            }
        }

        return winner_idx;
    }
};
```

---

## 10. Implementation Modules

### 10.1 Module Breakdown

```
src/
├── main.zig                    # Entry point
├── game.zig                    # Game controller
├── dictionary.zig              # Dictionary management
├── validation.zig              # Word validation
├── scoring.zig                 # Scoring engine
├── ai.zig                      # AI implementations
│   ├── scorer.zig
│   ├── novice.zig
│   ├── intermediate.zig
│   ├── expert.zig
│   ├── master.zig
│   └── personality.zig
├── ui.zig                      # Display/rendering
├── player.zig                  # Player management
├── types.zig                   # Shared data structures
├── stats.zig                   # Statistics tracking
└── utils.zig                   # Utilities

data/
├── dictionary.wc               # Compressed word database
├── categories.json             # Word categories
└── config.json                 # Game settings
```

### 10.2 Dependencies

```
build.zig:
  - std (Zig standard library)
  - zstd (compression)
  - Terminal capabilities library (for colors)
```

---

## 11. Testing Strategy

### 11.1 Test Categories

**Test Suite 1: Dictionary Tests**
- Load 50k words in <500ms
- O(1) word lookup
- All 26 starting letters have words
- Duplicate word detection

**Test Suite 2: Validation Tests**
- Valid words accepted
- Invalid words rejected
- Used word tracking
- Letter matching
- Edge cases (single letter, long words)

**Test Suite 3: AI Tests**
- Each difficulty level selects valid words
- AI never repeats words
- Novice plays faster than Expert
- Master beats other levels consistently

**Test Suite 4: Scoring Tests**
- Correct base score calculation
- Rarity bonuses applied correctly
- Combo multipliers work
- Difficulty modifiers stack properly

**Test Suite 5: Game Flow Tests**
- Game initializes correctly
- Turn switching works
- Game ends when no valid words
- Score tracking accurate

**Test Suite 6: Performance Tests**
- Dictionary load <500ms
- Word validation <1ms
- AI move (Master) <100ms
- Render update <10ms
- Memory usage <50MB

---

## 12. Build & Deployment

### 12.1 Build Configuration (build.zig)

```zig
pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const exe = b.addExecutable(.{
        .name = "word-chain",
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = optimize,
    });

    // Add compression support (optional zstd)
    exe.linkLibC();

    b.installArtifact(exe);

    // Run tests
    const test_step = b.step("test", "Run tests");
    // ... test configuration
}
```

### 12.2 Release Artifacts

- Binary: `word-chain` (Linux/macOS/Windows)
- Dictionary: `dictionary.wc` (compressed 50k words)
- Config: `config.json` (settings)
- README: Game instructions & examples

---

## 13. Quality Standards Checklist

- [ ] All code follows Zig idioms
- [ ] Zero allocations in hot paths (validation, rendering)
- [ ] Comprehensive error handling
- [ ] Cross-platform compatible (Linux, macOS, Windows)
- [ ] Well-documented with examples
- [ ] Modular design (independent modules)
- [ ] >80% test coverage
- [ ] Performance targets met
- [ ] Memory usage optimized
- [ ] Dictionary validates on load

---

## 14. Review Checklist

- [x] Dictionary loading efficiency reviewed
- [x] Word validation algorithm optimized
- [x] AI strategy effectiveness designed
- [x] Memory usage with large dictionary planned
- [x] Game balance considerations documented
- [x] Multiplayer architecture defined
- [x] Performance targets specified
- [x] Test strategy comprehensive
- [x] Build configuration planned

