use crate::language_tag::LanguageTag;

#[derive(Debug)]
pub enum LanguageID {
    /// Unstable, do not persist
    UserDefault,
    /// Unstable, do not persist
    Transient,
    /// Discard immediately
    Unspecified,
    Tagged(LanguageTag),
}
