/**
 * Jest Setup File - Mock Web Audio API for testing
 */

// Mock AudioContext
class MockAudioContext {
  currentTime = 0;
  destination = {};
  sampleRate = 44100;

  createOscillator() {
    return new MockOscillator();
  }

  createGain() {
    return new MockGain();
  }

  createBiquadFilter() {
    return new MockBiquadFilter();
  }
}

class MockOscillator {
  type = 'sine';
  frequency = { setValueAtTime: jest.fn(), linearRampToValueAtTime: jest.fn(), exponentialRampToValueAtTime: jest.fn() };
  detune = { value: 0 };

  connect = jest.fn();
  start = jest.fn();
  stop = jest.fn();
}

class MockGain {
  gain = { setValueAtTime: jest.fn(), linearRampToValueAtTime: jest.fn(), exponentialRampToValueAtTime: jest.fn(), value: 1 };

  connect = jest.fn();
}

class MockBiquadFilter {
  frequency = { value: 350 };
  type = 'lowpass';
  Q = { value: 1 };

  connect = jest.fn();
}

// Assign mock to window
window.AudioContext = MockAudioContext;
window.webkitAudioContext = MockAudioContext;

// Mock setTimeout to be more predictable in tests
global.requestAnimationFrame = (callback) => {
  return setTimeout(callback, 16);
};

global.cancelAnimationFrame = (id) => {
  clearTimeout(id);
};
