pub fn is_whitespace(ch: char) -> bool {
    ch.is_whitespace()
}

pub fn is_ident_start(ch: char) -> bool {
    unicode_ident::is_xid_start(ch)
}

pub fn is_ident_continue(ch: char) -> bool {
    unicode_ident::is_xid_continue(ch)
}

pub fn is_dec_digit(ch: char) -> bool {
    ch.is_ascii_digit()
}
