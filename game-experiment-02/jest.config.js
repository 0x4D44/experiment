module.exports = {
  preset: 'ts-jest',
  testEnvironment: 'jsdom',
  testMatch: ['**/*.test.ts'],
  moduleFileExtensions: ['ts', 'js', 'json'],
  collectCoverageFrom: [
    '*.ts',
    '!**/*.d.ts',
    '!**/*.test.ts'
  ],
  setupFilesAfterEnv: ['<rootDir>/jest.setup.js']
};
