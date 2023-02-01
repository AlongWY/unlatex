# UnLaTex

## Description

UnLaTex is a library for parsing and formatting LaTeX.

## Examples

```rust
let formatted = unlatex::format("E = mc^2").unwrap();
let ast = unlatex::parse("E = mc^2").unwrap();
```
