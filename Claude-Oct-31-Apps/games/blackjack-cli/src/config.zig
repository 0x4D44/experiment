/// Game configuration constants and settings
/// All monetary values are in cents (1 dollar = 100 cents)

pub const GameConfig = struct {
    // Bankroll settings
    pub const INITIAL_BANKROLL: i64 = 100_000; // $1000.00
    pub const MIN_BET: i64 = 500; // $5.00
    pub const MAX_BET: i64 = 50_000; // $500.00

    // Deck settings
    pub const DECK_COUNT: u32 = 4;
    pub const CARDS_PER_DECK: u32 = 52;
    pub const SUITS_COUNT: u32 = 4;
    pub const RANKS_COUNT: u32 = 13;

    // Penetration settings
    pub const RESHUFFLE_PENETRATION: f32 = 0.75; // 75% of deck used

    // Animation and display
    pub const CARD_DEAL_DELAY_MS: u64 = 100;
    pub const DEALER_ACTION_DELAY_MS: u64 = 500;

    // Files
    pub const STATS_FILE: []const u8 = ".blackjack_stats.json";
    pub const BANKROLL_FILE: []const u8 = ".blackjack_bankroll";

    // Limits
    pub const MAX_SPLIT_HANDS: u32 = 4;
    pub const MAX_CARDS_PER_HAND: u32 = 11; // Practical maximum before bust
};

/// Card rank values
pub const CardValue = enum(u8) {
    ACE = 1,
    TWO = 2,
    THREE = 3,
    FOUR = 4,
    FIVE = 5,
    SIX = 6,
    SEVEN = 7,
    EIGHT = 8,
    NINE = 9,
    TEN = 10,
    JACK = 11,
    QUEEN = 12,
    KING = 13,

    /// Get blackjack value for this rank
    pub fn blackjackValue(self: CardValue) u8 {
        return switch (self) {
            .ACE => 11, // Flexible, treated specially
            .TWO => 2,
            .THREE => 3,
            .FOUR => 4,
            .FIVE => 5,
            .SIX => 6,
            .SEVEN => 7,
            .EIGHT => 8,
            .NINE => 9,
            .TEN, .JACK, .QUEEN, .KING => 10,
        };
    }

    /// Get display character for this rank
    pub fn displayChar(self: CardValue) u8 {
        return switch (self) {
            .ACE => 'A',
            .TWO => '2',
            .THREE => '3',
            .FOUR => '4',
            .FIVE => '5',
            .SIX => '6',
            .SEVEN => '7',
            .EIGHT => '8',
            .NINE => '9',
            .TEN => 'T',
            .JACK => 'J',
            .QUEEN => 'Q',
            .KING => 'K',
        };
    }

    /// Get display string for this rank
    pub fn displayStr(self: CardValue) []const u8 {
        return switch (self) {
            .ACE => "A",
            .TWO => "2",
            .THREE => "3",
            .FOUR => "4",
            .FIVE => "5",
            .SIX => "6",
            .SEVEN => "7",
            .EIGHT => "8",
            .NINE => "9",
            .TEN => "10",
            .JACK => "J",
            .QUEEN => "Q",
            .KING => "K",
        };
    }
};

/// Card suits
pub const Suit = enum(u8) {
    SPADE = 0,
    HEART = 1,
    DIAMOND = 2,
    CLUB = 3,

    /// Get display symbol for this suit
    pub fn symbol(self: Suit) []const u8 {
        return switch (self) {
            .SPADE => "♠",
            .HEART => "♥",
            .DIAMOND => "♦",
            .CLUB => "♣",
        };
    }

    /// Get ASCII representation for this suit
    pub fn asciiChar(self: Suit) u8 {
        return switch (self) {
            .SPADE => 'S',
            .HEART => 'H',
            .DIAMOND => 'D',
            .CLUB => 'C',
        };
    }
};

/// Game states
pub const GameState = enum {
    INITIAL,
    WAITING_FOR_BET,
    BET_PLACED,
    CARDS_DEALT,
    INSURANCE_OFFERED,
    PLAYER_TURN,
    STAND_CHOSEN,
    DEALER_TURN,
    OUTCOME_DETERMINATION,
    HAND_COMPLETE,
    GAME_OVER,
};

/// Hand outcome types
pub const HandOutcome = enum {
    PLAYER_WIN,
    PLAYER_LOSS,
    PUSH,
    BLACKJACK,
    PLAYER_BUST,
    DEALER_BUST,
    INSURANCE_WIN,
    INSURANCE_LOSS,
};

/// Input commands
pub const Command = enum {
    HIT,
    STAND,
    DOUBLE,
    SPLIT,
    INSURANCE,
    QUIT,
    UNKNOWN,

    /// Parse a command from a character
    pub fn fromChar(ch: u8) Command {
        return switch (ch) {
            'h', 'H' => Command.HIT,
            's', 'S' => Command.STAND,
            'd', 'D' => Command.DOUBLE,
            'p', 'P' => Command.SPLIT,
            'i', 'I' => Command.INSURANCE,
            'q', 'Q' => Command.QUIT,
            else => Command.UNKNOWN,
        };
    }
};

/// Statistics tracking
pub const Statistics = struct {
    total_hands: u32 = 0,
    total_wins: u32 = 0,
    total_losses: u32 = 0,
    total_pushes: u32 = 0,
    total_blackjacks: u32 = 0,
    total_profit: i64 = 0, // In cents
    biggest_win: i64 = 0,
    biggest_loss: i64 = 0,
    total_bets_placed: i64 = 0, // Sum of all bets
    win_streak: i32 = 0,
    max_win_streak: i32 = 0,
    max_loss_streak: i32 = 0,

    /// Reset statistics to initial values
    pub fn reset(self: *Statistics) void {
        self.total_hands = 0;
        self.total_wins = 0;
        self.total_losses = 0;
        self.total_pushes = 0;
        self.total_blackjacks = 0;
        self.total_profit = 0;
        self.biggest_win = 0;
        self.biggest_loss = 0;
        self.total_bets_placed = 0;
        self.win_streak = 0;
        self.max_win_streak = 0;
        self.max_loss_streak = 0;
    }

    /// Record a hand result
    pub fn recordHand(
        self: *Statistics,
        outcome: HandOutcome,
        profit_loss: i64,
        bet_amount: i64,
    ) void {
        self.total_hands += 1;
        self.total_bets_placed += bet_amount;

        switch (outcome) {
            .PLAYER_WIN => {
                self.total_wins += 1;
                self.win_streak += 1;
                if (self.win_streak > self.max_win_streak) {
                    self.max_win_streak = self.win_streak;
                }
            },
            .PLAYER_LOSS => {
                self.total_losses += 1;
                self.win_streak -= 1;
                if (self.win_streak < -self.max_loss_streak) {
                    self.max_loss_streak = -self.win_streak;
                }
            },
            .PUSH => {
                self.total_pushes += 1;
            },
            .BLACKJACK => {
                self.total_wins += 1;
                self.total_blackjacks += 1;
                self.win_streak += 1;
                if (self.win_streak > self.max_win_streak) {
                    self.max_win_streak = self.win_streak;
                }
            },
            .PLAYER_BUST => {
                self.total_losses += 1;
                self.win_streak -= 1;
                if (self.win_streak < -self.max_loss_streak) {
                    self.max_loss_streak = -self.win_streak;
                }
            },
            .DEALER_BUST => {
                self.total_wins += 1;
                self.win_streak += 1;
                if (self.win_streak > self.max_win_streak) {
                    self.max_win_streak = self.win_streak;
                }
            },
            .INSURANCE_WIN => {
                // Handled separately
            },
            .INSURANCE_LOSS => {
                // Handled separately
            },
        }

        self.total_profit += profit_loss;

        if (profit_loss > self.biggest_win) {
            self.biggest_win = profit_loss;
        }

        if (profit_loss < self.biggest_loss) {
            self.biggest_loss = profit_loss;
        }
    }

    /// Get win rate as percentage
    pub fn winPercentage(self: Statistics) f32 {
        if (self.total_hands == 0) return 0.0;
        return (@as(f32, @floatFromInt(self.total_wins)) / @as(f32, @floatFromInt(self.total_hands))) * 100.0;
    }

    /// Get loss rate as percentage
    pub fn lossPercentage(self: Statistics) f32 {
        if (self.total_hands == 0) return 0.0;
        return (@as(f32, @floatFromInt(self.total_losses)) / @as(f32, @floatFromInt(self.total_hands))) * 100.0;
    }

    /// Get push rate as percentage
    pub fn pushPercentage(self: Statistics) f32 {
        if (self.total_hands == 0) return 0.0;
        return (@as(f32, @floatFromInt(self.total_pushes)) / @as(f32, @floatFromInt(self.total_hands))) * 100.0;
    }
};
