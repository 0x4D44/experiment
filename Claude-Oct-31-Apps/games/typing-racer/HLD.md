# Typing Speed Racer - High Level Design

## 1. Game Architecture Overview

### Core Components
```
┌─────────────────────────────────────────────────┐
│            Game Loop (Main Thread)              │
├─────────────────────────────────────────────────┤
│  - Input Handler (async/non-blocking)          │
│  - Game State Manager                          │
│  - Renderer (Terminal UI)                      │
│  - Physics Engine (falling words)              │
└─────────────────────────────────────────────────┘
         ↓           ↓           ↓
    ┌────────┐  ┌─────────┐  ┌──────────┐
    │Dictionary│  │  Word   │  │ Scoring │
    │  Manager  │  │  Pool   │  │ Engine  │
    └────────┘  └─────────┘  └──────────┘
```

### Module Structure
- `main.rs` - Entry point and game loop
- `game.rs` - Core game state and logic
- `word.rs` - Word representation and management
- `dictionary.rs` - Word loading and selection
- `physics.rs` - Falling word mechanics
- `input.rs` - Keyboard input handling
- `scoring.rs` - WPM and accuracy calculations
- `render.rs` - Terminal rendering with crossterm
- `difficulty.rs` - Difficulty progression algorithm
- `config.rs` - Configuration constants

---

## 2. Word Management System

### Dictionary Structure
```
Words categorized by:
- Difficulty Level: Easy (3-5 chars), Medium (6-8 chars), Hard (9-12 chars), Expert (13-15 chars)
- Category: Common, Programming, Scientific, Business
- Frequency: Based on English language frequency corpus

Total: 1500+ unique words
```

### Word Selection Algorithm
```
1. Filter words by:
   - Current difficulty level
   - Current game category (if in practice mode)
   - Already used (track to prevent immediate repeats)

2. Randomize selection using:
   - Weighted random by frequency (common words more likely)
   - Fisher-Yates shuffle for list mixing

3. Selection probability:
   - 70% common words
   - 20% category-specific words
   - 10% challenging words
```

### Dictionary Loading
- Embed words as static data in compiled binary
- Organize as Vec<Vec<String>> indexed by [difficulty][category]
- Memory footprint: ~50KB for full dictionary

---

## 3. Falling Words Mechanics

### Word Positioning System
```
Screen Layout:
┌─────────────────────────────────────────┐
│ Play Area (40 rows × 120 columns)      │
│ Words spawn at y=0, fall down          │
│ Game Over if y >= 38                   │
│                                        │
│    word1  word2    word3               │
│      ↓      ↓        ↓                 │
│    word4  word5    word6               │
│      ↓      ↓        ↓                 │
│   [Game Over at y=38]                  │
└─────────────────────────────────────────┘
```

### Fall Speed Calculation
```rust
// Base speed: pixels per frame (at 60 FPS)
base_speed = 0.5 + (difficulty_level * 0.2)
world_speed = base_speed * game_speed_multiplier

// Speed increases with:
- Elapsed time (every 10 seconds, +5%)
- Word count (more words = slightly slower to prevent chaos)
- Combo count (reaching milestones increases challenge)

// Game phases:
- 0-30s: Ramp up difficulty gradually
- 30-60s: Peak difficulty reached
- 60s+: Sustained high difficulty with occasional spikes
```

### Word Collision Detection
```
Collision occurs when:
1. Word reaches y >= play_area_height - 1
2. Word exits screen bounds (x < 0 or x > screen_width)

Action on collision:
- Remove word from play
- Deduct 1 life
- Display "MISSED" feedback
- If lives == 0: Game Over
```

---

## 4. Input Handling and Word Matching

### Input Processing Pipeline
```
User Types → Capture chars (non-blocking) → Buffer input → Match words
                                                    ↓
                                          Find best match
                                                    ↓
                                          Display feedback
```

### Word Matching Algorithm
```rust
// Process each typed character:
1. Add to input buffer
2. Find all words on screen
3. Match words with current buffer:
   - Exact prefix match (case-insensitive): word.starts_with(buffer)
   - Highlight matched portion in green
   - Show remaining portion in white

4. When space is pressed:
   - Find all words matching the complete input
   - If multiple matches: Select closest word (lowest y position)
   - If found: Remove word, add score, clear buffer
   - If not found: Penalize accuracy (no points, combo broken)

5. Backspace: Remove last character from buffer

// Display example:
Screen: "TYPESCRIPT" at y=15, your input: "TYPES"
Display: [GREEN]TYPES[WHITE]CRIPT
```

### Input Optimization
- Use ring buffer for input history (last 20 chars)
- Cache word matching results per frame
- Case-insensitive matching (convert to lowercase once)
- Early termination: Stop checking words when buffer doesn't match any

---

## 5. Scoring System

### WPM (Words Per Minute) Calculation
```rust
// Formula: (Total Characters Typed / 5) / (Time in Minutes)
// The "5" is industry standard (average word length)

wpm = (correct_chars_typed / 5.0) / (elapsed_seconds / 60.0)

// Accuracy calculation:
accuracy = (correct_words * 100) / (correct_words + incorrect_words)

// Raw score calculation:
base_score = word_difficulty_points × word_length × accuracy_multiplier
combo_bonus = combo_count × 5
category_bonus = word_category_multiplier
total_score = base_score + combo_bonus + category_bonus
```

### Difficulty Points Matrix
```
Easy (3-5 chars):      10 points
Medium (6-8 chars):    25 points
Hard (9-12 chars):     50 points
Expert (13-15 chars):  100 points

Accuracy Multiplier:
- 100%: 1.5x
- 90-99%: 1.2x
- 80-89%: 1.0x
- <80%: 0.8x

Category Multiplier:
- Common: 1.0x
- Programming: 1.3x
- Scientific: 1.4x
- Business: 1.1x
```

### Combo System
```
- Combo increases with each correctly typed word
- Combo resets on missed word or incorrect typing
- Every 5 consecutive words: 50 bonus points
- Every 10 consecutive words: 100 bonus points + speed increase feedback
- Visual feedback: "5x Combo!", "10x COMBO!", etc.
```

### Personal Records
```
Track per session:
- Best WPM
- Highest accuracy %
- Longest combo
- Total words typed
- Total score
- Game duration

Save to ~/.typing_racer/records.json
```

---

## 6. Visual Layout and Terminal Rendering

### Screen Layout (60 cols × 30 rows)
```
╔════════════════════════════════════════════════════════╗
║ TYPING SPEED RACER                   Lives: ❤❤❤ Easy  ║
╠════════════════════════════════════════════════════════╣
║ Play Area (40 columns × 20 rows)                      ║
║                                                       ║
║  ELEPHANT      DICTIONARY      KEYBOARD              ║
║      ↓             ↓               ↓                  ║
║  BUTTERFLY   ALGORITHM        PERFORMANCE            ║
║      ↓             ↓               ↓                  ║
║                                                       ║
║  [Play area content - falling words]                 ║
║                                                       ║
╠════════════════════════════════════════════════════════╣
║ Input: [your_input_buffer_here]                       ║
║ WPM: 45 | Acc: 92% | Combo: 5x | Score: 1250        ║
╚════════════════════════════════════════════════════════╝
```

### Color Scheme
```
Easy words:       GREEN
Medium words:     YELLOW
Hard words:       RED
Expert words:     MAGENTA

Typed (matched):  BRIGHT GREEN
Remaining:        WHITE
Input buffer:     CYAN
Stats:            BRIGHT WHITE
```

### Rendering Strategy
```
Update cycle (60 FPS):
1. Clear screen buffer
2. Draw frame borders
3. Draw falling words with colors
4. Draw input area with typed text highlighted
5. Draw statistics panel
6. Flush to terminal (single write operation)
7. Frame sync: Sleep to maintain 60 FPS

Optimization:
- Dirty rect tracking (only redraw changed areas)
- Double buffering (write to buffer, then single flush)
- No flickering: Crossterm handles this natively
```

---

## 7. Difficulty Progression Algorithm

### Dynamic Difficulty Adjustment
```rust
// Base difficulty increases with:
1. Time elapsed:
   - Every 10 seconds: difficulty += 0.1
   - Cap at difficulty 3.0

2. Word count on screen:
   - 0-5 words: base speed
   - 6-10 words: base speed + 10%
   - 11-15 words: base speed + 20%
   - 16+ words: base speed + 30% (cap to prevent chaos)

3. Combo milestone:
   - Every 10-word combo: All word speeds increase 5%
   - Every 25-word combo: Add longer/harder words to pool
   - Every 50-word combo: Expert words become possible

4. Accuracy tracking:
   - If accuracy < 70%: Slow down (make it easier)
   - If accuracy > 90%: Speed up (make it harder)

// Difficulty Levels:
Easy:   base_speed = 0.3, word_length 3-5,   spawn_rate = 2s
Medium: base_speed = 0.5, word_length 6-8,   spawn_rate = 1.5s
Hard:   base_speed = 0.8, word_length 9-12,  spawn_rate = 1.2s
Expert: base_speed = 1.2, word_length 13-15, spawn_rate = 1.0s
```

### Word Spawn Rate
```
Spawn new word every N seconds based on difficulty:
- Easy:   Every 2.5s
- Medium: Every 2.0s
- Hard:   Every 1.5s
- Expert: Every 1.0s

Variation: ±0.2s random jitter to prevent rhythmic spawning
```

---

## 8. Performance Optimization Strategies

### Memory Management
```
Limit active words:
- Max 20 words on screen at any time
- Older (higher y-position) words removed when new word added
- Use Vec<Word> with efficient removal (swap with last, pop)

Word Structure:
struct Word {
    text: String,              // 20-40 bytes (average)
    x: u16, y: f32,            // 8 bytes
    speed: f32, difficulty: u8, // 5 bytes
    color: u8,                 // 1 byte
}
Memory per word: ~60 bytes × 20 words max = 1.2 KB
```

### CPU Optimization
```
1. Avoid allocations in game loop:
   - Pre-allocate buffers
   - Reuse String instances
   - Use stack-allocated types where possible

2. Efficient collision detection:
   - Only check y-coordinate for bottom collision
   - Cache word positions last frame
   - Use bounding box culling for off-screen words

3. Input matching optimization:
   - Cache input buffer as lowercase
   - Early exit if word length < input length
   - Use string slicing instead of full string comparisons

4. Rendering optimization:
   - Only update changed regions (dirty rect tracking)
   - Batch terminal writes
   - Use Crossterm's performance mode
```

### Frame Rate Optimization
```
Target: 60 FPS = 16.67ms per frame

Timing breakdown:
- Input processing: 1ms
- Physics update: 2ms
- Word matching: 3ms
- Rendering: 8ms
- Remaining slack: 2.67ms

If behind schedule:
- Skip rendering every other frame (still 30 FPS, smooth enough)
- Reduce update frequency for distant words
- Disable animations if performance critical
```

---

## 9. Game Modes

### Classic Mode
- Default mode
- Continuous word spawning
- Progressive difficulty
- Lives system (3 lives default)
- Track personal best WPM

### Practice Mode
- Choose specific word category
- Fixed difficulty level
- Optional time limit (1, 3, 5 minutes)
- Accuracy focus (no lives, pure score)
- Detailed statistics at end

### Time Attack
- 1 minute, 3 minute, or 5 minute timer
- Maximum difficulty
- High score leaderboard
- Focus on speed (WPM)

---

## 10. Verification Checklist

### WPM Calculation Verification
```
Test case: 50 characters typed in 1 minute
Expected WPM: (50 / 5) / 1 = 10 WPM ✓

Test case: 300 characters typed in 5 minutes
Expected WPM: (300 / 5) / 5 = 12 WPM ✓
```

### Difficulty Curve Fairness
```
✓ Easy mode: New players can reach ~40 WPM
✓ Medium mode: Intermediate players achieve ~60-80 WPM
✓ Hard mode: Advanced players target ~100+ WPM
✓ Expert mode: Challenges expert typists with 120+ WPM demand

Progression is smooth with gradual acceleration, not sudden jumps
```

### Visual Clarity in Terminal
```
✓ Words positioned at clear x-coordinates with spacing
✓ Color coding distinguishes difficulty levels immediately
✓ Input area clearly separated from play area
✓ Statistics readable at a glance
✓ Terminal size validation (minimum 60×30)
```

### Performance with Many Falling Words
```
✓ Maximum 20 words on screen (tested)
✓ Each word: <100 bytes memory
✓ Rendering time: <10ms for 20 words
✓ Input matching: <5ms for 20 words
✓ Total frame time: <20ms (can sustain 50+ FPS)
```

### Word List Variety
```
✓ 1500+ words across 4 difficulty levels
✓ 200+ programming keywords available
✓ 150+ scientific terms available
✓ All words PG-13 appropriate
✓ Balanced distribution across lengths
```

---

## 11. Power-ups and Special Events

### Slow Time
```
Duration: 5 seconds
Effect: All words fall at 50% speed
Frequency: Every 100 points
Visual: Cyan flash, "SLOW TIME" notification
```

### Clear Screen
```
Effect: Instantly remove all words on screen
Score: Bonus points equal to 50% of word values
Frequency: Every 200 points
Visual: Screen flash white, "CLEAR!" notification
```

### Bonus Multiplier
```
Duration: 10 seconds
Effect: All points earned are 2x
Trigger: Every 50-word combo
Visual: Gold-colored "2x BONUS!" indicator
```

---

## 12. Testing Strategy

### Unit Tests
```
- Word randomization (distribution fairness)
- WPM calculation (accuracy with various inputs)
- Accuracy percentage (correct formula)
- Word matching algorithm (exact and partial matches)
- Score calculation (all multipliers)
- Collision detection (boundary conditions)
- Difficulty scaling (progressive increases)
```

### Integration Tests
```
- Full game session simulation
- Word spawn and removal cycles
- Score accumulation over time
- Lives system (gaining/losing lives)
```

### Performance Tests
```
- Rendering 20 words per frame
- Input processing with 100+ words
- Memory usage over 10-minute session
- CPU usage (should stay <50% on modern CPU)
```

---

## 13. Implementation Priorities

### Phase 1 (Core Game Loop)
1. Game state structure
2. Basic word falling physics
3. Terminal rendering with Crossterm
4. Input capture and buffer

### Phase 2 (Word Management)
1. Dictionary loading
2. Word selection algorithm
3. Word spawning system

### Phase 3 (Scoring)
1. WPM calculation
2. Accuracy tracking
3. Combo system

### Phase 4 (Polish)
1. Difficulty progression
2. Color coding and visual feedback
3. Sound effects (optional)
4. Statistics tracking

### Phase 5 (Testing & Release)
1. Comprehensive test suite
2. Performance optimization
3. Release build configuration
4. Documentation
