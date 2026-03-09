//! Horse body and coat marking SVG generation.
//!
//! Geometry based on the reference template (`horse_template.svg`).
//! The horse faces left in a side profile, with far-side elements
//! rendered darker for depth.

use crate::{CoatColour, CoatStyle};

/// Hoof colour (dark brown-black).
const HOOF: &str = "#2D1E16";
/// Detail colour for eye and nostril.
const DETAIL: &str = "#2D1E16";

/// Draw the base horse body (side profile facing left).
///
/// Layer order (back to front):
///   far ear → far hind leg → far front leg → tail →
///   body+head → near hind leg → near front leg →
///   near ear → face details
pub fn body_svg(colour: CoatColour) -> String {
    let mut s = String::with_capacity(4096);
    let dark = colour.shadow_hex();

    // -- Far ear (darker for depth) --
    s.push_str(&format!(
        r#"  <path d="M 125,75 Q 125,45 135,40 Q 140,60 130,78 Z" fill="{dark}"/>
"#
    ));

    // -- Far hind leg + hoof --
    s.push_str(&format!(
        r#"  <path d="M 345,185 Q 320,245 320,275 Q 320,305 325,345 L 343,345 Q 338,305 337,280 Q 345,245 355,215 Z" fill="{dark}"/>
  <path d="M 325,345 L 343,345 L 345,360 L 323,360 Z" fill="{HOOF}"/>
"#
    ));

    // -- Far front leg + hoof --
    s.push_str(&format!(
        r#"  <path d="M 180,230 C 175,270 175,340 180,350 L 195,350 C 190,310 190,270 195,230 Z" fill="{dark}"/>
  <path d="M 180,350 L 195,350 L 198,365 L 177,365 Z" fill="{HOOF}"/>
"#
    ));

    // -- Body and head (single continuous path) --
    s.push_str(
        r#"  <path d="M 43,120 C 70,95 90,60 120,70 C 150,80 170,110 190,140 C 230,160 280,160 330,148 C 360,145 370,180 360,220 C 350,260 330,250 310,240 C 250,270 200,270 170,230 C 140,200 130,180 120,160 C 110,140 100,130 90,130 C 60,140 43,135 43,120 Z"
    fill="url(#coat-grad)" filter="url(#soft-shadow)"/>
"#,
    );

    // -- Near hind leg + hoof --
    s.push_str(&format!(
        r#"  <path d="M 280,220 Q 285,250 305,280 Q 305,310 310,350 L 328,350 Q 323,310 322,285 Q 370,250 350,200 Z" fill="url(#coat-grad)"/>
  <path d="M 310,350 L 328,350 L 331,365 L 307,365 Z" fill="{HOOF}"/>
"#
    ));

    // -- Near front leg + hoof --
    s.push_str(&format!(
        r#"  <path d="M 140,185 C 155,270 155,310 160,350 L 178,350 C 173,310 173,270 180,225 Z" fill="url(#coat-grad)"/>
  <path d="M 160,350 L 178,350 L 181,365 L 157,365 Z" fill="{HOOF}"/>
"#
    ));

    s
}

/// Elements rendered in front of the mane: near ear and face details.
///
/// Called after the mane layer so the ear overlaps the mane naturally.
pub fn body_front_svg(_colour: CoatColour) -> String {
    let mut s = String::with_capacity(256);

    // -- Near ear --
    s.push_str(
        r#"  <path d="M 115,75 Q 110,40 125,35 Q 130,55 125,75 Z" fill="url(#coat-grad)"/>
"#,
    );

    // -- Face details: eye and nostril --
    s.push_str(&format!(
        r#"  <circle cx="95" cy="90" r="4" fill="{DETAIL}"/>
  <circle cx="56" cy="122" r="3" fill="{DETAIL}"/>
"#
    ));

    s
}

// =========================================================================
// Coat markings
// =========================================================================

/// Draw coat markings on top of the base body.
pub fn markings_svg(style: CoatStyle, colour: CoatColour) -> String {
    match style {
        CoatStyle::Plain => String::new(),
        CoatStyle::Socks => socks_svg(),
        CoatStyle::Blaze => blaze_svg(),
        CoatStyle::Painted => painted_svg(colour),
        CoatStyle::Starry => starry_svg(),
    }
}

/// White socks on all four lower legs.
fn socks_svg() -> String {
    let mut s = String::with_capacity(512);

    // Near front leg sock (lower portion)
    s.push_str(
        r##"  <path d="M 155,310 C 155,310 155,350 160,350 L 178,350 C 173,340 173,310 175,300 Z"
    fill="#FFFFFF" opacity="0.85"/>
"##,
    );
    // Near hind leg sock
    s.push_str(
        r##"  <path d="M 305,300 Q 305,310 310,350 L 328,350 Q 323,310 322,300 Z"
    fill="#FFFFFF" opacity="0.85"/>
"##,
    );
    // Far front leg sock
    s.push_str(
        r##"  <path d="M 175,310 C 175,330 175,340 180,350 L 195,350 C 190,340 190,310 192,300 Z"
    fill="#FFFFFF" opacity="0.85"/>
"##,
    );
    // Far hind leg sock
    s.push_str(
        r##"  <path d="M 320,305 Q 320,305 325,345 L 343,345 Q 338,305 337,300 Z"
    fill="#FFFFFF" opacity="0.85"/>
"##,
    );

    s
}

/// White blaze — thin accent line along the top edge of the muzzle.
fn blaze_svg() -> String {
    // Follows the top contour of the muzzle (the bridge of the nose),
    // flush with the upper face outline.  Runs from below the forehead
    // down to just before the nose tip, avoiding the eye and nostril.
    r##"  <path d="M 78,88 C 68,96 58,105 50,113"
    fill="none" stroke="#FFFFFF" stroke-width="4" stroke-linecap="round" opacity="0.85"/>
"##
    .to_string()
}

/// Contrasting colour patches on the body — organic curved region that
/// follows the barrel of the horse for a perspectively correct paint pattern.
fn painted_svg(colour: CoatColour) -> String {
    let patch = match colour {
        CoatColour::Black => "#FFFFFF",
        CoatColour::White => "#8B4513",
        _ => "#FFFFFF",
    };

    // A large organic patch across the barrel/flank, shaped by cubic
    // beziers that follow the body's curvature.  Kept well within the
    // body outline to avoid bleed.
    format!(
        r##"  <path d="M 215,165
    C 230,160 260,160 280,168
    C 305,178 318,198 315,218
    C 312,235 290,248 262,248
    C 238,248 218,238 210,220
    C 202,202 205,178 215,165
    Z"
    fill="{patch}" opacity="0.65"/>
  <path d="M 178,195
    C 186,187 200,183 208,192
    C 218,204 215,222 200,230
    C 188,236 178,226 176,214
    C 174,204 174,198 178,195
    Z"
    fill="{patch}" opacity="0.5"/>
"##
    )
}

/// Scattered star sparkles on the body.
fn starry_svg() -> String {
    let mut s = String::with_capacity(512);

    let stars: [(f32, f32, f32); 8] = [
        (200.0, 170.0, 4.0),
        (230.0, 210.0, 3.0),
        (270.0, 180.0, 5.0),
        (310.0, 195.0, 3.5),
        (180.0, 200.0, 3.0),
        (250.0, 160.0, 4.5),
        (330.0, 170.0, 3.0),
        (220.0, 240.0, 3.5),
    ];

    for (sx, sy, r) in &stars {
        let r = *r;
        let ir = r * 0.4;
        s.push_str(&format!(
            r##"  <path d="M {sx} {ty} L {irx} {iry} L {rx} {sy} L {irx2} {iry2} L {sx} {by} L {ilx2} {ily2} L {lx} {sy} L {ilx} {ily} Z"
    fill="#FFFFFF" opacity="0.7"/>
"##,
            ty = sy - r,
            rx = sx + r,
            by = sy + r,
            lx = sx - r,
            irx = sx + ir,
            iry = sy - ir,
            irx2 = sx + ir,
            iry2 = sy + ir,
            ilx = sx - ir,
            ily = sy - ir,
            ilx2 = sx - ir,
            ily2 = sy + ir,
        ));
    }

    s
}
