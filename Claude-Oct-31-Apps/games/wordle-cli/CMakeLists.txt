cmake_minimum_required(VERSION 3.16)
project(WordleGame VERSION 1.0.0 LANGUAGES CXX)

# Set C++ standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Wextra -Wpedantic")

# Add include directories
include_directories(${CMAKE_CURRENT_SOURCE_DIR}/include)

# Source files for the main application
set(SOURCES
    src/main.cpp
    src/guess_evaluator.cpp
    src/word_list.cpp
    src/input_validator.cpp
    src/daily_word_selector.cpp
    src/keyboard_visualizer.cpp
    src/ui_renderer.cpp
    src/statistics_manager.cpp
    src/wordle_game.cpp
)

# Create the main executable
add_executable(wordle ${SOURCES})

# Enable testing
enable_testing()

# Find Google Test
find_package(GTest QUIET)

if(NOT GTest_FOUND)
    message(STATUS "Google Test not found. Downloading and building from source...")

    # Download and build Google Test
    include(FetchContent)
    FetchContent_Declare(
        googletest
        URL https://github.com/google/googletest/archive/refs/tags/v1.13.0.zip
    )
    FetchContent_MakeAvailable(googletest)
endif()

# Test executable
set(TEST_SOURCES
    tests/test_guess_evaluator.cpp
    src/guess_evaluator.cpp
    src/input_validator.cpp
)

add_executable(test_guess_evaluator ${TEST_SOURCES})
target_link_libraries(test_guess_evaluator GTest::gtest GTest::gtest_main)
add_test(NAME GuessEvaluatorTest COMMAND test_guess_evaluator)

set(TEST_SOURCES_INPUT
    tests/test_input_validator.cpp
    src/input_validator.cpp
)

add_executable(test_input_validator ${TEST_SOURCES_INPUT})
target_link_libraries(test_input_validator GTest::gtest GTest::gtest_main)
add_test(NAME InputValidatorTest COMMAND test_input_validator)

set(TEST_SOURCES_WORDLIST
    tests/test_word_list.cpp
    src/word_list.cpp
    src/input_validator.cpp
)

add_executable(test_word_list ${TEST_SOURCES_WORDLIST})
target_link_libraries(test_word_list GTest::gtest GTest::gtest_main)
add_test(NAME WordListTest COMMAND test_word_list)

set(TEST_SOURCES_DAILY
    tests/test_daily_word_selector.cpp
    src/daily_word_selector.cpp
    src/word_list.cpp
    src/input_validator.cpp
)

add_executable(test_daily_word_selector ${TEST_SOURCES_DAILY})
target_link_libraries(test_daily_word_selector GTest::gtest GTest::gtest_main)
add_test(NAME DailyWordSelectorTest COMMAND test_daily_word_selector)

set(TEST_SOURCES_STATS
    tests/test_statistics_manager.cpp
    src/statistics_manager.cpp
)

add_executable(test_statistics_manager ${TEST_SOURCES_STATS})
target_link_libraries(test_statistics_manager GTest::gtest GTest::gtest_main)
add_test(NAME StatisticsManagerTest COMMAND test_statistics_manager)

# Output directory for executables
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

# Print build info
message(STATUS "Building Wordle Game v${PROJECT_VERSION}")
message(STATUS "C++ Standard: ${CMAKE_CXX_STANDARD}")
message(STATUS "Build Type: ${CMAKE_BUILD_TYPE}")
