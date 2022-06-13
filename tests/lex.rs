use upon::{Engine, Syntax};

#[test]
fn lex_while_eof() {
    let err = Engine::new().compile("lorem {{ ipsum").unwrap_err();
    assert_eq!(
        format!("{:#}", err),
        "
   |
 1 | lorem {{ ipsum
   |               ^ expected end expression, found EOF
"
    );
}

#[test]
fn lex_overlapping_delimiters() {
    let syntax = Syntax::builder().expr("{", "}").block("{{", "}}").build();
    Engine::with_syntax(syntax)
        .compile("lorem { ipsum } {{ if dolor }} {{ endif }} sit amet")
        .unwrap();
}

#[test]
fn lex_overlapping_delimiters_flipped() {
    let syntax = Syntax::builder().expr("{{", "}}").block("{", "}").build();
    Engine::with_syntax(syntax)
        .compile("lorem {{ ipsum }} { if dolor } { endif } sit amet")
        .unwrap();
}

#[test]
fn lex_unexpected_end_expr() {
    let err = Engine::new()
        .compile("lorem ipsum }} dolor sit amet")
        .unwrap_err();
    assert_eq!(
        format!("{:#}", err),
        "
   |
 1 | lorem ipsum }} dolor sit amet
   |             ^^ unexpected end expression
"
    );
}

#[test]
fn lex_unexpected_end_block() {
    let err = Engine::new()
        .compile("lorem ipsum %} dolor sit amet")
        .unwrap_err();
    assert_eq!(
        format!("{:#}", err),
        "
   |
 1 | lorem ipsum %} dolor sit amet
   |             ^^ unexpected end block
"
    );
}

#[test]
fn lex_unclosed_begin_expr() {
    let err = Engine::new()
        .compile("lorem ipsum {{ {{ dolor sit amet")
        .unwrap_err();
    assert_eq!(
        format!("{:#}", err),
        "
   |
 1 | lorem ipsum {{ {{ dolor sit amet
   |             ^^ unclosed begin expression
"
    );
}

#[test]
fn lex_unclosed_begin_block() {
    let err = Engine::new()
        .compile("lorem ipsum {% {{ dolor sit amet")
        .unwrap_err();
    assert_eq!(
        format!("{:#}", err),
        "
   |
 1 | lorem ipsum {% {{ dolor sit amet
   |             ^^ unclosed begin block
"
    );
}

#[test]
fn lex_unexpected_end_block_after_start() {
    let err = Engine::new()
        .compile("lorem ipsum {{ %} dolor sit amet")
        .unwrap_err();
    assert_eq!(
        format!("{:#}", err),
        "
   |
 1 | lorem ipsum {{ %} dolor sit amet
   |                ^^ unexpected end block
"
    );
}

#[test]
fn lex_unexpected_character() {
    let err = Engine::new()
        .compile("lorem ipsum {{ ✨ }} dolor sit amet")
        .unwrap_err();
    assert_eq!(
        format!("{:#}", err),
        "
   |
 1 | lorem ipsum {{ ✨ }} dolor sit amet
   |                ^^ unexpected character
"
    );
}
