# Reaction Timer Game - High-Level Design

## 1. System Architecture Overview

### 1.1 Core Components
```
┌─────────────────────────────────────────────────────────┐
│                    REACTION TIMER                       │
├─────────────────────────────────────────────────────────┤
│ ┌───────────────┐  ┌───────────────┐  ┌─────────────┐  │
│ │ Input Handler │  │ Timer Manager │  │ Test Engine │  │
│ │   (Latency)   │  │  (Precision)  │  │  (Logic)    │  │
│ └───────────────┘  └───────────────┘  └─────────────┘  │
├─────────────────────────────────────────────────────────┤
│ ┌───────────────┐  ┌───────────────┐  ┌─────────────┐  │
│ │  Statistics   │  │ Visualization │  │ Persistence │  │
│ │  (Analytics)  │  │  (Display)    │  │  (Storage)  │  │
│ └───────────────┘  └───────────────┘  └─────────────┘  │
├─────────────────────────────────────────────────────────┤
│ ┌──────────────────────────────────────────────────────┐│
│ │        Random Delay Generator / RNG                  ││
│ │    (Prevents Predictable Patterns / Secure)          ││
│ └──────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────┘
```

## 2. High-Precision Timing Architecture

### 2.1 Microsecond Precision Goals
- **Target Precision**: 1 microsecond (0.001ms)
- **Minimum Quantization**: 100 nanoseconds
- **Platform Support**: POSIX (Linux/macOS) and Windows
- **Drift Compensation**: Account for system clock drift over sessions

### 2.2 Timing Implementation Strategy

#### POSIX Systems (Linux, macOS)
```c
// Use CLOCK_MONOTONIC_RAW for drift-free measurements
// Provides unaffected monotonic timer
struct timespec start, end;
clock_gettime(CLOCK_MONOTONIC_RAW, &start);
// ... perform action ...
clock_gettime(CLOCK_MONOTONIC_RAW, &end);

uint64_t microseconds = (end.tv_sec - start.tv_sec) * 1000000 +
                        (end.tv_nsec - start.tv_nsec) / 1000;
```

#### Windows Systems
```c
// Use QueryPerformanceCounter for high-resolution timing
LARGE_INTEGER start, end, frequency;
QueryPerformanceFrequency(&frequency);
QueryPerformanceCounter(&start);
// ... perform action ...
QueryPerformanceCounter(&end);

uint64_t microseconds = ((end.QuadPart - start.QuadPart) * 1000000) /
                        frequency.QuadPart;
```

### 2.3 Timer Resolution Considerations
- System timer granularity varies (typically 1-15ms on consumer systems)
- Measure jitter and report it alongside results
- Use multiple consecutive measurements to detect system artifacts
- Account for context switches and CPU cache effects

### 2.4 Timing Accuracy Verification
- Calibration phase: Measure known 1-second intervals
- Drift detection: Compare against reference timebase
- Outlier filtering: Remove measurements > 3 sigma from mean
- Cold-start compensation: Ignore first 2-3 warmup measurements

## 3. Input Latency Minimization Strategy

### 3.1 Sources of Input Latency
1. **Keyboard Hardware**: 1-5ms (USB polling rate)
2. **OS Buffering**: 0-10ms (depends on scheduler)
3. **Terminal Driver**: 5-50ms (line buffering mode)
4. **Application Processing**: 0-1ms (software overhead)
- **Total Typical Latency**: 6-66ms (human perception is ~100ms)

### 3.2 Latency Reduction Techniques
```
Raw Mode Implementation:
┌──────────────────┐
│ Raw Terminal     │ Disable line buffering
│ Mode (tcsetattr) │ Capture individual keystrokes
│ Enable           │ Non-canonical input mode
└──────────────────┘

Non-blocking Detection:
┌──────────────────┐
│ fcntl(O_NONBLOCK)│ Don't wait for input
│ OR               │ Poll input availability
│ select()/poll()  │ Use system multiplexing
└──────────────────┘

Timing Window:
┌──────────────────┐
│ High-frequency   │ Sample input every 1ms
│ Input Sampling   │ Catch edge timing
│ (poll loop)      │ Minimize detection delay
└──────────────────┘
```

### 3.3 Input Handler Architecture
```c
typedef struct {
    struct termios original_tty;  // Saved terminal state
    int raw_mode_enabled;          // Flag for cleanup
    uint64_t key_timestamp;        // Capture time
    char input_key;                // What was pressed
} InputHandler;

// Implementation approach:
// 1. Save original terminal settings
// 2. Set raw mode (tcsetattr with CBREAK + no echo)
// 3. Use select() with 1ms timeout for polling
// 4. Timestamp immediately upon detection
// 5. Restore terminal on exit
```

### 3.4 Platform-Specific Optimizations
- **Linux**: Use epoll() for sub-millisecond latency detection
- **macOS**: Use kqueue() for efficient event notification
- **Windows**: Use WaitForMultipleObjects() with ~1ms timeout

## 4. Statistical Analysis System

### 4.1 Running Statistics
```
Track in real-time:
- Count (n)
- Sum (Σx)
- Sum of squares (Σx²)
- Minimum
- Maximum
- All individual values for percentiles

Calculated on demand:
- Mean = Σx / n
- Variance = (Σx² - (Σx)²/n) / (n-1)  [Welford's method]
- Std Dev = √Variance
- Percentiles (P25, P50, P75, P90, P95, P99)
- Median = P50
```

### 4.2 Percentile Calculation Methods
```c
// For accurate percentiles with small datasets (n < 1000)
// Use Linear Interpolation Method (Type 7 / R-7)
double percentile = x[k] + g * (x[k+1] - x[k])
// where: h = (n-1) * p/100
//        k = floor(h), g = h - k

// Pre-sort array for O(1) lookup after initial O(n log n) sort
```

### 4.3 Outlier Detection
```
Tukey Fence Method:
- Q1 = 25th percentile
- Q3 = 75th percentile
- IQR = Q3 - Q1
- Lower fence = Q1 - 1.5 * IQR
- Upper fence = Q3 + 1.5 * IQR
- Mark measurements outside fences as outliers
- Report percentage of outliers (fatigue indicator)
```

### 4.4 Fatigue Analysis
```
Segment session into blocks of 10 measurements:
- Block 1 avg: t1
- Block 2 avg: t2
- ...
- Block N avg: tN

Fatigue Score = (tN - t1) / t1 * 100
- Negative = Improvement
- 0-5% = Normal
- 5-10% = Mild fatigue
- >10% = Significant fatigue
```

## 5. Multiple Test Types Design

### 5.1 Test Type Definitions

#### Type 1: Simple Reaction
```
Mechanics:
- Show "GET READY..."
- Random delay (1-5 seconds)
- Display "GO!"
- Measure time to any keypress
- Record in milliseconds

Difficulty: Baseline
Measures: Raw reaction speed
```

#### Type 2: Choice Reaction
```
Mechanics:
- Display 4 keys: A S D F
- Highlight one key randomly
- Random delay (1-5 seconds)
- Measure time to press CORRECT key
- Penalize wrong key (-50ms) or restart

Difficulty: Medium (requires visual processing)
Measures: Processing time + reaction
```

#### Type 3: Color Reaction
```
Mechanics:
- Display colored boxes (Red/Green/Blue/Yellow)
- Instruction: "React to RED only"
- Random color appears with delay
- False alarms penalized (-100ms)

Difficulty: High (requires discrimination)
Measures: Selective attention + reaction
```

#### Type 4: Sequence Memory
```
Mechanics:
- Show sequence of 3-5 lights/beeps: "1-3-2-4-1"
- Delay, then prompt: "Reproduce sequence"
- Count keypresses matching sequence
- Accuracy score: (correct/total) * 100%

Difficulty: Very High (working memory)
Measures: Memory + accuracy
```

#### Type 5: Pattern Recognition
```
Mechanics:
- Display pattern: Random dots in grid
- Target pattern: "React only to X shape"
- Multiple patterns shown with delays
- React only to matching patterns

Difficulty: High (pattern processing)
Measures: Pattern recognition speed
```

#### Type 6: Audio Reaction
```
Mechanics:
- Silence (variable delay 1-5s)
- Bell sound (ASCII BEL 0x07)
- Measure reaction time to audio cue

Difficulty: Medium
Measures: Audio processing latency
```

#### Type 7: Inhibition Test (Go/No-Go)
```
Mechanics:
- Show symbols: "X = Press, O = Don't Press"
- Random sequence of X's and O's
- False alarms are wrong
- Track accuracy and reaction time

Difficulty: Very High (impulse control)
Measures: Executive function + reaction
```

### 5.2 Test Generation Parameters
```c
typedef struct {
    int test_type;           // 1-7 as above
    int num_trials;          // 5, 10, 50, or continuous
    int min_delay_ms;        // 1000 - 5000
    int max_delay_ms;
    uint32_t random_seed;    // For reproducibility
    int difficulty_level;    // 1-10 (affects delays/complexity)
} TestConfig;
```

## 6. Data Persistence & Leaderboards

### 6.1 Storage Format
```
Directory Structure:
~/.reaction-timer/
├── stats/
│   ├── session-YYYY-MM-DD-HHmmss.json
│   ├── daily.json (aggregated daily stats)
│   └── all-time.json
├── leaderboards/
│   ├── simple-reaction.json
│   ├── choice-reaction.json
│   └── ... (one per test type)
└── config.json
```

### 6.2 Session JSON Format
```json
{
  "session_id": "20240630-142530",
  "date": "2024-06-30T14:25:30Z",
  "test_type": "simple_reaction",
  "test_mode": "standard_10",
  "results": [
    {"trial": 1, "reaction_time_ms": 287, "outlier": false},
    {"trial": 2, "reaction_time_ms": 294, "outlier": false},
    {"trial": 3, "reaction_time_ms": 281, "outlier": false}
  ],
  "stats": {
    "average_ms": 287,
    "median_ms": 287,
    "std_dev_ms": 6,
    "min_ms": 281,
    "max_ms": 294,
    "percentile_25": 284,
    "percentile_75": 290,
    "percentile_90": 293,
    "outlier_count": 0
  },
  "metadata": {
    "system": "Linux",
    "time_of_day": "afternoon",
    "duration_seconds": 142
  }
}
```

### 6.3 Leaderboard Entry Format
```json
{
  "rank": 1,
  "user": "player_name",
  "date": "2024-06-30",
  "average_ms": 218,
  "test_mode": "standard_10",
  "test_type": "simple_reaction",
  "device": "local",
  "region": "global"
}
```

## 7. Visual Feedback Systems

### 7.1 Display Architecture
```
┌─────────────────────────────────────────────────┐
│ Display Layer (ANSI codes + Unicode)            │
├─────────────────────────────────────────────────┤
│ - Color codes (ANSI 256-color)                  │
│ - Box drawing (Unicode Box-drawing characters)  │
│ - Progress bars (Block characters: ░▒▓)         │
│ - Graph plotting (Braille/block patterns)       │
└─────────────────────────────────────────────────┘

Constraints:
- Compatible with 80x24 minimum terminal
- Graceful degradation for limited terminals
- Clear mode before each test
- Consistent color scheme
```

### 7.2 Color Scheme
```
Status Indicators:
- Green (#00AA00): Success / Fast reaction
- Yellow (#FFAA00): Caution / Slow reaction
- Red (#FF0000): Error / Too slow
- Blue (#0066FF): Information / Instruction
- Gray (#666666): Neutral / Waiting

Contextual Usage:
- Test status: Blue box with white text
- Results: Green for good, Yellow for medium, Red for slow
- Statistics: Neutral gray for data presentation
- Graphs: Mixed colors for visual interest
```

### 7.3 Animation Frames

#### Ready State
```
Frame 1 (0.1s): "  GET READY...  "
Frame 2 (0.1s): " GET READY...  "
Frame 3 (0.1s): "GET READY...  "
(pulsing effect with changing spaces)
```

#### Waiting State
```
Frame 1: "      .       "
Frame 2: "      ..      "
Frame 3: "      ...     "
Frame 4: "      ....    "
(rotating dots during random delay)
```

#### Result Celebration
```
Great (< 250ms):
        ⚡ EXCELLENT! ⚡

Good (< 300ms):
        ✓ GOOD! ✓

Average (< 350ms):
        → Average

Slow (> 350ms):
        Slow...
```

## 8. Performance Measurement Methodology

### 8.1 Timing Validation
```c
typedef struct {
    uint64_t system_tick;           // Raw system timestamp
    uint64_t microseconds;          // Converted to microseconds
    double milliseconds;            // Converted to milliseconds
    uint8_t is_valid;               // Passed sanity checks
    uint8_t is_outlier;             // Statistical outlier
    uint8_t is_false_start;         // Reaction too early
} TimingRecord;

// Validation checks:
// 1. is_valid = (microseconds > 0 && microseconds < 10000000) ? 1 : 0
// 2. is_false_start = (microseconds < expected_delay - 50000) ? 1 : 0
// 3. is_outlier = (abs(ms - mean) > 3 * stdev) ? 1 : 0
```

### 8.2 Benchmark Methodology
```
Warm-up Phase:
- Run 5 dummy reactions (not counted)
- Allows CPU cache to warm up
- Stabilizes scheduler behavior

Measurement Phase:
- Run configured number of trials
- Record each in microsecond precision
- Track any anomalies

Cool-down Phase:
- Optional fatigue assessment
- Session summary generation
```

### 8.3 Performance Reporting
```
Metrics to Report:
- Average reaction time (ms)
- Median reaction time (ms)
- Standard deviation (ms)
- Best time (ms)
- Worst time (ms)
- Consistency score (1-100)
- Percentile vs population (1-100%)
- Session duration (seconds)
- Fatigue trend (improvement %)
- Number of outliers (count)
- False start count (count)
```

## 9. Cross-Platform Timing Considerations

### 9.1 Platform Differences
```
Linux:
- CLOCK_MONOTONIC_RAW: ~1ns resolution
- Context switches: Frequent
- Timer drift: Negligible
- Recommended: Use raw mode + epoll()

macOS:
- mach_absolute_time(): ~40ns resolution
- Context switches: Moderate
- Timer drift: Minimal
- Recommended: Use raw mode + kqueue()

Windows:
- QueryPerformanceCounter: ~100ns resolution
- Context switches: Frequent
- Timer drift: Can accumulate over time
- Recommended: Synchronize with system timer periodically

Common:
- All support microsecond resolution
- All subject to CPU clock frequency scaling
- All affected by system load
```

### 9.2 Timer Calibration Per Platform
```c
// POSIX calibration
void calibrate_posix_timer(void) {
    struct timespec ts, te;
    clock_gettime(CLOCK_MONOTONIC_RAW, &ts);
    sleep(1);
    clock_gettime(CLOCK_MONOTONIC_RAW, &te);
    // Verify close to 1,000,000 microseconds
}

// Windows calibration
void calibrate_windows_timer(void) {
    LARGE_INTEGER start, end, freq;
    QueryPerformanceFrequency(&freq);
    QueryPerformanceCounter(&start);
    Sleep(1000);
    QueryPerformanceCounter(&end);
    // Verify (end - start) / freq ≈ 1 second
}
```

## 10. System Requirements

### 10.1 Minimum Requirements
- **CPU**: Single core capable of >1GHz
- **RAM**: 10MB for session history
- **Terminal**: 80x24 characters, 256-color support
- **OS**: Linux, macOS, or Windows Vista+
- **C Compiler**: GCC 5.0+, Clang 3.5+, or MSVC 2015+

### 10.2 Optimal Requirements
- **CPU**: Multi-core with >2GHz
- **RAM**: 50MB for large leaderboard datasets
- **Terminal**: 120x40+ characters, true color (24-bit)
- **OS**: Modern distribution (Ubuntu 18.04+, macOS 10.12+, Windows 10+)
- **Compiler**: GCC 9.0+, Clang 10+, or MSVC 2019+

### 10.3 Supported Platforms
```
Primary:
- Linux (x86_64, ARM64)
- macOS (Intel, Apple Silicon)
- Windows (x86_64)

Secondary (compatibility layer):
- FreeBSD
- OpenBSD
- WSL2
```

## 11. Error Handling Strategy

### 11.1 Critical Errors (Halt Execution)
```
- Timer initialization failed
- Terminal mode change failed
- Memory allocation failed
- Corrupt session data
```

### 11.2 Non-Critical Errors (Log & Continue)
```
- Failed to write session to disk
- Leaderboard sync failed
- Color not available in terminal
- File permission denied
```

### 11.3 Recovery Strategies
```
Timer Fail:
- Attempt fallback timer mechanism
- Use lower precision if needed
- Warn user of accuracy limitations

Terminal Fail:
- Gracefully degrade to basic mode
- Disable animations and colors
- Continue with functional core
```

## 12. Implementation Phases

### Phase 1: Core Timing (Days 1-2)
- Timer abstraction layer
- Platform-specific implementations
- Calibration system
- Unit tests for timing accuracy

### Phase 2: Input & Display (Days 3-4)
- Terminal mode handling
- Input latency measurement
- Basic UI framework
- Color support

### Phase 3: Game Logic (Days 5-6)
- Test engine implementation
- All 7 test types
- Basic statistics
- Trial loop

### Phase 4: Advanced Features (Days 7-8)
- Full statistics suite
- Leaderboards
- Data persistence
- Performance analysis

### Phase 5: Polish & Testing (Days 9-10)
- Comprehensive test suite
- Performance benchmarking
- Documentation
- Cross-platform validation

## 13. Design Decisions

### Why Microsecond Precision?
- Human reaction time variance is in microseconds
- System jitter is also microsecond-scale
- Provides future-proofing for improved input hardware
- Matches professional reaction time measurement systems (like those used in esports)

### Why Raw Terminal Mode?
- Reduces input latency from ~20ms to ~1ms
- Provides consistent baseline across platforms
- Professional timing measurement requires this

### Why Multiple Test Types?
- Different neural pathways: Simple (baseline), Choice (processing), Inhibition (control)
- Provides comprehensive assessment of reflexes
- Allows training of specific skills
- More engaging for repeated use

### Why Persistent Data?
- Enables trend analysis over time
- Motivates improvement through visible progress
- Leaderboards create competitive engagement
- Training analytics guide practice

## 14. Success Criteria

### Timing Accuracy
- Single measurement error < 5ms (average)
- Consecutive runs produce consistent results
- Calibration within 1% of actual time
- No systematic bias per platform

### Input Responsiveness
- Detect keypress within 5ms of physical event
- No noticeable lag in visual feedback
- Platform consistency ±2ms

### Statistics Quality
- Percentile calculations match professional tools
- Outlier detection effective (90%+ accuracy)
- Fatigue trends visible within 10 trials

### User Experience
- Complete test cycle in <2 minutes
- Clear, professional visual presentation
- Responsive to all input
- Graceful error handling

## 15. Known Limitations & Future Work

### Current Limitations
- Single-player mode only
- No network leaderboard sync
- Text-only visualization (no graphics library)
- Limited to standard terminal colors
- No mouse input support
- No external device synchronization

### Future Enhancements
- Cloud leaderboard integration
- Multi-player competition mode
- Voice/external sensor input
- Graphical UI version
- Mobile app integration
- AI-powered training recommendations
- Biometric correlation (heart rate, stress)
