// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::vec::Vec;
use core::str::CharIndices;
use icu_provider::prelude::*;

use crate::complex::{Dictionary, LstmPayloads};
use crate::indices::{Latin1Indices, Utf16Indices};
use crate::provider::*;
use crate::rule_segmenter::*;

/// Sentence break iterator for an `str` (a UTF-8 string).
pub type SentenceBreakIteratorUtf8<'l, 's> = RuleBreakIterator<'l, 's, SentenceBreakTypeUtf8>;

/// Sentence break iterator for a Latin-1 (8-bit) string.
pub type SentenceBreakIteratorLatin1<'l, 's> = RuleBreakIterator<'l, 's, SentenceBreakTypeLatin1>;

/// Sentence break iterator for a UTF-16 string.
pub type SentenceBreakIteratorUtf16<'l, 's> = RuleBreakIterator<'l, 's, SentenceBreakTypeUtf16>;

/// Supports loading sentence break data, and creating sentence break iterators for different string
/// encodings. Please see the [module-level documentation](crate) for its usages.
pub struct SentenceBreakSegmenter {
    payload: DataPayload<SentenceBreakDataV1Marker>,
    dictionary: Dictionary,
    lstm: LstmPayloads,
}

impl SentenceBreakSegmenter {
    pub fn try_new<D>(provider: &D) -> Result<Self, DataError>
    where
        D: DataProvider<SentenceBreakDataV1Marker> + ?Sized,
    {
        let payload = provider.load(Default::default())?.take_payload()?;
        let dictionary = Dictionary::default();
        let lstm = LstmPayloads::default();
        Ok(Self {
            payload,
            dictionary,
            lstm,
        })
    }

    /// Create a sentence break iterator for an `str` (a UTF-8 string).
    pub fn segment_str<'l, 's>(&'l self, input: &'s str) -> SentenceBreakIteratorUtf8<'l, 's> {
        SentenceBreakIteratorUtf8 {
            iter: input.char_indices(),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            data: self.payload.get(),
            dictionary: &self.dictionary,
            lstm: &self.lstm,
        }
    }

    /// Create a sentence break iterator for a Latin-1 (8-bit) string.
    pub fn segment_latin1<'l, 's>(
        &'l self,
        input: &'s [u8],
    ) -> SentenceBreakIteratorLatin1<'l, 's> {
        SentenceBreakIteratorLatin1 {
            iter: Latin1Indices::new(input),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            data: self.payload.get(),
            dictionary: &self.dictionary,
            lstm: &self.lstm,
        }
    }

    /// Create a sentence break iterator for a UTF-16 string.
    pub fn segment_utf16<'l, 's>(&'l self, input: &'s [u16]) -> SentenceBreakIteratorUtf16<'l, 's> {
        SentenceBreakIteratorUtf16 {
            iter: Utf16Indices::new(input),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            data: self.payload.get(),
            dictionary: &self.dictionary,
            lstm: &self.lstm,
        }
    }
}

pub struct SentenceBreakTypeUtf8;

impl<'l, 's> RuleBreakType<'l, 's> for SentenceBreakTypeUtf8 {
    type IterAttr = CharIndices<'s>;
    type CharType = char;

    fn get_current_position_character_len(iter: &RuleBreakIterator<Self>) -> usize {
        iter.current_pos_data.unwrap().1.len_utf8()
    }

    fn handle_complex_language(
        _: &mut RuleBreakIterator<Self>,
        _: Self::CharType,
    ) -> Option<usize> {
        panic!("not reachable")
    }
}

pub struct SentenceBreakTypeLatin1;

impl<'l, 's> RuleBreakType<'l, 's> for SentenceBreakTypeLatin1 {
    type IterAttr = Latin1Indices<'s>;
    type CharType = u8;

    fn get_current_position_character_len(_: &RuleBreakIterator<Self>) -> usize {
        panic!("not reachable")
    }

    fn handle_complex_language(
        _: &mut RuleBreakIterator<Self>,
        _: Self::CharType,
    ) -> Option<usize> {
        panic!("not reachable")
    }
}

pub struct SentenceBreakTypeUtf16;

impl<'l, 's> RuleBreakType<'l, 's> for SentenceBreakTypeUtf16 {
    type IterAttr = Utf16Indices<'s>;
    type CharType = u32;

    fn get_current_position_character_len(iter: &RuleBreakIterator<Self>) -> usize {
        let ch = iter.current_pos_data.unwrap().1;
        if ch >= 0x10000 {
            2
        } else {
            1
        }
    }

    fn handle_complex_language(
        _: &mut RuleBreakIterator<Self>,
        _: Self::CharType,
    ) -> Option<usize> {
        panic!("not reachable")
    }
}
