use ress::prelude::*;

#[test]
fn role() {
    compare(
        "role myRole { foo() { return 1 } }",
        // "class myRole { foo() { return 1 } }",
        &[
            // Token::Keyword(Keyword::Class("class")),
            Token::Keyword(Keyword::Role("role")),
            Token::Ident("myRole".into()),
            Token::Punct(Punct::OpenBrace),
            Token::Ident("foo".into()),
            Token::Punct(Punct::OpenParen),
            Token::Punct(Punct::CloseParen),
            Token::Punct(Punct::OpenBrace),
            Token::Keyword(Keyword::Return("return")),
            Token::Number("1".into()),
            Token::Punct(Punct::CloseBrace),
            Token::Punct(Punct::CloseBrace),
        ],
    );
}

fn compare(js: &str, expectation: &[Token<&str>]) {
    for (i, (par, ex)) in panicking_scanner(js).zip(expectation.iter()).enumerate() {
        assert_eq!((i, &par), (i, ex));
    }
}

fn panicking_scanner(js: &str) -> impl Iterator<Item = Token<&str>> {
    Scanner::new(js).map(|r| r.unwrap().token)
}
