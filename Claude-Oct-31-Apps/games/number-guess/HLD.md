# Number Guess Game - High Level Design (HLD)

## 1. Game Architecture Overview

### Core Components
```
┌─────────────────────────────────────────────┐
│         Number Guess Game Engine            │
├─────────────────────────────────────────────┤
│  ┌──────────────────────────────────────┐   │
│  │  Main Game Loop                      │   │
│  │  - Menu System                       │   │
│  │  - Game Mode Selection               │   │
│  │  - Difficulty Configuration          │   │
│  └──────────────────────────────────────┘   │
│  ┌──────────────────────────────────────┐   │
│  │  Game Logic Engine                   │   │
│  │  - Random Number Generation          │   │
│  │  - Input Validation                  │   │
│  │  - Feedback Generation               │   │
│  │  - State Management                  │   │
│  └──────────────────────────────────────┘   │
│  ┌──────────────────────────────────────┐   │
│  │  AI System                           │   │
│  │  - Binary Search AI                  │   │
│  │  - Random Guesser AI                 │   │
│  │  - Probabilistic AI                  │   │
│  └──────────────────────────────────────┘   │
│  ┌──────────────────────────────────────┐   │
│  │  Hint System                         │   │
│  │  - Mathematical Properties           │   │
│  │  - Hot/Cold Proximity                │   │
│  │  - Smart Hints                       │   │
│  └──────────────────────────────────────┘   │
│  ┌──────────────────────────────────────┐   │
│  │  Statistics & Achievements           │   │
│  │  - Game Statistics                   │   │
│  │  - Win Streaks                       │   │
│  │  - Achievement Tracking              │   │
│  └──────────────────────────────────────┘   │
│  ┌──────────────────────────────────────┐   │
│  │  UI/Display System                   │   │
│  │  - ASCII Art & Formatting            │   │
│  │  - Progress Visualization            │   │
│  │  - Menus & Navigation                │   │
│  └──────────────────────────────────────┘   │
└─────────────────────────────────────────────┘
```

### File Structure
```
games/number-guess/
├── main.c              # Entry point and main game loop
├── game.c              # Core game logic
├── game.h              # Game structures and prototypes
├── ai.c                # AI opponent implementations
├── ai.h                # AI function prototypes
├── hint.c              # Hint generation and math analysis
├── hint.h              # Hint function prototypes
├── stats.c             # Statistics tracking and persistence
├── stats.h             # Statistics structures
├── ui.c                # Display and UI functions
├── ui.h                # UI function prototypes
├── random.c            # Random number generation
├── random.h            # Random generation prototypes
├── test.c              # Comprehensive test suite
├── Makefile            # Build configuration
├── README.md           # User documentation
├── HLD.md              # This document
└── DATA/
    └── stats.dat       # Persistent game statistics (runtime)
```

---

## 2. Game Mode System

### Classic Mode
**Description**: Player guesses the computer's secret number.
- Computer picks a random number
- Player provides guesses
- Game gives feedback (too high/low)
- Win when number is guessed correctly

**Feedback Types**:
- "Too High" - guess exceeds secret
- "Too Low" - guess is below secret
- "Warmer" - moving closer to secret
- "Colder" - moving away from secret
- "Perfect!" - correct guess

### Reverse Mode
**Description**: Computer tries to guess the player's number.
- Player thinks of a secret number
- Computer makes intelligent guesses
- Player provides feedback (higher/lower/correct)
- Different AI difficulty levels show different strategies

**AI Strategies**:
- Easy: Random guessing
- Medium: Binary search with minor deviations
- Hard: Binary search with pattern recognition
- Expert: Probabilistic with history analysis

### Versus AI Mode
**Description**: Race against the computer to guess faster.
- Both play simultaneously with the same number range
- Both try to reach the secret number first
- Whoever guesses correctly wins
- Can select AI difficulty level

**Win Conditions**:
- Player wins by guessing before AI
- AI wins if it reaches the number first
- Show final statistics and comparison

### Challenge Mode
**Description**: Limited guesses with point multipliers.
- Number range: 1-1000 (default)
- Limited guess budget (3, 5, 7 guesses depending on difficulty)
- Points = (Max Guesses - Guesses Used) * Difficulty Multiplier
- Consecutive wins increase multiplier
- Maintain leaderboard

**Scoring**:
- Easy: 5 point base multiplier, 7 guesses allowed
- Medium: 10 point base multiplier, 5 guesses allowed
- Hard: 20 point base multiplier, 3 guesses allowed

### Memory Mode
**Description**: Remember and guess number sequences.
- Computer shows sequence of random numbers
- Player tries to guess the next number
- Sequence length increases with rounds
- Tests pattern recognition and memory

**Progression**:
- Round 1: Show 1 number, guess 1
- Round 2: Show 2 numbers, guess 1
- Round 3: Show 3 numbers, guess 2
- Continue until player fails

### Daily Challenge
**Description**: One special challenge per day with leaderboard.
- Same challenge for all players on same day
- Defined number range and guess limit
- Submit score to daily leaderboard
- Check statistics for past challenges

---

## 3. Difficulty Level System

### Mathematical Approach
```
Difficulty scaling uses exponential growth:
Range = 10 * (2 ^ (difficulty_level - 1))
Max_Guesses = 5 + (difficulty_level * 2.5)
Hint_Penalty = 2 ^ difficulty_level

Difficulty Levels:
- Easy (Level 1):      Range 1-10,    Unlimited guesses, Free hints
- Medium (Level 2):    Range 1-100,   ~10 guesses,      1 hint per guess
- Hard (Level 3):      Range 1-1000,  ~15 guesses,      1 hint per 2 guesses
- Expert (Level 4):    Range 1-10000, ~20 guesses,      1 hint per 3 guesses
- Master (Level 5):    Range 1-100000,~25 guesses,      1 hint per 5 guesses
- Custom:              User defined, Calculated dynamically
```

### Fairness Analysis
- Easy mode emphasizes learning and fun (no pressure)
- Medium mode introduces strategic thinking
- Hard mode requires optimal guessing (binary search)
- Expert mode tests both speed and strategy
- Custom mode allows personalized difficulty

### Optimal Performance
For N possible values:
- Optimal guesses needed = ceil(log2(N)) + 1
- Easy (10):     ~5 guesses
- Medium (100):  ~8 guesses
- Hard (1000):   ~11 guesses
- Expert (10000): ~15 guesses

---

## 4. Hint System Algorithm

### Mathematical Properties Analyzer
```c
struct NumberProperties {
    int is_prime;           // Prime number check
    int is_perfect_square;  // Perfect square check
    int is_fibonacci;       // Fibonacci sequence check
    int is_even;            // Even/Odd
    int digit_sum;          // Sum of digits
    int divisor_count;      // Number of divisors
    int largest_prime_factor;
};
```

### Hint Generation Strategy
1. **Range-based Hints**:
   - "It's in the lower/upper half"
   - "Between X and Y"
   - Show visual progress bar

2. **Mathematical Hints** (based on actual number):
   - "It's a prime number"
   - "It's a perfect square"
   - "It's divisible by X"
   - "Sum of digits equals X"
   - "It's a Fibonacci number"

3. **Proximity Hints**:
   - "You're getting warmer!" (within 10% of range)
   - "Very close!" (within 5% of range)
   - "Ice cold" (far from answer)

4. **History-based Hints**:
   - "You've been guessing too high"
   - "Try the middle of remaining range"
   - Show pattern in your guesses

### Hint Accuracy Quality
- All hints must be 100% accurate
- Hints should provide meaningful information
- Avoid spoilers (never reveal exact number)
- Hints can be requested once per N guesses (difficulty-dependent)

---

## 5. AI Opponent Implementation Strategy

### Random Guesser (Easy AI)
```
Algorithm:
1. Generate random number in range
2. Make guess
3. If feedback is "correct", win
4. If feedback is "too high/low", guess randomly again
5. No learning from feedback

Characteristics:
- 50% chance to guess correctly within expected guesses
- Very unpredictable
- Educational value: Shows importance of strategy
```

### Binary Search AI (Medium AI)
```
Algorithm:
1. Initialize: low = range_min, high = range_max
2. Guess midpoint = (low + high) / 2
3. If "too high": high = midpoint - 1
4. If "too low": low = midpoint + 1
5. Repeat until correct

Characteristics:
- Always solves in O(log N) guesses
- Optimal for finding unknown number
- Very fast convergence
```

### Probabilistic AI (Hard AI)
```
Algorithm:
1. Maintain probability distribution over remaining numbers
2. After each guess, update probabilities based on feedback
3. Consider past guesses and patterns
4. Weight guess selection by probability (exploit vs explore)
5. Add small random deviations to simulate human-like play

Characteristics:
- More realistic than pure binary search
- Can exploit patterns in player's feedback
- Takes slightly longer but plays more human-like
```

### Machine Learning AI (Expert AI)
```
Algorithm:
1. Analyze historical guesses for patterns
2. Build model of number distribution
3. Use Bayesian updating for probability estimation
4. Weight current guess by historical success
5. Adapt strategy based on secret number characteristics

Characteristics:
- Can detect if secret number has patterns (even, prime, etc.)
- Adapts strategy over multiple games
- Provides most challenging opponent
```

---

## 6. Statistics Tracking Approach

### Runtime Statistics
```c
struct PlayerStats {
    int total_games_played;
    int total_wins;
    int total_losses;
    int current_streak;
    int longest_streak;

    int games_by_difficulty[5];
    int wins_by_difficulty[5];

    int total_guesses;
    int best_game_guesses;
    int worst_game_guesses;
    int avg_guesses;

    float win_rate;
    float avg_time_seconds;

    int achievements_unlocked;
    int achievements_list[20];

    time_t last_game_time;
    time_t total_playtime;
};
```

### Persistent Storage
- Save to `DATA/stats.dat` in binary format
- Load on startup for continuity
- Backup system for data integrity
- Daily challenge scores stored separately

### Achievement System
```
- First Win: Win first game
- Streak Master: Win 5 consecutive games
- Perfect Round: Beat Hard difficulty on first guess (luck)
- Speed Demon: Guess under 20 seconds (Easy mode)
- Mathematician: Win Medium difficulty without hints
- Expert Solver: Beat Expert difficulty
- AI Slayer: Defeat Expert AI in Versus mode
- Memory Champion: Complete Memory mode (5+ rounds)
- No Hints Run: Win without using any hints
- Comeback King: Win after using all but 1 guess
- Diversity: Play all 5 game modes
- Collector: Unlock 10 achievements
```

### Leaderboard System
- Track top 10 scores globally
- Daily challenge leaderboard
- Difficulty-specific leaderboards
- Fastest completion times

---

## 7. Random Number Generation Strategy

### Cryptographic Quality RNG
```c
// Use system entropy for seed
#ifdef __unix__
    int fd = open("/dev/urandom", O_RDONLY);
    unsigned seed;
    read(fd, &seed, sizeof(seed));
    close(fd);
#else
    // Windows fallback: use time + pid
    unsigned seed = time(NULL) ^ getpid();
#endif

// Use MT19937 (Mersenne Twister) for distribution quality
// Avoid simple rand() which has poor statistical properties
```

### Distribution Quality
- Uniform distribution across range
- High period (2^19937 - 1)
- Good statistical properties
- Passes NIST randomness tests

### Seeding Strategy
- Use system entropy (`/dev/urandom` on Unix)
- Combine with time and process ID for fallback
- Reseed after each game to avoid patterns
- Allow custom seed for reproducible challenges

---

## 8. User Experience Flow

### Menu Structure
```
┌─────────────────────────────────────┐
│   MAIN MENU                         │
├─────────────────────────────────────┤
│ 1. Play Game                        │
│ 2. View Statistics                  │
│ 3. View Leaderboard                 │
│ 4. Daily Challenge                  │
│ 5. Settings                         │
│ 6. Help / Tutorial                  │
│ 7. Exit                             │
└─────────────────────────────────────┘
     |
     v
┌─────────────────────────────────────┐
│   GAME MODE SELECT                  │
├─────────────────────────────────────┤
│ 1. Classic (Guess computer's #)     │
│ 2. Reverse (Computer guesses yours) │
│ 3. Versus AI (Race)                 │
│ 4. Challenge (Limited guesses)      │
│ 5. Memory (Sequences)               │
│ 6. Custom Settings                  │
│ 7. Back to Main Menu                │
└─────────────────────────────────────┘
     |
     v
┌─────────────────────────────────────┐
│   DIFFICULTY SELECT                 │
├─────────────────────────────────────┤
│ 1. Easy (1-10, unlimited)           │
│ 2. Medium (1-100, ~10 guesses)      │
│ 3. Hard (1-1000, ~15 guesses)       │
│ 4. Expert (1-10000, ~20 guesses)    │
│ 5. Master (1-100000, ~25 guesses)   │
│ 6. Custom (User defined)            │
│ 7. Back                             │
└─────────────────────────────────────┘
     |
     v
    GAME SESSION
```

### In-Game UI
```
┌─────────────────────────────────────┐
│   NUMBER GUESS MASTER v1.0          │
├─────────────────────────────────────┤
│ Game Mode: Classic                  │
│ Difficulty: Medium                  │
│                                     │
│ Range: [1 ------------|--- 100]    │
│ Guesses: 3/10  |  Time: 00:45      │
│                                     │
│ Previous Guesses: 50↑ 25↓ 37↓       │
│                                     │
│ Enter your guess (1-100): █         │
│ > Too Low! Getting warmer...        │
│                                     │
│ Hint available? (h for hint, g for guess)
│                                     │
│ [ESC] Menu  [H] Hint  [S] Statistics│
└─────────────────────────────────────┘
```

---

## 9. Scoring System Design

### Classic Mode Scoring
```
Base Points = 100
- Subtract points for each guess: Points - (Guesses * 2)
- Time bonus: +5 points per second saved (capped at 30 points)
- Difficulty multiplier: Easy=1x, Medium=2x, Hard=4x
- Hint usage: -10 points per hint
- Final Score = (100 - (Guesses * 2) + Time_Bonus) * Difficulty_Multiplier - (Hints * 10)
```

### Challenge Mode Scoring
```
Points = (Max_Guesses - Guesses_Used) * Difficulty_Multiplier
- Easy (7 guesses):    Base 5 points per unused guess
- Medium (5 guesses):  Base 10 points per unused guess
- Hard (3 guesses):    Base 20 points per unused guess

Win Streak Multiplier:
- 1st win: 1x multiplier
- 2-3 wins: 1.5x multiplier
- 4-5 wins: 2x multiplier
- 6+ wins: 3x multiplier

Example:
  Hard difficulty, used 2 guesses (1 remaining), 3-win streak
  Points = (3 - 2) * 20 * 2 = 40 points
```

### Reverse Mode Scoring
```
Points based on AI performance:
- Random AI beats in N guesses: N points
- Binary Search AI beats in N guesses: N/2 points (harder to beat)
- Probabilistic AI beats in N guesses: N/4 points

Player wins by beating AI's guess count:
- Beat Random AI: +50 points
- Beat Binary Search AI: +100 points
- Beat Probabilistic AI: +150 points
```

### Memory Mode Scoring
```
Points = (Number of Rounds Completed - 1) * Difficulty_Multiplier
- Round 1 correct: 0 points
- Round 2 correct: 1 * Multiplier
- Round 3 correct: 2 * Multiplier
- Etc.

Difficulty Multipliers:
- Easy: 5 points per round
- Medium: 10 points per round
- Hard: 20 points per round
```

---

## 10. Technical Implementation Details

### Core Data Structures
```c
// Game session state
struct GameSession {
    int secret_number;
    int range_min, range_max;
    int guess_count;
    int max_guesses;
    int hints_used;
    int hints_available;
    time_t start_time;
    int game_mode;
    int difficulty_level;
    int guesses_history[100];
    int game_state;  // ACTIVE, WON, LOST
};

// AI state
struct AIOpponent {
    int strategy;
    int guess_count;
    int current_guess;
    int range_min, range_max;
};

// Player profile
struct Player {
    char name[50];
    struct PlayerStats stats;
    struct GameSession current_session;
};
```

### Key Algorithms

#### Binary Search for Range Narrowing
```c
void narrow_range(int guess, int feedback, int *min, int *max) {
    if (feedback == TOO_HIGH) {
        *max = guess - 1;
    } else if (feedback == TOO_LOW) {
        *min = guess + 1;
    }
}
```

#### Prime Number Check
```c
int is_prime(int num) {
    if (num < 2) return 0;
    if (num == 2) return 1;
    if (num % 2 == 0) return 0;
    for (int i = 3; i * i <= num; i += 2) {
        if (num % i == 0) return 0;
    }
    return 1;
}
```

#### Fibonacci Check
```c
int is_fibonacci(int num) {
    // A number is Fibonacci if one of (5*n^2 + 4) or (5*n^2 - 4) is perfect square
    int a = 5 * num * num + 4;
    int b = 5 * num * num - 4;
    return is_perfect_square(a) || is_perfect_square(b);
}
```

---

## 11. Testing Strategy

### Unit Tests
- Random number generation: Bounds, distribution, entropy
- Hint system: Accuracy, variety, usefulness
- AI strategies: Convergence, optimality, correctness
- Score calculation: All formulas, edge cases
- Statistics: Persistence, accuracy, correctness
- Input validation: Boundary values, invalid inputs

### Integration Tests
- Complete game sessions: All modes
- AI vs Player: Multiple difficulties
- Menu navigation: State management
- Data persistence: Load/save statistics

### Test Coverage Goals
- Minimum 80% code coverage
- 100% coverage of core game logic
- All hint algorithms tested
- All AI strategies tested
- All difficulty calculations verified

---

## 12. Design Decisions & Rationale

### Why Pure C?
- Educational value: Low-level control, memory management
- Portability: POSIX compliance ensures cross-platform
- Performance: Direct system calls without abstraction layers
- No external dependencies: Single executable

### Why Multiple Game Modes?
- Engagement: Different playstyles suit different users
- Replayability: Variety prevents boredom
- Learning: Different modes teach different concepts
- Competition: Versus mode adds multiplayer tension

### Why Modular Design?
- Maintainability: Each module has clear responsibility
- Testability: Modules can be tested independently
- Extensibility: Easy to add new features
- Debugging: Isolated issues are easier to fix

### Why Persistent Statistics?
- Player motivation: Visible progress
- Competition: Leaderboards create engagement
- Analysis: Identify difficulty balance issues
- Achievement: Unlock rewards for milestones

---

## 13. HLD Review Checklist

### Game Mode Variety & Fun Factor
- [x] 5 distinct game modes with different mechanics
- [x] Progressive difficulty curve
- [x] Competitive elements (Versus AI, Leaderboards)
- [x] Memory/Skill challenges (Memory Mode)
- [x] Daily rotation (Daily Challenge)

### Mathematical Fairness
- [x] Exponential scaling ensures balanced difficulty curve
- [x] Guess limits calculated based on information theory
- [x] Optimal strategies (binary search) defined mathematically
- [x] Scoring system rewards skillful play
- [x] AI opponents use mathematically sound algorithms

### Hint Quality
- [x] Multiple hint types (range, mathematical, proximity)
- [x] All hints guaranteed 100% accurate
- [x] Hints don't spoil the answer
- [x] Hint frequency balanced by difficulty
- [x] Educational hints teach mathematical concepts

### AI Intelligence
- [x] 4 distinct difficulty levels with increasing sophistication
- [x] Random Guesser: Shows need for strategy
- [x] Binary Search: Optimal information-theoretic approach
- [x] Probabilistic: More human-like and challenging
- [x] Machine Learning: Adapts to player patterns

### User Experience
- [x] Clear menu navigation
- [x] Intuitive feedback (high/low/warmer/colder)
- [x] Visual progress indicators
- [x] Statistics tracking for motivation
- [x] Help system for learning

---

## 14. Future Enhancement Ideas

### Phase 2 Enhancements
- Network multiplayer (TCP/IP)
- Replay analysis: Analyze past games
- AI training mode: Teach AI through games
- Custom difficulty profiles
- Themed number categories (movie years, sports stats)

### Phase 3 Enhancements
- Web API integration for leaderboards
- Mobile app companion
- Tournament brackets
- Achievements syncing
- Speedrun mode with global timer

---

## 15. Success Metrics

### Gameplay
- Average session length: 5-15 minutes
- Completion rate: >70% of games finished
- Replay rate: Players return for multiple games
- Win rate: 70-80% for appropriate difficulty

### Code Quality
- No memory leaks (verified with valgrind)
- No buffer overflows
- >80% test coverage
- Clear, documented code
- POSIX compliant

### User Engagement
- Daily active players (if multiplayer)
- Average score progression
- Achievement unlock rate
- Leaderboard participation
- Difficulty progression patterns

---

END OF HLD DOCUMENT
