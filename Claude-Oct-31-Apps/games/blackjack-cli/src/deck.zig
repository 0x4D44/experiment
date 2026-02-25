/// Card deck management
/// Implements standard playing cards with shuffling and dealing

const std = @import("std");
const mem = std.mem;
const config = @import("config.zig");

/// Represents a single playing card
pub const Card = struct {
    rank: config.CardValue,
    suit: config.Suit,

    /// Get the blackjack value for this card
    /// Aces return 11 (must be handled specially in hand evaluation)
    pub fn value(self: Card) u8 {
        return self.rank.blackjackValue();
    }

    /// Check if this card is an ace
    pub fn isAce(self: Card) bool {
        return self.rank == .ACE;
    }

    /// Get display string for this card (e.g., "AH" for Ace of Hearts)
    pub fn display(self: Card, allocator: mem.Allocator) ![]u8 {
        return try std.fmt.allocPrint(
            allocator,
            "{s}{s}",
            .{ self.rank.displayStr(), self.suit.symbol() },
        );
    }
};

/// Represents a deck of cards with shuffling capability
pub const Deck = struct {
    cards: std.ArrayList(Card),
    cards_dealt: u32 = 0,
    deck_count: u32,
    prng: std.Random.Xoroshiro128,

    /// Initialize a new deck with the specified number of decks
    pub fn init(allocator: mem.Allocator, deck_count: u32) !Deck {
        var cards = std.ArrayList(Card).init(allocator);
        try cards.ensureTotalCapacity(deck_count * config.GameConfig.CARDS_PER_DECK);

        // Create cards for each deck
        for (0..deck_count) |_| {
            for (0..config.GameConfig.SUITS_COUNT) |suit_idx| {
                for (0..config.GameConfig.RANKS_COUNT) |rank_idx| {
                    const suit: config.Suit = @enumFromInt(suit_idx);
                    const rank: config.CardValue = @enumFromInt(rank_idx + 1);

                    try cards.append(Card{
                        .rank = rank,
                        .suit = suit,
                    });
                }
            }
        }

        // Initialize PRNG with system entropy
        var seed: u64 = undefined;
        try std.posix.getrandom(std.mem.asBytes(&seed));
        var prng = std.Random.Xoroshiro128.init(seed);

        var deck = Deck{
            .cards = cards,
            .deck_count = deck_count,
            .prng = prng,
        };

        // Perform initial shuffle
        deck.shuffle();

        return deck;
    }

    /// Deinitialize the deck and free memory
    pub fn deinit(self: *Deck) void {
        self.cards.deinit();
    }

    /// Shuffle the deck using Fisher-Yates algorithm
    /// Performs a thorough shuffle to prevent card counting
    pub fn shuffle(self: *Deck) void {
        self.cards_dealt = 0;

        const random = self.prng.random();
        const cards_len = self.cards.items.len;

        // Fisher-Yates shuffle
        // Start from the end and swap with a random earlier position
        var i = cards_len;
        while (i > 1) {
            i -= 1;
            const j = random.intRangeLessThan(usize, 0, i + 1);

            // Swap cards[i] and cards[j]
            const temp = self.cards.items[i];
            self.cards.items[i] = self.cards.items[j];
            self.cards.items[j] = temp;
        }
    }

    /// Reshuffle if penetration exceeds threshold
    pub fn checkAndReshuffle(self: *Deck) bool {
        const total_cards = @as(f32, @floatFromInt(self.cards.items.len));
        const penetration = @as(f32, @floatFromInt(self.cards_dealt)) / total_cards;

        if (penetration >= config.GameConfig.RESHUFFLE_PENETRATION) {
            self.shuffle();
            return true;
        }

        return false;
    }

    /// Get the current penetration level (0.0 to 1.0)
    pub fn getPenetration(self: Deck) f32 {
        const total_cards = @as(f32, @floatFromInt(self.cards.items.len));
        return @as(f32, @floatFromInt(self.cards_dealt)) / total_cards;
    }

    /// Get number of cards remaining in deck
    pub fn cardsRemaining(self: Deck) u32 {
        return @as(u32, @intCast(self.cards.items.len)) - self.cards_dealt;
    }

    /// Deal a single card from the deck
    /// Returns a Card or error if deck is empty (should not happen with reshuffle)
    pub fn dealCard(self: *Deck) ?Card {
        if (self.cards_dealt >= self.cards.items.len) {
            return null;
        }

        const card = self.cards.items[self.cards.items.len - 1 - self.cards_dealt];
        self.cards_dealt += 1;

        return card;
    }

    /// Deal multiple cards at once
    pub fn dealCards(self: *Deck, count: u32) !std.ArrayList(Card) {
        var dealt_cards = std.ArrayList(Card).init(self.cards.items.allocator);
        try dealt_cards.ensureTotalCapacity(count);

        for (0..count) |_| {
            if (self.dealCard()) |card| {
                dealt_cards.appendAssumeCapacity(card);
            } else {
                dealt_cards.deinit();
                return error.DeckEmpty;
            }
        }

        return dealt_cards;
    }

    /// Reset deck for testing purposes
    pub fn reset(self: *Deck) void {
        self.cards_dealt = 0;
        self.shuffle();
    }

    /// Get card count for statistics
    pub fn getCardCount(self: Deck) struct {
        total: u32,
        dealt: u32,
        remaining: u32,
    } {
        return .{
            .total = @as(u32, @intCast(self.cards.items.len)),
            .dealt = self.cards_dealt,
            .remaining = self.cardsRemaining(),
        };
    }
};

// ============================================================================
// TESTS
// ============================================================================

const testing = std.testing;

test "Deck initialization creates correct card count" {
    var deck = try Deck.init(testing.allocator, 1);
    defer deck.deinit();

    try testing.expectEqual(deck.cards.items.len, 52);
}

test "Deck initialization with multiple decks" {
    var deck = try Deck.init(testing.allocator, 4);
    defer deck.deinit();

    try testing.expectEqual(deck.cards.items.len, 208);
}

test "Deck shuffle resets cards_dealt counter" {
    var deck = try Deck.init(testing.allocator, 1);
    defer deck.deinit();

    _ = deck.dealCard(); // Deal one card
    _ = deck.dealCard(); // Deal another
    try testing.expectEqual(deck.cards_dealt, 2);

    deck.shuffle(); // Reset
    try testing.expectEqual(deck.cards_dealt, 0);
}

test "Deal card increments counter" {
    var deck = try Deck.init(testing.allocator, 1);
    defer deck.deinit();

    const initial = deck.cards_dealt;
    _ = deck.dealCard();
    try testing.expectEqual(deck.cards_dealt, initial + 1);
}

test "Cards remaining calculation" {
    var deck = try Deck.init(testing.allocator, 1);
    defer deck.deinit();

    const remaining_start = deck.cardsRemaining();
    try testing.expectEqual(remaining_start, 52);

    _ = deck.dealCard();
    const remaining_after = deck.cardsRemaining();
    try testing.expectEqual(remaining_after, 51);
}

test "Penetration calculation" {
    var deck = try Deck.init(testing.allocator, 1);
    defer deck.deinit();

    const initial_penetration = deck.getPenetration();
    try testing.expectEqual(initial_penetration, 0.0);

    // Deal 26 cards (50% of 52)
    for (0..26) |_| {
        _ = deck.dealCard();
    }

    const half_penetration = deck.getPenetration();
    try testing.expect(half_penetration > 0.49 and half_penetration < 0.51);
}

test "Reshuffle at 75% penetration" {
    var deck = try Deck.init(testing.allocator, 1);
    defer deck.deinit();

    // Deal 39 cards (75% of 52)
    for (0..39) |_| {
        _ = deck.dealCard();
    }

    const should_reshuffle = deck.checkAndReshuffle();
    try testing.expect(should_reshuffle);
    try testing.expectEqual(deck.cards_dealt, 0);
}

test "Card isAce" {
    const ace = Card{ .rank = .ACE, .suit = .HEART };
    const king = Card{ .rank = .KING, .suit = .HEART };

    try testing.expect(ace.isAce());
    try testing.expect(!king.isAce());
}

test "Card value" {
    const ace = Card{ .rank = .ACE, .suit = .HEART };
    const five = Card{ .rank = .FIVE, .suit = .HEART };
    const king = Card{ .rank = .KING, .suit = .HEART };

    try testing.expectEqual(ace.value(), 11);
    try testing.expectEqual(five.value(), 5);
    try testing.expectEqual(king.value(), 10);
}
