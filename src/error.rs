use alloc::{format, string::String};
use core::fmt;

#[derive(Clone, Debug)]
pub enum Error {
    Typed {
        error_type: ErrorType,
        file_index: usize,
    },
    Custom(String),
}

#[derive(Debug, Clone, Copy)]
pub enum ErrorType {
    InvalidMagic {
        wrong_magic: u32,
    },
    IncompatibleVersion {
        last_comp_version: u32,
        library_supported_version: u32,
    },
    HeaderTooShort {
        header_length: u32,
        at_least_length: u32,
    },
    StructureIndex {
        current_index: u32,
        bound_index: u32,
        structure_or_string: bool,
        overflow_or_underflow: bool,
    },
    StringEofUnexpected,
    SliceEofUnexpected {
        expected_length: u32,
        remaining_length: u32,
    },
    TableStringOffset {
        given_offset: u32,
        bound_offset: u32,
    },
    TagEofUnexpected {
        current_index: u32,
        bound_index: u32,
    },
    InvalidTagId {
        wrong_id: u32,
    },
    ExpectStructBegin,
    ExpectStructEnd,
    NoRemainingTags,
    InvalidSerdeTypeLength {
        expected_length: u8,
    },
    Utf8(core::str::Utf8Error),
}

impl Error {
    #[inline]
    pub fn invalid_magic(wrong_magic: u32, file_index: usize) -> Error {
        Error::Typed {
            error_type: ErrorType::InvalidMagic { wrong_magic },
            file_index,
        }
    }
    #[inline]
    pub fn incompatible_version(
        last_comp_version: u32,
        library_supported_version: u32,
        file_index: usize,
    ) -> Error {
        Error::Typed {
            error_type: ErrorType::IncompatibleVersion {
                last_comp_version,
                library_supported_version,
            },
            file_index,
        }
    }
    #[inline]
    pub fn header_too_short(header_length: u32, at_least_length: u32, file_index: usize) -> Error {
        Error::Typed {
            error_type: ErrorType::HeaderTooShort {
                header_length,
                at_least_length,
            },
            file_index,
        }
    }
    #[inline]
    pub fn structure_index_underflow(
        begin_index: u32,
        at_least_index: u32,
        file_index: usize,
    ) -> Error {
        Error::Typed {
            error_type: ErrorType::StructureIndex {
                current_index: begin_index,
                bound_index: at_least_index,
                structure_or_string: true,
                overflow_or_underflow: false,
            },
            file_index,
        }
    }
    #[inline]
    pub fn structure_index_overflow(
        end_index: u32,
        at_most_index: u32,
        file_index: usize,
    ) -> Error {
        Error::Typed {
            error_type: ErrorType::StructureIndex {
                current_index: end_index,
                bound_index: at_most_index,
                structure_or_string: true,
                overflow_or_underflow: true,
            },
            file_index,
        }
    }
    #[inline]
    pub fn string_index_underflow(
        begin_index: u32,
        at_least_index: u32,
        file_index: usize,
    ) -> Error {
        Error::Typed {
            error_type: ErrorType::StructureIndex {
                current_index: begin_index,
                bound_index: at_least_index,
                structure_or_string: false,
                overflow_or_underflow: false,
            },
            file_index,
        }
    }
    #[inline]
    pub fn string_index_overflow(end_index: u32, at_most_index: u32, file_index: usize) -> Error {
        Error::Typed {
            error_type: ErrorType::StructureIndex {
                current_index: end_index,
                bound_index: at_most_index,
                structure_or_string: false,
                overflow_or_underflow: true,
            },
            file_index,
        }
    }
    #[inline]
    pub fn string_eof_unpexpected(file_index: usize) -> Error {
        Error::Typed {
            error_type: ErrorType::StringEofUnexpected,
            file_index,
        }
    }
    #[inline]
    pub fn slice_eof_unpexpected(
        expected_length: u32,
        remaining_length: u32,
        file_index: usize,
    ) -> Error {
        Error::Typed {
            error_type: ErrorType::SliceEofUnexpected {
                expected_length,
                remaining_length,
            },
            file_index,
        }
    }
    #[inline]
    pub fn table_string_offset(given_offset: u32, bound_offset: u32, file_index: usize) -> Error {
        Error::Typed {
            error_type: ErrorType::TableStringOffset {
                given_offset,
                bound_offset,
            },
            file_index,
        }
    }
    #[inline]
    pub fn tag_eof_unexpected(current_index: u32, bound_index: u32, file_index: usize) -> Error {
        Error::Typed {
            error_type: ErrorType::TagEofUnexpected {
                current_index,
                bound_index,
            },
            file_index,
        }
    }
    #[inline]
    pub fn invalid_tag_id(wrong_id: u32, file_index: usize) -> Error {
        Error::Typed {
            error_type: ErrorType::InvalidTagId { wrong_id },
            file_index,
        }
    }
    #[inline]
    pub fn invalid_serde_type_length(expected_length: u8, file_index: usize) -> Error {
        Error::Typed {
            error_type: ErrorType::InvalidSerdeTypeLength { expected_length },
            file_index,
        }
    }
    #[inline]
    pub fn utf8(error: core::str::Utf8Error, file_index: usize) -> Error {
        Error::Typed {
            error_type: ErrorType::Utf8(error),
            file_index,
        }
    }
    #[inline]
    pub fn expected_struct_begin() -> Error {
        Error::Typed {
            error_type: ErrorType::ExpectStructBegin,
            file_index: 0,
        }
    }
    #[inline]
    pub fn expected_struct_end() -> Error {
        Error::Typed {
            error_type: ErrorType::ExpectStructEnd,
            file_index: 0,
        }
    }
    #[inline]
    pub fn no_remaining_tags() -> Error {
        Error::Typed {
            error_type: ErrorType::NoRemainingTags,
            file_index: 0,
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;

impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        Self::Custom(format!("{}", msg))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}