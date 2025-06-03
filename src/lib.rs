//!
// #![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

#[derive(Debug)]
pub struct StrSplit<'haystack, 'delimiter> {
    remainder: Option<&'haystack str>,
    delimiter: &'delimiter str,
}

impl<'haystack, 'delimiter> StrSplit<'haystack, 'delimiter> {
    pub fn new(haystack: &'haystack str, delimiter: &'delimiter str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'haystack, 'delimiter> Iterator for StrSplit<'haystack, 'delimiter> {
    type Item = &'haystack str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut remainder) = self.remainder {
            if let Some(next_delim) = remainder.find(self.delimiter) {
                let until_delim = &remainder[..next_delim];
                *remainder = &remainder[(next_delim + self.delimiter.len())..];
                Some(until_delim)
            } else {
                self.remainder.take()
            }
        } else {
            None
        }
    }
}

// gives you the string until; the first occourance of that character;
fn until_char<'s>(s: &'s str, c: char) -> &'s str {
    StrSplit::new(s, &format!("{}", c))
        .next()
        .expect("StrSplit always gives at least one result")
}

#[test]
fn until_char_test() {
    assert_eq!(until_char("Hello World", 'o'), "Hell")
}

#[test]
fn it_works() {
    let heystack = "a b c d e";
    let letters = StrSplit::new(heystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d", "e"].into_iter()));
}

#[test]
fn it_works_again() {
    let heystack = "a b c d ";
    let letters = StrSplit::new(heystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d", ""].into_iter()));
}
