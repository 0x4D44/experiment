# Blackjack CLI - High Level Design

## 1. Project Overview
A professional-grade Blackjack game for the command line using Zig, implementing authentic casino rules with visual card representation, betting system, and comprehensive statistics tracking.

## 2. Game Architecture

### 2.1 Core Components
```
Main Game Loop
├── Deck Manager (shuffling, dealing, composition)
├── Hand Evaluator (value calculation, special cases)
├── Game State Machine (betting, playing, dealer, payout)
├── Bankroll Manager (balance, bets, payouts)
├── Player Interface (input handling, rendering)
└── Statistics Engine (tracking, persistence)
```

### 2.2 Module Structure
```
src/
├── main.zig              (Game loop, orchestration)
├── deck.zig              (Card definitions, deck management)
├── hand.zig              (Hand evaluation, card combination)
├── game.zig              (Game state, rules engine)
├── ui.zig                (Terminal rendering, ASCII art)
├── bankroll.zig          (Bet management, payouts)
├── stats.zig             (Statistics tracking, persistence)
├── config.zig            (Game configuration constants)
└── game_test.zig         (Comprehensive test suite)
```

## 3. Card and Deck Management

### 3.1 Card Representation
- Standard 52-card deck (13 ranks × 4 suits)
- Ranks: A(1/11), 2-9, 10, J, Q, K
- Suits: ♠, ♥, ♦, ♣
- Card structure: rank + suit + blackjack value
- Support for multiple deck shoes (1-8 decks)

### 3.2 Deck Operations
- **Initialization**: Create standard 52-card deck
- **Shuffling**: Fisher-Yates algorithm for fair randomization
  - Reshuffle at 75% penetration to prevent card counting
  - True random number generation using system entropy
- **Dealing**: Draw cards from deck (return from end for efficiency)
- **Penetration Tracking**: Monitor cards dealt vs. cards remaining

### 3.3 Card Counting Prevention
- Track penetration level (cards used / total cards)
- Auto-reshuffle when penetration > 75%
- Notification to player: "Deck has been reshuffled"
- Multiple deck shoe support (default: 4 decks)

## 4. Hand Evaluation Logic

### 4.1 Hand Value Calculation
- **Hard Hand**: No aces, or aces counted as 1
- **Soft Hand**: Aces counted as 11 (flexible ace counting)
- **Blackjack**: 21 with exactly 2 cards (Ace + 10-value card)
  - Must be detected at deal time
  - Not possible after hitting

### 4.2 Ace Handling (Complex Logic)
```
Algorithm:
1. Sum all cards, counting aces as 11
2. While sum > 21 AND soft aces exist:
   - Convert one ace from 11 to 1 (subtract 10)
3. Return (value, is_soft)

Examples:
- A,5: soft 16 (11+5)
- A,5,5: hard 21 (1+5+5) [only 1 ace becomes hard]
- A,A,9: hard 21 (1+1+9)
- A,K: blackjack (11+10=21, exactly 2 cards)
- A,K,A: hard 12 (1+10+1) [soft ace becomes hard]
```

### 4.3 Special Conditions
- **Blackjack Detection**: Exactly 2 cards with sum of 21
- **Bust**: Value > 21
- **Push**: Player and dealer tie
- **21 non-blackjack**: Hit to 21 (loses to blackjack)

## 5. Dealer AI Logic

### 5.1 Dealer Rules
- **Stand on 17**: Never hit on hard 17 or higher
- **Soft 17 Rule**: Hit on soft 17 (A+6), stand on soft 18+
- **No Decision Making**: Purely mechanical rule-based
- **Visible Hand**: Reveal hole card before dealer plays

### 5.2 Dealer Play Sequence
1. After all players finish their hands
2. Flip hole card (reveal full hand)
3. If dealer has blackjack: resolve immediately
4. Otherwise: apply hit/stand rules
5. Dealer plays to completion before payouts

## 6. Betting System and Bankroll Management

### 6.1 Bankroll Structure
- **Initial Balance**: $1000
- **Minimum Bet**: $5
- **Maximum Bet**: $500
- **Persistent Storage**: Save to JSON file between sessions
- **Currency**: USD ($)

### 6.2 Bet Management
- **Single Hand Betting**: One bet per hand (before split)
- **Split Handling**: Each split hand has equal bet amount
- **Double Down**: Doubles the original bet
- **Insurance**: Half of original bet (2:1 payout)
- **Bankroll Validation**: Cannot bet more than available balance

### 6.3 Payout Calculations
```
Outcomes:
- Player Blackjack vs Dealer non-BJ: Bet × 2.5 (3:2 payout)
- Player 21 vs Dealer BJ: Push (original bet returned)
- Player 21 vs Dealer non-BJ: Bet × 2 (1:1 payout)
- Player Win: Bet × 2
- Push: Bet × 1 (returned)
- Player Bust: Bet × 0 (lost)
- Dealer Bust (player didn't bust): Bet × 2
- Insurance Win (dealer BJ): Insurance bet × 3
- Insurance Lose: Insurance bet × 0
```

### 6.4 State Validation
- Check sufficient funds before allowing bet
- Check bet within range [$5, $500]
- Prevent betting more than bankroll
- Track win/loss per hand for statistics

## 7. Game State Machine

### 7.1 States
```
INITIAL
  ├─→ WAITING_FOR_BET
  │   └─→ BET_PLACED
  │       └─→ CARDS_DEALT
  │           ├─→ PLAYER_TURN
  │           │   ├─→ HIT_PHASE
  │           │   ├─→ DOUBLE_PHASE
  │           │   ├─→ SPLIT_PHASE
  │           │   └─→ STAND_PHASE
  │           │       └─→ DEALER_TURN
  │           │           ├─→ INSURANCE_PHASE
  │           │           ├─→ DEALER_PLAY
  │           │           └─→ OUTCOME_DETERMINATION
  │           └─→ HAND_COMPLETE
  └─→ GAME_OVER
```

### 7.2 State Transitions
- **BET_PLACED**: Wait for dealing confirmation or amount change
- **CARDS_DEALT**: Move to PLAYER_TURN or handle immediate outcomes (both blackjacks)
- **PLAYER_TURN**: Accept H/S/D/P/I (context-dependent)
- **DEALER_TURN**: After player stands or busts
- **OUTCOME_DETERMINATION**: Calculate payouts, update bankroll
- **HAND_COMPLETE**: Next hand prompt or GAME_OVER

### 7.3 Immediate Outcomes (After Deal)
- **Both Blackjack**: Push (draw)
- **Player Blackjack Only**: Player wins (3:2 payout)
- **Dealer Blackjack Only**: Player loses
- **No Blackjack**: Continue to player turn

## 8. Multi-Hand Support (Splitting)

### 8.1 Split Rules
- **Eligible**: Exactly 2 cards with same rank
- **Limit**: Maximum 4 hands (standard casino rule)
- **Each Hand Plays**: Independently after split
- **Same Bet**: Each split hand gets equal bet amount
- **Sequence**: Play hands left to right

### 8.2 Split Aces Special Case
- **Can Split**: Yes
- **Cards per Hand**: Exactly 1 card (can't hit)
- **Value**: A+10 is blackjack (not 21)
- **Payout**: 1:1 (not 3:2)
- **Restriction**: Cannot split after first split (house rules)

### 8.3 Soft Hand Splitting
- **Example**: A,6 (soft 17) can split
- **Result**: Two hands each starting with A,X

## 9. Game Features and Options

### 9.1 Insurance Bet
- **Trigger**: Dealer shows Ace
- **Amount**: Exactly half of original bet
- **Timing**: Before dealer checks blackjack
- **Payout**: 2:1 (Insurance bet × 3 total if dealer BJ)
- **Loss**: Insurance bet × 0 if dealer no BJ

### 9.2 Double Down
- **Eligible Ranks**: Initial hand totaling 9, 10, or 11
- **Restriction**: Only on 2-card hand
- **Action**: Double the bet, receive exactly 1 card, stand immediately
- **Not Splitting**: Cannot split after doubling down

### 9.3 Surrender (Optional)
- **Status**: Not implemented (standard casino blackjack)
- **Alternative**: Player simply chooses to play out hand

### 9.4 No-Hole-Card Rules
- **Dealer shows both cards initially**
- **Simplification**: Reduces complexity, matches some casinos
- **Insurance**: Can still be offered (dealer checks for BJ)

## 10. User Interface and Rendering

### 10.1 Card Display (ASCII Art)
```
Rank representation:
╔═══╗   2-9: digit or J/Q/K/A
║ K ║
║ ♥ ║
╚═══╝

Hidden card: ┌───┐
             │ ? │
             └───┘

Card values shown:
- Pip cards (2-9): numeric value
- Face cards (J,Q,K): "10" value
- Ace: "A" with flexible evaluation
```

### 10.2 Screen Layout
```
┌─────────────────────────────────────┐
│   BLACKJACK - Casino Style          │
├─────────────────────────────────────┤
│ Bankroll: $1000 | Bet: $25          │
│ Hands: 1 | Wins: 0 | Losses: 0      │
├─────────────────────────────────────┤
│                                     │
│ Dealer's Hand (17):                 │
│ ┌───┐ ┌───┐                         │
│ │10♠│ │ 7♦│                         │
│ └───┘ └───┘                         │
│                                     │
│ Your Hand (18):                     │
│ ┌───┐ ┌───┐                         │
│ │ K♥│ │ 8♣│                         │
│ └───┘ └───┘                         │
│                                     │
│ (H)it (S)tand (D)ouble >            │
└─────────────────────────────────────┘
```

### 10.3 Input Handling
- **Single Character Input**: H, S, D, P, I, B, Q
- **Bet Adjustment**: B command to modify bet
- **Case Insensitive**: Accept both upper and lower
- **Input Validation**: Reject invalid commands
- **Context Sensitivity**: Show available options only

### 10.4 Animations
- **Card Dealing**: Slight delay between cards (100ms)
- **Dealer Play**: Show each dealer hit with delay
- **Bust/Win**: Highlight result with visual emphasis
- **Smooth Movement**: No jarring transitions

## 11. Statistics and Persistence

### 11.1 Tracked Metrics
```
Statistics Object:
- total_hands: u32 (lifetime hands played)
- total_wins: u32
- total_losses: u32
- total_pushes: u32
- total_blackjacks: u32
- total_profit: i64 (in cents, can be negative)
- biggest_win: i64
- biggest_loss: i64
- total_bets_placed: u64 (sum of all bets)
- win_streak: i32 (current streak, negative = loss streak)
- max_win_streak: i32
- max_loss_streak: i32
```

### 11.2 Session Statistics Display
```
╔════════════════════════════════════╗
║      SESSION STATISTICS            ║
╠════════════════════════════════════╣
║ Hands Played:        42            ║
║ Wins:                18 (42.9%)    ║
║ Losses:              20 (47.6%)    ║
║ Pushes:              4  (9.5%)     ║
║ Blackjacks:          5             ║
║                                    ║
║ Profit/Loss:         -$125.50      ║
║ Biggest Win:         +$250.00      ║
║ Biggest Loss:        -$75.50       ║
║ Win Streak:          -3 hands      ║
║ Longest Streak:      +6 hands      ║
╚════════════════════════════════════╝
```

### 11.3 Persistence
- **File Format**: JSON (human-readable)
- **Location**: `./.blackjack_stats.json`
- **Auto-Save**: After each hand completes
- **Load on Startup**: Restore previous session stats
- **Path Format**: Relative to current working directory

### 11.4 Bankroll Persistence
- **File Format**: Plain text or JSON
- **Location**: `./.blackjack_bankroll`
- **Load on Startup**: Restore previous bankroll
- **Update**: After each hand payout

## 12. Configuration and Constants

### 12.1 Game Constants
```zig
const INITIAL_BANKROLL = 100_000; // $1000.00 in cents
const MIN_BET = 500;              // $5.00
const MAX_BET = 50_000;           // $500.00
const DECK_COUNT = 4;             // 4-deck shoe
const RESHUFFLE_PENETRATION = 0.75; // 75% threshold
const CARD_DELAY_MS = 100;        // Dealing animation
const DEALER_DELAY_MS = 500;      // Dealer play animation
```

### 12.2 Configurable Options
- Number of decks (1-8)
- Minimum/maximum bets
- Soft 17 rule (dealer hits/stands)
- Animation delays
- Statistics file location

## 13. Error Handling and Edge Cases

### 13.1 Edge Cases
- **Exact 21 with 3+ cards**: Not blackjack, valid win
- **Multiple splits with 21**: Each is 1:1, not 3:2
- **Split aces with 10**: Blackjack-like but 1:1 payout
- **Bankroll < minimum bet**: Game ends
- **Deck exhaustion**: Immediate reshuffle
- **All hands split**: Play sequential hands correctly

### 13.2 Error Conditions
- **Invalid input**: Ignore gracefully
- **Insufficient funds**: Prevent betting
- **Memory allocation**: Handle gracefully (exit with message)
- **File I/O errors**: Continue game, warn user
- **Random number generation**: Fallback to deterministic if needed

### 13.3 Invariants
- Sum of all hands <= deck count × 52
- Bankroll always >= 0
- Current bet <= bankroll
- No duplicate cards in active hand
- Exactly one game state active at a time

## 14. Testing Strategy

### 14.1 Unit Tests
- **Deck Tests**: Shuffling randomness, card distribution
- **Hand Value Tests**: All ace combinations, edge cases
- **Payout Tests**: All outcome types, fractional cents
- **Split Tests**: Eligibility, multiple splits, ace splits
- **Dealer Logic Tests**: Hard 17, soft 17, edge cases
- **Bankroll Tests**: Bet validation, payout application

### 14.2 Integration Tests
- **Full Hand Plays**: Deal to completion, payout
- **Multiple Hands**: Split scenarios with payouts
- **Bankroll Depletion**: Game ending conditions
- **Statistics**: Tracking and persistence

### 14.3 Test Coverage Target
- >80% line coverage
- >90% critical function coverage
- All edge cases documented

## 15. Performance Considerations

### 15.1 Memory Management
- Allocate deck once (no repeated allocations)
- Use stack for temporary values
- Free resources at game end
- No memory leaks in error paths

### 15.2 Runtime Performance
- Shuffle: O(n) Fisher-Yates
- Hand evaluation: O(n) where n=cards in hand (~10 max)
- Dealing: O(1) amortized
- State transitions: O(1)
- Target: Responsive gameplay (<10ms per action)

### 15.3 I/O Optimization
- Batch terminal writes
- Buffer output when possible
- Minimize file I/O (load once, save once per hand)

## 16. Future Enhancements (Not in MVP)

### 16.1 Advanced Features
- Basic strategy hints (optimal plays)
- Card counting practice mode (running/true count)
- Surrendering (early/late)
- Side bets (optional casino side bets)
- Multiplayer support (multiple players vs dealer)
- Statistics charts (win rate over time)
- Undo last action
- Save/restore game state

### 16.2 UI Improvements
- Color support for suits
- Sound effects (terminal bell)
- Smoke animation on bust
- Chip stack visualization
- Player avatar/name

### 16.3 Accessibility
- High contrast mode
- Screen reader support
- Keyboard customization
- Larger text option

## 17. Implementation Checklist

- [ ] Phase 1: HLD Complete (this document)
- [ ] Phase 2: HLD Review
- [ ] Phase 3: Comprehensive Test Suite
- [ ] Phase 4: Core Implementation
  - [ ] Deck module with shuffling
  - [ ] Hand evaluation logic
  - [ ] Game state machine
  - [ ] Bankroll management
  - [ ] Dealer AI
  - [ ] UI rendering
  - [ ] Statistics tracking
- [ ] Phase 5: Build and Package
  - [ ] build.zig configuration
  - [ ] README with rules
  - [ ] Release builds
  - [ ] Documentation

## 18. Zig-Specific Considerations

### 18.1 Type System
- Use enum for GameState, CardRank, CardSuit
- Struct for Card, Hand, GameState
- Tagged union for maybe types
- Array types for fixed-size collections

### 18.2 Memory Model
- Stack allocation for game state
- Allocator for dynamic arrays (deck, hands)
- Proper defer for cleanup
- No global mutable state except RNG

### 18.3 Error Handling
- Error union types for fallible operations
- try! for error propagation
- catch blocks for recovery
- No panics in user-facing code

### 18.4 Concurrency
- Single-threaded implementation
- No async/await needed
- No thread safety issues

---

**Document Version**: 1.0
**Status**: Ready for Review
**Last Updated**: 2025-10-31
