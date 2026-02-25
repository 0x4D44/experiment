/**
 * Beat Maker Quest - Test Suite
 * Comprehensive tests for game logic, sequencer, and puzzle mode
 */

import {
  SequencerEngine,
  PuzzleMode,
  PatternStorage,
  Note,
  SequencerGrid,
  Instrument,
} from './beat-maker-quest';

// ============================================================================
// SEQUENCER ENGINE TESTS
// ============================================================================

describe('SequencerEngine', () => {
  let engine: SequencerEngine;

  beforeEach(() => {
    engine = new SequencerEngine(8, 16, 120);
  });

  describe('Initialization', () => {
    it('should create an empty grid with specified dimensions', () => {
      const grid = engine.getGrid();
      expect(grid.tracks).toBe(8);
      expect(grid.steps).toBe(16);
      expect(grid.cells).toHaveLength(8);
      expect(grid.cells[0]).toHaveLength(16);
    });

    it('should initialize with 8 instruments', () => {
      const instruments = engine.getInstruments();
      expect(instruments.size).toBe(8);
    });

    it('should set initial BPM correctly', () => {
      expect(engine.getBPM()).toBe(120);
    });

    it('should initialize in stopped state', () => {
      expect(engine.isPlayingState()).toBe(false);
    });

    it('should start at step 0', () => {
      expect(engine.getCurrentStep()).toBe(0);
    });
  });

  describe('Note Management', () => {
    it('should set a note in the grid', () => {
      const note: Note = { pitch: 0, octave: 4, velocity: 100 };
      engine.setNote(0, 0, note);
      expect(engine.getNote(0, 0)).toEqual(note);
    });

    it('should clear a note from the grid', () => {
      const note: Note = { pitch: 5, octave: 4, velocity: 80 };
      engine.setNote(2, 4, note);
      engine.setNote(2, 4, null);
      expect(engine.getNote(2, 4)).toBeNull();
    });

    it('should return null for unset notes', () => {
      expect(engine.getNote(1, 1)).toBeNull();
    });

    it('should throw error for invalid track', () => {
      const note: Note = { pitch: 0, octave: 4, velocity: 100 };
      expect(() => engine.setNote(10, 0, note)).toThrow();
    });

    it('should throw error for invalid step', () => {
      const note: Note = { pitch: 0, octave: 4, velocity: 100 };
      expect(() => engine.setNote(0, 20, note)).toThrow();
    });

    it('should handle negative track gracefully', () => {
      const note: Note = { pitch: 0, octave: 4, velocity: 100 };
      expect(() => engine.setNote(-1, 0, note)).toThrow();
    });

    it('should allow multiple notes on same track', () => {
      const note1: Note = { pitch: 0, octave: 4, velocity: 100 };
      const note2: Note = { pitch: 5, octave: 4, velocity: 80 };
      engine.setNote(0, 0, note1);
      engine.setNote(0, 4, note2);
      expect(engine.getNote(0, 0)).toEqual(note1);
      expect(engine.getNote(0, 4)).toEqual(note2);
    });
  });

  describe('BPM Control', () => {
    it('should set valid BPM', () => {
      engine.setBPM(140);
      expect(engine.getBPM()).toBe(140);
    });

    it('should reject BPM below minimum (40)', () => {
      expect(() => engine.setBPM(39)).toThrow();
    });

    it('should reject BPM above maximum (300)', () => {
      expect(() => engine.setBPM(301)).toThrow();
    });

    it('should accept boundary BPMs', () => {
      engine.setBPM(40);
      expect(engine.getBPM()).toBe(40);
      engine.setBPM(300);
      expect(engine.getBPM()).toBe(300);
    });

    it('should handle fractional BPM values', () => {
      engine.setBPM(120.5);
      expect(engine.getBPM()).toBe(120.5);
    });
  });

  describe('Grid Management', () => {
    it('should clear entire grid', () => {
      const note: Note = { pitch: 0, octave: 4, velocity: 100 };
      engine.setNote(0, 0, note);
      engine.setNote(1, 5, note);
      engine.clearGrid();
      expect(engine.getNote(0, 0)).toBeNull();
      expect(engine.getNote(1, 5)).toBeNull();
    });

    it('should preserve grid dimensions after clearing', () => {
      engine.clearGrid();
      const grid = engine.getGrid();
      expect(grid.tracks).toBe(8);
      expect(grid.steps).toBe(16);
    });
  });

  describe('Playback State', () => {
    it('should report correct playback state', () => {
      expect(engine.isPlayingState()).toBe(false);
      engine.play();
      expect(engine.isPlayingState()).toBe(true);
      engine.stop();
      expect(engine.isPlayingState()).toBe(false);
    });

    it('should not play if already playing', () => {
      engine.play();
      engine.play(); // Should not error
      expect(engine.isPlayingState()).toBe(true);
    });

    it('should return to step 0 after stop', () => {
      engine.play();
      // Simulate some steps by manually incrementing (in real scenario, timer would do this)
      engine.stop();
      expect(engine.getCurrentStep()).toBe(0);
    });
  });

  describe('Pattern Export/Import', () => {
    it('should export current pattern', () => {
      const note: Note = { pitch: 3, octave: 4, velocity: 90 };
      engine.setNote(0, 0, note);
      engine.setBPM(140);

      const pattern = engine.exportPattern();

      expect(pattern.name).toBeDefined();
      expect(pattern.bpm).toBe(140);
      expect(pattern.grid.tracks).toBe(8);
      expect(pattern.grid.steps).toBe(16);
      expect(pattern.createdAt).toBeLessThanOrEqual(Date.now());
    });

    it('should import pattern correctly', () => {
      const note: Note = { pitch: 5, octave: 3, velocity: 85 };
      const note2: Note = { pitch: 7, octave: 4, velocity: 95 };

      const grid: SequencerGrid = {
        tracks: 8,
        steps: 16,
        cells: Array(8)
          .fill(null)
          .map((_, i) => {
            const row = new Array(16).fill(null);
            if (i === 0) row[0] = note;
            if (i === 2) row[5] = note2;
            return row;
          }),
      };

      const pattern = {
        id: 'test-pattern',
        name: 'Test Pattern',
        bpm: 160,
        grid,
        createdAt: Date.now(),
        modifiedAt: Date.now(),
      };

      engine.importPattern(pattern);
      expect(engine.getBPM()).toBe(160);
      expect(engine.getNote(0, 0)).toEqual(note);
      expect(engine.getNote(2, 5)).toEqual(note2);
    });

    it('should deep copy grid on export', () => {
      const note: Note = { pitch: 0, octave: 4, velocity: 100 };
      engine.setNote(0, 0, note);

      const pattern1 = engine.exportPattern();
      engine.setNote(0, 0, null);
      engine.setNote(0, 1, note);

      const pattern2 = engine.exportPattern();

      expect(pattern1.grid.cells[0][0]).not.toBeNull();
      expect(pattern2.grid.cells[0][0]).toBeNull();
    });
  });

  describe('Instrument Information', () => {
    it('should have 8 predefined instruments', () => {
      const instruments = engine.getInstruments();
      expect(instruments.size).toBe(8);

      const instrumentNames = Array.from(instruments.values()).map((i) => i.name);
      expect(instrumentNames).toContain('Kick Drum');
      expect(instrumentNames).toContain('Snare');
      expect(instrumentNames).toContain('Bass');
    });

    it('should have distinct colors for instruments', () => {
      const instruments = engine.getInstruments();
      const colors = new Set(Array.from(instruments.values()).map((i) => i.color));
      expect(colors.size).toBe(8); // All unique
    });

    it('should have categorized instrument types', () => {
      const instruments = engine.getInstruments();
      const types = Array.from(instruments.values()).map((i) => i.type);
      expect(types).toContain('drums');
      expect(types).toContain('bass');
      expect(types).toContain('melody');
    });
  });
});

// ============================================================================
// PUZZLE MODE TESTS
// ============================================================================

describe('PuzzleMode', () => {
  let puzzleMode: PuzzleMode;

  beforeEach(() => {
    puzzleMode = new PuzzleMode();
  });

  describe('Levels', () => {
    it('should have at least 5 puzzle levels', () => {
      const levels = puzzleMode.getLevels();
      expect(levels.length).toBeGreaterThanOrEqual(5);
    });

    it('should have levels with unique IDs', () => {
      const levels = puzzleMode.getLevels();
      const ids = levels.map((l) => l.id);
      const uniqueIds = new Set(ids);
      expect(uniqueIds.size).toBe(ids.length);
    });

    it('should have levels with difficulty ratings', () => {
      const levels = puzzleMode.getLevels();
      const difficulties = levels.map((l) => l.difficulty);
      expect(difficulties.every((d) => ['easy', 'medium', 'hard'].includes(d))).toBe(true);
    });

    it('should have increasing difficulty progression', () => {
      const levels = puzzleMode.getLevels();
      const difficultyOrder = { easy: 0, medium: 1, hard: 2 };
      let lastDifficulty = -1;

      for (const level of levels) {
        const currentDifficulty = difficultyOrder[level.difficulty];
        expect(currentDifficulty).toBeGreaterThanOrEqual(lastDifficulty);
        lastDifficulty = currentDifficulty;
      }
    });

    it('should have target patterns in all levels', () => {
      const levels = puzzleMode.getLevels();
      for (const level of levels) {
        expect(level.targetPattern).toBeDefined();
        expect(level.targetPattern.grid).toBeDefined();
        expect(level.targetPattern.bpm).toBeGreaterThan(0);
      }
    });

    it('should have descriptions for all levels', () => {
      const levels = puzzleMode.getLevels();
      for (const level of levels) {
        expect(level.description).toBeTruthy();
        expect(level.description.length).toBeGreaterThan(0);
      }
    });

    it('should have positive max scores', () => {
      const levels = puzzleMode.getLevels();
      for (const level of levels) {
        expect(level.maxScore).toBeGreaterThan(0);
      }
    });
  });

  describe('Level Navigation', () => {
    it('should start at first level', () => {
      const currentLevel = puzzleMode.getCurrentLevel();
      expect(currentLevel).not.toBeNull();
      expect(currentLevel!.id).toBe('level1');
    });

    it('should move to next level', () => {
      const firstLevel = puzzleMode.getCurrentLevel();
      const moved = puzzleMode.nextLevel();
      const secondLevel = puzzleMode.getCurrentLevel();

      expect(moved).toBe(true);
      expect(secondLevel!.id).not.toBe(firstLevel!.id);
    });

    it('should return false when at last level', () => {
      const levels = puzzleMode.getLevels();
      for (let i = 0; i < levels.length; i++) {
        puzzleMode.setCurrentLevel(i);
      }

      const canMove = puzzleMode.nextLevel();
      expect(canMove).toBe(false);
    });

    it('should set current level by index', () => {
      const levels = puzzleMode.getLevels();
      const targetIndex = Math.min(2, levels.length - 1);

      const success = puzzleMode.setCurrentLevel(targetIndex);
      expect(success).toBe(true);

      const currentLevel = puzzleMode.getCurrentLevel();
      expect(currentLevel).toEqual(levels[targetIndex]);
    });

    it('should return false for invalid level index', () => {
      const success = puzzleMode.setCurrentLevel(999);
      expect(success).toBe(false);
    });

    it('should get level by index', () => {
      const levels = puzzleMode.getLevels();
      const retrieved = puzzleMode.getLevel(0);
      expect(retrieved).toEqual(levels[0]);
    });

    it('should return null for out of bounds level', () => {
      const level = puzzleMode.getLevel(999);
      expect(level).toBeNull();
    });
  });

  describe('Score Calculation', () => {
    it('should calculate perfect score for exact match', () => {
      const level = puzzleMode.getCurrentLevel()!;
      const targetGrid = level.targetPattern.grid;
      const score = puzzleMode.calculateScore(targetGrid, targetGrid, level.maxScore);

      expect(score).toBe(level.maxScore);
    });

    it('should calculate zero score for empty grid', () => {
      const level = puzzleMode.getCurrentLevel()!;
      const emptyGrid: SequencerGrid = {
        tracks: 8,
        steps: 16,
        cells: Array(8)
          .fill(null)
          .map(() => new Array(16).fill(null)),
      };
      const targetGrid = level.targetPattern.grid;

      const score = puzzleMode.calculateScore(emptyGrid, targetGrid, level.maxScore);
      expect(score).toBe(0);
    });

    it('should calculate partial score for partial match', () => {
      const level = puzzleMode.getCurrentLevel()!;
      const targetGrid = level.targetPattern.grid;

      // Create player grid with only first half of target notes
      const playerGrid: SequencerGrid = {
        tracks: 8,
        steps: 16,
        cells: targetGrid.cells.map((track) =>
          track.map((note, j) => {
            // Only include notes in first 8 steps
            return j < 8 ? note : null;
          })
        ),
      };

      const targetNoteCount = targetGrid.cells.flat().filter((n) => n !== null).length;

      // Only test if target has enough notes
      if (targetNoteCount > 2) {
        const score = puzzleMode.calculateScore(playerGrid, targetGrid, level.maxScore);
        expect(score).toBeGreaterThanOrEqual(0);
        expect(score).toBeLessThanOrEqual(level.maxScore);
      }
    });

    it('should return 0 for empty target grid', () => {
      const emptyGrid: SequencerGrid = {
        tracks: 8,
        steps: 16,
        cells: Array(8)
          .fill(null)
          .map(() => new Array(16).fill(null)),
      };
      const playerGrid: SequencerGrid = {
        tracks: 8,
        steps: 16,
        cells: Array(8)
          .fill(null)
          .map(() => new Array(16).fill(null)),
      };

      const score = puzzleMode.calculateScore(playerGrid, emptyGrid, 100);
      expect(score).toBe(0);
    });

    it('should be case-sensitive for pitch and octave', () => {
      const level = puzzleMode.getCurrentLevel()!;
      const targetGrid = level.targetPattern.grid;

      // Create player grid with wrong octaves
      const playerGrid: SequencerGrid = {
        tracks: 8,
        steps: 16,
        cells: targetGrid.cells.map((track) =>
          track.map((note) =>
            note
              ? { ...note, octave: note.octave + 1 }
              : null
          )
        ),
      };

      // If target has notes, score should be lower
      const targetNoteCount = targetGrid.cells.flat().filter((n) => n !== null).length;
      if (targetNoteCount > 0) {
        const score = puzzleMode.calculateScore(playerGrid, targetGrid, level.maxScore);
        expect(score).toBeLessThan(level.maxScore);
      }
    });
  });
});

// ============================================================================
// PATTERN STORAGE TESTS
// ============================================================================

describe('PatternStorage', () => {
  let storage: PatternStorage;

  beforeEach(() => {
    // Clear localStorage before each test
    localStorage.clear();
    storage = new PatternStorage();
  });

  afterEach(() => {
    localStorage.clear();
  });

  describe('Save and Retrieve', () => {
    it('should save a pattern to storage', () => {
      const pattern = {
        id: 'test-1',
        name: 'Test Pattern',
        bpm: 120,
        grid: {
          tracks: 8,
          steps: 16,
          cells: Array(8)
            .fill(null)
            .map(() => new Array(16).fill(null)),
        },
        createdAt: Date.now(),
        modifiedAt: Date.now(),
      };

      storage.savePattern(pattern);
      const retrieved = storage.getPattern('test-1');

      expect(retrieved).toEqual(pattern);
    });

    it('should retrieve all saved patterns', () => {
      const patterns = [
        {
          id: 'test-1',
          name: 'Pattern 1',
          bpm: 120,
          grid: { tracks: 8, steps: 16, cells: Array(8).fill(null).map(() => new Array(16).fill(null)) },
          createdAt: Date.now(),
          modifiedAt: Date.now(),
        },
        {
          id: 'test-2',
          name: 'Pattern 2',
          bpm: 140,
          grid: { tracks: 8, steps: 16, cells: Array(8).fill(null).map(() => new Array(16).fill(null)) },
          createdAt: Date.now(),
          modifiedAt: Date.now(),
        },
      ];

      patterns.forEach((p) => storage.savePattern(p));
      const retrieved = storage.getAllPatterns();

      expect(retrieved).toHaveLength(2);
      expect(retrieved).toContainEqual(patterns[0]);
      expect(retrieved).toContainEqual(patterns[1]);
    });

    it('should return null for non-existent pattern', () => {
      const retrieved = storage.getPattern('non-existent');
      expect(retrieved).toBeNull();
    });
  });

  describe('Update Pattern', () => {
    it('should update existing pattern', () => {
      const originalPattern = {
        id: 'test-1',
        name: 'Original Name',
        bpm: 120,
        grid: { tracks: 8, steps: 16, cells: Array(8).fill(null).map(() => new Array(16).fill(null)) },
        createdAt: Date.now(),
        modifiedAt: Date.now(),
      };

      storage.savePattern(originalPattern);

      const updatedPattern = {
        ...originalPattern,
        name: 'Updated Name',
        bpm: 140,
        modifiedAt: Date.now(),
      };

      storage.savePattern(updatedPattern);
      const retrieved = storage.getPattern('test-1');

      expect(retrieved!.name).toBe('Updated Name');
      expect(retrieved!.bpm).toBe(140);
    });

    it('should maintain only one copy per ID', () => {
      const pattern = {
        id: 'test-1',
        name: 'Pattern',
        bpm: 120,
        grid: { tracks: 8, steps: 16, cells: Array(8).fill(null).map(() => new Array(16).fill(null)) },
        createdAt: Date.now(),
        modifiedAt: Date.now(),
      };

      storage.savePattern(pattern);
      storage.savePattern(pattern);
      storage.savePattern(pattern);

      const allPatterns = storage.getAllPatterns();
      expect(allPatterns).toHaveLength(1);
    });
  });

  describe('Delete Pattern', () => {
    it('should delete a pattern', () => {
      const pattern = {
        id: 'test-1',
        name: 'Pattern',
        bpm: 120,
        grid: { tracks: 8, steps: 16, cells: Array(8).fill(null).map(() => new Array(16).fill(null)) },
        createdAt: Date.now(),
        modifiedAt: Date.now(),
      };

      storage.savePattern(pattern);
      storage.deletePattern('test-1');
      const retrieved = storage.getPattern('test-1');

      expect(retrieved).toBeNull();
    });

    it('should handle deleting non-existent pattern gracefully', () => {
      expect(() => storage.deletePattern('non-existent')).not.toThrow();
    });
  });

  describe('Clear Storage', () => {
    it('should clear all patterns', () => {
      const pattern1 = {
        id: 'test-1',
        name: 'Pattern 1',
        bpm: 120,
        grid: { tracks: 8, steps: 16, cells: Array(8).fill(null).map(() => new Array(16).fill(null)) },
        createdAt: Date.now(),
        modifiedAt: Date.now(),
      };

      const pattern2 = {
        id: 'test-2',
        name: 'Pattern 2',
        bpm: 140,
        grid: { tracks: 8, steps: 16, cells: Array(8).fill(null).map(() => new Array(16).fill(null)) },
        createdAt: Date.now(),
        modifiedAt: Date.now(),
      };

      storage.savePattern(pattern1);
      storage.savePattern(pattern2);
      storage.clearAllPatterns();

      const allPatterns = storage.getAllPatterns();
      expect(allPatterns).toHaveLength(0);
    });
  });

  describe('JSON Export/Import', () => {
    it('should export pattern as JSON string', () => {
      const pattern = {
        id: 'test-1',
        name: 'Test Pattern',
        bpm: 120,
        grid: { tracks: 8, steps: 16, cells: Array(8).fill(null).map(() => new Array(16).fill(null)) },
        createdAt: Date.now(),
        modifiedAt: Date.now(),
      };

      const json = storage.exportAsJSON(pattern);
      expect(typeof json).toBe('string');
      expect(json).toContain('test-1');
      expect(json).toContain('Test Pattern');
    });

    it('should import pattern from JSON string', () => {
      const originalPattern = {
        id: 'test-1',
        name: 'Test Pattern',
        bpm: 120,
        grid: { tracks: 8, steps: 16, cells: Array(8).fill(null).map(() => new Array(16).fill(null)) },
        createdAt: Date.now(),
        modifiedAt: Date.now(),
      };

      const json = storage.exportAsJSON(originalPattern);
      const imported = storage.importFromJSON(json);

      expect(imported.id).toBe(originalPattern.id);
      expect(imported.name).toBe(originalPattern.name);
      expect(imported.bpm).toBe(originalPattern.bpm);
    });

    it('should roundtrip patterns through JSON', () => {
      const pattern = {
        id: 'test-1',
        name: 'Test Pattern',
        bpm: 120,
        grid: {
          tracks: 8,
          steps: 16,
          cells: Array(8)
            .fill(null)
            .map((_, i) => {
              const row = new Array(16).fill(null);
              if (i === 0 && 0 < 16) row[0] = { pitch: 5, octave: 3, velocity: 100 };
              return row;
            }),
        },
        createdAt: Date.now(),
        modifiedAt: Date.now(),
      };

      const json = storage.exportAsJSON(pattern);
      const imported = storage.importFromJSON(json);

      expect(imported).toEqual(pattern);
    });
  });
});

// ============================================================================
// INTEGRATION TESTS
// ============================================================================

describe('Integration Tests', () => {
  let engine: SequencerEngine;
  let puzzleMode: PuzzleMode;
  let storage: PatternStorage;

  beforeEach(() => {
    engine = new SequencerEngine(8, 16, 120);
    puzzleMode = new PuzzleMode();
    storage = new PatternStorage();
    localStorage.clear();
  });

  afterEach(() => {
    localStorage.clear();
  });

  it('should create, save, and load a pattern', () => {
    const note: Note = { pitch: 0, octave: 4, velocity: 100 };
    engine.setNote(0, 0, note);
    engine.setBPM(140);

    const pattern = engine.exportPattern();
    pattern.name = 'My First Beat';
    storage.savePattern(pattern);

    const engine2 = new SequencerEngine();
    const loaded = storage.getPattern(pattern.id);
    expect(loaded).not.toBeNull();
    engine2.importPattern(loaded!);

    expect(engine2.getBPM()).toBe(140);
    expect(engine2.getNote(0, 0)).toEqual(note);
  });

  it('should work through a complete puzzle workflow', () => {
    const level = puzzleMode.getCurrentLevel()!;
    expect(level).not.toBeNull();

    // Create a player pattern that matches the target
    const targetGrid = level.targetPattern.grid;
    const playerEngine = new SequencerEngine();
    playerEngine.importPattern({
      id: 'player',
      name: 'Player Solution',
      bpm: level.targetPattern.bpm,
      grid: JSON.parse(JSON.stringify(targetGrid)), // Copy target grid
      createdAt: Date.now(),
      modifiedAt: Date.now(),
    });

    const score = puzzleMode.calculateScore(
      playerEngine.getGrid(),
      targetGrid,
      level.maxScore
    );

    expect(score).toBe(level.maxScore);
  });

  it('should handle multiple patterns in storage', () => {
    for (let i = 0; i < 5; i++) {
      const engine2 = new SequencerEngine();
      const note: Note = { pitch: i, octave: 4, velocity: 100 };
      engine2.setNote(0, 0, note);
      const pattern = engine2.exportPattern();
      pattern.name = `Pattern ${i}`;
      storage.savePattern(pattern);
    }

    const allPatterns = storage.getAllPatterns();
    expect(allPatterns).toHaveLength(5);

    for (let i = 0; i < 5; i++) {
      const pattern = allPatterns.find((p) => p.name === `Pattern ${i}`);
      expect(pattern).toBeDefined();
    }
  });
});
