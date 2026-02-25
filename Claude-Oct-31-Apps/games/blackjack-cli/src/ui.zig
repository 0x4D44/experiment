/// Terminal UI rendering and input handling

const std = @import("std");
const mem = std.mem;
const config = @import("config.zig");
const deck = @import("deck.zig");
const hand = @import("hand.zig");
const game = @import("game.zig");

const stdout = std.io.getStdOut();
const stdin = std.io.getStdIn();

/// Clear the terminal screen
pub fn clearScreen() void {
    stdout.writeAll("\x1B[2J\x1B[H") catch {};
}

/// Move cursor to position
pub fn moveCursor(x: u32, y: u32) void {
    const writer = stdout.writer();
    std.fmt.format(writer, "\x1B[{};{}H", .{ y, x }) catch {};
}

/// Set text color (ANSI codes)
pub const Color = enum {
    RESET,
    RED,
    GREEN,
    YELLOW,
    BLUE,
    MAGENTA,
    CYAN,
    WHITE,

    pub fn code(self: Color) []const u8 {
        return switch (self) {
            .RESET => "\x1B[0m",
            .RED => "\x1B[31m",
            .GREEN => "\x1B[32m",
            .YELLOW => "\x1B[33m",
            .BLUE => "\x1B[34m",
            .MAGENTA => "\x1B[35m",
            .CYAN => "\x1B[36m",
            .WHITE => "\x1B[37m",
        };
    }
};

pub fn setColor(color: Color) void {
    stdout.writeAll(color.code()) catch {};
}

/// Render a single card
pub fn renderCard(card: deck.Card, allocator: mem.Allocator) ![]u8 {
    const rank_str = card.rank.displayStr();
    const suit = card.suit.symbol();

    return try std.fmt.allocPrint(allocator, "┌───┐\n│{s:>2}{s}│\n└───┘", .{ rank_str, suit });
}

/// Render a hidden card
pub fn renderHiddenCard(allocator: mem.Allocator) ![]u8 {
    return try std.fmt.allocPrint(allocator, "┌───┐\n│ ? │\n└───┘", .{});
}

/// Render a hand (multiple cards in a row)
pub fn renderHand(h: hand.Hand, show_hole_card: bool, allocator: mem.Allocator) ![]u8 {
    var result = std.ArrayList(u8).init(allocator);
    defer result.deinit();

    const writer = result.writer();

    for (h.cards.items, 0..) |card, i| {
        // Hide first card if showing hole card
        if (!show_hole_card and i == 1) {
            const hidden = try renderHiddenCard(allocator);
            defer allocator.free(hidden);
            try writer.print("{s}", .{hidden});
        } else {
            const card_str = try renderCard(card, allocator);
            defer allocator.free(card_str);
            try writer.print("{s}", .{card_str});
        }

        if (i < h.cards.items.len - 1) {
            try writer.print(" ", .{});
        }
    }

    return result.toOwnedSlice();
}

/// Display the game header
pub fn displayHeader() void {
    const writer = stdout.writer();

    writer.print("\n", .{}) catch {};
    writer.print("═══════════════════════════════════════════\n", .{}) catch {};
    writer.print("         BLACKJACK - Casino Style\n", .{}) catch {};
    writer.print("═══════════════════════════════════════════\n", .{}) catch {};
}

/// Display bankroll and bet information
pub fn displayBankroll(bankroll: i64, current_bet: i64, hands: u32, wins: u32, losses: u32) void {
    const writer = stdout.writer();

    const bankroll_dollars = @as(f32, @floatFromInt(bankroll)) / 100.0;
    const bet_dollars = @as(f32, @floatFromInt(current_bet)) / 100.0;

    writer.print("\nBankroll: ${d:.2} | Bet: ${d:.2}\n", .{ bankroll_dollars, bet_dollars }) catch {};
    writer.print("Hands: {} | Wins: {} | Losses: {}\n", .{ hands, wins, losses }) catch {};
    writer.print("─────────────────────────────────────────\n", .{}) catch {};
}

/// Display the dealer's hand
pub fn displayDealerHand(h: hand.Hand, show_hole: bool, allocator: mem.Allocator) void {
    const writer = stdout.writer();

    writer.print("\nDealer's Hand", .{}) catch {};

    if (show_hole) {
        writer.print(" (Value: {}):\n", .{h.value}) catch {};
    } else {
        writer.print(":\n", .{}) catch {};
    }

    const hand_str = renderHand(h, show_hole, allocator) catch return;
    defer allocator.free(hand_str);

    writer.print("{s}\n", .{hand_str}) catch {};
}

/// Display the player's hand
pub fn displayPlayerHand(h: hand.Hand, hand_index: u32, total_hands: u32, allocator: mem.Allocator) void {
    const writer = stdout.writer();

    writer.print("\nYour Hand", .{}) catch {};

    if (total_hands > 1) {
        writer.print(" (Hand {}/{})", .{ hand_index + 1, total_hands }) catch {};
    }

    if (h.value > 21) {
        writer.print(" - BUST:\n", .{}) catch {};
        setColor(.RED);
    } else {
        writer.print(" (Value: {}):\n", .{h.value}) catch {};
    }

    const hand_str = renderHand(h, true, allocator) catch return;
    defer allocator.free(hand_str);

    writer.print("{s}\n", .{hand_str}) catch {};
    setColor(.RESET);
}

/// Display available commands
pub fn displayCommands(
    can_hit: bool,
    can_stand: bool,
    can_double: bool,
    can_split: bool,
    can_insurance: bool,
) void {
    const writer = stdout.writer();

    writer.print("\nAvailable commands: ", .{}) catch {};

    if (can_hit) writer.print("[H]it ", .{}) catch {};
    if (can_stand) writer.print("[S]tand ", .{}) catch {};
    if (can_double) writer.print("[D]ouble ", .{}) catch {};
    if (can_split) writer.print("s[P]lit ", .{}) catch {};
    if (can_insurance) writer.print("[I]nsurance ", .{}) catch {};

    writer.print("[Q]uit\n> ", .{}) catch {};
}

/// Display game result
pub fn displayResult(outcome: config.HandOutcome) void {
    const writer = stdout.writer();

    const color = switch (outcome) {
        .PLAYER_WIN, .BLACKJACK, .DEALER_BUST => Color.GREEN,
        .PLAYER_LOSS, .PLAYER_BUST => Color.RED,
        .PUSH => Color.YELLOW,
        else => Color.WHITE,
    };

    setColor(color);

    const result_text = switch (outcome) {
        .PLAYER_WIN => "YOU WIN!",
        .PLAYER_LOSS => "Dealer wins",
        .PLAYER_BUST => "BUST - You lose",
        .BLACKJACK => "BLACKJACK! You win!",
        .DEALER_BUST => "Dealer busts - You win!",
        .PUSH => "PUSH - It's a tie",
        else => "Hand complete",
    };

    writer.print("\n{s}\n", .{result_text}) catch {};
    setColor(.RESET);
}

/// Display statistics
pub fn displayStatistics(stats: config.Statistics, allocator: mem.Allocator) void {
    const writer = stdout.writer();

    writer.print("\n╔════════════════════════════════════╗\n", .{}) catch {};
    writer.print("║      SESSION STATISTICS            ║\n", .{}) catch {};
    writer.print("╠════════════════════════════════════╣\n", .{}) catch {};

    writer.print("║ Hands Played:        {:<3}          ║\n", .{stats.total_hands}) catch {};

    if (stats.total_hands > 0) {
        const win_pct = stats.winPercentage();
        const loss_pct = stats.lossPercentage();
        const push_pct = stats.pushPercentage();

        writer.print("║ Wins:                {:<2} ({:.1}%)    ║\n", .{ stats.total_wins, win_pct }) catch {};
        writer.print("║ Losses:              {:<2} ({:.1}%)    ║\n", .{ stats.total_losses, loss_pct }) catch {};
        writer.print("║ Pushes:              {:<2} ({:.1}%)    ║\n", .{ stats.total_pushes, push_pct }) catch {};
    }

    writer.print("║ Blackjacks:          {:<3}          ║\n", .{stats.total_blackjacks}) catch {};
    writer.print("║                                    ║\n", .{}) catch {};

    const profit = @as(f32, @floatFromInt(stats.total_profit)) / 100.0;
    const biggest_win = @as(f32, @floatFromInt(stats.biggest_win)) / 100.0;
    const biggest_loss = @as(f32, @floatFromInt(stats.biggest_loss)) / 100.0;

    if (stats.total_profit >= 0) {
        setColor(.GREEN);
    } else {
        setColor(.RED);
    }

    writer.print("║ Profit/Loss:         ${d:>7.2}      ║\n", .{profit}) catch {};
    setColor(.RESET);

    writer.print("║ Biggest Win:         ${d:>7.2}      ║\n", .{biggest_win}) catch {};
    writer.print("║ Biggest Loss:        ${d:>7.2}      ║\n", .{biggest_loss}) catch {};

    if (stats.win_streak > 0) {
        writer.print("║ Win Streak:          +{} hands       ║\n", .{stats.win_streak}) catch {};
    } else if (stats.win_streak < 0) {
        writer.print("║ Loss Streak:         {} hands        ║\n", .{-stats.win_streak}) catch {};
    }

    writer.print("║ Longest Streak:      +{} hands       ║\n", .{stats.max_win_streak}) catch {};

    writer.print("╚════════════════════════════════════╝\n", .{}) catch {};
}

/// Display an error message
pub fn displayError(message: []const u8) void {
    const writer = stdout.writer();

    setColor(.RED);
    writer.print("\nError: {s}\n", .{message}) catch {};
    setColor(.RESET);
}

/// Display a message
pub fn displayMessage(message: []const u8) void {
    const writer = stdout.writer();

    writer.print("\n{s}\n", .{message}) catch {};
}

/// Read a single character input (non-blocking style)
pub fn readCommand() !config.Command {
    var buf: [1]u8 = undefined;
    const bytes_read = try stdin.read(&buf);

    if (bytes_read == 0) {
        return .UNKNOWN;
    }

    return config.Command.fromChar(buf[0]);
}

/// Read a line of input
pub fn readLine(allocator: mem.Allocator) ![]u8 {
    var buffer = std.ArrayList(u8).init(allocator);
    try stdin.reader().readUntilDelimiterArrayList(&buffer, '\n', 256);
    return buffer.toOwnedSlice();
}

/// Wait for user confirmation
pub fn waitForConfirmation() void {
    const reader = stdin.reader();
    var buf: [1]u8 = undefined;

    stdout.writeAll("\nPress Enter to continue...") catch {};
    _ = reader.read(&buf) catch {};
}

/// Full game display (dealer and player hands with values)
pub fn displayGameState(
    g: *game.Game,
    show_dealer_hole: bool,
    allocator: mem.Allocator,
) void {
    clearScreen();
    displayHeader();
    displayBankroll(g.bankroll, g.current_bet, g.statistics.total_hands, g.statistics.total_wins, g.statistics.total_losses);

    // Display dealer hand
    displayDealerHand(g.dealer_hand, show_dealer_hole, allocator);

    // Display player hands
    for (g.player_hands.items, 0..) |h, i| {
        displayPlayerHand(h, @intCast(i), g.player_hands.items.len, allocator);
    }
}
