#[derive(Clone, PartialEq, Debug)]
pub enum Token {
    BaseAndExt,
    Basename,
    Character(char),
    Dirname,
    Job,
    JobTotal,
    Placeholder,
    RemoveExtension,
    Slot,
}

/// Takes the command arguments as the input and reduces it into tokens,
/// which allows for easier management of string manipulation later on.
pub fn tokenize(template: &str) -> Vec<Token> {
    // When set to true, the characters following will be collected into `pattern`.
    let mut matching = false;
    // This vector will contain the complete list of parsed tokens.
    let mut tokens = Vec::new();
    // `pattern` is a buffer for the currently-matched pattern. IE: {./}
    let mut pattern = String::with_capacity(4);

    for character in template.chars() {
        match (character, matching) {
            // This condition initiates the pattern matching
            ('{', false) => matching = true,
            // This condition ends the pattern matching process
            ('}', true)  => {
                matching = false;
                if pattern.is_empty() {
                    tokens.push(Token::Placeholder);
                } else {
                    match match_token(&pattern) {
                        // If the token is a match, add the token.
                        Some(token) => tokens.push(token),
                        // If the token is not a match, write each character out.
                        None => {
                            tokens.push(Token::Character('{'));
                            for character in pattern.chars() {
                                tokens.push(Token::Character(character));
                            }
                            tokens.push(Token::Character('}'));
                        }
                    }
                    // Clear the pattern buffer as we have already tokenized it.
                    pattern.clear();
                }
            },
            (_, false)  => tokens.push(Token::Character(character)),
            (_, true) => pattern.push(character)
        }
    }

    // If matching is still enabled, add the contents of `pattern` as `Token::Character`s.
    if matching {
        tokens.push(Token::Character('{'));
        for character in pattern.chars() {
            tokens.push(Token::Character(character));
        }
    }

    tokens
}

/// Matches a pattern to it's associated token.
fn match_token(pattern: &str) -> Option<Token> {
    match pattern {
        "."  => Some(Token::RemoveExtension),
        "#"  => Some(Token::Job),
        "%"  => Some(Token::Slot),
        "/"  => Some(Token::Basename),
        "//" => Some(Token::Dirname),
        "/." => Some(Token::BaseAndExt),
        "#^" => Some(Token::JobTotal),
        _    => None
    }
}

#[test]
fn tokenizer_character() {
    assert_eq!(tokenize("foo"), vec![Token::Character('f'), Token::Character('o'),
        Token::Character('o')]);
}

#[test]
fn tokenizer_placeholder() {
    assert_eq!(tokenize("{}"), vec![Token::Placeholder]);
}

#[test]
fn tokenizer_remove_extension() {
    assert_eq!(tokenize("{.}"), vec![Token::RemoveExtension]);
}

#[test]
fn tokenizer_basename() {
    assert_eq!(tokenize("{/}"), vec![Token::Basename]);
}

#[test]
fn tokenizer_dirname() {
    assert_eq!(tokenize("{//}"), vec![Token::Dirname]);
}

#[test]
fn tokenizer_base_and_ext() {
    assert_eq!(tokenize("{/.}"), vec![Token::BaseAndExt]);
}

#[test]
fn tokenizer_slot() {
    assert_eq!(tokenize("{%}"), vec![Token::Slot]);
}

#[test]
fn tokenizer_job() {
    assert_eq!(tokenize("{#}"), vec![Token::Job]);
}

#[test]
fn tokenizer_jobtotal() {
    assert_eq!(tokenize("{#^}"), vec![Token::JobTotal]);
}

#[test]
fn tokenizer_multiple() {
    assert_eq!(tokenize("foo {} bar"), vec![Token::Character('f'), Token::Character('o'),
        Token::Character('o'), Token::Character(' '), Token::Placeholder, Token::Character(' '),
        Token::Character('b'), Token::Character('a'), Token::Character('r')]);
}

#[test]
fn tokenizer_no_space() {
    assert_eq!(tokenize("foo{}bar"), vec![Token::Character('f'), Token::Character('o'),
        Token::Character('o'), Token::Placeholder, Token::Character('b'), Token::Character('a'),
        Token::Character('r')]);
}
