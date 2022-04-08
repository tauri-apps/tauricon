export default {
  globals: {
    __DEV__: true
  },
  setupFilesAfterEnv: ['<rootDir>/test/jest/jest.setup.js'],
  // noStackTrace: true,
  // bail: true,
  // cache: false,
  // verbose: true,
  // watch: true,

  // TODO: coverage does not work with esm
  // collectCoverage: true,
  // coverageDirectory: '<rootDir>/test/jest/coverage',
  // collectCoverageFrom: [
  //   '<rootDir>/bin/**/*.js',
  //   '<rootDir>/helpers/**/*.js',
  //   '<rootDir>/api/**/*.js'
  // ],
  // coverageReporters: ['json-summary', 'text', 'lcov'],
  // coverageThreshold: {
  //   global: {
  //  branches: 50,
  //  functions: 50,
  //  lines: 50,
  //  statements: 50
  //   }
  // },
  testMatch: [
    '<rootDir>/test/jest/__tests__/**/*.spec.js',
    '<rootDir>/test/jest/__tests__/**/*.test.js'
  ],
  moduleFileExtensions: ['ts', 'js', 'json'],
  moduleNameMapper: {
    '^~/(.*)$': '<rootDir>/$1',
    '^dist/(.*)$': '<rootDir>/dist/$1',
    '^bin/(.*)$': '<rootDir>/bin/$1',
    '^helpers/(.*)$': '<rootDir>/src/helpers/$1',
    '^api/(.*)$': '<rootDir>/src/$1',
    '^templates/(.*)$': '<rootDir>/src/templates/$1',
    '^test/(.*)$': '<rootDir>/test/$1',
    '../../package.json': '<rootDir>/package.json',
    'node:(.*)$': '$1',
    '#ansi-styles': 'chalk/source/vendor/ansi-styles/index.js',
    '#supports-color': 'chalk/source/vendor/supports-color/index.js'
  },
  transform: {
    '\\.toml$': 'jest-transform-toml',
    '\\.(js|ts)$': 'babel-jest'
  },
  extensionsToTreatAsEsm: ['.ts']
}
