use serde::Deserialize;
use tui::style::Color;

use self::de::deserialize_option_color_hex_string;

macro_rules! def_theme_struct_with_defaults {
    ($($name:ident => $color:expr),+) => {
        #[derive(Debug, Clone, Copy, Deserialize)]
        pub struct Theme {
            $(
                #[serde(deserialize_with = "deserialize_option_color_hex_string")]
                #[serde(default)]
                $name: Option<Color>,
            )+
        }
        impl Theme {
            $(
                #[inline]
                pub fn $name(self) -> Color {
                    self.$name.unwrap_or($color)
                }
            )+
        }
        impl Default for Theme {
            fn default() -> Theme {
                Self {
                    $( $name: Some($color), )+
                }
            }
        }
    };
}

def_theme_struct_with_defaults!(
    text_primary => Color::White,
    text_secondary => Color::Magenta,
    border_color => Color::Magenta
);

fn hex_to_color(hex: &str) -> Option<Color> {
    if hex.len() == 7 {
        let hash = &hex[0..1];
        let r = u8::from_str_radix(&hex[1..3], 16);
        let g = u8::from_str_radix(&hex[3..5], 16);
        let b = u8::from_str_radix(&hex[5..7], 16);

        return match (hash, r, g, b) {
            ("#", Ok(r), Ok(g), Ok(b)) => Some(Color::Rgb(r, g, b)),
            _ => None,
        };
    }

    None
}

mod de {
    use std::fmt;

    use serde::de::{self, Error, Unexpected, Visitor};

    use super::{hex_to_color, Color};

    pub(crate) fn deserialize_option_color_hex_string<'de, D>(
        deserializer: D,
    ) -> Result<Option<Color>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct ColorVisitor;

        impl<'de> Visitor<'de> for ColorVisitor {
            type Value = Option<Color>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a hex string in the format of '#ff0000'")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                if let Some(color) = hex_to_color(s) {
                    return Ok(Some(color));
                }

                Err(de::Error::invalid_value(Unexpected::Str(s), &self))
            }
        }

        deserializer.deserialize_any(ColorVisitor)
    }
}
