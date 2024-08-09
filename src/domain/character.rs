//! Character

use std::fmt;

/// Underline mode
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum UnderlineMode {
    #[default]
    None,
    Single,
    Double,
}

impl fmt::Display for UnderlineMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnderlineMode::None => write!(f, "none"),
            UnderlineMode::Single => write!(f, "single"),
            UnderlineMode::Double => write!(f, "double"),
        }
    }
}

/// Text font
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Font {
    #[default]
    A,
    B,
    C,
}

impl fmt::Display for Font {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Font::A => write!(f, "font A"),
            Font::B => write!(f, "font B"),
            Font::C => write!(f, "font C"),
        }
    }
}

/// Character page code
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum PageCode {
    #[default]
    PC437,
    Katakana,
    PC850,
    PC860,
    PC863,
    PC865,
    Hiragana,
    PC851,
    PC853,
    PC857,
    PC737,
    ISO8859_7,
    WPC1252,
    PC866,
    PC852,
    PC858,
    PC720,
    WPC775,
    PC855,
    PC861,
    PC862,
    PC864,
    PC869,
    ISO8859_2,
    ISO8859_15,
    PC1098,
    PC1118,
    PC1119,
    PC1125,
    WPC1250,
    WPC1251,
    WPC1253,
    WPC1254,
    WPC1255,
    WPC1256,
    WPC1257,
    WPC1258,
    KZ1048,
}

impl fmt::Display for PageCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PageCode::PC437 => write!(f, "PC437"),
            PageCode::Katakana => write!(f, "Katakana"),
            PageCode::PC850 => write!(f, "PC850"),
            PageCode::PC860 => write!(f, "PC860"),
            PageCode::PC863 => write!(f, "PC863"),
            PageCode::PC865 => write!(f, "PC865"),
            PageCode::Hiragana => write!(f, "Hiragana"),
            PageCode::PC851 => write!(f, "PC851"),
            PageCode::PC853 => write!(f, "PC853"),
            PageCode::PC857 => write!(f, "PC857"),
            PageCode::PC737 => write!(f, "PC737"),
            PageCode::ISO8859_7 => write!(f, "ISO8859_7"),
            PageCode::WPC1252 => write!(f, "WPC1252"),
            PageCode::PC866 => write!(f, "PC866"),
            PageCode::PC852 => write!(f, "PC852"),
            PageCode::PC858 => write!(f, "PC858"),
            PageCode::PC720 => write!(f, "PC720"),
            PageCode::WPC775 => write!(f, "WPC775"),
            PageCode::PC855 => write!(f, "PC855"),
            PageCode::PC861 => write!(f, "PC861"),
            PageCode::PC862 => write!(f, "PC862"),
            PageCode::PC864 => write!(f, "PC864"),
            PageCode::PC869 => write!(f, "PC869"),
            PageCode::ISO8859_2 => write!(f, "ISO8859_2"),
            PageCode::ISO8859_15 => write!(f, "ISO8859_15"),
            PageCode::PC1098 => write!(f, "PC1098"),
            PageCode::PC1118 => write!(f, "PC1118"),
            PageCode::PC1119 => write!(f, "PC1119"),
            PageCode::PC1125 => write!(f, "PC1125"),
            PageCode::WPC1250 => write!(f, "WPC1250"),
            PageCode::WPC1251 => write!(f, "WPC1251"),
            PageCode::WPC1253 => write!(f, "WPC1253"),
            PageCode::WPC1254 => write!(f, "WPC1254"),
            PageCode::WPC1255 => write!(f, "WPC1255"),
            PageCode::WPC1256 => write!(f, "WPC1256"),
            PageCode::WPC1257 => write!(f, "WPC1257"),
            PageCode::WPC1258 => write!(f, "WPC1258"),
            PageCode::KZ1048 => write!(f, "KZ1048"),
        }
    }
}

impl From<PageCode> for u8 {
    fn from(value: PageCode) -> Self {
        match value {
            PageCode::PC437 => 0,
            PageCode::Katakana => 1,
            PageCode::PC850 => 2,
            PageCode::PC860 => 3,
            PageCode::PC863 => 4,
            PageCode::PC865 => 5,
            PageCode::Hiragana => 6,
            PageCode::PC851 => 11,
            PageCode::PC853 => 12,
            PageCode::PC857 => 13,
            PageCode::PC737 => 14,
            PageCode::ISO8859_7 => 15,
            PageCode::WPC1252 => 16,
            PageCode::PC866 => 17,
            PageCode::PC852 => 18,
            PageCode::PC858 => 19,
            PageCode::PC720 => 32,
            PageCode::WPC775 => 33,
            PageCode::PC855 => 34,
            PageCode::PC861 => 35,
            PageCode::PC862 => 36,
            PageCode::PC864 => 37,
            PageCode::PC869 => 38,
            PageCode::ISO8859_2 => 39,
            PageCode::ISO8859_15 => 40,
            PageCode::PC1098 => 41,
            PageCode::PC1118 => 42,
            PageCode::PC1119 => 43,
            PageCode::PC1125 => 44,
            PageCode::WPC1250 => 45,
            PageCode::WPC1251 => 46,
            PageCode::WPC1253 => 47,
            PageCode::WPC1254 => 48,
            PageCode::WPC1255 => 49,
            PageCode::WPC1256 => 50,
            PageCode::WPC1257 => 51,
            PageCode::WPC1258 => 52,
            PageCode::KZ1048 => 53,
        }
    }
}

/// Character page code
#[derive(Debug)]
pub enum CharacterSet {
    USA,
    France,
    Germany,
    UK,
    Denmark1,
    Sweden,
    Italy,
    Spain1,
    Japan,
    Norway,
    Denmark2,
    Spain2,
    LatinAmerica,
    Korea,
    SloveniaCroatia,
    China,
    Vietnam,
    Arabia,
    IndiaDevanagari,
    IndiaBengali,
    IndiaTamil,
    IndiaTelugu,
    IndiaAssamese,
    IndiaOriya,
    IndiaKannada,
    IndiaMalayalam,
    IndiaGujarati,
    IndiaPunjabi,
    IndiaMarathi,
}

impl From<CharacterSet> for u8 {
    fn from(value: CharacterSet) -> Self {
        match value {
            CharacterSet::USA => 0,
            CharacterSet::France => 1,
            CharacterSet::Germany => 2,
            CharacterSet::UK => 3,
            CharacterSet::Denmark1 => 4,
            CharacterSet::Sweden => 5,
            CharacterSet::Italy => 6,
            CharacterSet::Spain1 => 7,
            CharacterSet::Japan => 8,
            CharacterSet::Norway => 9,
            CharacterSet::Denmark2 => 10,
            CharacterSet::Spain2 => 11,
            CharacterSet::LatinAmerica => 12,
            CharacterSet::Korea => 13,
            CharacterSet::SloveniaCroatia => 14,
            CharacterSet::China => 15,
            CharacterSet::Vietnam => 16,
            CharacterSet::Arabia => 17,
            CharacterSet::IndiaDevanagari => 66,
            CharacterSet::IndiaBengali => 67,
            CharacterSet::IndiaTamil => 68,
            CharacterSet::IndiaTelugu => 69,
            CharacterSet::IndiaAssamese => 70,
            CharacterSet::IndiaOriya => 71,
            CharacterSet::IndiaKannada => 72,
            CharacterSet::IndiaMalayalam => 73,
            CharacterSet::IndiaGujarati => 74,
            CharacterSet::IndiaPunjabi => 75,
            CharacterSet::IndiaMarathi => 82,
        }
    }
}
