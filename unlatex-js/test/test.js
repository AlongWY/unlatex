import assert from 'assert';
import { latexFormat } from '../dist/unlatex.esm.js';
assert.equal(latexFormat('Hello, \\textit{world}!'), 'Hello, \\textit{world}!');
