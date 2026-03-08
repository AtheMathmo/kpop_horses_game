//! Horse tack (saddle, bridle, blanket) SVG generation.
//!
//! The horse faces left. Key reference points:
//! - Back/saddle area: ≈(255, 160)
//! - Belly: ≈(240, 260)
//! - Head center: ≈(85, 90)
//! - Muzzle/nose: ≈(50, 122)

use crate::TackStyle;

const LEATHER_BROWN: &str = "#6B3A2A";
const LEATHER_DARK: &str = "#4A2818";
const METAL_SILVER: &str = "#C0C0C0";
const BLANKET_PURPLE: &str = "#6B1FA8";
const BLANKET_GOLD: &str = "#FFD700";
const BRIDLE_BLACK: &str = "#222222";

/// Draw tack (saddle, bridle, blanket) on the horse.
pub fn tack_svg(style: TackStyle) -> String {
    match style {
        TackStyle::None => String::new(),
        TackStyle::WesternSaddle => western_saddle(),
        TackStyle::EnglishSaddle => english_saddle(),
        TackStyle::Blanket => blanket(),
        TackStyle::Bridle => bridle(),
    }
}

// ---------------------------------------------------------------------------
// Western Saddle — big, prominent horn and skirt
// ---------------------------------------------------------------------------

fn western_saddle() -> String {
    // Saddle sits on the back, centred around (255, 158)
    let cx = 255.0_f32;
    let cy = 158.0_f32;

    let mut s = String::with_capacity(1024);

    // Saddle blanket (under saddle)
    s.push_str(&format!(
        r#"  <ellipse cx="{cx}" cy="{bcy}" rx="60" ry="18"
    fill="{BLANKET_PURPLE}" opacity="0.6"/>
"#,
        bcy = cy + 8.0,
    ));

    // Saddle body
    s.push_str(&format!(
        r##"  <path d="M {lx} {cy}
    Q {lx} {ty}, {cx} {ty}
    Q {rx} {ty}, {rx} {cy}
    Q {rx} {by}, {cx} {by}
    Q {lx} {by}, {lx} {cy}
    Z"
    fill="{LEATHER_BROWN}" stroke="{LEATHER_DARK}" stroke-width="1.5"/>
"##,
        lx = cx - 42.0,
        rx = cx + 42.0,
        ty = cy - 22.0,
        by = cy + 16.0,
    ));

    // Horn (pommel) — on the front (left) side for a left-facing horse
    let horn_x = cx - 28.0;
    let horn_y = cy - 28.0;
    s.push_str(&format!(
        r#"  <rect x="{hx}" y="{horn_y}" width="8" height="18" rx="3" ry="3"
    fill="{LEATHER_DARK}"/>
  <circle cx="{hcx}" cy="{horn_y}" r="6" fill="{LEATHER_DARK}"/>
"#,
        hx = horn_x - 4.0,
        hcx = horn_x,
    ));

    // Stirrup (hangs from saddle centre)
    let stirrup_y = cy + 85.0;
    s.push_str(&format!(
        r##"  <line x1="{cx}" y1="{cy}" x2="{cx}" y2="{stirrup_y}"
    stroke="{LEATHER_DARK}" stroke-width="2"/>
  <path d="M {slx} {stirrup_y} Q {cx} {sby} {srx} {stirrup_y}"
    fill="none" stroke="{METAL_SILVER}" stroke-width="3"/>
"##,
        slx = cx - 8.0,
        srx = cx + 8.0,
        sby = stirrup_y + 12.0,
    ));

    // Girth strap (under belly)
    s.push_str(&format!(
        r##"  <line x1="{gx}" y1="{gy1}" x2="{gx}" y2="{gy2}"
    stroke="{LEATHER_DARK}" stroke-width="3"/>
"##,
        gx = cx - 5.0,
        gy1 = cy + 14.0,
        gy2 = cy + 95.0,
    ));

    s
}

// ---------------------------------------------------------------------------
// English Saddle — sleek, minimal
// ---------------------------------------------------------------------------

fn english_saddle() -> String {
    let cx = 255.0_f32;
    let cy = 158.0_f32;

    let mut s = String::with_capacity(512);

    // Saddle body (sleeker, flatter shape)
    s.push_str(&format!(
        r##"  <path d="M {lx} {cy}
    Q {lx} {ty}, {cx} {ty2}
    Q {rx} {ty}, {rx} {cy}
    Q {rx} {by}, {cx} {by2}
    Q {lx} {by}, {lx} {cy}
    Z"
    fill="{LEATHER_BROWN}" stroke="{LEATHER_DARK}" stroke-width="1.5"/>
"##,
        lx = cx - 38.0,
        rx = cx + 38.0,
        ty = cy - 16.0,
        ty2 = cy - 18.0,
        by = cy + 12.0,
        by2 = cy + 10.0,
    ));

    // Knee roll (front bump)
    s.push_str(&format!(
        r#"  <ellipse cx="{kx}" cy="{cy}" rx="6" ry="10"
    fill="{LEATHER_DARK}" opacity="0.5"/>
"#,
        kx = cx - 30.0,
    ));

    // Stirrup
    let stirrup_y = cy + 78.0;
    s.push_str(&format!(
        r##"  <line x1="{cx}" y1="{cy}" x2="{cx}" y2="{stirrup_y}"
    stroke="{LEATHER_DARK}" stroke-width="1.5"/>
  <path d="M {slx} {stirrup_y} Q {cx} {sby} {srx} {stirrup_y}"
    fill="none" stroke="{METAL_SILVER}" stroke-width="2.5"/>
"##,
        slx = cx - 7.0,
        srx = cx + 7.0,
        sby = stirrup_y + 10.0,
    ));

    // Girth
    s.push_str(&format!(
        r##"  <line x1="{gx}" y1="{gy1}" x2="{gx}" y2="{gy2}"
    stroke="{LEATHER_DARK}" stroke-width="2.5"/>
"##,
        gx = cx - 5.0,
        gy1 = cy + 10.0,
        gy2 = cy + 90.0,
    ));

    s
}

// ---------------------------------------------------------------------------
// Blanket — decorative saddle blanket (no saddle)
// ---------------------------------------------------------------------------

fn blanket() -> String {
    let cx = 255.0_f32;
    let cy = 155.0_f32;

    let mut s = String::with_capacity(512);

    // Main blanket shape draped over the back (convex top follows the sway)
    // The horse's back dips ~15px in the centre relative to the edges.
    s.push_str(&format!(
        r##"  <path d="M {lx} {ty_l}
    Q {cx} {ty_mid}, {rx} {ty_r}
    L {rx2} {by}
    Q {cx} {by2}, {lx2} {by}
    Z"
    fill="{BLANKET_PURPLE}" opacity="0.8"/>
"##,
        lx = cx - 58.0,
        rx = cx + 58.0,
        ty_l = cy - 12.0,  // front edge (slightly higher — near withers)
        ty_mid = cy + 2.0, // centre dips down (convex, follows back)
        ty_r = cy - 8.0,   // rear edge (slightly higher — near croup)
        rx2 = cx + 53.0,
        lx2 = cx - 53.0,
        by = cy + 35.0,
        by2 = cy + 40.0,
    ));

    // Decorative border stripes (also convex)
    s.push_str(&format!(
        r##"  <path d="M {lx} {sy_l}
    Q {cx} {sy_mid}, {rx} {sy_r}"
    fill="none" stroke="{BLANKET_GOLD}" stroke-width="3" opacity="0.9"/>
  <path d="M {lx2} {sy3}
    Q {cx} {sy4}, {rx2} {sy3}"
    fill="none" stroke="{BLANKET_GOLD}" stroke-width="3" opacity="0.9"/>
"##,
        lx = cx - 56.0,
        rx = cx + 56.0,
        sy_l = cy - 7.0,
        sy_mid = cy + 6.0,
        sy_r = cy - 3.0,
        lx2 = cx - 51.0,
        rx2 = cx + 51.0,
        sy3 = cy + 30.0,
        sy4 = cy + 35.0,
    ));

    // Diamond pattern in centre
    s.push_str(&format!(
        r##"  <path d="M {cx} {dy1} L {dx2} {cy} L {cx} {dy2} L {dx1} {cy} Z"
    fill="{BLANKET_GOLD}" opacity="0.6"/>
"##,
        dy1 = cy - 2.0,
        dy2 = cy + 22.0,
        dx1 = cx - 15.0,
        dx2 = cx + 15.0,
    ));

    s
}

// ---------------------------------------------------------------------------
// Bridle — head harness with reins
// ---------------------------------------------------------------------------

fn bridle() -> String {
    // Key anatomy: nostril (56,122), eye (95,90), ear base (115-125,75),
    //              ear tip (125,35), nose tip (43,120), back ≈(250,155)

    let mut s = String::with_capacity(1024);

    // -- Bit: placed just below the nostril, clearly in the mouth --
    s.push_str(&format!(
        r#"  <circle cx="56" cy="128" r="3.5" fill="{METAL_SILVER}"/>
"#
    ));

    // -- Cheek piece: single continuous strap from the bit, up parallel to
    //    the muzzle (below the eye), to under the ear --
    s.push_str(&format!(
        r##"  <path d="M 56,128 C 68,115 82,102 112,85"
    fill="none" stroke="{BRIDLE_BLACK}" stroke-width="3"/>
"##
    ));

    // -- Noseband: strap around the muzzle, perpendicular to the cheek piece --
    s.push_str(&format!(
        r##"  <path d="M 48,115 C 55,120 65,120 72,115"
    fill="none" stroke="{BRIDLE_BLACK}" stroke-width="2.5"/>
"##
    ));

    // -- Ear straps: from the top of the cheek piece, two lines go up
    //    to either side of the ear base --
    s.push_str(&format!(
        r##"  <line x1="112" y1="85" x2="113" y2="75"
    stroke="{BRIDLE_BLACK}" stroke-width="2.5"/>
  <line x1="112" y1="85" x2="127" y2="75"
    stroke="{BRIDLE_BLACK}" stroke-width="2.5"/>
"##
    ));

    // -- Throat latch: short strap from the ear loop under the jaw --
    s.push_str(&format!(
        r##"  <path d="M 122,75 C 118,82 108,88 100,92"
    fill="none" stroke="{BRIDLE_BLACK}" stroke-width="2"/>
"##
    ));

    // -- Reins: from the bit, loop down to the chest, then curve
    //    back up and over the horse's back --
    s.push_str(&format!(
        r##"  <path d="M 56,128
    C 75,155 110,175 145,172
    C 180,168 215,158 250,155"
    fill="none" stroke="{BRIDLE_BLACK}" stroke-width="2.5" stroke-linecap="round"/>
"##
    ));

    s
}
