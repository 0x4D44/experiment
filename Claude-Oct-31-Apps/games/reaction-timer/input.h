/**
 * input.h - Low-latency input handling interface
 *
 * Provides platform-optimized input detection with minimal
 * latency using raw terminal mode and non-blocking I/O.
 */

#ifndef INPUT_H
#define INPUT_H

#include <stdint.h>
#include <stddef.h>

/* Maximum latency from key press to detection (milliseconds) */
#define INPUT_MAX_LATENCY_MS 5

/* Input polling interval (microseconds) */
#define INPUT_POLL_INTERVAL_US 100

/* Maximum simultaneous key presses to track */
#define INPUT_MAX_KEYS 4

typedef struct {
    char key;                   /* The key that was pressed */
    uint64_t timestamp_us;      /* When it was detected (microseconds) */
    uint8_t is_valid;           /* Passed validation checks */
} InputEvent;

typedef struct {
    int raw_mode_enabled;       /* Current terminal mode flag */
    uint64_t last_event_time;   /* Timestamp of last input */
    uint8_t previous_key;       /* Last key pressed (debounce) */
} InputState;

/**
 * Initialize input system with raw terminal mode.
 * Disables line buffering, echo, and canonical input.
 *
 * Returns: 0 on success, -1 on failure
 */
int input_init(void);

/**
 * Shutdown input system and restore terminal to original state.
 * Must be called before program exit.
 *
 * Returns: 0 on success, -1 on failure
 */
int input_shutdown(void);

/**
 * Check if input is available without blocking.
 * Uses select() or equivalent for cross-platform compatibility.
 *
 * Returns: 1 if input available, 0 if no input, -1 on error
 */
int input_available(void);

/**
 * Read input with timestamp.
 * Must call input_available() first to avoid blocking.
 * Timestamp captured at detection time.
 *
 * Returns: 0 if input read successfully, -1 on error
 */
int input_read_event(InputEvent *event);

/**
 * Wait for input with maximum timeout.
 * Internally uses high-frequency polling for minimal latency.
 * Returns as soon as key is detected.
 *
 * timeout_ms: Maximum milliseconds to wait (0 = infinite)
 *
 * Returns: 0 on input detected, 1 on timeout, -1 on error
 */
int input_wait_timeout(uint32_t timeout_ms, InputEvent *event);

/**
 * Flush any buffered input.
 * Call after showing a prompt to clear stale keypresses.
 *
 * Returns: 0 on success, -1 on error
 */
int input_flush(void);

/**
 * Detect if keypress was too early (before stimulus).
 * Used for false-start detection in reaction tests.
 *
 * press_time_us: Time of keypress (microseconds)
 * stimulus_time_us: Time stimulus was displayed
 * threshold_us: Minimum time before stimulus (default 50000)
 *
 * Returns: 1 if false start, 0 otherwise
 */
int input_is_false_start(uint64_t press_time_us, uint64_t stimulus_time_us,
                          uint64_t threshold_us);

/**
 * Measure input latency for current system.
 * Run several times to get accurate baseline.
 * Should be called during initialization.
 *
 * Returns: Estimated latency in microseconds, 0 on failure
 */
uint64_t input_measure_latency(void);

/**
 * Get current input system state.
 * For diagnostics and debugging.
 *
 * Returns: Pointer to input state (read-only)
 */
const InputState* input_get_state(void);

/**
 * Convert input key code to displayable character.
 * Handles special keys (space, enter, etc.).
 *
 * key: Raw key code
 * Returns: Human-readable key name
 */
const char* input_key_to_string(char key);

#endif /* INPUT_H */
