# Wordle Clone - High Level Design

## 1. Game Architecture Overview

### Core Components
The application follows a modular architecture with clear separation of concerns:

```
┌─────────────────────────────────────────┐
│         WordleGame (Main Controller)    │
├─────────────────────────────────────────┤
│  - Game State Management                │
│  - Turn Flow Control                    │
│  - User Input Handling                  │
└─────────────────────────────────────────┘
         ↓              ↓              ↓
  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐
  │ WordList     │ │ Guess        │ │ Statistics   │
  │ Manager      │ │ Evaluator    │ │ Manager      │
  │              │ │              │ │              │
  │ - Valid      │ │ - Feedback   │ │ - Streaks    │
  │   words      │ │ - Colors     │ │ - Win rate   │
  │ - Answer     │ │ - Validation │ │ - Persistence│
  │   words      │ │              │ │              │
  └──────────────┘ └──────────────┘ └──────────────┘
         ↓              ↓              ↓
  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐
  │ UI Renderer  │ │ Keyboard     │ │ File System  │
  │              │ │ Visualizer   │ │              │
  │ - Grid       │ │              │ │ - Save/Load  │
  │ - Messages   │ │ - Letter     │ │ - Config     │
  │              │ │   colors     │ │              │
  └──────────────┘ └──────────────┘ └──────────────┘
```

## 2. Class Design

### 2.1 WordList (Resource Manager)
```cpp
class WordList {
private:
    std::vector<std::string> answerWords;      // ~2,300 words
    std::unordered_set<std::string> validWords; // ~12,000 words
    std::unordered_map<std::string, std::string> wordDefinitions;

public:
    bool isValidWord(const std::string& word) const;
    bool isAnswerWord(const std::string& word) const;
    std::string getRandomAnswerWord() const;
    std::string getWordDefinition(const std::string& word) const;
    void loadFromFile(const std::string& validPath, const std::string& answerPath);
    size_t getAnswerCount() const;
    size_t getValidCount() const;
};
```

### 2.2 LetterFeedback (Value Object)
```cpp
enum class LetterStatus {
    GRAY,      // Not in word
    YELLOW,    // In word, wrong position
    GREEN      // Correct position
};

struct LetterFeedback {
    char letter;
    LetterStatus status;
    int position;
};
```

### 2.3 GuessEvaluator (Core Game Logic)
```cpp
class GuessEvaluator {
private:
    std::string answerWord;

public:
    GuessEvaluator(const std::string& answer);

    // Main evaluation function
    std::vector<LetterFeedback> evaluate(const std::string& guess);

    // Helper functions
    bool hasLetter(char letter) const;
    int getLetterCount(char letter) const;
    std::string getAnswerWord() const;

private:
    // Core logic for handling repeated letters correctly
    LetterStatus evaluateLetter(const std::string& guess, int position);
    void handleRepeatedLetters(std::vector<LetterFeedback>& feedback,
                               const std::string& guess);
};
```

**Repeated Letter Handling:**
- First pass: Mark all exact matches (green)
- Second pass: Mark wrong positions (yellow), respecting duplicate counts
- Remaining unmatched letters: Mark as gray
- If letter appears multiple times: only mark as yellow/green for available instances

Example:
```
Answer: ROBOT
Guess:  FLOOR
F → Gray (not in ROBOT)
L → Gray (not in ROBOT)
O → Yellow (in ROBOT but position 4 not 3)
O → Yellow (in ROBOT but position 5 not 4)
R → Green (correct position)

Answer: SPORT
Guess:  STOOL
S → Green (pos 1)
T → Green (pos 2)
O → Yellow (in word, pos 5, not pos 3)
O → Gray (second O already matched)
L → Gray (not in SPORT)
```

### 2.4 GameState (Model)
```cpp
struct GameState {
    // Game identification
    std::string gameId;           // UUID or date-based
    std::string answerWord;
    std::string gameMode;         // "daily" or "practice"

    // Gameplay data
    std::vector<std::string> guesses;
    std::vector<std::vector<LetterFeedback>> feedbacks;
    int remainingGuesses;         // Starts at 6
    bool isGameOver;
    bool isWon;
    std::chrono::system_clock::time_point startTime;
    std::chrono::system_clock::time_point endTime;

    // Constraints
    bool hardMode;                // Must use revealed hints
    std::set<char> revealedLetters;

    // Utility functions
    bool isValidGuess(const WordList& wordList) const;
    int getGuessCount() const;
    bool hasRevealed(char letter) const;
};
```

### 2.5 KeyboardVisualizer (UI Component)
```cpp
class KeyboardVisualizer {
private:
    std::map<char, LetterStatus> letterStatus;
    static const std::string KEYBOARD_LAYOUT[3];

public:
    void updateLetter(char letter, LetterStatus status);
    void reset();
    void render() const;

private:
    std::string colorize(char letter, LetterStatus status) const;
    std::string getANSIColor(LetterStatus status) const;
};
```

**Keyboard Layout:**
```
Q W E R T Y U I O P
 A S D F G H J K L
  Z X C V B N M
```

### 2.6 UIRenderer (Presentation Layer)
```cpp
class UIRenderer {
public:
    // Game display
    static void renderGameGrid(const GameState& state, const WordList& wordList);
    static void renderGuessLine(const std::string& guess,
                                const std::vector<LetterFeedback>& feedback);
    static void renderEmptyLines(int count);
    static void renderStats(const Statistics& stats);
    static void renderGameOver(bool won, const std::string& answer, int guesses);

    // Messages
    static void showMessage(const std::string& msg, const std::string& type);
    static void clearScreen();

    // ANSI color codes
    static const std::string GREEN;      // ✓ Correct position
    static const std::string YELLOW;     // ~ Wrong position
    static const std::string GRAY;       // ✗ Not in word
    static const std::string RESET;
    static const std::string BOLD;
};
```

### 2.7 Statistics (Data Model & Persistence)
```cpp
struct Statistics {
    // Aggregate stats
    int gamesPlayed;
    int gamesWon;
    int currentStreak;
    int maxStreak;

    // Distribution of guesses (1-6)
    std::array<int, 6> guessDistribution;

    // Daily tracking
    std::map<std::string, GameState> dailyGames;  // Date → GameState
    std::string lastPlayedDate;

    // Calculation methods
    float getWinPercentage() const;
    int getAverageGuesses() const;
    std::string generateShareEmoji() const;
};

class StatisticsManager {
private:
    Statistics stats;
    std::string statsFilePath;

public:
    void loadFromFile(const std::string& path);
    void saveToFile() const;
    void recordGame(const GameState& game);
    void updateStreak(bool won);
    const Statistics& getStats() const;
    std::string generateShareText(const GameState& game) const;
};
```

### 2.8 DailyWordSelector (Deterministic Randomness)
```cpp
class DailyWordSelector {
public:
    // Returns same word for entire day (UTC)
    static std::string getDailyWord(const WordList& wordList);

    // Seed based on current date
    static uint32_t getDaySeed();

    // Get word for specific date (for testing/history)
    static std::string getWordForDate(const WordList& wordList,
                                      const std::chrono::system_clock::time_point& date);
};
```

Algorithm:
1. Get current UTC date
2. Hash date to uint32_t (e.g., YYYYMMDD format)
3. Seed std::mt19937 with hash
4. Use modulo on answer word list to get index
5. Return word at that index

### 2.9 InputValidator (Input Sanitization)
```cpp
class InputValidator {
public:
    static bool isValidGuessFormat(const std::string& input);
    static std::string sanitize(const std::string& input);
    static bool isAlphabetic(const std::string& input);
    static std::string toUpperCase(const std::string& input);
};
```

Validation rules:
- Exactly 5 characters
- Only alphabetic characters
- After trimming and case conversion
- No special characters or spaces

### 2.10 WordleGame (Main Controller)
```cpp
class WordleGame {
private:
    GameState currentGame;
    WordList wordList;
    StatisticsManager statsManager;
    KeyboardVisualizer keyboard;
    UIRenderer renderer;
    GuessEvaluator evaluator;

public:
    // Game lifecycle
    void initialize(const std::string& mode = "daily");
    void run();
    void cleanup();

    // Game actions
    void processGuess(const std::string& guess);
    bool validateGuess(const std::string& guess);
    void evaluateGuess(const std::string& guess);
    void updateGameState();
    void displayGameState();

    // Game ending
    void endGame(bool won);
    void displayResults();
    void saveGame();

private:
    void displayWelcome();
    void handleInvalidInput(const std::string& reason);
    bool isHardModeValid(const std::string& guess) const;
};
```

## 3. Word List Management

### 3.1 Data Sources
- Embedded word lists (CSV or binary format)
- Valid words: `resources/valid_words.txt` (~12,000 words)
- Answer words: `resources/answer_words.txt` (~2,300 words)

### 3.2 Word List Format
```
# simple text file, one word per line
ABOUT
ABOVE
ABUSE
...
```

### 3.3 Validation Strategy
- `validWords`: std::unordered_set for O(1) lookup
- `answerWords`: std::vector (for indexing) + set membership check
- Case-insensitive comparison (stored as uppercase)

## 4. Guess Validation and Feedback System

### 4.1 Validation Pipeline
```
Raw Input
    ↓
[Format Check] → Must be 5 letters
    ↓
[Sanitize] → Uppercase, trim whitespace
    ↓
[Word List Check] → Must be in valid_words
    ↓
[Hard Mode Check] → If enabled, must use revealed hints
    ↓
Valid ✓
```

### 4.2 Feedback Generation
```
Input: Guess + Answer
    ↓
[Exact Matches] → Mark all position matches as GREEN
    ↓
[Letter Inventory] → Count remaining unmatched letters
    ↓
[Wrong Position] → Mark available letters as YELLOW
    ↓
[Remaining] → Mark unmatched as GRAY
    ↓
Output: LetterFeedback array
```

## 5. Color Coding Strategy (ANSI Escape Codes)

### 5.1 Terminal Colors
```cpp
const std::string GREEN  = "\033[42m";   // Green background
const std::string YELLOW = "\033[43m";   // Yellow background
const std::string GRAY   = "\033[47m";   // Light gray background
const std::string RESET  = "\033[0m";    // Reset formatting
const std::string BOLD   = "\033[1m";    // Bold text
const std::string BLACK_TEXT = "\033[30m"; // Black text
```

### 5.2 Visual Representation
```
Actual display:
    [G] [G] [Y] [G] [G]

    Where each cell is a letter with background color

Share format (emoji):
    🟩🟩🟨🟩🟩
    ⬜🟩🟩🟩🟩
    🟩🟩🟩🟩🟩
```

## 6. Game State Persistence

### 6.1 Directory Structure
```
~/.wordle/
├── stats.json          # Overall statistics
├── games/
│   ├── 2024-10-31.json # Daily game save
│   ├── 2024-10-30.json
│   └── ...
└── config.json         # User preferences
```

### 6.2 Statistics File (stats.json)
```json
{
  "gamesPlayed": 42,
  "gamesWon": 31,
  "currentStreak": 5,
  "maxStreak": 12,
  "guessDistribution": [2, 8, 12, 7, 2, 0],
  "lastPlayedDate": "2024-10-31",
  "winPercentage": 73.81
}
```

### 6.3 Game Save File (YYYY-MM-DD.json)
```json
{
  "gameId": "2024-10-31-daily",
  "date": "2024-10-31",
  "gameMode": "daily",
  "answerWord": "PLANT",
  "guesses": ["STARE", "PLANT"],
  "feedbacks": [
    [
      {"letter": "S", "status": "GRAY"},
      {"letter": "T", "status": "GREEN"},
      {"letter": "A", "status": "YELLOW"},
      {"letter": "R", "status": "GRAY"},
      {"letter": "E", "status": "GRAY"}
    ],
    [
      {"letter": "P", "status": "GREEN"},
      {"letter": "L", "status": "GREEN"},
      {"letter": "A", "status": "GREEN"},
      {"letter": "N", "status": "GREEN"},
      {"letter": "T", "status": "GREEN"}
    ]
  ],
  "isWon": true,
  "startTime": "2024-10-31T08:00:00Z",
  "endTime": "2024-10-31T08:02:30Z",
  "hardMode": false
}
```

### 6.4 Persistence Operations
```cpp
void StatisticsManager::loadFromFile(const std::string& path);
void StatisticsManager::saveToFile() const;
void GameState::saveToFile(const std::string& path) const;
static GameState GameState::loadFromFile(const std::string& path);
```

## 7. Keyboard Visualization Component

### 7.1 Visual Layout
```
Initial state (all gray):
[Q] [W] [E] [R] [T] [Y] [U] [I] [O] [P]
[A] [S] [D] [F] [G] [H] [J] [K] [L]
  [Z] [X] [C] [V] [B] [N] [M]

After first guess "STARE":
[Q] [W] [E] [R⚠] [T✓] [Y] [U] [I] [O] [P]
[A⚠] [S✗] [D] [F] [G] [H] [J] [K] [L]
  [Z] [X] [C] [V] [B] [N] [M]

Legend:
✓ = Green (correct position)
⚠ = Yellow (wrong position)
✗ = Gray (not in word)
```

### 7.2 Implementation
```cpp
class KeyboardVisualizer {
private:
    std::map<char, LetterStatus> letterStatus;

public:
    void updateLetter(char letter, LetterStatus status);
    void render() const;
};
```

Rules:
- Once a letter is marked YELLOW or GREEN, it cannot revert to GRAY
- GREEN takes precedence over YELLOW
- Show updates after each guess

## 8. Daily Word Selection Algorithm

### 8.1 Requirements
- Same word for all players on a given day
- Deterministic (no external calls)
- Predictable for testing
- Unique words across days

### 8.2 Algorithm
```
Input: Current date (auto-detected or provided)
    ↓
Convert date to seed:
  - Format: YYYYMMDD (e.g., "20241031")
  - Convert to uint32_t via hash
    ↓
Seed Mersenne Twister:
  - std::mt19937 rng(seed)
    ↓
Generate index:
  - index = rng() % answerWords.size()
    ↓
Return answerWords[index]
```

### 8.3 Implementation
```cpp
std::string DailyWordSelector::getDailyWord(const WordList& wordList) {
    auto now = std::chrono::system_clock::now();
    auto t = std::chrono::system_clock::to_time_t(now);
    auto tm = std::gmtime(&t);

    uint32_t seed = (1900 + tm->tm_year) * 10000 +
                    (tm->tm_mon + 1) * 100 +
                    tm->tm_mday;

    std::mt19937 rng(seed);
    size_t index = rng() % wordList.getAnswerCount();
    return wordList.getAnswerWord(index);
}
```

## 9. Statistics Tracking System

### 9.1 Metrics Tracked
```cpp
struct Statistics {
    // Basic counters
    int gamesPlayed;              // Total games
    int gamesWon;                 // Won games
    int currentStreak;            // Current win streak
    int maxStreak;                // Longest streak

    // Guess distribution
    std::array<int, 6> guessDistribution;  // [1-6 guesses]

    // Time-based
    std::string lastPlayedDate;
    std::map<std::string, bool> dailyResults;  // Date → won/lost
};
```

### 9.2 Win Percentage Calculation
```cpp
float getWinPercentage() const {
    if (gamesPlayed == 0) return 0.0f;
    return (gamesWon * 100.0f) / gamesPlayed;
}
```

### 9.3 Streak Management
- Increment on win
- Reset to 0 on loss
- Only for consecutive days in daily mode
- Max streak is all-time best

### 9.4 Share Format (Emoji Grid)
```
Wordle 123 2/6

🟩🟩🟨⬜⬜
🟩🟩🟩🟩🟩

(Copyable emoji representation of game result)
```

## 10. Edge Cases & Special Rules

### 10.1 Repeated Letters
**Example 1:**
```
Answer: ROBOT
Guess:  FLOOR
Result: F-Gray, L-Gray, O-Yellow, O-Gray, R-Green
Reason: Only one O matches position 1 of the answer, second O marked gray
```

**Example 2:**
```
Answer: SPEED
Guess:  ERASE
Result: E-Green, R-Gray, A-Gray, S-Yellow, E-Yellow
Reason: Position 1 E matches green, position 5 E matches but wrong position (yellow)
```

### 10.2 Hard Mode Constraints
- After revealing a letter (green or yellow), it must be used in all subsequent guesses
- Enforced at validation time
- Optional toggle at game start

### 10.3 Invalid Input Handling
- Non-alphabetic characters: Show error, prompt retry
- Wrong length: "Must be 5 letters"
- Not in word list: "Not in word list"
- Hard mode violation: "You must use [revealed letters]"

## 11. Build & Dependency Strategy

### 11.1 Dependencies
- C++17 standard library only (no external libraries for core game)
- Optional: nlohmann/json for JSON serialization
- Testing: Google Test or Catch2

### 11.2 CMake Structure
```
wordle-cli/
├── CMakeLists.txt
├── src/
│   ├── CMakeLists.txt
│   ├── main.cpp
│   ├── wordle_game.cpp
│   ├── word_list.cpp
│   ├── guess_evaluator.cpp
│   ├── statistics_manager.cpp
│   ├── keyboard_visualizer.cpp
│   ├── ui_renderer.cpp
│   └── [headers in include/]
├── include/
│   ├── wordle_game.h
│   ├── word_list.h
│   ├── [other headers]
├── tests/
│   ├── CMakeLists.txt
│   ├── test_guess_evaluator.cpp
│   ├── test_word_list.cpp
│   ├── [other tests]
└── resources/
    ├── valid_words.txt
    └── answer_words.txt
```

## 12. File I/O & Resource Management

### 12.1 Word Lists Loading
- Load at startup into memory
- Valid words in unordered_set for O(1) lookup
- Answer words in vector for indexed access
- Embedded in binary or loaded from `resources/` directory

### 12.2 User Data Directory
```
$HOME/.wordle/
├── stats.json
├── config.json
└── games/
    └── [daily saves]
```

### 12.3 Error Handling
- Try/catch for file I/O
- Graceful degradation if stats can't load
- Warn user but continue game

## 13. Test Coverage Plan

### Phase 3 Tests
1. **GuessEvaluator Tests**
   - Exact match (all green)
   - All wrong letters (all gray)
   - Mixed feedback
   - Repeated letters (various scenarios)
   - Yellow letter positions

2. **WordList Tests**
   - Valid word lookup
   - Answer word lookup
   - Case insensitivity

3. **InputValidator Tests**
   - Length validation
   - Alphabetic check
   - Sanitization (trim, uppercase)

4. **DailyWordSelector Tests**
   - Same word for same day
   - Different word for different days
   - Deterministic behavior

5. **Statistics Tests**
   - Win/loss recording
   - Streak calculation
   - Percentage calculation
   - Guess distribution

6. **File I/O Tests**
   - Save/load statistics
   - Save/load game state

## 14. Runtime Flow

### Game Initialization
```
1. Create WordleGame instance
2. Load word lists from resources/
3. Load statistics from ~/.wordle/
4. Determine game mode (daily/practice)
5. Select answer word (daily: via selector, practice: random)
6. Initialize empty GameState
```

### Main Game Loop
```
while (!gameState.isGameOver) {
    1. Display current board state
    2. Display keyboard with guessed letters
    3. Get user input
    4. Validate input
    5. Evaluate guess (get feedback)
    6. Update game state
    7. Check win/loss condition
}
```

### Game Ending
```
1. Display final board
2. Show answer
3. Show statistics
4. Option to share results
5. Save game to stats
6. Option to play again / exit
```

## 15. Code Quality Standards

### Memory Management
- Use smart pointers (unique_ptr, shared_ptr) where necessary
- RAII principles throughout
- No raw pointers except temporary function parameters
- Run with valgrind to verify no leaks

### C++ Best Practices
- const correctness
- Move semantics where appropriate
- Range-based for loops
- No magic numbers (use named constants)
- Comprehensive error handling

### Code Organization
- Clear separation of concerns
- Each class has single responsibility
- Header files in `include/`
- Implementation in `src/`
- Minimal dependencies between modules

## 16. User Interface Flow

### Main Menu
```
WORDLE Clone v1.0

[D]aily Wordle
[P]lay Practice
[S]tatistics
[Q]uit

Enter choice:
```

### In-Game Display
```
WORDLE                              Guess 1 of 6

[G] [Y] [Y] [G] [G]
[Y] [Y] [G] [Y] [Y]
_ _ _ _ _

Q W E R T Y U I O P
A S D F G H J K L
  Z X C V B N M

Enter guess (5 letters):
```

### Results Screen
```
WORDLE Complete!

You won in 3 guesses!

[G] [Y] [Y] [G] [G]
[Y] [Y] [G] [Y] [Y]
[G] [G] [G] [G] [G]

Statistics:
- Games: 42
- Win%: 73.81%
- Streak: 5
- Distribution: 2,8,12,7,2,0

[C]opy results  [P]lay again  [M]enu
```

---

This HLD provides a comprehensive blueprint for implementing a feature-complete Wordle clone with proper architecture, clear interfaces, and room for expansion.
