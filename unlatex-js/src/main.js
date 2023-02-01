import Prettier from "prettier/esm/standalone.mjs";
import {parse} from "@unified-latex/unified-latex-util-parse";
import {prettierPluginLatex} from "@unified-latex/unified-latex-prettier";

/**
 * Format `source` LaTeX code using Prettier to format/render
 * the code.
 *
 * @export
 * @param [source] - code to be formatted
 * @param [printWidth=80] - Specify the line length that the printer will wrap on.
 * @param [useTabs=false] - Indent lines with tabs instead of spaces.
 * @param [tabWidth=2] - Specify the number of spaces per indentation-level.
 * @param [document_only=false] - Only format the document environment.
 * @returns formatted code
 */
export function latexFormat(source, printWidth = 80, useTabs = false, tabWidth = 2, document_only = false) {
    let start = source.search(/\\begin{document}/);
    if (!document_only || start < 0) {
        start = 0;
    }
    return Prettier.format(source, {
        printWidth: printWidth,
        useTabs: useTabs,
        tabWidth: tabWidth,
        rangeStart: start,
        parser: "latex-parser",
        plugins: [prettierPluginLatex],
    });
}

/**
 * Parse `source` LaTeX code using the unified-latex parser.
 *
 * @export
 * @param source - code to be parsed
 * @returns parsed ast
 */
export function latexParse(source) {
    return parse(source);
}

/**
 * Parse `source` LaTeX code using the unified-latex parser.
 *
 * @export
 * @param source - code to be parsed
 * @returns parsed ast dump to json
 */
export function latexJParse(source) {
    return JSON.stringify(parse(source));
}
