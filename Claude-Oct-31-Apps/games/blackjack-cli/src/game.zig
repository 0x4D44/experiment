/// Core game logic and state management
/// Implements the game state machine and rules engine

const std = @import("std");
const mem = std.mem;
const config = @import("config.zig");
const deck = @import("deck.zig");
const hand = @import("hand.zig");

/// Main game context
pub const Game = struct {
    allocator: mem.Allocator,
    state: config.GameState = .INITIAL,
    bankroll: i64,
    current_bet: i64 = 0,
    insurance_bet: i64 = 0,

    deck_manager: deck.Deck,
    dealer_hand: hand.Hand,
    player_hands: std.ArrayList(hand.Hand),
    current_hand_index: u32 = 0,

    statistics: config.Statistics = .{},
    can_split: bool = true,
    can_double: bool = true,
    allow_insurance: bool = false,

    /// Initialize a new game
    pub fn init(allocator: mem.Allocator, initial_bankroll: ?i64) !Game {
        const bankroll = initial_bankroll orelse config.GameConfig.INITIAL_BANKROLL;
        const game_deck = try deck.Deck.init(allocator, config.GameConfig.DECK_COUNT);

        var player_hands = std.ArrayList(hand.Hand).init(allocator);
        try player_hands.ensureTotalCapacity(config.GameConfig.MAX_SPLIT_HANDS);

        return Game{
            .allocator = allocator,
            .bankroll = bankroll,
            .deck_manager = game_deck,
            .dealer_hand = hand.Hand.init(allocator),
            .player_hands = player_hands,
        };
    }

    /// Deinitialize the game
    pub fn deinit(self: *Game) void {
        self.deck_manager.deinit();
        self.dealer_hand.deinit();

        for (self.player_hands.items) |*h| {
            h.deinit();
        }
        self.player_hands.deinit();
    }

    /// Place a bet
    /// Returns error if bet is invalid
    pub fn placeBet(self: *Game, bet_amount: i64) !void {
        // Validate bet amount
        if (bet_amount < config.GameConfig.MIN_BET) {
            return error.BetTooLow;
        }

        if (bet_amount > config.GameConfig.MAX_BET) {
            return error.BetTooHigh;
        }

        if (bet_amount > self.bankroll) {
            return error.InsufficientFunds;
        }

        self.current_bet = bet_amount;
        self.state = .BET_PLACED;
    }

    /// Deal initial hands (2 cards each to player and dealer)
    pub fn dealInitialHands(self: *Game) !void {
        // Clear previous hands
        self.dealer_hand.clear();
        for (self.player_hands.items) |*h| {
            h.clear();
        }
        self.player_hands.clearRetainingCapacity();

        // Check for reshuffle
        if (self.deck_manager.checkAndReshuffle()) {
            // Deck was reshuffled
        }

        // Create initial player hand
        const initial_hand = hand.Hand.init(self.allocator);
        try self.player_hands.append(initial_hand);
        self.current_hand_index = 0;

        // Deal 2 cards to player
        if (self.deck_manager.dealCard()) |card| {
            try self.player_hands.items[0].addCard(card);
        } else {
            return error.DeckEmpty;
        }

        if (self.deck_manager.dealCard()) |card| {
            try self.player_hands.items[0].addCard(card);
        } else {
            return error.DeckEmpty;
        }

        // Deal 2 cards to dealer
        if (self.deck_manager.dealCard()) |card| {
            try self.dealer_hand.addCard(card);
        } else {
            return error.DeckEmpty;
        }

        if (self.deck_manager.dealCard()) |card| {
            try self.dealer_hand.addCard(card);
        } else {
            return error.DeckEmpty;
        }

        self.state = .CARDS_DEALT;

        // Check for blackjacks and insurance opportunity
        if (self.dealer_hand.cards.items[0].isAce()) {
            self.allow_insurance = true;
            self.state = .INSURANCE_OFFERED;
        } else if (self.shouldEndHandImmediately()) {
            // Both blackjack or dealer BJ, determine outcome
            self.state = .OUTCOME_DETERMINATION;
        } else {
            self.state = .PLAYER_TURN;
        }
    }

    /// Check if hand should end immediately after deal (both BJ or dealer BJ)
    fn shouldEndHandImmediately(self: Game) bool {
        const player = self.player_hands.items[self.current_hand_index];

        // Dealer blackjack (always ends hand)
        if (self.dealer_hand.isBlackjack()) {
            return true;
        }

        // Player blackjack (unless dealer also has it, which is checked above)
        if (player.isBlackjack()) {
            return true;
        }

        return false;
    }

    /// Hit (take another card) on current hand
    pub fn hitCurrentHand(self: *Game) !void {
        if (self.deck_manager.dealCard()) |card| {
            try self.player_hands.items[self.current_hand_index].addCard(card);

            // Check for bust or completion
            const current = self.player_hands.items[self.current_hand_index];
            if (current.isBust()) {
                // Try to move to next hand
                if (self.current_hand_index + 1 < self.player_hands.items.len) {
                    self.current_hand_index += 1;
                    self.state = .PLAYER_TURN;
                } else {
                    // All hands played
                    self.state = .DEALER_TURN;
                }
            }
        } else {
            return error.DeckEmpty;
        }
    }

    /// Stand on current hand
    pub fn standCurrentHand(self: *Game) !void {
        // Move to next hand if available
        if (self.current_hand_index + 1 < self.player_hands.items.len) {
            self.current_hand_index += 1;
            self.state = .PLAYER_TURN;
        } else {
            // All hands played, dealer's turn
            self.state = .DEALER_TURN;
        }
    }

    /// Double down on current hand
    /// Only valid on 2-card hands with value 9, 10, or 11
    pub fn doubleDown(self: *Game) !void {
        const current = self.player_hands.items[self.current_hand_index];

        if (!current.canDoubleDown()) {
            return error.CannotDoubleDown;
        }

        if (self.bankroll < self.current_bet) {
            return error.InsufficientFunds;
        }

        self.current_bet *= 2;
        self.bankroll -= self.current_bet / 2; // Subtract the additional bet

        // Deal one more card and stand automatically
        if (self.deck_manager.dealCard()) |card| {
            try self.player_hands.items[self.current_hand_index].addCard(card);
        } else {
            return error.DeckEmpty;
        }

        // Move to next hand or dealer turn
        try self.standCurrentHand();
    }

    /// Split current hand
    /// Only valid with exactly 2 cards of same rank
    pub fn splitCurrentHand(self: *Game) !void {
        const current = self.player_hands.items[self.current_hand_index];

        if (!current.canSplit()) {
            return error.CannotSplit;
        }

        if (self.player_hands.items.len >= config.GameConfig.MAX_SPLIT_HANDS) {
            return error.MaxSplitsReached;
        }

        if (self.bankroll < self.current_bet) {
            return error.InsufficientFunds;
        }

        // Remove second card from current hand
        const second_card = current.cards.items[1];
        self.player_hands.items[self.current_hand_index].cards.items.pop();
        self.player_hands.items[self.current_hand_index].recalculateValue();

        // Create new hand with the second card
        var new_hand = hand.Hand.init(self.allocator);
        try new_hand.addCard(second_card);
        try self.player_hands.append(new_hand);

        // Deduct additional bet from bankroll
        self.bankroll -= self.current_bet;

        // Deal one card to each hand
        if (self.deck_manager.dealCard()) |card| {
            try self.player_hands.items[self.current_hand_index].addCard(card);
        } else {
            return error.DeckEmpty;
        }

        if (self.deck_manager.dealCard()) |card| {
            try self.player_hands.items[self.player_hands.items.len - 1].addCard(card);
        } else {
            return error.DeckEmpty;
        }

        // Stay on first hand
        self.state = .PLAYER_TURN;
    }

    /// Take insurance bet
    /// Only available when dealer shows an ace
    pub fn takeInsurance(self: *Game) !void {
        if (!self.allow_insurance) {
            return error.InsuranceNotAvailable;
        }

        self.insurance_bet = self.current_bet / 2;

        if (self.bankroll < self.insurance_bet) {
            return error.InsufficientFunds;
        }

        self.bankroll -= self.insurance_bet;
        self.allow_insurance = false;
        self.state = .PLAYER_TURN;
    }

    /// Decline insurance
    pub fn declineInsurance(self: *Game) !void {
        if (!self.allow_insurance) {
            return error.InsuranceNotAvailable;
        }

        self.allow_insurance = false;
        self.state = .PLAYER_TURN;
    }

    /// Play dealer hand according to rules
    /// Dealer stands on hard 17+, hits on soft 17
    pub fn playDealerHand(self: *Game) !void {
        while (true) {
            // Check dealer's hand
            if (self.dealer_hand.value >= 17) {
                // Hard 17 or higher: stand
                if (!self.dealer_hand.is_soft) {
                    break;
                }
                // Soft hand: only stand if >= 18
                if (self.dealer_hand.value >= 18) {
                    break;
                }
            }

            // Hit
            if (self.deck_manager.dealCard()) |card| {
                try self.dealer_hand.addCard(card);
            } else {
                return error.DeckEmpty;
            }

            // Check for bust
            if (self.dealer_hand.isBust()) {
                break;
            }
        }

        self.state = .OUTCOME_DETERMINATION;
    }

    /// Determine outcomes for all hands
    pub fn determineOutcomes(self: *Game) !void {
        for (0..self.player_hands.items.len) |i| {
            const player_hand = self.player_hands.items[i];
            var outcome = hand.evaluateHandWithBlackjack(player_hand, self.dealer_hand);

            if (outcome == null) {
                outcome = hand.evaluateHand(player_hand, self.dealer_hand);
            }

            // Calculate payout
            const payout = calculatePayout(player_hand, self.dealer_hand, outcome, self.current_bet);

            // Update bankroll
            if (outcome.? != .PLAYER_BUST and outcome.? != .PLAYER_LOSS) {
                self.bankroll += payout;
            }

            // Update statistics
            const profit = payout - self.current_bet;
            self.statistics.recordHand(outcome.?, profit, self.current_bet);

            // Handle insurance
            if (self.insurance_bet > 0) {
                if (self.dealer_hand.isBlackjack()) {
                    // Insurance wins 2:1
                    self.bankroll += (self.insurance_bet * 3);
                    self.statistics.total_profit += self.insurance_bet * 2;
                }
                // If dealer doesn't have BJ, insurance is lost
                self.insurance_bet = 0;
            }
        }

        self.state = .HAND_COMPLETE;
    }

    /// Get current player hand
    pub fn getCurrentHand(self: Game) ?*hand.Hand {
        if (self.current_hand_index < self.player_hands.items.len) {
            return &self.player_hands.items[self.current_hand_index];
        }
        return null;
    }

    /// Check if current hand can be split
    pub fn canSplitCurrentHand(self: Game) bool {
        if (self.player_hands.items.len >= config.GameConfig.MAX_SPLIT_HANDS) {
            return false;
        }

        const current = self.player_hands.items[self.current_hand_index];
        return current.canSplit() and self.bankroll >= self.current_bet;
    }

    /// Check if current hand can be doubled
    pub fn canDoubleCurrentHand(self: Game) bool {
        const current = self.player_hands.items[self.current_hand_index];
        return current.canDoubleDown() and self.bankroll >= self.current_bet;
    }

    /// Check if game should end (bankroll depleted)
    pub fn isGameOver(self: Game) bool {
        return self.bankroll < config.GameConfig.MIN_BET;
    }

    /// Reset for next hand
    pub fn resetForNewHand(self: *Game) !void {
        self.insurance_bet = 0;
        self.allow_insurance = false;
        self.state = .WAITING_FOR_BET;
    }
};

/// Calculate payout for a hand
fn calculatePayout(
    player_hand: hand.Hand,
    dealer_hand: hand.Hand,
    outcome: ?config.HandOutcome,
    original_bet: i64,
) i64 {
    if (outcome == null) {
        return 0;
    }

    return switch (outcome.?) {
        .PLAYER_WIN => original_bet * 2, // 1:1
        .PLAYER_LOSS => 0, // Lose bet
        .PUSH => original_bet, // Return bet
        .BLACKJACK => original_bet + (original_bet * 3 / 2), // 3:2 (1 + 1.5)
        .PLAYER_BUST => 0,
        .DEALER_BUST => original_bet * 2, // 1:1
        .INSURANCE_WIN => 0, // Handled separately
        .INSURANCE_LOSS => 0, // Handled separately
    };
}

// ============================================================================
// TESTS
// ============================================================================

const testing = std.testing;

test "Game initialization" {
    var game = try Game.init(testing.allocator, null);
    defer game.deinit();

    try testing.expectEqual(game.bankroll, config.GameConfig.INITIAL_BANKROLL);
    try testing.expectEqual(game.state, .INITIAL);
}

test "Place valid bet" {
    var game = try Game.init(testing.allocator, null);
    defer game.deinit();

    try game.placeBet(5000);
    try testing.expectEqual(game.current_bet, 5000);
    try testing.expectEqual(game.state, .BET_PLACED);
}

test "Reject bet below minimum" {
    var game = try Game.init(testing.allocator, null);
    defer game.deinit();

    const result = game.placeBet(400);
    try testing.expect(result == error.BetTooLow);
}

test "Reject bet above maximum" {
    var game = try Game.init(testing.allocator, null);
    defer game.deinit();

    const result = game.placeBet(60000);
    try testing.expect(result == error.BetTooHigh);
}

test "Reject bet exceeding bankroll" {
    var game = try Game.init(testing.allocator, 1000);
    defer game.deinit();

    const result = game.placeBet(50000);
    try testing.expect(result == error.InsufficientFunds);
}

test "Game is not over with sufficient funds" {
    var game = try Game.init(testing.allocator, null);
    defer game.deinit();

    try testing.expect(!game.isGameOver());
}

test "Game is over when bankroll < minimum bet" {
    var game = try Game.init(testing.allocator, 400);
    defer game.deinit();

    try testing.expect(game.isGameOver());
}
