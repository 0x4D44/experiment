#include "wordle_game.h"
#include <iostream>

int main() {
    try {
        WordleGame game;
        game.run();
        game.cleanup();
    } catch (const std::exception& e) {
        std::cerr << "Error: " << e.what() << std::endl;
        return 1;
    }
    return 0;
}
