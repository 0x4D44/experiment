/**
 * Beat Maker Quest - Music/Rhythm Creation Game
 * A grid-based sequencer with puzzle and sandbox modes
 */

// ============================================================================
// DATA STRUCTURES AND TYPES
// ============================================================================

export interface Note {
  pitch: number; // 0-11 representing C-B
  octave: number; // Octave level
  velocity: number; // 0-127
}

export interface Instrument {
  id: string;
  name: string;
  type: 'drums' | 'bass' | 'melody' | 'percussion' | 'keys';
  color: string;
}

export interface SequencerGrid {
  tracks: number; // Number of instrument tracks
  steps: number; // Number of time steps
  cells: (Note | null)[][]; // grid[track][step]
}

export interface Pattern {
  id: string;
  name: string;
  bpm: number;
  grid: SequencerGrid;
  createdAt: number;
  modifiedAt: number;
}

export interface PuzzleLevel {
  id: string;
  name: string;
  difficulty: 'easy' | 'medium' | 'hard';
  targetPattern: Pattern;
  description: string;
  maxScore: number;
}

export interface GameState {
  mode: 'sandbox' | 'puzzle';
  currentPattern: Pattern;
  isPlaying: boolean;
  currentStep: number;
  bpm: number;
  activeTrack: number;
  selectedNote: Note | null;
  score: number;
  currentPuzzle: PuzzleLevel | null;
}

// ============================================================================
// SEQUENCER ENGINE
// ============================================================================

export class SequencerEngine {
  private grid: SequencerGrid;
  private instruments: Map<number, Instrument>;
  private audioContext: AudioContext;
  private bpm: number;
  private isPlaying: boolean = false;
  private currentStep: number = 0;
  private stepTimer: number | null = null;
  private scheduledNotes: Map<number, OscillatorNode[]> = new Map();

  constructor(tracks: number = 8, steps: number = 16, bpm: number = 120) {
    this.grid = this.createEmptyGrid(tracks, steps);
    this.instruments = this.initializeInstruments();
    this.audioContext = new (window.AudioContext || (window as any).webkitAudioContext)();
    this.bpm = bpm;
  }

  private createEmptyGrid(tracks: number, steps: number): SequencerGrid {
    const cells: (Note | null)[][] = [];
    for (let i = 0; i < tracks; i++) {
      cells[i] = new Array(steps).fill(null);
    }
    return { tracks, steps, cells };
  }

  private initializeInstruments(): Map<number, Instrument> {
    const instruments = new Map<number, Instrument>();
    const instrumentDefs: Instrument[] = [
      { id: 'kick', name: 'Kick Drum', type: 'drums', color: '#FF0000' },
      { id: 'snare', name: 'Snare', type: 'drums', color: '#FF6B6B' },
      { id: 'hihat', name: 'Hi-Hat', type: 'drums', color: '#FFA500' },
      { id: 'tom', name: 'Tom', type: 'drums', color: '#FFD700' },
      { id: 'bass', name: 'Bass', type: 'bass', color: '#0000FF' },
      { id: 'lead', name: 'Lead', type: 'melody', color: '#00FF00' },
      { id: 'pad', name: 'Pad', type: 'keys', color: '#FF00FF' },
      { id: 'strings', name: 'Strings', type: 'melody', color: '#00FFFF' },
    ];

    instrumentDefs.forEach((inst, index) => {
      instruments.set(index, inst);
    });
    return instruments;
  }

  setNote(track: number, step: number, note: Note | null): void {
    if (track < 0 || track >= this.grid.tracks || step < 0 || step >= this.grid.steps) {
      throw new Error(`Invalid grid position: track ${track}, step ${step}`);
    }
    this.grid.cells[track][step] = note;
  }

  getNote(track: number, step: number): Note | null {
    if (track < 0 || track >= this.grid.tracks || step < 0 || step >= this.grid.steps) {
      return null;
    }
    return this.grid.cells[track][step];
  }

  getGrid(): SequencerGrid {
    return this.grid;
  }

  getInstruments(): Map<number, Instrument> {
    return this.instruments;
  }

  setBPM(bpm: number): void {
    if (bpm < 40 || bpm > 300) {
      throw new Error('BPM must be between 40 and 300');
    }
    this.bpm = bpm;
  }

  getBPM(): number {
    return this.bpm;
  }

  play(): void {
    if (this.isPlaying) return;
    this.isPlaying = true;
    this.scheduleNextStep();
  }

  stop(): void {
    this.isPlaying = false;
    this.currentStep = 0;
    if (this.stepTimer !== null) {
      clearTimeout(this.stepTimer);
      this.stepTimer = null;
    }
    this.stopAllNotes();
  }

  pause(): void {
    this.isPlaying = false;
    if (this.stepTimer !== null) {
      clearTimeout(this.stepTimer);
      this.stepTimer = null;
    }
    this.stopAllNotes();
  }

  isPlayingState(): boolean {
    return this.isPlaying;
  }

  getCurrentStep(): number {
    return this.currentStep;
  }

  private scheduleNextStep(): void {
    if (!this.isPlaying) return;

    // Play notes at current step
    for (let track = 0; track < this.grid.tracks; track++) {
      const note = this.grid.cells[track][this.currentStep];
      if (note !== null) {
        this.playNote(track, note);
      }
    }

    // Schedule next step
    const stepDuration = (60 / this.bpm) * 1000; // milliseconds per step
    this.currentStep = (this.currentStep + 1) % this.grid.steps;
    this.stepTimer = window.setTimeout(() => this.scheduleNextStep(), stepDuration);
  }

  private playNote(track: number, note: Note): void {
    const instrument = this.instruments.get(track);
    if (!instrument) return;

    const frequency = this.noteToFrequency(note.pitch, note.octave);
    this.playTone(track, frequency, instrument.type, note.velocity);
  }

  private noteToFrequency(pitch: number, octave: number): number {
    // A4 = 440 Hz
    // pitch: 0-11 (C to B)
    // octave: 0-8
    const noteIndex = octave * 12 + pitch;
    const semitoneOffset = noteIndex - 57; // A4 is at index 57
    return 440 * Math.pow(2, semitoneOffset / 12);
  }

  private playTone(track: number, frequency: number, type: string, velocity: number): void {
    const now = this.audioContext.currentTime;
    const duration = (60 / this.bpm) * 1000 / 1000; // In seconds

    const oscillator = this.audioContext.createOscillator();
    const gain = this.audioContext.createGain();

    // Set waveform based on instrument type
    switch (type) {
      case 'drums':
        oscillator.type = 'sine';
        break;
      case 'bass':
        oscillator.type = 'square';
        break;
      case 'melody':
        oscillator.type = 'triangle';
        break;
      case 'keys':
        oscillator.type = 'sine';
        break;
      default:
        oscillator.type = 'sine';
    }

    oscillator.frequency.setValueAtTime(frequency, now);

    // ADSR Envelope
    const attack = 0.01;
    const decay = 0.1;
    const sustain = 0.7;
    const release = 0.2;

    gain.gain.setValueAtTime(0, now);
    gain.gain.linearRampToValueAtTime((velocity / 127) * 0.3, now + attack);
    gain.gain.exponentialRampToValueAtTime(sustain * (velocity / 127) * 0.3, now + attack + decay);
    gain.gain.setValueAtTime(sustain * (velocity / 127) * 0.3, now + attack + decay);
    gain.gain.exponentialRampToValueAtTime(0.001, now + duration - release);

    oscillator.connect(gain);
    gain.connect(this.audioContext.destination);

    oscillator.start(now);
    oscillator.stop(now + duration);

    if (!this.scheduledNotes.has(track)) {
      this.scheduledNotes.set(track, []);
    }
    this.scheduledNotes.get(track)!.push(oscillator);
  }

  private stopAllNotes(): void {
    this.scheduledNotes.forEach((oscillators) => {
      oscillators.forEach((osc) => {
        try {
          osc.stop();
        } catch (e) {
          // Already stopped
        }
      });
    });
    this.scheduledNotes.clear();
  }

  clearGrid(): void {
    this.grid = this.createEmptyGrid(this.grid.tracks, this.grid.steps);
  }

  exportPattern(): Pattern {
    return {
      id: this.generateId(),
      name: 'Untitled Pattern',
      bpm: this.bpm,
      grid: JSON.parse(JSON.stringify(this.grid)), // Deep copy
      createdAt: Date.now(),
      modifiedAt: Date.now(),
    };
  }

  importPattern(pattern: Pattern): void {
    this.grid = JSON.parse(JSON.stringify(pattern.grid));
    this.bpm = pattern.bpm;
  }

  private generateId(): string {
    return Math.random().toString(36).substr(2, 9);
  }
}

// ============================================================================
// PUZZLE MODE LOGIC
// ============================================================================

export class PuzzleMode {
  private levels: PuzzleLevel[] = [];
  private currentLevelIndex: number = 0;

  constructor() {
    this.initializeLevels();
  }

  private initializeLevels(): void {
    this.levels = [
      this.createLevel1(),
      this.createLevel2(),
      this.createLevel3(),
      this.createLevel4(),
      this.createLevel5(),
    ];
  }

  private createLevel1(): PuzzleLevel {
    const grid = this.createEmptyGrid();
    // Simple drum pattern
    grid.cells[0][0] = { pitch: 0, octave: 2, velocity: 100 }; // Kick
    grid.cells[0][4] = { pitch: 0, octave: 2, velocity: 100 };
    grid.cells[0][8] = { pitch: 0, octave: 2, velocity: 100 };
    grid.cells[0][12] = { pitch: 0, octave: 2, velocity: 100 };

    return {
      id: 'level1',
      name: 'Basic Beat',
      difficulty: 'easy',
      targetPattern: {
        id: 'level1-pattern',
        name: 'Basic Beat Pattern',
        bpm: 120,
        grid,
        createdAt: Date.now(),
        modifiedAt: Date.now(),
      },
      description: 'Place kick drum beats at steps 0, 4, 8, and 12',
      maxScore: 100,
    };
  }

  private createLevel2(): PuzzleLevel {
    const grid = this.createEmptyGrid();
    // Kick pattern
    grid.cells[0][0] = { pitch: 0, octave: 2, velocity: 100 };
    grid.cells[0][4] = { pitch: 0, octave: 2, velocity: 100 };
    grid.cells[0][8] = { pitch: 0, octave: 2, velocity: 100 };
    grid.cells[0][12] = { pitch: 0, octave: 2, velocity: 100 };
    // Snare pattern
    grid.cells[1][2] = { pitch: 1, octave: 3, velocity: 90 };
    grid.cells[1][6] = { pitch: 1, octave: 3, velocity: 90 };
    grid.cells[1][10] = { pitch: 1, octave: 3, velocity: 90 };
    grid.cells[1][14] = { pitch: 1, octave: 3, velocity: 90 };

    return {
      id: 'level2',
      name: 'Drum Duo',
      difficulty: 'easy',
      targetPattern: {
        id: 'level2-pattern',
        name: 'Drum Duo Pattern',
        bpm: 120,
        grid,
        createdAt: Date.now(),
        modifiedAt: Date.now(),
      },
      description: 'Layer kick and snare drums to create a rhythm',
      maxScore: 100,
    };
  }

  private createLevel3(): PuzzleLevel {
    const grid = this.createEmptyGrid();
    // Kick
    grid.cells[0][0] = { pitch: 0, octave: 2, velocity: 100 };
    grid.cells[0][4] = { pitch: 0, octave: 2, velocity: 100 };
    grid.cells[0][8] = { pitch: 0, octave: 2, velocity: 100 };
    grid.cells[0][12] = { pitch: 0, octave: 2, velocity: 100 };
    // Snare
    grid.cells[1][4] = { pitch: 1, octave: 3, velocity: 90 };
    grid.cells[1][12] = { pitch: 1, octave: 3, velocity: 90 };
    // Bass line
    grid.cells[4][0] = { pitch: 5, octave: 1, velocity: 80 };
    grid.cells[4][4] = { pitch: 7, octave: 1, velocity: 80 };
    grid.cells[4][8] = { pitch: 5, octave: 1, velocity: 80 };
    grid.cells[4][12] = { pitch: 9, octave: 1, velocity: 80 };

    return {
      id: 'level3',
      name: 'Groove Master',
      difficulty: 'medium',
      targetPattern: {
        id: 'level3-pattern',
        name: 'Groove Master Pattern',
        bpm: 120,
        grid,
        createdAt: Date.now(),
        modifiedAt: Date.now(),
      },
      description: 'Create a complete groove with drums and bass',
      maxScore: 150,
    };
  }

  private createLevel4(): PuzzleLevel {
    const grid = this.createEmptyGrid();
    // Complex drum pattern
    for (let i = 0; i < 16; i++) {
      if (i % 4 === 0) grid.cells[0][i] = { pitch: 0, octave: 2, velocity: 100 }; // Kick
      if (i % 8 === 2 || i % 8 === 6) grid.cells[1][i] = { pitch: 1, octave: 3, velocity: 90 }; // Snare
      if (i % 2 === 0) grid.cells[2][i] = { pitch: 2, octave: 3, velocity: 80 }; // HiHat
    }
    // Melody
    const melody = [0, 2, 4, 5, 7, 9, 11, 0];
    for (let i = 0; i < 8; i++) {
      grid.cells[5][i * 2] = { pitch: melody[i], octave: 4, velocity: 85 };
    }

    return {
      id: 'level4',
      name: 'Complex Rhythm',
      difficulty: 'hard',
      targetPattern: {
        id: 'level4-pattern',
        name: 'Complex Rhythm Pattern',
        bpm: 140,
        grid,
        createdAt: Date.now(),
        modifiedAt: Date.now(),
      },
      description: 'Create a complex pattern with drums and a melodic line',
      maxScore: 200,
    };
  }

  private createLevel5(): PuzzleLevel {
    const grid = this.createEmptyGrid();
    // Full arrangement
    for (let i = 0; i < 16; i++) {
      if (i % 4 === 0) grid.cells[0][i] = { pitch: 0, octave: 2, velocity: 100 };
      if ((i + 2) % 4 === 0) grid.cells[1][i] = { pitch: 1, octave: 3, velocity: 90 };
      if (i % 2 === 0) grid.cells[2][i] = { pitch: 2, octave: 3, velocity: 80 };
      grid.cells[3][i] = { pitch: (i % 4) + 3, octave: 3, velocity: 70 };
    }
    // Bass line variations
    const bassLine = [0, 0, 5, 5, 0, 0, 7, 7];
    for (let i = 0; i < 8; i++) {
      grid.cells[4][i * 2] = { pitch: bassLine[i], octave: 1, velocity: 85 };
    }

    return {
      id: 'level5',
      name: 'Full Production',
      difficulty: 'hard',
      targetPattern: {
        id: 'level5-pattern',
        name: 'Full Production Pattern',
        bpm: 128,
        grid,
        createdAt: Date.now(),
        modifiedAt: Date.now(),
      },
      description: 'Recreate a full multi-track production',
      maxScore: 250,
    };
  }

  private createEmptyGrid(): SequencerGrid {
    const cells: (Note | null)[][] = [];
    for (let i = 0; i < 8; i++) {
      cells[i] = new Array(16).fill(null);
    }
    return { tracks: 8, steps: 16, cells };
  }

  getLevels(): PuzzleLevel[] {
    return this.levels;
  }

  getLevel(index: number): PuzzleLevel | null {
    return index >= 0 && index < this.levels.length ? this.levels[index] : null;
  }

  getCurrentLevel(): PuzzleLevel | null {
    return this.levels[this.currentLevelIndex] || null;
  }

  nextLevel(): boolean {
    if (this.currentLevelIndex < this.levels.length - 1) {
      this.currentLevelIndex++;
      return true;
    }
    return false;
  }

  setCurrentLevel(index: number): boolean {
    if (index >= 0 && index < this.levels.length) {
      this.currentLevelIndex = index;
      return true;
    }
    return false;
  }

  calculateScore(playerGrid: SequencerGrid, targetGrid: SequencerGrid, maxScore: number): number {
    let correctNotes = 0;
    let totalTargetNotes = 0;

    for (let track = 0; track < targetGrid.tracks; track++) {
      for (let step = 0; step < targetGrid.steps; step++) {
        const targetNote = targetGrid.cells[track][step];
        const playerNote = playerGrid.cells[track][step];

        if (targetNote !== null) {
          totalTargetNotes++;
          if (
            playerNote !== null &&
            playerNote.pitch === targetNote.pitch &&
            playerNote.octave === targetNote.octave
          ) {
            correctNotes++;
          }
        }
      }
    }

    if (totalTargetNotes === 0) return 0;
    return Math.round((correctNotes / totalTargetNotes) * maxScore);
  }
}

// ============================================================================
// STORAGE AND PERSISTENCE
// ============================================================================

export class PatternStorage {
  private readonly STORAGE_KEY = 'beat_maker_patterns';

  savePattern(pattern: Pattern): void {
    const patterns = this.getAllPatterns();
    const existingIndex = patterns.findIndex((p) => p.id === pattern.id);

    if (existingIndex >= 0) {
      patterns[existingIndex] = pattern;
    } else {
      patterns.push(pattern);
    }

    localStorage.setItem(this.STORAGE_KEY, JSON.stringify(patterns));
  }

  getPattern(id: string): Pattern | null {
    const patterns = this.getAllPatterns();
    return patterns.find((p) => p.id === id) || null;
  }

  getAllPatterns(): Pattern[] {
    const stored = localStorage.getItem(this.STORAGE_KEY);
    return stored ? JSON.parse(stored) : [];
  }

  deletePattern(id: string): void {
    const patterns = this.getAllPatterns();
    const filtered = patterns.filter((p) => p.id !== id);
    localStorage.setItem(this.STORAGE_KEY, JSON.stringify(filtered));
  }

  clearAllPatterns(): void {
    localStorage.removeItem(this.STORAGE_KEY);
  }

  exportAsJSON(pattern: Pattern): string {
    return JSON.stringify(pattern, null, 2);
  }

  importFromJSON(json: string): Pattern {
    return JSON.parse(json);
  }
}
