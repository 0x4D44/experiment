/// Hand evaluation and scoring logic
/// Implements blackjack hand value calculation with proper ace handling

const std = @import("std");
const mem = std.mem;
const config = @import("config.zig");
const deck = @import("deck.zig");

/// Represents a player or dealer hand
pub const Hand = struct {
    cards: std.ArrayList(deck.Card),
    value: u8 = 0,
    is_soft: bool = false, // True if hand contains a usable (11-valued) ace
    is_blackjack: bool = false,

    /// Initialize a new hand
    pub fn init(allocator: mem.Allocator) Hand {
        return Hand{
            .cards = std.ArrayList(deck.Card).init(allocator),
        };
    }

    /// Deinitialize and free memory
    pub fn deinit(self: *Hand) void {
        self.cards.deinit();
    }

    /// Add a card to the hand and recalculate value
    pub fn addCard(self: *Hand, card: deck.Card) !void {
        try self.cards.append(card);
        self.recalculateValue();
    }

    /// Add multiple cards at once
    pub fn addCards(self: *Hand, cards_to_add: std.ArrayList(deck.Card)) !void {
        try self.cards.appendSlice(cards_to_add.items);
        self.recalculateValue();
    }

    /// Calculate and update hand value
    /// This implements the complex ace-handling algorithm for blackjack
    pub fn recalculateValue(self: *Hand) void {
        if (self.cards.items.len == 0) {
            self.value = 0;
            self.is_soft = false;
            self.is_blackjack = false;
            return;
        }

        // Check for blackjack first (natural 21 with exactly 2 cards)
        if (self.cards.items.len == 2) {
            var sum: u8 = 0;
            var ace_count: u8 = 0;

            for (self.cards.items) |card| {
                sum += card.value();
                if (card.isAce()) {
                    ace_count += 1;
                }
            }

            if (sum == 21) {
                self.value = 21;
                self.is_blackjack = true;
                self.is_soft = (ace_count > 0);
                return;
            }
        } else {
            self.is_blackjack = false;
        }

        // Calculate hand value using soft/hard aces
        var sum: u8 = 0;
        var ace_count: u8 = 0;

        // First pass: sum all cards, count aces as 11
        for (self.cards.items) |card| {
            sum += card.value();
            if (card.isAce()) {
                ace_count += 1;
            }
        }

        // Second pass: convert aces from 11 to 1 as needed to avoid bust
        self.is_soft = false;
        var soft_aces = ace_count;

        while (sum > 21 and soft_aces > 0) {
            sum -= 10; // Convert one ace from 11 to 1 (11 - 1 = 10 difference)
            soft_aces -= 1;
        }

        // If we still have an ace counted as 11 (not converted), it's a soft hand
        self.is_soft = (soft_aces > 0);
        self.value = sum;
    }

    /// Check if hand is bust (> 21)
    pub fn isBust(self: Hand) bool {
        return self.value > 21;
    }

    /// Check if hand is exactly 21
    pub fn isTwentyOne(self: Hand) bool {
        return self.value == 21;
    }

    /// Check if hand is blackjack (natural 21 with exactly 2 cards)
    pub fn isBlackjack(self: Hand) bool {
        return self.is_blackjack;
    }

    /// Check if hand can be split
    /// Requires exactly 2 cards with the same rank
    pub fn canSplit(self: Hand) bool {
        if (self.cards.items.len != 2) {
            return false;
        }

        return self.cards.items[0].rank == self.cards.items[1].rank;
    }

    /// Check if hand can double down
    /// Requires exactly 2 cards with value 9, 10, or 11
    pub fn canDoubleDown(self: Hand) bool {
        if (self.cards.items.len != 2) {
            return false;
        }

        return self.value == 9 or self.value == 10 or self.value == 11;
    }

    /// Get the number of cards in hand
    pub fn cardCount(self: Hand) u32 {
        return @as(u32, @intCast(self.cards.items.len));
    }

    /// Check if hand contains only split aces
    /// (special case where split aces cannot receive more than 1 card each)
    pub fn isSplitAceHand(self: Hand) bool {
        // A split ace hand has exactly 2 cards, both aces
        // This is only true right after the split
        if (self.cards.items.len != 2) {
            return false;
        }

        for (self.cards.items) |card| {
            if (!card.isAce()) {
                return false;
            }
        }

        return true;
    }

    /// Get a string representation of the hand (for debugging)
    pub fn toString(self: Hand, allocator: mem.Allocator) ![]u8 {
        var result = std.ArrayList(u8).init(allocator);
        defer result.deinit();

        for (self.cards.items, 0..) |card, i| {
            if (i > 0) {
                try result.appendSlice(", ");
            }

            const card_str = try card.display(allocator);
            defer allocator.free(card_str);
            try result.appendSlice(card_str);
        }

        try result.writer().print(" = {} ({}soft)", .{
            self.value,
            if (self.is_soft) "" else "not ",
        });

        return result.toOwnedSlice();
    }

    /// Clear the hand (for reuse)
    pub fn clear(self: *Hand) void {
        self.cards.clearRetainingCapacity();
        self.value = 0;
        self.is_soft = false;
        self.is_blackjack = false;
    }
};

// ============================================================================
// HAND COMPARISON AND EVALUATION
// ============================================================================

/// Determine the outcome of a single hand
pub fn evaluateHand(player_hand: Hand, dealer_hand: Hand) ?config.HandOutcome {
    // Player bust
    if (player_hand.isBust()) {
        return .PLAYER_BUST;
    }

    // Dealer bust
    if (dealer_hand.isBust()) {
        return .DEALER_BUST;
    }

    // Both have valid hands, compare values
    if (player_hand.value > dealer_hand.value) {
        return .PLAYER_WIN;
    } else if (player_hand.value < dealer_hand.value) {
        return .PLAYER_LOSS;
    } else {
        return .PUSH;
    }
}

/// Determine outcome considering blackjacks
pub fn evaluateHandWithBlackjack(player_hand: Hand, dealer_hand: Hand) ?config.HandOutcome {
    // Player blackjack
    if (player_hand.isBlackjack()) {
        if (dealer_hand.isBlackjack()) {
            return .PUSH; // Both blackjack = push
        }
        return .BLACKJACK; // Player blackjack beats non-blackjack 21
    }

    // Dealer blackjack (player doesn't have it)
    if (dealer_hand.isBlackjack()) {
        return .PLAYER_LOSS;
    }

    // No blackjacks, use regular evaluation
    return evaluateHand(player_hand, dealer_hand);
}

// ============================================================================
// TESTS
// ============================================================================

const testing = std.testing;

test "Hand initialization" {
    const hand = Hand.init(testing.allocator);
    try testing.expectEqual(hand.value, 0);
    try testing.expect(!hand.is_soft);
    try testing.expect(!hand.is_blackjack);
}

test "Single card value" {
    var hand = Hand.init(testing.allocator);
    defer hand.deinit();

    const five = deck.Card{ .rank = .FIVE, .suit = .HEART };
    try hand.addCard(five);

    try testing.expectEqual(hand.value, 5);
    try testing.expect(!hand.is_soft);
}

test "Ace high (soft 11)" {
    var hand = Hand.init(testing.allocator);
    defer hand.deinit();

    const ace = deck.Card{ .rank = .ACE, .suit = .HEART };
    try hand.addCard(ace);

    try testing.expectEqual(hand.value, 11);
    try testing.expect(hand.is_soft);
}

test "Ace high plus 5 = soft 16" {
    var hand = Hand.init(testing.allocator);
    defer hand.deinit();

    const ace = deck.Card{ .rank = .ACE, .suit = .HEART };
    const five = deck.Card{ .rank = .FIVE, .suit = .HEART };

    try hand.addCard(ace);
    try hand.addCard(five);

    try testing.expectEqual(hand.value, 16);
    try testing.expect(hand.is_soft);
}

test "Ace forced to 1 on bust" {
    var hand = Hand.init(testing.allocator);
    defer hand.deinit();

    const ace = deck.Card{ .rank = .ACE, .suit = .HEART };
    const ten = deck.Card{ .rank = .TEN, .suit = .HEART };
    const five = deck.Card{ .rank = .FIVE, .suit = .HEART };

    try hand.addCard(ace); // 11
    try hand.addCard(ten); // 21
    try hand.addCard(five); // Would be 26, so ace becomes 1 = 16

    try testing.expectEqual(hand.value, 16);
    try testing.expect(!hand.is_soft);
}

test "Multiple aces: A+A+9 = 21" {
    var hand = Hand.init(testing.allocator);
    defer hand.deinit();

    const ace1 = deck.Card{ .rank = .ACE, .suit = .HEART };
    const ace2 = deck.Card{ .rank = .ACE, .suit = .DIAMOND };
    const nine = deck.Card{ .rank = .NINE, .suit = .HEART };

    try hand.addCard(ace1); // 11
    try hand.addCard(ace2); // 22, convert to 1 = 12
    try hand.addCard(nine); // 21

    try testing.expectEqual(hand.value, 21);
    try testing.expect(!hand.is_soft); // Both aces are hard
}

test "Blackjack: A+K" {
    var hand = Hand.init(testing.allocator);
    defer hand.deinit();

    const ace = deck.Card{ .rank = .ACE, .suit = .HEART };
    const king = deck.Card{ .rank = .KING, .suit = .HEART };

    try hand.addCard(ace);
    try hand.addCard(king);

    try testing.expect(hand.isBlackjack());
    try testing.expectEqual(hand.value, 21);
}

test "Not blackjack: A+K+10" {
    var hand = Hand.init(testing.allocator);
    defer hand.deinit();

    const ace = deck.Card{ .rank = .ACE, .suit = .HEART };
    const king = deck.Card{ .rank = .KING, .suit = .HEART };
    const ten = deck.Card{ .rank = .TEN, .suit = .HEART };

    try hand.addCard(ace);
    try hand.addCard(king);
    try hand.addCard(ten);

    try testing.expect(!hand.isBlackjack());
    try testing.expectEqual(hand.value, 21);
}

test "Bust detection" {
    var hand = Hand.init(testing.allocator);
    defer hand.deinit();

    const king = deck.Card{ .rank = .KING, .suit = .HEART };
    const queen = deck.Card{ .rank = .QUEEN, .suit = .HEART };
    const five = deck.Card{ .rank = .FIVE, .suit = .HEART };

    try hand.addCard(king); // 10
    try hand.addCard(queen); // 20
    try hand.addCard(five); // 25 (bust)

    try testing.expect(hand.isBust());
}

test "Can split: pair of 7s" {
    var hand = Hand.init(testing.allocator);
    defer hand.deinit();

    const seven1 = deck.Card{ .rank = .SEVEN, .suit = .HEART };
    const seven2 = deck.Card{ .rank = .SEVEN, .suit = .DIAMOND };

    try hand.addCard(seven1);
    try hand.addCard(seven2);

    try testing.expect(hand.canSplit());
}

test "Cannot split: different ranks" {
    var hand = Hand.init(testing.allocator);
    defer hand.deinit();

    const seven = deck.Card{ .rank = .SEVEN, .suit = .HEART };
    const eight = deck.Card{ .rank = .EIGHT, .suit = .DIAMOND };

    try hand.addCard(seven);
    try hand.addCard(eight);

    try testing.expect(!hand.canSplit());
}

test "Cannot split: more than 2 cards" {
    var hand = Hand.init(testing.allocator);
    defer hand.deinit();

    const seven1 = deck.Card{ .rank = .SEVEN, .suit = .HEART };
    const seven2 = deck.Card{ .rank = .SEVEN, .suit = .DIAMOND };
    const five = deck.Card{ .rank = .FIVE, .suit = .HEART };

    try hand.addCard(seven1);
    try hand.addCard(seven2);
    try hand.addCard(five);

    try testing.expect(!hand.canSplit());
}

test "Can double down: 10" {
    var hand = Hand.init(testing.allocator);
    defer hand.deinit();

    const five = deck.Card{ .rank = .FIVE, .suit = .HEART };
    const five2 = deck.Card{ .rank = .FIVE, .suit = .DIAMOND };

    try hand.addCard(five);
    try hand.addCard(five2);

    try testing.expect(hand.canDoubleDown());
}

test "Cannot double down: 12" {
    var hand = Hand.init(testing.allocator);
    defer hand.deinit();

    const five = deck.Card{ .rank = .FIVE, .suit = .HEART };
    const seven = deck.Card{ .rank = .SEVEN, .suit = .DIAMOND };

    try hand.addCard(five);
    try hand.addCard(seven);

    try testing.expect(!hand.canDoubleDown());
}

test "Hand clear" {
    var hand = Hand.init(testing.allocator);
    defer hand.deinit();

    const five = deck.Card{ .rank = .FIVE, .suit = .HEART };
    try hand.addCard(five);

    try testing.expectEqual(hand.value, 5);

    hand.clear();

    try testing.expectEqual(hand.value, 0);
    try testing.expectEqual(hand.cardCount(), 0);
}

test "Player blackjack beats 21" {
    const player = blk: {
        var h = Hand.init(testing.allocator);
        defer h.deinit();
        try h.addCard(deck.Card{ .rank = .ACE, .suit = .HEART });
        try h.addCard(deck.Card{ .rank = .KING, .suit = .HEART });
        break :blk h;
    };

    const dealer = blk: {
        var h = Hand.init(testing.allocator);
        defer h.deinit();
        try h.addCard(deck.Card{ .rank = .SEVEN, .suit = .HEART });
        try h.addCard(deck.Card{ .rank = .SEVEN, .suit = .DIAMOND });
        try h.addCard(deck.Card{ .rank = .SEVEN, .suit = .CLUB });
        break :blk h;
    };

    const outcome = evaluateHandWithBlackjack(player, dealer);
    try testing.expect(outcome == .BLACKJACK);
}
