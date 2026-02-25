// Rhythm Runner Game
type TimingAccuracy = 'perfect' | 'good' | 'miss';
class AudioSystem {
  bpm: number;
  beatDuration: number;
  startTime: number = 0;
  isPlaying: boolean = false;
  constructor(bpm: number) {
    this.bpm = bpm;
    this.beatDuration = (60 / this.bpm) * 1000;
  }
  start() { if (!this.isPlaying) { this.startTime = Date.now(); this.isPlaying = true; } }
  stop() { this.isPlaying = false; }
  getElapsedTime() { return !this.isPlaying ? 0 : Date.now() - this.startTime; }
  getBeatPosition() { const e = this.getElapsedTime(); return (e % this.beatDuration) / this.beatDuration; }
  getCurrentBeat() { return Math.floor(this.getElapsedTime() / this.beatDuration); }
  checkTiming(): TimingAccuracy { const b = this.getBeatPosition(); const t = Math.abs((b - 0.5) * this.beatDuration); return t <= 50 ? 'perfect' : t <= 100 ? 'good' : 'miss'; }
  setBPM(n: number) { this.bpm = n; this.beatDuration = (60 / n) * 1000; }
  getBPM() { return this.bpm; }
  reset() { this.startTime = 0; this.isPlaying = false; }
  update() { return false; }
}
