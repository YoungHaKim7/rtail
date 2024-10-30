use super::tables;
// pub use tables::UNICODE_VERSION;

mod private {
    pub trait Sealed {}
    // #[cfg(not(feature = "cjk"))]
    impl Sealed for char {}
    // #[cfg(not(feature = "cjk"))]
    impl Sealed for str {}
    // #[cfg(feature = "cjk")]
    // impl<T: ?Sized> Sealed for T {}
}

pub trait UnicodeWidthChar: private::Sealed {
    /// Returns the character's displayed width in columns, or `None` if the
    /// character is a control character.
    ///
    /// This function treats characters in the Ambiguous category according
    /// to [Unicode Standard Annex #11](http://www.unicode.org/reports/tr11/)
    /// as 1 column wide. This is consistent with the recommendations for non-CJK
    /// contexts, or when the context cannot be reliably determined.
    fn width(self) -> Option<usize>;
}

impl UnicodeWidthChar for char {
    #[inline]
    fn width(self) -> Option<usize> {
        tables::single_char_width(self)
    }

    // #[cfg(feature = "cjk")]
    // #[inline]
    // fn width_cjk(self) -> Option<usize> {
    //     tables::single_char_width_cjk(self)
    // }
}

pub trait UnicodeWidthStr: private::Sealed {
    /// Returns the string's displayed width in columns.
    ///
    /// This function treats characters in the Ambiguous category according
    /// to [Unicode Standard Annex #11](http://www.unicode.org/reports/tr11/)
    /// as 1 column wide. This is consistent with the recommendations for
    /// non-CJK contexts, or when the context cannot be reliably determined.
    fn width(&self) -> usize;

    /// Returns the string's displayed width in columns.
    ///
    /// This function treats characters in the Ambiguous category according
    /// to [Unicode Standard Annex #11](http://www.unicode.org/reports/tr11/)
    /// as 2 column wide. This is consistent with the recommendations for
    /// CJK contexts.
    // #[cfg(feature = "cjk")]
    // fn width_cjk(&self) -> usize;
}

impl UnicodeWidthStr for str {
    #[inline]
    fn width(&self) -> usize {
        tables::str_width(self)
    }

    // #[cfg(feature = "cjk")]
    // #[inline]
    // fn width_cjk(&self) -> usize {
    //     tables::str_width_cjk(self)
    // }
}
