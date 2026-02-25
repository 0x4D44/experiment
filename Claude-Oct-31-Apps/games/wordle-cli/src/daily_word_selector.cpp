#include "daily_word_selector.h"
#include <ctime>
#include <random>
#include <iomanip>
#include <sstream>

std::string DailyWordSelector::getDailyWord(const WordList& wordList) {
    auto today = getToday();
    return getWordForDate(wordList, today);
}

std::string DailyWordSelector::getWordForDate(const WordList& wordList,
                                             std::chrono::system_clock::time_point date) {
    if (wordList.getAnswerCount() == 0) {
        return "";
    }

    uint32_t seed = getSeedForDate(date);
    std::mt19937 rng(seed);
    size_t index = rng() % wordList.getAnswerCount();
    return wordList.getAnswerWord(index);
}

std::chrono::system_clock::time_point DailyWordSelector::getToday() {
    auto now = std::chrono::system_clock::now();
    auto t = std::chrono::system_clock::to_time_t(now);
    auto tm = std::gmtime(&t);

    // Reset time to midnight UTC
    tm->tm_hour = 0;
    tm->tm_min = 0;
    tm->tm_sec = 0;

    return std::chrono::system_clock::from_time_t(std::mktime(tm));
}

uint32_t DailyWordSelector::getDaySeed() {
    auto today = getToday();
    return getSeedForDate(today);
}

uint32_t DailyWordSelector::getSeedForDate(std::chrono::system_clock::time_point date) {
    auto t = std::chrono::system_clock::to_time_t(date);
    auto tm = std::gmtime(&t);

    // Format as YYYYMMDD
    uint32_t seed = (1900 + tm->tm_year) * 10000 +
                    (tm->tm_mon + 1) * 100 +
                    tm->tm_mday;

    return seed;
}

std::string DailyWordSelector::formatDateAsSeed(std::chrono::system_clock::time_point date) {
    auto t = std::chrono::system_clock::to_time_t(date);
    auto tm = std::gmtime(&t);

    std::stringstream ss;
    ss << std::put_time(tm, "%Y%m%d");
    return ss.str();
}
