// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

// rollup.config.js
import resolve from '@rollup/plugin-node-resolve'
import commonjs from '@rollup/plugin-commonjs'
import typescript from '@rollup/plugin-typescript'
import swc from 'rollup-plugin-swc'
import pkg from './package.json'

export default {
  input: {
    tauricon: './src/tauricon.ts'
  },
  treeshake: true,
  perf: true,
  output: [
    {
      dir: 'dist/',
      entryFileNames: '[name].js',
      format: 'esm',
      exports: 'named',
      globals: {}
    },
    {
      dir: 'dist/',
      entryFileNames: '[name].cjs',
      format: 'cjs',
      chunkFileNames: '[name]-[hash].cjs',
      exports: 'named',
      globals: {}
    }
  ],
  plugins: [
    commonjs({}),
    resolve({
      // pass custom options to the resolve plugin
      customResolveOptions: {
        moduleDirectories: ['node_modules']
      }
    }),
    typescript({
      tsconfig: './tsconfig.json'
    }),
    swc({
      minify: true,
      jsc: {
        minify: {},
        parser: {
          syntax: 'typescript'
        },
        target: 'es5'
      }
    })
    // babel({
    //   configFile: false,
    //   presets: [['@babel/preset-env'], ['@babel/preset-typescript']]
    // }),
    // terser()
  ],
  external: [
    ...Object.keys(pkg.dependencies || {}),
    ...Object.keys(pkg.peerDependencies || {})
  ]
}
