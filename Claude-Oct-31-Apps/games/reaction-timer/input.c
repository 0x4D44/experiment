/**
 * input.c - Low-latency input handling
 *
 * Raw terminal mode input with minimal latency.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <termios.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/select.h>
#include "input.h"
#include "timer.h"

/* Global input state */
static InputState global_input = {0};
static struct termios original_tty = {0};

/**
 * Initialize input system with raw terminal mode.
 */
int input_init(void) {
    struct termios tty;

    /* Get current terminal settings */
    if (tcgetattr(STDIN_FILENO, &tty) != 0) {
        return -1;
    }

    /* Save original settings */
    memcpy(&original_tty, &tty, sizeof(struct termios));

    /* Set raw mode */
    tty.c_lflag &= ~(ICANON | ECHO);
    tty.c_cc[VMIN] = 0;
    tty.c_cc[VTIME] = 0;

    if (tcsetattr(STDIN_FILENO, TCSAFLUSH, &tty) != 0) {
        return -1;
    }

    /* Set non-blocking mode */
    int flags = fcntl(STDIN_FILENO, F_GETFL, 0);
    if (fcntl(STDIN_FILENO, F_SETFL, flags | O_NONBLOCK) != 0) {
        tcsetattr(STDIN_FILENO, TCSAFLUSH, &original_tty);
        return -1;
    }

    global_input.raw_mode_enabled = 1;
    return 0;
}

/**
 * Shutdown input system and restore terminal.
 */
int input_shutdown(void) {
    if (global_input.raw_mode_enabled) {
        tcsetattr(STDIN_FILENO, TCSAFLUSH, &original_tty);
        global_input.raw_mode_enabled = 0;
    }
    return 0;
}

/**
 * Check if input is available without blocking.
 */
int input_available(void) {
    fd_set readfds;
    struct timeval tv;

    FD_ZERO(&readfds);
    FD_SET(STDIN_FILENO, &readfds);

    tv.tv_sec = 0;
    tv.tv_usec = 0;

    int ret = select(STDIN_FILENO + 1, &readfds, NULL, NULL, &tv);

    if (ret > 0 && FD_ISSET(STDIN_FILENO, &readfds)) {
        return 1;
    }

    return 0;
}

/**
 * Read input with timestamp.
 */
int input_read_event(InputEvent *event) {
    if (event == NULL) {
        return -1;
    }

    int ch = getchar();
    if (ch == EOF) {
        return -1;
    }

    event->key = (char)ch;
    event->timestamp_us = timer_get_time_us();
    event->is_valid = 1;

    return 0;
}

/**
 * Wait for input with timeout.
 */
int input_wait_timeout(uint32_t timeout_ms, InputEvent *event) {
    uint64_t start = timer_get_time_us();
    uint64_t timeout_us = (uint64_t)timeout_ms * 1000;

    while (1) {
        if (input_available()) {
            if (input_read_event(event) == 0) {
                return 0;
            }
        }

        uint64_t elapsed = timer_elapsed_us(start, timer_get_time_us());
        if (elapsed >= timeout_us && timeout_ms > 0) {
            return 1;  /* Timeout */
        }

        usleep(1000);  /* Sleep 1ms between polls */
    }
}

/**
 * Flush buffered input.
 */
int input_flush(void) {
    while (input_available()) {
        InputEvent event;
        if (input_read_event(&event) != 0) {
            break;
        }
    }
    return 0;
}

/**
 * Detect false start.
 */
int input_is_false_start(uint64_t press_time_us, uint64_t stimulus_time_us,
                          uint64_t threshold_us) {
    if (press_time_us < stimulus_time_us) {
        uint64_t diff = stimulus_time_us - press_time_us;
        return (diff > threshold_us) ? 1 : 0;
    }
    return 0;
}

/**
 * Measure input latency.
 */
uint64_t input_measure_latency(void) {
    /* This would require special hardware timing */
    /* For now, return a reasonable estimate */
    return 2000;  /* ~2ms typical latency */
}

/**
 * Get input state.
 */
const InputState* input_get_state(void) {
    return &global_input;
}

/**
 * Convert key to string.
 */
const char* input_key_to_string(char key) {
    static char buf[16];

    switch (key) {
        case ' ':
            return "SPACE";
        case '\n':
            return "ENTER";
        case '\t':
            return "TAB";
        case 27:
            return "ESC";
        case 127:
            return "BACKSPACE";
        default:
            snprintf(buf, sizeof(buf), "%c", key);
            return buf;
    }
}
