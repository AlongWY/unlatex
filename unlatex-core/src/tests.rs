use super::*;

#[test]
fn test_format() {
    let formatted = format(r#"\section*{Really Cool Math}Below you'll find some really cool math.

Check it out!\begin{enumerate}
    \item[(a)] Hi there
\item$e^2$ is math mode! \[\begin{bmatrix}12&3^e\\\pi&0\end{bmatrix}\]
\end{enumerate}"#).unwrap();

    assert_eq!(formatted,
               r#"\section*{Really Cool Math}
Below you'll find some really cool math.

Check it out!
\begin{enumerate}
  \item[(a)] Hi there

  \item $e^{2}$ is math mode!
    \[
      \begin{bmatrix}
        12  & 3^{e} \\
        \pi & 0
      \end{bmatrix}
    \]
\end{enumerate}"#);
}


#[test]
fn test_parse() {
    let ast = parse(r#"\section*{Really Cool Math}
Below you'll find some really cool math.

Check it out!
\begin{enumerate}
  \item[(a)] Hi there

  \item $e^{2}$ is math mode!
    \[
      \begin{bmatrix}
        12  & 3^{e} \\
        \pi & 0
      \end{bmatrix}
    \]
\end{enumerate}"#).unwrap();

    println!("{:?}", ast);
}

#[test]
fn test_jparse() {
    let ast = jparse(r#"\section*{Really Cool Math}
Below you'll find some really cool math.

Check it out!
\begin{enumerate}
  \item[(a)] Hi there

  \item $e^{2}$ is math mode!
    \[
      \begin{bmatrix}
        12  & 3^{e} \\
        \pi & 0
      \end{bmatrix}
    \]
\end{enumerate}"#).unwrap();

    println!("{}", ast);
}
