use upon::Engine;

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
   |             ^^ unexpected end tag
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
   |             ^^ unexpected end tag
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
   |             ^^ unclosed begin tag
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
   |             ^^ unclosed begin tag
"
    );
}

#[test]
fn lex_unexpected_end_tag() {
    let err = Engine::new()
        .compile("lorem ipsum {{ %} dolor sit amet")
        .unwrap_err();
    assert_eq!(
        format!("{:#}", err),
        "
   |
 1 | lorem ipsum {{ %} dolor sit amet
   |                ^^ unexpected end tag
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