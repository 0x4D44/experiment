#!/bin/bash

# Web Tetris - Comprehensive Test Runner
# Runs all available tests and generates a summary

echo "========================================"
echo "  WEB TETRIS - TEST SUITE RUNNER"
echo "========================================"
echo ""

PASSED=0
FAILED=0

# Test 1: JavaScript Syntax Check
echo "1. Checking JavaScript syntax..."
if node -c tetris.js 2>/dev/null; then
    echo "   ✅ PASSED - No syntax errors"
    ((PASSED++))
else
    echo "   ❌ FAILED - Syntax errors found"
    ((FAILED++))
fi
echo ""

# Test 2: Game Logic Tests
echo "2. Running game logic tests..."
if node test_game_logic.js > /dev/null 2>&1; then
    echo "   ✅ PASSED - All logic tests passed"
    ((PASSED++))
else
    echo "   ❌ FAILED - Some logic tests failed"
    ((FAILED++))
fi
echo ""

# Test 3: Code Analysis
echo "3. Running code analysis..."
if node check_issues.js > /dev/null 2>&1; then
    echo "   ✅ PASSED - No issues found"
    ((PASSED++))
else
    echo "   ❌ FAILED - Issues detected"
    ((FAILED++))
fi
echo ""

# Test 4: Check required files exist
echo "4. Checking required files..."
ALL_FILES_EXIST=true

if [ ! -f "index.html" ]; then
    echo "   ❌ Missing: index.html"
    ALL_FILES_EXIST=false
fi

if [ ! -f "tetris.js" ]; then
    echo "   ❌ Missing: tetris.js"
    ALL_FILES_EXIST=false
fi

if [ ! -f "test.html" ]; then
    echo "   ❌ Missing: test.html"
    ALL_FILES_EXIST=false
fi

if [ "$ALL_FILES_EXIST" = true ]; then
    echo "   ✅ PASSED - All required files exist"
    ((PASSED++))
else
    echo "   ❌ FAILED - Some files missing"
    ((FAILED++))
fi
echo ""

# Test 5: Check file sizes (sanity check)
echo "5. Checking file integrity..."
INDEX_SIZE=$(wc -c < index.html)
TETRIS_SIZE=$(wc -c < tetris.js)

if [ "$INDEX_SIZE" -gt 1000 ] && [ "$TETRIS_SIZE" -gt 1000 ]; then
    echo "   ✅ PASSED - Files have expected content"
    ((PASSED++))
else
    echo "   ❌ FAILED - Files may be corrupted or empty"
    ((FAILED++))
fi
echo ""

# Summary
echo "========================================"
echo "  TEST SUMMARY"
echo "========================================"
TOTAL=$((PASSED + FAILED))
echo "Total Tests:  $TOTAL"
echo "Passed:       $PASSED ✅"
echo "Failed:       $FAILED"
echo ""

if [ "$FAILED" -eq 0 ]; then
    echo "🎉 ALL TESTS PASSED!"
    echo ""
    echo "✅ Game is COMPETITION READY!"
    echo ""
    echo "To play the game:"
    echo "  ./PLAY.sh"
    echo ""
    echo "To view test results in browser:"
    echo "  open test.html"
    echo "  open validate_game.html"
    echo ""
    exit 0
else
    echo "⚠️  SOME TESTS FAILED"
    echo ""
    echo "Please review the failed tests above."
    exit 1
fi
