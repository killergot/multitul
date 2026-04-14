use crate::macros::string_newtype_macro::string_newtype;

string_newtype!(Hash);

impl PartialEq<Option<Hash>> for Hash {
    fn eq(&self, other: &Option<Hash>) -> bool {
        if let Some(hash) = other {
            self.0 == hash.0
        } else {
            false
        }
    }
}
