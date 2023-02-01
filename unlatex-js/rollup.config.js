import terser from '@rollup/plugin-terser';
import resolve from '@rollup/plugin-node-resolve';
import commonjs from '@rollup/plugin-commonjs';
import typescript from '@rollup/plugin-typescript';
import nodePolyfills from 'rollup-plugin-polyfill-node';

export default [
    // CommonJS (for Node) and ES module (for bundlers) build.
    // (We could have three entries in the configuration array
    // instead of two, but it's quicker to generate multiple
    // builds from a single configuration where possible, using
    // an array for the `output` option, where we can specify
    // `file` and `format` for each target)
    {
        input: 'src/main.js',
        plugins: [
            nodePolyfills(),
            resolve(), // so Rollup can find packages
            commonjs(), // so Rollup can convert packages to an ES module
            typescript(), // so Rollup can convert TypeScript to JavaScript
            terser() // so Rollup can minify JavaScript
        ],
        output: [
            {
                name: 'unlatex',
                format: 'umd',
                file: "dist/unlatex.umd.js",
            },
            {
                name: 'unlatex',
                format: 'cjs',
                file: "dist/unlatex.cjs.js",
            },
            {
                name: 'unlatex',
                format: 'es',
                file: "dist/unlatex.esm.js",
            }
        ]
    }
];
