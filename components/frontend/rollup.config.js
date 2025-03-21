import resolve from '@rollup/plugin-node-resolve';
import commonjs from '@rollup/plugin-commonjs';
import typescript from 'rollup-plugin-typescript2';
import postcss from 'rollup-plugin-postcss';

export default {
  input: 'typescript/mobility-map.ts',
  output: {
    dir: 'assets',
    format: 'iife',
    sourcemap: true
  },
  plugins: [
    resolve(),
    commonjs(),
    typescript(),
    postcss({
      extensions: ['.css', '.scss'],
      extract: true
    })
  ]
};
