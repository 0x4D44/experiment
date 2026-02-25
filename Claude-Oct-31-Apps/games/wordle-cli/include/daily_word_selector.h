#ifndef WORDLE_DAILY_WORD_SELECTOR_H
#define WORDLE_DAILY_WORD_SELECTOR_H

#include "common.h"
#include "word_list.h"

class DailyWordSelector {
public:
    // Get the daily word (same for all players on a given day)
    static std::string getDailyWord(const WordList& wordList);

    // Get the daily word for a specific date
    static std::string getWordForDate(const WordList& wordList,
                                     std::chrono::system_clock::time_point date);

    // Get today's date in UTC
    static std::chrono::system_clock::time_point getToday();

    // Get seed for today
    static uint32_t getDaySeed();

    // Get seed for a specific date
    static uint32_t getSeedForDate(std::chrono::system_clock::time_point date);

private:
    // Convert date to YYYYMMDD format
    static std::string formatDateAsSeed(std::chrono::system_clock::time_point date);
};

#endif // WORDLE_DAILY_WORD_SELECTOR_H
