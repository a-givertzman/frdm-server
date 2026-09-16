///
/// Color representation
#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    Black,
    Navy,
    DarkBlue,
    MediumBlue,
    Blue,
    DarkGreen,
    Green,
    Teal,
    DarkCyan,
    DeepSkyBlue,
    DarkTurquoise,
    MediumSpringGreen,
    Lime,
    SpringGreen,
    Aqua,
    Cyan,
    MidnightBlue,
    DodgerBlue,
    LightSeaGreen,
    ForestGreen,
    SeaGreen,
    DarkSlateGray,
    DarkSlateGrey,
    LimeGreen,
    MediumSeaGreen,
    Turquoise,
    RoyalBlue,
    SteelBlue,
    DarkSlateBlue,
    MediumTurquoise,
    Indigo,
    DarkOliveGreen,
    CadetBlue,
    CornflowerBlue,
    RebeccaPurple,
    MediumAquaMarine,
    DimGray,
    DimGrey,
    SlateBlue,
    OliveDrab,
    SlateGray,
    SlateGrey,
    LightSlateGray,
    LightSlateGrey,
    MediumSlateBlue,
    LawnGreen,
    Chartreuse,
    Aquamarine,
    Maroon,
    Purple,
    Olive,
    Gray,
    Grey,
    SkyBlue,
    LightSkyBlue,
    BlueViolet,
    DarkRed,
    DarkMagenta,
    SaddleBrown,
    DarkSeaGreen,
    LightGreen,
    MediumPurple,
    DarkViolet,
    PaleGreen,
    DarkOrchid,
    YellowGreen,
    Sienna,
    Brown,
    DarkGray,
    DarkGrey,
    LightBlue,
    GreenYellow,
    PaleTurquoise,
    LightSteelBlue,
    PowderBlue,
    FireBrick,
    DarkGoldenRod,
    MediumOrchid,
    RosyBrown,
    DarkKhaki,
    Silver,
    MediumVioletRed,
    IndianRed,
    Peru,
    Chocolate,
    Tan,
    LightGray,
    LightGrey,
    Thistle,
    Orchid,
    GoldenRod,
    PaleVioletRed,
    Crimson,
    Gainsboro,
    Plum,
    BurlyWood,
    LightCyan,
    Lavender,
    DarkSalmon,
    Violet,
    PaleGoldenRod,
    LightCoral,
    Khaki,
    AliceBlue,
    HoneyDew,
    Azure,
    SandyBrown,
    Wheat,
    Beige,
    WhiteSmoke,
    MintCream,
    GhostWhite,
    Salmon,
    AntiqueWhite,
    Linen,
    LightGoldenRodYellow,
    OldLace,
    Red,
    Fuchsia,
    Magenta,
    DeepPink,
    OrangeRed,
    Tomato,
    HotPink,
    Coral,
    DarkOrange,
    LightSalmon,
    Orange,
    LightPink,
    Pink,
    Gold,
    PeachPuff,
    NavajoWhite,
    Moccasin,
    Bisque,
    MistyRose,
    BlanchedAlmond,
    PapayaWhip,
    LavenderBlush,
    SeaShell,
    Cornsilk,
    LemonChiffon,
    FloralWhite,
    Snow,
    Yellow,
    LightYellow,
}
impl Color {
    ///
    /// Returns hex of RGB
    #[allow(unused)]
    fn hex(&self) -> usize {
        match self {
            Self::Black => 0x000000,
            Self::Navy => 0x000080,
            Self::DarkBlue => 0x00008B,
            Self::MediumBlue => 0x0000CD,
            Self::Blue => 0x0000FF,
            Self::DarkGreen => 0x006400,
            Self::Green => 0x008000,
            Self::Teal => 0x008080,
            Self::DarkCyan => 0x008B8B,
            Self::DeepSkyBlue => 0x00BFFF,
            Self::DarkTurquoise => 0x00CED1,
            Self::MediumSpringGreen => 0x00FA9A,
            Self::Lime => 0x00FF00,
            Self::SpringGreen => 0x00FF7F,
            Self::Aqua => 0x00FFFF,
            Self::Cyan => 0x00FFFF,
            Self::MidnightBlue => 0x191970,
            Self::DodgerBlue => 0x1E90FF,
            Self::LightSeaGreen => 0x20B2AA,
            Self::ForestGreen => 0x228B22,
            Self::SeaGreen => 0x2E8B57,
            Self::DarkSlateGray => 0x2F4F4F,
            Self::DarkSlateGrey => 0x2F4F4F,
            Self::LimeGreen => 0x32CD32,
            Self::MediumSeaGreen => 0x3CB371,
            Self::Turquoise => 0x40E0D0,
            Self::RoyalBlue => 0x4169E1,
            Self::SteelBlue => 0x4682B4,
            Self::DarkSlateBlue => 0x483D8B,
            Self::MediumTurquoise => 0x48D1CC,
            Self::Indigo => 0x4B0082,
            Self::DarkOliveGreen => 0x556B2F,
            Self::CadetBlue => 0x5F9EA0,
            Self::CornflowerBlue => 0x6495ED,
            Self::RebeccaPurple => 0x663399,
            Self::MediumAquaMarine => 0x66CDAA,
            Self::DimGray => 0x696969,
            Self::DimGrey => 0x696969,
            Self::SlateBlue => 0x6A5ACD,
            Self::OliveDrab => 0x6B8E23,
            Self::SlateGray => 0x708090,
            Self::SlateGrey => 0x708090,
            Self::LightSlateGray => 0x778899,
            Self::LightSlateGrey => 0x778899,
            Self::MediumSlateBlue => 0x7B68EE,
            Self::LawnGreen => 0x7CFC00,
            Self::Chartreuse => 0x7FFF00,
            Self::Aquamarine => 0x7FFFD4,
            Self::Maroon => 0x800000,
            Self::Purple => 0x800080,
            Self::Olive => 0x808000,
            Self::Gray => 0x808080,
            Self::Grey => 0x808080,
            Self::SkyBlue => 0x87CEEB,
            Self::LightSkyBlue => 0x87CEFA,
            Self::BlueViolet => 0x8A2BE2,
            Self::DarkRed => 0x8B0000,
            Self::DarkMagenta => 0x8B008B,
            Self::SaddleBrown => 0x8B4513,
            Self::DarkSeaGreen => 0x8FBC8F,
            Self::LightGreen => 0x90EE90,
            Self::MediumPurple => 0x9370DB,
            Self::DarkViolet => 0x9400D3,
            Self::PaleGreen => 0x98FB98,
            Self::DarkOrchid => 0x9932CC,
            Self::YellowGreen => 0x9ACD32,
            Self::Sienna => 0xA0522D,
            Self::Brown => 0xA52A2A,
            Self::DarkGray => 0xA9A9A9,
            Self::DarkGrey => 0xA9A9A9,
            Self::LightBlue => 0xADD8E6,
            Self::GreenYellow => 0xADFF2F,
            Self::PaleTurquoise => 0xAFEEEE,
            Self::LightSteelBlue => 0xB0C4DE,
            Self::PowderBlue => 0xB0E0E6,
            Self::FireBrick => 0xB22222,
            Self::DarkGoldenRod => 0xB8860B,
            Self::MediumOrchid => 0xBA55D3,
            Self::RosyBrown => 0xBC8F8F,
            Self::DarkKhaki => 0xBDB76B,
            Self::Silver => 0xC0C0C0,
            Self::MediumVioletRed => 0xC71585,
            Self::IndianRed => 0xCD5C5C,
            Self::Peru => 0xCD853F,
            Self::Chocolate => 0xD2691E,
            Self::Tan => 0xD2B48C,
            Self::LightGray => 0xD3D3D3,
            Self::LightGrey => 0xD3D3D3,
            Self::Thistle => 0xD8BFD8,
            Self::Orchid => 0xDA70D6,
            Self::GoldenRod => 0xDAA520,
            Self::PaleVioletRed => 0xDB7093,
            Self::Crimson => 0xDC143C,
            Self::Gainsboro => 0xDCDCDC,
            Self::Plum => 0xDDA0DD,
            Self::BurlyWood => 0xDEB887,
            Self::LightCyan => 0xE0FFFF,
            Self::Lavender => 0xE6E6FA,
            Self::DarkSalmon => 0xE9967A,
            Self::Violet => 0xEE82EE,
            Self::PaleGoldenRod => 0xEEE8AA,
            Self::LightCoral => 0xF08080,
            Self::Khaki => 0xF0E68C,
            Self::AliceBlue => 0xF0F8FF,
            Self::HoneyDew => 0xF0FFF0,
            Self::Azure => 0xF0FFFF,
            Self::SandyBrown => 0xF4A460,
            Self::Wheat => 0xF5DEB3,
            Self::Beige => 0xF5F5DC,
            Self::WhiteSmoke => 0xF5F5F5,
            Self::MintCream => 0xF5FFFA,
            Self::GhostWhite => 0xF8F8FF,
            Self::Salmon => 0xFA8072,
            Self::AntiqueWhite => 0xFAEBD7,
            Self::Linen => 0xFAF0E6,
            Self::LightGoldenRodYellow => 0xFAFAD2,
            Self::OldLace => 0xFDF5E6,
            Self::Red => 0xFF0000,
            Self::Fuchsia => 0xFF00FF,
            Self::Magenta => 0xFF00FF,
            Self::DeepPink => 0xFF1493,
            Self::OrangeRed => 0xFF4500,
            Self::Tomato => 0xFF6347,
            Self::HotPink => 0xFF69B4,
            Self::Coral => 0xFF7F50,
            Self::DarkOrange => 0xFF8C00,
            Self::LightSalmon => 0xFFA07A,
            Self::Orange => 0xFFA500,
            Self::LightPink => 0xFFB6C1,
            Self::Pink => 0xFFC0CB,
            Self::Gold => 0xFFD700,
            Self::PeachPuff => 0xFFDAB9,
            Self::NavajoWhite => 0xFFDEAD,
            Self::Moccasin => 0xFFE4B5,
            Self::Bisque => 0xFFE4C4,
            Self::MistyRose => 0xFFE4E1,
            Self::BlanchedAlmond => 0xFFEBCD,
            Self::PapayaWhip => 0xFFEFD5,
            Self::LavenderBlush => 0xFFF0F5,
            Self::SeaShell => 0xFFF5EE,
            Self::Cornsilk => 0xFFF8DC,
            Self::LemonChiffon => 0xFFFACD,
            Self::FloralWhite => 0xFFFAF0,
            Self::Snow => 0xFFFAFA,
            Self::Yellow => 0xFFFF00,
            Self::LightYellow => 0xFFFFE0,
        }
    }
}
///
/// Properties for the [Color]
#[allow(unused)]
pub trait ColorProps<T> {
    ///
    /// Returns Red channel value
    fn r(&self) -> T;
    ///
    /// Returns Green channel value
    fn g(&self) -> T;
    ///
    /// Returns Blue channel value
    fn b(&self) -> T;
    ///
    /// Returns RGB value
    fn rgb(&self) -> [T; 3];
    ///
    /// Returns RGB value
    fn bgr(&self) -> [T; 3];
    ///
    /// Returns RGB with alpha
    fn rgba(&self, alpha: T) -> [T; 4];
    ///
    /// Returns BGR with alpha
    fn bgra(&self, alpha: T) -> [T; 4];
}
//
//
impl ColorProps<u8> for Color {
    ///
    /// Returns Red channel value
    fn r(&self) -> u8 {
        self.rgb()[0]
    }
    ///
    /// Returns Green channel value
    fn g(&self) -> u8 {
        self.rgb()[1]
    }
    ///
    /// Returns Blue channel value
    fn b(&self) -> u8 {
        self.rgb()[2]
    }
    ///
    /// Returns RGB value
    fn rgb(&self) -> [u8; 3] {
        let bytes = u32::to_be_bytes(self.hex() as u32);
        [bytes[1], bytes[2], bytes[3]]
    }
    ///
    /// Returns RGB value
    fn bgr(&self) -> [u8; 3] {
        let rgb = self.rgb();
        [rgb[2], rgb[1], rgb[0]]
    }
    ///
    /// Returns RGB with alpha
    fn rgba(&self, alpha: u8) -> [u8; 4] {
        let rgb = self.rgb();
        [rgb[0], rgb[1], rgb[2], alpha]
    }
    ///
    /// Returns BGR with alpha
    fn bgra(&self, alpha: u8) -> [u8; 4] {
        let rgb = self.rgb();
        [rgb[2], rgb[1], rgb[0], alpha]
    }
}
//
//
impl ColorProps<f64> for Color {
    ///
    /// Returns Red channel value
    fn r(&self) -> f64 {
        self.rgb()[0]
    }
    ///
    /// Returns Green channel value
    fn g(&self) -> f64 {
        self.rgb()[1]
    }
    ///
    /// Returns Blue channel value
    fn b(&self) -> f64 {
        self.rgb()[2]
    }
    ///
    /// Returns RGB value
    fn rgb(&self) -> [f64; 3] {
        let bytes = u32::to_be_bytes(self.hex() as u32);
        [bytes[1] as f64, bytes[2] as f64, bytes[3] as f64]
    }
    ///
    /// Returns RGB value
    fn bgr(&self) -> [f64; 3] {
        let rgb = self.rgb();
        [rgb[2], rgb[1], rgb[0]]
    }
    ///
    /// Returns RGB with alpha
    fn rgba(&self, alpha: f64) -> [f64; 4] {
        let rgb = self.rgb();
        [rgb[0], rgb[1], rgb[2], alpha]
    }
    ///
    /// Returns BGR with alpha
    fn bgra(&self, alpha: f64) -> [f64; 4] {
        let rgb = self.rgb();
        [rgb[2], rgb[1], rgb[0], alpha]
    }
}

        // match self {
        //     Self::Black => [0x00, 0x00, 0x00],
        //     Self::Navy => [0x00, 0x00, 0x80],
        //     Self::DarkBlue => [0x00, 0x00, 0x8B],
        //     Self::MediumBlue => [0x00, 0x00, 0xCD],
        //     Self::Blue => [0x00, 0x00, 0xFF],
        //     Self::DarkGreen => [0x00, 0x64, 0x00],
        //     Self::Green => [0x00, 0x80, 0x00],
        //     Self::Teal => [0x00, 0x80, 0x80],
        //     Self::DarkCyan => [0x00, 0x8B, 0x8B],
        //     Self::DeepSkyBlue => [0x00, 0xBF, 0xFF],
        //     Self::DarkTurquoise => [0x00, 0xCE, 0xD1],
        //     Self::MediumSpringGreen => [0x00, 0xFA, 0x9A],
        //     Self::Lime => [0x00, 0xFF, 0x00],
        //     Self::SpringGreen => [0x00, 0xFF, 0x7F],
        //     Self::Aqua => [0x00, 0xFF, 0xFF],
        //     Self::Cyan => [0x00, 0xFF, 0xFF],
        //     Self::MidnightBlue => [0x19, 0x19, 0x70],
        //     Self::DodgerBlue => [0x1E, 0x90, 0xFF],
        //     Self::LightSeaGreen => [0x20, 0xB2, 0xAA],
        //     Self::ForestGreen => [0x22, 0x8B, 0x22],
        //     Self::SeaGreen => [0x2E, 0x8B, 0x57],
        //     Self::DarkSlateGray => [0x2F, 0x4F, 0x4F],
        //     Self::DarkSlateGrey => [0x2F, 0x4F, 0x4F],
        //     Self::LimeGreen => [0x32, 0xCD, 0x32],
        //     Self::MediumSeaGreen => [0x3C, 0xB3, 0x71],
        //     Self::Turquoise => [0x40, 0xE0, 0xD0],
        //     Self::RoyalBlue => [0x41, 0x69, 0xE1],
        //     Self::SteelBlue => [0x46, 0x82, 0xB4],
        //     Self::DarkSlateBlue => [0x48, 0x3D, 0x8B],
        //     Self::MediumTurquoise => [0x48, 0xD1, 0xCC],
        //     Self::Indigo => [0x4B, 0x00, 0x82],
        //     Self::DarkOliveGreen => [0x55, 0x6B, 0x2F],
        //     Self::CadetBlue => [0x5F, 0x9E, 0xA0],
        //     Self::CornflowerBlue => [0x64, 0x95, 0xED],
        //     Self::RebeccaPurple => [0x66, 0x33, 0x99],
        //     Self::MediumAquaMarine => [0x66, 0xCD, 0xAA],
        //     Self::DimGray => [0x69, 0x69, 0x69],
        //     Self::DimGrey => [0x69, 0x69, 0x69],
        //     Self::SlateBlue => [0x6A, 0x5A, 0xCD],
        //     Self::OliveDrab => [0x6B, 0x8E, 0x23],
        //     Self::SlateGray => [0x70, 0x80, 0x90],
        //     Self::SlateGrey => [0x70, 0x80, 0x90],
        //     Self::LightSlateGray => [0x77, 0x88, 0x99],
        //     Self::LightSlateGrey => [0x77, 0x88, 0x99],
        //     Self::MediumSlateBlue => [0x7B, 0x68, 0xEE],
        //     Self::LawnGreen => [0x7C, 0xFC, 0x00],
        //     Self::Chartreuse => [0x7F, 0xFF, 0x00],
        //     Self::Aquamarine => [0x7F, 0xFF, 0xD4],
        //     Self::Maroon => [0x80, 0x00, 0x00],
        //     Self::Purple => [0x80, 0x00, 0x80],
        //     Self::Olive => [0x80, 0x80, 0x00],
        //     Self::Gray => [0x80, 0x80, 0x80],
        //     Self::Grey => [0x80, 0x80, 0x80],
        //     Self::SkyBlue => [0x87, 0xCE, 0xEB],
        //     Self::LightSkyBlue => [0x87, 0xCE, 0xFA],
        //     Self::BlueViolet => [0x8A, 0x2B, 0xE2],
        //     Self::DarkRed => [0x8B, 0x00, 0x00],
        //     Self::DarkMagenta => [0x8B, 0x00, 0x8B],
        //     Self::SaddleBrown => [0x8B, 0x45, 0x13],
        //     Self::DarkSeaGreen => [0x8F, 0xBC, 0x8F],
        //     Self::LightGreen => [0x90, 0xEE, 0x90],
        //     Self::MediumPurple => [0x93, 0x70, 0xDB],
        //     Self::DarkViolet => [0x94, 0x00, 0xD3],
        //     Self::PaleGreen => [0x98, 0xFB, 0x98],
        //     Self::DarkOrchid => [0x99, 0x32, 0xCC],
        //     Self::YellowGreen => [0x9A, 0xCD, 0x32],
        //     Self::Sienna => [0xA0, 0x52, 0x2D],
        //     Self::Brown => [0xA5, 0x2A, 0x2A],
        //     Self::DarkGray => [0xA9, 0xA9, 0xA9],
        //     Self::DarkGrey => [0xA9, 0xA9, 0xA9],
        //     Self::LightBlue => [0xAD, 0xD8, 0xE6],
        //     Self::GreenYellow => [0xAD, 0xFF, 0x2F],
        //     Self::PaleTurquoise => [0xAF, 0xEE, 0xEE],
        //     Self::LightSteelBlue => [0xB0, 0xC4, 0xDE],
        //     Self::PowderBlue => [0xB0, 0xE0, 0xE6],
        //     Self::FireBrick => [0xB2, 0x22, 0x22],
        //     Self::DarkGoldenRod => [0xB8, 0x86, 0x0B],
        //     Self::MediumOrchid => [0xBA, 0x55, 0xD3],
        //     Self::RosyBrown => [0xBC, 0x8F, 0x8F],
        //     Self::DarkKhaki => [0xBD, 0xB7, 0x6B],
        //     Self::Silver => [0xC0, 0xC0, 0xC0],
        //     Self::MediumVioletRed => [0xC7, 0x15, 0x85],
        //     Self::IndianRed => [0xCD, 0x5C, 0x5C],
        //     Self::Peru => [0xCD, 0x85, 0x3F],
        //     Self::Chocolate => [0xD2, 0x69, 0x1E],
        //     Self::Tan => [0xD2, 0xB4, 0x8C],
        //     Self::LightGray => [0xD3, 0xD3, 0xD3],
        //     Self::LightGrey => [0xD3, 0xD3, 0xD3],
        //     Self::Thistle => [0xD8, 0xBF, 0xD8],
        //     Self::Orchid => [0xDA, 0x70, 0xD6],
        //     Self::GoldenRod => [0xDA, 0xA5, 0x20],
        //     Self::PaleVioletRed => [0xDB, 0x70, 0x93],
        //     Self::Crimson => [0xDC, 0x14, 0x3C],
        //     Self::Gainsboro => [0xDC, 0xDC, 0xDC],
        //     Self::Plum => [0xDD, 0xA0, 0xDD],
        //     Self::BurlyWood => [0xDE, 0xB8, 0x87],
        //     Self::LightCyan => [0xE0, 0xFF, 0xFF],
        //     Self::Lavender => [0xE6, 0xE6, 0xFA],
        //     Self::DarkSalmon => [0xE9, 0x96, 0x7A],
        //     Self::Violet => [0xEE, 0x82, 0xEE],
        //     Self::PaleGoldenRod => [0xEE, 0xE8, 0xAA],
        //     Self::LightCoral => [0xF0, 0x80, 0x80],
        //     Self::Khaki => [0xF0, 0xE6, 0x8C],
        //     Self::AliceBlue => [0xF0, 0xF8, 0xFF],
        //     Self::HoneyDew => [0xF0, 0xFF, 0xF0],
        //     Self::Azure => [0xF0, 0xFF, 0xFF],
        //     Self::SandyBrown => [0xF4, 0xA4, 0x60],
        //     Self::Wheat => [0xF5, 0xDE, 0xB3],
        //     Self::Beige => [0xF5, 0xF5, 0xDC],
        //     Self::WhiteSmoke => [0xF5, 0xF5, 0xF5],
        //     Self::MintCream => [0xF5, 0xFF, 0xFA],
        //     Self::GhostWhite => [0xF8, 0xF8, 0xFF],
        //     Self::Salmon => [0xFA, 0x80, 0x72],
        //     Self::AntiqueWhite => [0xFA, 0xEB, 0xD7],
        //     Self::Linen => [0xFA, 0xF0, 0xE6],
        //     Self::LightGoldenRodYellow => [0xFA, 0xFA, 0xD2],
        //     Self::OldLace => [0xFD, 0xF5, 0xE6],
        //     Self::Red => [0xFF, 0x00, 0x00],
        //     Self::Fuchsia => [0xFF, 0x00, 0xFF],
        //     Self::Magenta => [0xFF, 0x00, 0xFF],
        //     Self::DeepPink => [0xFF, 0x14, 0x93],
        //     Self::OrangeRed => [0xFF, 0x45, 0x00],
        //     Self::Tomato => [0xFF, 0x63, 0x47],
        //     Self::HotPink => [0xFF, 0x69, 0xB4],
        //     Self::Coral => [0xFF, 0x7F, 0x50],
        //     Self::DarkOrange => [0xFF, 0x8C, 0x00],
        //     Self::LightSalmon => [0xFF, 0xA0, 0x7A],
        //     Self::Orange => [0xFF, 0xA5, 0x00],
        //     Self::LightPink => [0xFF, 0xB6, 0xC1],
        //     Self::Pink => [0xFF, 0xC0, 0xCB],
        //     Self::Gold => [0xFF, 0xD7, 0x00],
        //     Self::PeachPuff => [0xFF, 0xDA, 0xB9],
        //     Self::NavajoWhite => [0xFF, 0xDE, 0xAD],
        //     Self::Moccasin => [0xFF, 0xE4, 0xB5],
        //     Self::Bisque => [0xFF, 0xE4, 0xC4],
        //     Self::MistyRose => [0xFF, 0xE4, 0xE1],
        //     Self::BlanchedAlmond => [0xFF, 0xEB, 0xCD],
        //     Self::PapayaWhip => [0xFF, 0xEF, 0xD5],
        //     Self::LavenderBlush => [0xFF, 0xF0, 0xF5],
        //     Self::SeaShell => [0xFF, 0xF5, 0xEE],
        //     Self::Cornsilk => [0xFF, 0xF8, 0xDC],
        //     Self::LemonChiffon => [0xFF, 0xFA, 0xCD],
        //     Self::FloralWhite => [0xFF, 0xFA, 0xF0],
        //     Self::Snow => [0xFF, 0xFA, 0xFA],
        //     Self::Yellow => [0xFF, 0xFF, 0x00],
        //     Self::LightYellow => [0xFF, 0xFF, 0xE0],
        // }
