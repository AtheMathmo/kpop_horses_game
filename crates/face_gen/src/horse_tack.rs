//! Horse tack (saddle, bridle, blanket) SVG generation.

use crate::{HORSE_CX, HORSE_CY, TackStyle};

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
    let cx = HORSE_CX + 10.0;
    let cy = HORSE_CY - 30.0;

    let mut s = String::with_capacity(1024);

    // Saddle blanket (under saddle)
    s.push_str(&format!(
        r#"  <ellipse cx="{cx}" cy="{bcy}" rx="65" ry="20"
    fill="{BLANKET_PURPLE}" opacity="0.6"/>
"#,
        bcy = cy + 10.0,
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
        lx = cx - 45.0,
        rx = cx + 45.0,
        ty = cy - 25.0,
        by = cy + 18.0,
    ));

    // Horn (pommel)
    let horn_x = cx + 25.0;
    let horn_y = cy - 30.0;
    s.push_str(&format!(
        r#"  <rect x="{hx}" y="{horn_y}" width="8" height="18" rx="3" ry="3"
    fill="{LEATHER_DARK}"/>
  <circle cx="{hcx}" cy="{horn_y}" r="6" fill="{LEATHER_DARK}"/>
"#,
        hx = horn_x - 4.0,
        hcx = horn_x,
    ));

    // Stirrup
    let stirrup_x = cx;
    let stirrup_y = HORSE_CY + 40.0;
    s.push_str(&format!(
        r##"  <line x1="{cx}" y1="{cy}" x2="{stirrup_x}" y2="{stirrup_y}"
    stroke="{LEATHER_DARK}" stroke-width="2"/>
  <path d="M {slx} {stirrup_y} Q {stirrup_x} {sby} {srx} {stirrup_y}"
    fill="none" stroke="{METAL_SILVER}" stroke-width="3"/>
"##,
        slx = stirrup_x - 8.0,
        srx = stirrup_x + 8.0,
        sby = stirrup_y + 12.0,
    ));

    // Girth strap
    s.push_str(&format!(
        r##"  <line x1="{gx}" y1="{gy1}" x2="{gx}" y2="{gy2}"
    stroke="{LEATHER_DARK}" stroke-width="3"/>
"##,
        gx = cx - 5.0,
        gy1 = cy + 15.0,
        gy2 = HORSE_CY + 55.0,
    ));

    s
}

// ---------------------------------------------------------------------------
// English Saddle — sleek, minimal
// ---------------------------------------------------------------------------

fn english_saddle() -> String {
    let cx = HORSE_CX + 10.0;
    let cy = HORSE_CY - 28.0;

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
        lx = cx - 40.0,
        rx = cx + 40.0,
        ty = cy - 18.0,
        ty2 = cy - 20.0,
        by = cy + 14.0,
        by2 = cy + 12.0,
    ));

    // Knee rolls (small bumps on front)
    s.push_str(&format!(
        r#"  <ellipse cx="{kx}" cy="{cy}" rx="6" ry="10"
    fill="{LEATHER_DARK}" opacity="0.5"/>
"#,
        kx = cx + 32.0,
    ));

    // Stirrup leather and iron
    let stirrup_x = cx;
    let stirrup_y = HORSE_CY + 35.0;
    s.push_str(&format!(
        r##"  <line x1="{cx}" y1="{cy}" x2="{stirrup_x}" y2="{stirrup_y}"
    stroke="{LEATHER_DARK}" stroke-width="1.5"/>
  <path d="M {slx} {stirrup_y} Q {stirrup_x} {sby} {srx} {stirrup_y}"
    fill="none" stroke="{METAL_SILVER}" stroke-width="2.5"/>
"##,
        slx = stirrup_x - 7.0,
        srx = stirrup_x + 7.0,
        sby = stirrup_y + 10.0,
    ));

    // Girth
    s.push_str(&format!(
        r##"  <line x1="{gx}" y1="{gy1}" x2="{gx}" y2="{gy2}"
    stroke="{LEATHER_DARK}" stroke-width="2.5"/>
"##,
        gx = cx - 5.0,
        gy1 = cy + 12.0,
        gy2 = HORSE_CY + 50.0,
    ));

    s
}

// ---------------------------------------------------------------------------
// Blanket — decorative saddle blanket (no saddle)
// ---------------------------------------------------------------------------

fn blanket() -> String {
    let cx = HORSE_CX + 10.0;
    let cy = HORSE_CY - 20.0;

    let mut s = String::with_capacity(512);

    // Main blanket shape
    s.push_str(&format!(
        r##"  <path d="M {lx} {ty}
    Q {cx} {ty2}, {rx} {ty}
    L {rx2} {by}
    Q {cx} {by2}, {lx2} {by}
    Z"
    fill="{BLANKET_PURPLE}" opacity="0.8"/>
"##,
        lx = cx - 60.0,
        rx = cx + 60.0,
        ty = cy - 10.0,
        ty2 = cy - 18.0,
        rx2 = cx + 55.0,
        lx2 = cx - 55.0,
        by = cy + 35.0,
        by2 = cy + 40.0,
    ));

    // Decorative border stripe
    s.push_str(&format!(
        r##"  <path d="M {lx} {sy}
    Q {cx} {sy2}, {rx} {sy}"
    fill="none" stroke="{BLANKET_GOLD}" stroke-width="3" opacity="0.9"/>
  <path d="M {lx2} {sy3}
    Q {cx} {sy4}, {rx2} {sy3}"
    fill="none" stroke="{BLANKET_GOLD}" stroke-width="3" opacity="0.9"/>
"##,
        lx = cx - 58.0,
        rx = cx + 58.0,
        sy = cy - 5.0,
        sy2 = cy - 12.0,
        lx2 = cx - 53.0,
        rx2 = cx + 53.0,
        sy3 = cy + 30.0,
        sy4 = cy + 35.0,
    ));

    // Diamond pattern in center
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
    let head_x = HORSE_CX + 145.0;
    let head_y = HORSE_CY - 130.0;
    let muzzle_x = head_x + 12.0;
    let muzzle_y = head_y + 38.0;

    let mut s = String::with_capacity(512);

    // Crown piece (over ears)
    s.push_str(&format!(
        r##"  <path d="M {lx} {ly} Q {cx} {ty} {rx} {ry}"
    fill="none" stroke="{BRIDLE_BLACK}" stroke-width="3"/>
"##,
        lx = head_x - 15.0,
        ly = head_y - 35.0,
        cx = head_x + 5.0,
        ty = head_y - 48.0,
        rx = head_x + 20.0,
        ry = head_y - 35.0,
    ));

    // Cheek piece (vertical strap down side of face)
    s.push_str(&format!(
        r##"  <line x1="{cx}" y1="{cy1}" x2="{cx2}" y2="{cy2}"
    stroke="{BRIDLE_BLACK}" stroke-width="3"/>
"##,
        cx = head_x + 22.0,
        cy1 = head_y - 35.0,
        cx2 = muzzle_x + 10.0,
        cy2 = muzzle_y - 5.0,
    ));

    // Noseband
    s.push_str(&format!(
        r##"  <path d="M {lx} {ny} Q {mx} {ny2} {rx} {ny}"
    fill="none" stroke="{BRIDLE_BLACK}" stroke-width="2.5"/>
"##,
        lx = muzzle_x - 15.0,
        ny = muzzle_y - 8.0,
        mx = muzzle_x,
        ny2 = muzzle_y - 2.0,
        rx = muzzle_x + 15.0,
    ));

    // Bit (small circle at mouth)
    s.push_str(&format!(
        r#"  <circle cx="{bx}" cy="{by}" r="3" fill="{METAL_SILVER}"/>
"#,
        bx = muzzle_x - 10.0,
        by = muzzle_y + 5.0,
    ));

    // Reins (hanging down from bit)
    s.push_str(&format!(
        r##"  <path d="M {rx} {ry}
    Q {rcx} {rcy}, {rex} {rey}"
    fill="none" stroke="{BRIDLE_BLACK}" stroke-width="2.5" stroke-linecap="round"/>
"##,
        rx = muzzle_x - 10.0,
        ry = muzzle_y + 5.0,
        rcx = head_x - 30.0,
        rcy = HORSE_CY - 30.0,
        rex = HORSE_CX + 60.0,
        rey = HORSE_CY - 10.0,
    ));

    s
}
