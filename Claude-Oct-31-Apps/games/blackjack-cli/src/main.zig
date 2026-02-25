/// Blackjack CLI - Main game loop and orchestration

const std = @import("std");
const mem = std.mem;
const config = @import("config.zig");
const game = @import("game.zig");
const ui = @import("ui.zig");

const stdout = std.io.getStdOut();

/// Main game orchestrator
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Initialize game
    var g = try game.Game.init(allocator, null);
    defer g.deinit();

    ui.clearScreen();
    ui.displayHeader();
    ui.displayMessage("Welcome to Blackjack CLI!");
    ui.displayMessage("Rules: Blackjack pays 3:2, Dealer stands on 17");
    ui.displayMessage("Type [H]it, [S]tand, [D]ouble, s[P]lit, or [Q]uit\n");

    ui.waitForConfirmation();

    // Main game loop
    while (!g.isGameOver()) {
        // Betting phase
        try bettingPhase(&g, allocator);

        // Deal initial hands
        try g.dealInitialHands();

        // Play the hand(s)
        if (g.state == .INSURANCE_OFFERED) {
            try insurancePhase(&g);
        }

        if (g.state == .PLAYER_TURN) {
            try playerPhase(&g, allocator);
        }

        // Dealer plays
        if (g.state == .DEALER_TURN) {
            try g.playDealerHand();
            ui.displayGameState(&g, true, allocator);
        }

        // Determine outcomes
        if (g.state == .OUTCOME_DETERMINATION) {
            try g.determineOutcomes();
            displayOutcomes(&g, allocator);
        }

        // Ask for next hand
        try g.resetForNewHand();

        if (!g.isGameOver()) {
            ui.displayMessage("\nReady for the next hand? Press Enter...");
            ui.waitForConfirmation();
        }
    }

    // Game over
    ui.clearScreen();
    ui.displayHeader();
    ui.displayMessage("GAME OVER - Bankroll depleted!");
    ui.displayStatistics(g.statistics, allocator);
}

/// Handle betting phase
fn bettingPhase(g: *game.Game, allocator: mem.Allocator) !void {
    ui.clearScreen();
    ui.displayHeader();

    const writer = stdout.writer();

    while (g.state != .BET_PLACED) {
        const bankroll_dollars = @as(f32, @floatFromInt(g.bankroll)) / 100.0;

        try writer.print("\nBankroll: ${d:.2}\n", .{bankroll_dollars});
        try writer.print("Bet amount (${} - ${}): ", .{
            config.GameConfig.MIN_BET / 100,
            @min(config.GameConfig.MAX_BET / 100, @as(i64, @intCast(g.bankroll)) / 100),
        });

        const input = ui.readLine(allocator) catch continue;
        defer allocator.free(input);

        const trimmed = std.mem.trim(u8, input, " \n\r\t");
        const bet_amount = std.fmt.parseInt(i64, trimmed, 10) catch {
            ui.displayError("Invalid amount, please enter a number");
            continue;
        };

        g.placeBet(bet_amount * 100) catch |err| {
            const msg = switch (err) {
                error.BetTooLow => "Bet too low (minimum $5)",
                error.BetTooHigh => "Bet too high (maximum $500)",
                error.InsufficientFunds => "Insufficient funds",
                else => "Invalid bet",
            };
            ui.displayError(msg);
        };
    }
}

/// Handle insurance phase
fn insurancePhase(g: *game.Game, allocator: mem.Allocator) !void {
    ui.displayGameState(g, false, allocator);

    const writer = stdout.writer();
    try writer.print("\nDealer shows an Ace. Take insurance? (Y/N): ", .{});

    const input = ui.readLine(allocator) catch return;
    defer allocator.free(input);

    const choice = std.mem.trim(u8, input, " \n\r\t");

    if (choice.len > 0 and (choice[0] == 'y' or choice[0] == 'Y')) {
        try g.takeInsurance();
    } else {
        try g.declineInsurance();
    }
}

/// Handle player turn
fn playerPhase(g: *game.Game, allocator: mem.Allocator) !void {
    while (g.state == .PLAYER_TURN) {
        ui.displayGameState(g, false, allocator);

        const current_hand = g.getCurrentHand();
        if (current_hand == null) break;

        const can_hit = current_hand.?.value < 21;
        const can_stand = true;
        const can_double = g.canDoubleCurrentHand();
        const can_split = g.canSplitCurrentHand();

        ui.displayCommands(can_hit, can_stand, can_double, can_split, false);

        const cmd = ui.readCommand() catch .UNKNOWN;

        switch (cmd) {
            .HIT => {
                if (can_hit) {
                    try g.hitCurrentHand();
                } else {
                    ui.displayError("Cannot hit on 21 or higher");
                }
            },
            .STAND => {
                try g.standCurrentHand();
            },
            .DOUBLE => {
                if (can_double) {
                    try g.doubleDown();
                } else {
                    ui.displayError("Cannot double down on this hand");
                }
            },
            .SPLIT => {
                if (can_split) {
                    try g.splitCurrentHand();
                } else {
                    ui.displayError("Cannot split this hand");
                }
            },
            .INSURANCE => {
                ui.displayError("Insurance only available after deal");
            },
            .QUIT => {
                try endGame(&g, allocator);
                std.process.exit(0);
            },
            .UNKNOWN => {
                ui.displayError("Unknown command");
            },
        }
    }
}

/// Display outcomes for all hands
fn displayOutcomes(g: *game.Game, allocator: mem.Allocator) void {
    ui.displayGameState(g, true, allocator);

    const writer = stdout.writer();

    for (0..g.player_hands.items.len) |i| {
        const player_hand = g.player_hands.items[i];

        // Determine outcome for this hand
        var outcome = game.hand.evaluateHandWithBlackjack(player_hand, g.dealer_hand);
        if (outcome == null) {
            outcome = game.hand.evaluateHand(player_hand, g.dealer_hand);
        }

        if (g.player_hands.items.len > 1) {
            try writer.print("\nHand {} result:\n", .{i + 1});
        } else {
            try writer.print("\nResult:\n", .{});
        }

        if (outcome) |o| {
            ui.displayResult(o);
        }
    }

    if (g.insurance_bet > 0) {
        const insurance_dollars = @as(f32, @floatFromInt(g.insurance_bet)) / 100.0;

        if (g.dealer_hand.isBlackjack()) {
            try writer.print("Insurance won: ${d:.2}\n", .{insurance_dollars * 2});
        } else {
            try writer.print("Insurance lost: -${d:.2}\n", .{insurance_dollars});
        }
    }

    try writer.print("\nNew bankroll: ${d:.2}\n", .{@as(f32, @floatFromInt(g.bankroll)) / 100.0});
}

/// End game and display final statistics
fn endGame(g: *game.Game, allocator: mem.Allocator) !void {
    ui.clearScreen();
    ui.displayHeader();
    ui.displayMessage("Thanks for playing!");
    ui.displayStatistics(g.statistics, allocator);
}

// ============================================================================
// Module imports for compilation
// ============================================================================

// These imports ensure that all modules are compiled and their tests are run
const _ = @import("config.zig");
const _ = @import("deck.zig");
const hand = @import("hand.zig");
const _ = hand;
const game_module = @import("game.zig");
const _ = game_module;
