declare module 'web-audio-api' {
  export class AudioContext {
    constructor();
    createOscillator(): unknown;
    createGain(): unknown;
    createStereoPanner?(): unknown;
    createPanner(): unknown;
    readonly currentTime: number;
    readonly destination: unknown;
    close(): Promise<void>;
  }
}
