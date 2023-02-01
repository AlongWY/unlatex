var global;
try {
    global = Function('return this')();
} catch (e) {
    global = window;
}
global.latexFormat = unlatex.latexFormat;
global.latexJParse = unlatex.latexJParse;
global.latexParse = unlatex.latexParse;

