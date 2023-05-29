use crate::{language_id::LanguageID, language_tag::LanguageTag};

use num_traits::FromPrimitive;

pub struct KeyboardIdentifier(pub usize);

impl KeyboardIdentifier {
    // this might be parsed incorrectly, but it certainly isn't 0 as is
    /// This field is reserved for future use. It MUST be 0.
    #[must_use] pub const fn get_raw_reserved(&self) -> u32 {
        self.with_mask(0b1111_1111_1111_0000_0000_0000_0000_0000)
    }

    #[must_use] pub const fn get_raw_sort_id(&self) -> u32 {
        self.with_mask(0b1111_0000_0000_0000_0000)
    }

    #[must_use] pub fn get_language_id(&self) -> Option<LanguageID> {
        match self.get_raw_language_id() {
            0x0C00 => Some(LanguageID::UserDefault),
            0x3000 | 0x3400 | 0x3800 | 0x3C00 | 0x4000 | 0x4400 | 0x4800 | 0x4C00 => {
                Some(LanguageID::Transient)
            }
            0x1000 => Some(LanguageID::Unspecified),
            x => LanguageTag::from_u16(x).map(LanguageID::Tagged),
        }
    }

    #[must_use] pub const fn get_raw_language_id(&self) -> u16 {
        self.with_mask(0b1111_1111_1111_1111) as u16
    }

    const fn with_mask(&self, mask: u32) -> u32 {
        self.0 as u32 & mask
    }
}
