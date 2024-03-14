//! List of page codes

use crate::domain::PageCode;
use crate::errors::PrinterError;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::iter::{IntoIterator, Iterator};

/// Page codes table list
#[derive(Debug, Clone, Copy)]
pub(crate) enum PageCodeTable {
    PC437,
    PC852,
    PC858,
    PC860,
    PC865,
    ISO8859_2,
    ISO8859_7,
    ISO8859_15,
    WPC1252,
}

impl PageCodeTable {
    /// Get the table for the page code
    pub(crate) fn get_table(&self) -> &HashMap<char, u8> {
        match self {
            Self::PC437 => &PC437_TABLE,
            Self::PC852 => &PC852_TABLE,
            Self::PC858 => &PC858_TABLE,
            Self::PC860 => &PC860_TABLE,
            Self::PC865 => &PC865_TABLE,
            Self::ISO8859_2 => &ISO8859_2_TABLE,
            Self::ISO8859_7 => &ISO8859_7_TABLE,
            Self::ISO8859_15 => &ISO8859_15_TABLE,
            Self::WPC1252 => &WPC1252_TABLE,
        }
    }
}

impl TryFrom<PageCode> for PageCodeTable {
    type Error = PrinterError;

    fn try_from(value: PageCode) -> Result<Self, Self::Error> {
        match value {
            PageCode::PC437 => Ok(Self::PC437),
            PageCode::PC852 => Ok(Self::PC852),
            PageCode::PC858 => Ok(Self::PC858),
            PageCode::PC860 => Ok(Self::PC860),
            PageCode::PC865 => Ok(Self::PC865),
            PageCode::ISO8859_2 => Ok(Self::ISO8859_2),
            PageCode::ISO8859_7 => Ok(Self::ISO8859_7),
            PageCode::ISO8859_15 => Ok(Self::ISO8859_15),
            PageCode::WPC1252 => Ok(Self::WPC1252),
            _ => Err(PrinterError::Input(format!("no table for this page code: {value}"))),
        }
    }
}

lazy_static! {
    /// PC437 Page code table
    static ref PC437_TABLE: HashMap<char, u8> = [
        'Ç', 'ü', 'é', 'â', 'ä', 'à', 'å', 'ç', 'ê', 'ë', 'è', 'ï', 'î', 'ì', 'Ä', 'Å',
        'É', 'æ', 'Æ', 'ô', 'ö', 'ò', 'û', 'ù', 'ÿ', 'Ö', 'Ü', '¢', '£', '¥', '₧', 'ƒ',
        'á', 'í', 'ó', 'ú', 'ñ', 'Ñ', 'ª', 'º', '¿', '⌐', '¬', '½', '¼', '¡', '«', '»',
        '░', '▒', '▓', '│', '┤', '╡', '╢', '╖', '╕', '╣', '║', '╗', '╝', '╜', '╛', '┐',
        '└', '┴', '┬', '├', '─', '┼', '╞', '╟', '╚', '╔', '╩', '╦', '╠', '═', '╬', '╧',
        '╨', '╤', '╥', '╙', '╘', '╒', '╓', '╫', '╪', '┘', '┌', '█', '▄', '▌', '▐', '▀',
        'α', 'ß', 'Γ', 'π', 'Σ', 'σ', 'µ', 'τ', 'Φ', 'Θ', 'Ω', 'δ', '∞', 'φ', 'ε', '∩',
        '≡', '±', '≥', '≤', '⌠', '⌡', '÷', '≈', '°', '∙', '·', '√', 'ⁿ', '²', '■', '\u{00A0}',
    ]
    .into_iter().enumerate()
    .map(|(i, c)| (c, (i + 128) as u8))
    .collect();

    /// PC852 Page code table
    static ref PC852_TABLE: HashMap<char, u8> = [
        'Ç', 'ü', 'é', 'â', 'ä', 'ů', 'ć', 'ç', 'ł', 'ë', 'Ő', 'ő', 'î', 'Ź', 'Ä', 'Ć',
        'É', 'Ĺ', 'ĺ', 'ô', 'ö', 'Ľ', 'ľ', 'Ś', 'ś', 'Ö', 'Ü', 'Ť', 'ť', 'Ł', '×', 'č',
        'á', 'í', 'ó', 'ú', 'Ą', 'ą', 'Ž', 'ž', 'Ę', 'ę', '¬', 'ź', 'Č', 'ş', '«', '»',
        '░', '▒', '▓', '│', '┤', 'Á', 'Â', 'Ě', 'Ş', '╣', '║', '╗', '╝', 'Ż', 'ż', '┐',
        '└', '┴', '┬', '├', '─', '┼', 'Ă', 'ă', '╚', '╔', '╩', '╦', '╠', '═', '╬', '¤',
        'đ', 'Đ', 'Ď', 'Ë', 'ď', 'Ň', 'Í', 'Î', 'ě', '┘', '┌', '█', '▄', 'Ţ', 'Ů', '▀',
        'Ó', 'ß', 'Ô', 'Ń', 'ń', 'ň', 'Š', 'š', 'Ŕ', 'Ú', 'ŕ', 'Ű', 'ý', 'Ý', 'ţ', '´',
        '\u{AD}', '˝', '˛', 'ˇ', '˘', '§', '÷', '¸', '°', '¨', '˙', 'ű', 'Ř', 'ř', '■', '\u{00A0}',
    ]
    .into_iter().enumerate()
    .map(|(i, c)| (c, (i + 128) as u8))
    .collect();

    /// PC858 Page code table
    static ref PC858_TABLE: HashMap<char, u8> = [
        'Ç', 'ü', 'é', 'â', 'ä', 'à', 'å', 'ç', 'ê', 'ë', 'è', 'ï', 'î', 'ì', 'Ä', 'Å',
        'É', 'æ', 'Æ', 'ô', 'ö', 'ò', 'û', 'ù', 'ÿ', 'Ö', 'Ü', 'ø', '£', 'Ø', '×', 'ƒ',
        'á', 'í', 'ó', 'ú', 'ñ', 'Ñ', 'ª', 'º', '®', '⌐', '¬', '½', '¼', '¡', '«', '»',
        '░', '▒', '▓', '│', '┤', 'Á', 'Â', 'À', '©', '╣', '║', '╗', '╝', '¢', '¥', '┐',
        '└', '┴', '┬', '├', '─', '┼', 'ã', 'Ã', '╚', '╔', '╩', '╦', '╠', '═', '╬', '¤',
        'ð', 'Ð', 'Ê', 'Ë', 'È', '€', 'Í', 'Î', 'Ï', '┘', '┌', '█', '▄', '¦', 'Ì', '▀',
        'Ó', 'ß', 'Ô', 'Ô', 'õ', 'Õ', 'µ', 'þ', 'Þ', 'Ú', 'Û', 'Ù', 'ý', 'Ý', '¯', '´',
        '-', '±', '‗', '¾', '¶', '§', '÷', '¸', '°', '¨', '·', '¹', '³', '²', '■', '\u{00A0}']
    .into_iter().enumerate()
    .map(|(i, c)| (c, (i + 128) as u8))
    .collect();

    /// PC860 Page code table
    static ref PC860_TABLE: HashMap<char, u8> = [
        'Ç', 'ü', 'é', 'â', 'ã', 'à', 'Á', 'ç', 'ê', 'Ê', 'è', 'Í', 'Ô', 'ì', 'Ã', 'Â',
        'É', 'À', 'È', 'ô', 'õ', 'ò', 'Ú', 'ù', 'Ì', 'Õ', 'Ü', '¢', '£', 'Ù', '₧', 'Ó',
        'á', 'í', 'ó', 'ú', 'ñ', 'Ñ', 'ª', 'º', '¿', 'Ò', '¬', '½', '¼', '¡', '«', '»',
        '░', '▒', '▓', '│', '┤', '╡', '╢', '╖', '╕', '╣', '║', '╗', '╝', '╜', '╛', '┐',
        '└', '┴', '┬', '├', '─', '┼', '╞', '╟', '╚', '╔', '╩', '╦', '╠', '═', '╬', '╧',
        '╨', '╤', '╥', '╙', '╘', '╒', '╓', '╫', '╪', '┘', '┌', '█', '▄', '▌', '▐', '▀',
        'α', 'ß', 'Γ', 'π', 'Σ', 'σ', 'µ', 'τ', 'Φ', 'Θ', 'Ω', 'δ', '∞', 'φ', 'ε', '∩',
        '≡', '±', '≥', '≤', '⌠', '⌡', '÷', '≈', '°', '∙', '·', '√', 'ⁿ', '²', '■', '\u{00A0}',
    ]
    .into_iter().enumerate()
    .map(|(i, c)| (c, (i + 128) as u8))
    .collect();

    /// PC865 Page code table
    static ref PC865_TABLE: HashMap<char, u8> = [
        'Ç', 'ü', 'é', 'â', 'ä', 'à', 'å', 'ç', 'ê', 'ë', 'è', 'ï', 'î', 'ì', 'Ä', 'Å',
        'É', 'æ', 'Æ', 'ô', 'ö', 'ò', 'û', 'ù', 'ÿ', 'Ö', 'Ü', 'ø', '£', 'Ø', '₧', 'ƒ',
        'á', 'í', 'ó', 'ú', 'ñ', 'Ñ', 'ª', 'º', '¿', '⌐', '¬', '½', '¼', '¡', '«', '¤',
        '░', '▒', '▓', '│', '┤', '╡', '╢', '╖', '╕', '╣', '║', '╗', '╝', '╜', '╛', '┐',
        '└', '┴', '┬', '├', '─', '┼', '╞', '╟', '╚', '╔', '╩', '╦', '╠', '═', '╬', '╧',
        '╨', '╤', '╥', '╙', '╘', '╒', '╓', '╫', '╪', '┘', '┌', '█', '▄', '▌', '▐', '▀',
        'α', 'ß', 'Γ', 'π', 'Σ', 'σ', 'µ', 'τ', 'Φ', 'Θ', 'Ω', 'δ', '∞', 'φ', 'ε', '∩',
        '≡', '±', '≥', '≤', '⌠', '⌡', '÷', '≈', '°', '∙', '·', '√', 'ⁿ', '²', '■', '\u{00A0}',
    ]
    .into_iter().enumerate()
    .map(|(i, c)| (c, (i + 128) as u8))
    .collect();

    /// ISO8859_2 Page code table
    static ref ISO8859_2_TABLE: HashMap<char, u8> = [
        '\u{00A0}', // NO-BREAK SPACE
        'Ą', '˘', 'Ł', '¤', 'Ľ', 'Ś', '§', '¨', 'Š', 'Ş', 'Ť', 'Ź',
        '\u{00AD}', // SOFT HYPHEN
        'Ž', 'Ż',
        '°', 'ą', '˛', 'ł', '´', 'ľ', 'ś', 'ˇ', '¸', 'š', 'ş', 'ť', 'ź', '˝', 'ž', 'ż',
        'Ŕ', 'Á', 'Â', 'Ă', 'Ä', 'Ĺ', 'Ć', 'Ç', 'Č', 'É', 'Ę', 'Ë', 'Ě', 'Í', 'Î', 'Ď',
        'Đ', 'Ń', 'Ň', 'Ó', 'Ô', 'Ő', 'Ö', '×', 'Ř', 'Ů', 'Ú', 'Ű', 'Ü', 'Ý', 'Ţ', 'ß',
        'ŕ', 'á', 'â', 'ă', 'ä', 'ĺ', 'ć', 'ç', 'č', 'é', 'ę', 'ë', 'ě', 'í', 'î', 'ď',
        'đ', 'ń', 'ň', 'ó', 'ô', 'ő', 'ö', '÷', 'ř', 'ů', 'ú', 'ű', 'ü', 'ý', 'ţ', '˙',
    ]
    .into_iter().enumerate()
    .map(|(i, c)| (c, (i + 0xA0) as u8))
    .collect();

    /// ISO8859_7 Page code table
    /// Uses '\0' as placeholder for empty spots
    static ref ISO8859_7_TABLE: HashMap<char, u8> = [
        '\u{00A0}', // NO-BREAK SPACE
        '‘', '’', '£', '€', '₯', '¦', '§', '¨', '©', 'ͺ', '«', '¬',
        '\u{00AD}', // SOFT HYPHEN
        '\0', '―',
        '°', '±', '²', '³', '΄', '΅', 'Ά', '·', 'Έ', 'Ή', 'Ί', '»', 'Ό', '½', 'Ύ', 'Ώ',
        'ΐ', 'Α', 'Β', 'Γ', 'Δ', 'Ε', 'Ζ', 'Η', 'Θ', 'Ι', 'Κ', 'Λ', 'Μ', 'Ν', 'Ξ', 'Ο',
        'Π', 'Ρ', '\0', 'Σ', 'Τ', 'Υ', 'Φ', 'Χ', 'Ψ', 'Ω', 'Ϊ', 'Ϋ', 'ά', 'έ', 'ή', 'ί',
        'ΰ', 'α', 'β', 'γ', 'δ', 'ε', 'ζ', 'η', 'θ', 'ι', 'κ', 'λ', 'μ', 'ν', 'ξ', 'ο',
        'π', 'ρ', 'ς', 'σ', 'τ', 'υ', 'φ', 'χ', 'ψ', 'ω', 'ϊ', 'ϋ', 'ό', 'ύ', 'ώ',
    ]
    .into_iter().enumerate()
    .filter(|(_, c)| *c != '\0')
    .map(|(i, c)| (c, (i + 0xA0) as u8))
    .collect();

    /// ISO8859_15 Page code table
    static ref ISO8859_15_TABLE: HashMap<char, u8> = [
        '\u{00A0}', // NO-BREAK SPACE
        '¡', '¢', '£', '€', '¥', 'Š', '§', 'š', '©', 'ª', '«', '¬',
        '\u{00AD}', // SOFT HYPHEN
        '®', '¯',
        '°', '±', '²', '³', 'Ž', 'µ', '¶', '·', 'ž', '¹', 'º', '»', 'Œ', 'œ', 'Ÿ', '¿',
        'À', 'Á', 'Â', 'Ã', 'Ä', 'Å', 'Æ', 'Ç', 'È', 'É', 'Ê', 'Ë', 'Ì', 'Í', 'Î', 'Ï',
        'Ð', 'Ñ', 'Ò', 'Ó', 'Ô', 'Õ', 'Ö', '×', 'Ø', 'Ù', 'Ú', 'Û', 'Ü', 'Ý', 'Þ', 'ß',
        'à', 'á', 'â', 'ã', 'ä', 'å', 'æ', 'ç', 'è', 'é', 'ê', 'ë', 'ì', 'í', 'î', 'ï',
        'ð', 'ñ', 'ò', 'ó', 'ô', 'õ', 'ö', '÷', 'ø', 'ù', 'ú', 'û', 'ü', 'ý', 'þ', 'ÿ',
    ]
    .into_iter().enumerate()
    .map(|(i, c)| (c, (i + 0xA0) as u8))
    .collect();

    /// WPC1252 Page code table
    /// Uses '\0' as placeholder for empty spots
    static ref WPC1252_TABLE: HashMap<char, u8> = [
        '€', '\0', '‚', 'ƒ', '„', '…', '†', '‡', 'ˆ', '‰', 'Š', '‹', 'Œ', '\0', 'Ž', '\0',
        '\0', '‘', '’', '“', '”', '•', '–', '—', '˜', '™', 'š', '›', 'œ', '\0', 'ž', 'Ÿ',
        '\u{00A0}', '¡', '¢', '£', '¤', '¥', '¦', '§', '¨', '©', 'ª', '«', '¬', '\u{00AD}', '®', '¯',
        '°', '±', '²', '³', '´', 'µ', '¶', '·', '¸', '¹', 'º', '»', '¼', '½', '¾', '¿',
        'À', 'Á', 'Â', 'Ã', 'Ä', 'Å', 'Æ', 'Ç', 'È', 'É', 'Ê', 'Ë', 'Ì', 'Í', 'Î', 'Ï',
        'Ð', 'Ñ', 'Ò', 'Ó', 'Ô', 'Õ', 'Ö', '×', 'Ø', 'Ù', 'Ú', 'Û', 'Ü', 'Ý', 'Þ', 'ß',
        'à', 'á', 'â', 'ã', 'ä', 'å', 'æ', 'ç', 'è', 'é', 'ê', 'ë', 'ì', 'í', 'î', 'ï',
        'ð', 'ñ', 'ò', 'ó', 'ô', 'õ', 'ö', '÷', 'ø', 'ù', 'ú', 'û', 'ü', 'ý', 'þ', 'ÿ',
    ]
    .into_iter().enumerate()
    .filter(|(_, c)| *c != '\0')
    .map(|(i, c)| (c, (i + 128) as u8))
    .collect();
}
