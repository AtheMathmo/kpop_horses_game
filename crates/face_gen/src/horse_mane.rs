//! Horse mane SVG generation — various decorative mane styles.

use crate::{HORSE_CX, HORSE_CY, ManeStyle};

const MANE_BASE: &str = "#3A0066";
const MANE_HIGHLIGHT: &str = "#6B1FA8";
const FLOWER_PINK: &str = "#FF69B4";
const FLOWER_YELLOW: &str = "#FFD700";
const RIBBON_PINK: &str = "#FF007F";
const RIBBON_BLUE: &str = "#00BFFF";

/// Draw the mane on top of the horse's neck.
pub fn mane_svg(style: ManeStyle) -> String {
    match style {
        ManeStyle::Flowing => flowing_mane(),
        ManeStyle::Braided => braided_mane(),
        ManeStyle::Flowers => flowers_mane(),
        ManeStyle::Ribbons => ribbons_mane(),
        ManeStyle::Mohawk => mohawk_mane(),
    }
}

// ---------------------------------------------------------------------------
// Flowing — long wavy mane cascading down neck
// ---------------------------------------------------------------------------

fn flowing_mane() -> String {
    let neck_x = HORSE_CX + 100.0;
    let top_y = HORSE_CY - 160.0;

    let mut s = String::with_capacity(1024);

    // Main flowing mane shape along the neck
    s.push_str(&format!(
        r##"  <path d="M {sx} {sy}
    Q {c1x} {c1y}, {c2x} {c2y}
    Q {c3x} {c3y}, {c4x} {c4y}
    L {bx} {by}
    Q {c5x} {c5y}, {c6x} {c6y}
    Z"
    fill="{MANE_BASE}"/>
"##,
        sx = neck_x + 10.0,
        sy = top_y,
        c1x = neck_x + 25.0,
        c1y = top_y + 30.0,
        c2x = neck_x + 15.0,
        c2y = top_y + 70.0,
        c3x = neck_x + 5.0,
        c3y = top_y + 100.0,
        c4x = neck_x - 10.0,
        c4y = top_y + 130.0,
        bx = neck_x - 25.0,
        by = top_y + 120.0,
        c5x = neck_x - 15.0,
        c5y = top_y + 60.0,
        c6x = neck_x - 5.0,
        c6y = top_y + 10.0,
    ));

    // Highlight strands
    for i in 0..3 {
        let offset = i as f32 * 15.0;
        s.push_str(&format!(
            r##"  <path d="M {sx} {sy} Q {cx} {cy} {ex} {ey}"
    fill="none" stroke="{MANE_HIGHLIGHT}" stroke-width="2.5" stroke-linecap="round" opacity="0.4"/>
"##,
            sx = neck_x + 5.0 - offset * 0.3,
            sy = top_y + 10.0 + offset,
            cx = neck_x + 15.0 - offset * 0.2,
            cy = top_y + 40.0 + offset,
            ex = neck_x + 5.0 - offset * 0.5,
            ey = top_y + 70.0 + offset,
        ));
    }

    s
}

// ---------------------------------------------------------------------------
// Braided — three-strand braid down the neck
// ---------------------------------------------------------------------------

fn braided_mane() -> String {
    let neck_x = HORSE_CX + 105.0;
    let top_y = HORSE_CY - 155.0;

    let mut s = String::with_capacity(1024);

    // Base braid shape
    s.push_str(&format!(
        r##"  <path d="M {sx} {sy}
    L {bx} {by}
    L {bx2} {by}
    L {sx2} {sy}
    Z"
    fill="{MANE_BASE}"/>
"##,
        sx = neck_x,
        sy = top_y,
        bx = neck_x - 20.0,
        by = top_y + 120.0,
        bx2 = neck_x - 8.0,
        sx2 = neck_x + 12.0,
    ));

    // Braid cross-pattern
    for i in 0..6 {
        let y = top_y + 10.0 + i as f32 * 18.0;
        let x_base = neck_x + 2.0 - i as f32 * 2.5;
        let dir = if i % 2 == 0 { 1.0 } else { -1.0 };
        s.push_str(&format!(
            r##"  <path d="M {x1} {y} Q {cx} {cy} {x2} {y2}"
    fill="none" stroke="{MANE_HIGHLIGHT}" stroke-width="3" stroke-linecap="round" opacity="0.5"/>
"##,
            x1 = x_base - 5.0,
            cx = x_base + dir * 8.0,
            cy = y + 9.0,
            x2 = x_base + 5.0,
            y2 = y + 18.0,
        ));
    }

    // Braid tie at bottom
    let tie_y = top_y + 118.0;
    let tie_x = neck_x - 14.0;
    s.push_str(&format!(
        r#"  <circle cx="{tie_x}" cy="{tie_y}" r="6" fill="{RIBBON_PINK}" opacity="0.8"/>
"#
    ));

    s
}

// ---------------------------------------------------------------------------
// Flowers — mane decorated with flowers
// ---------------------------------------------------------------------------

fn flowers_mane() -> String {
    let neck_x = HORSE_CX + 100.0;
    let top_y = HORSE_CY - 160.0;

    let mut s = flowing_base(neck_x, top_y);

    // Scatter flowers along the mane
    let flower_positions = [
        (neck_x + 10.0, top_y + 15.0, FLOWER_PINK),
        (neck_x + 5.0, top_y + 45.0, FLOWER_YELLOW),
        (neck_x - 2.0, top_y + 75.0, FLOWER_PINK),
        (neck_x - 8.0, top_y + 105.0, FLOWER_YELLOW),
    ];

    for (fx, fy, colour) in &flower_positions {
        // Petals (5 small circles around center)
        let r = 4.5;
        for angle_i in 0..5 {
            let angle = angle_i as f32 * std::f32::consts::TAU / 5.0;
            let px = fx + angle.cos() * 5.0;
            let py = fy + angle.sin() * 5.0;
            s.push_str(&format!(
                r#"  <circle cx="{px}" cy="{py}" r="{r}" fill="{colour}" opacity="0.85"/>
"#
            ));
        }
        // Center
        s.push_str(&format!(
            r##"  <circle cx="{fx}" cy="{fy}" r="3" fill="#FFFFFF"/>
"##
        ));
    }

    s
}

// ---------------------------------------------------------------------------
// Ribbons — coloured ribbons woven through mane
// ---------------------------------------------------------------------------

fn ribbons_mane() -> String {
    let neck_x = HORSE_CX + 100.0;
    let top_y = HORSE_CY - 160.0;

    let mut s = flowing_base(neck_x, top_y);

    // Weave ribbons through the mane
    let ribbons = [(RIBBON_PINK, 0.0), (RIBBON_BLUE, 8.0)];

    for (colour, x_offset) in &ribbons {
        s.push_str(&format!(
            r##"  <path d="M {sx} {sy}
    Q {c1x} {c1y}, {c2x} {c2y}
    Q {c3x} {c3y}, {c4x} {c4y}"
    fill="none" stroke="{colour}" stroke-width="4" stroke-linecap="round" opacity="0.8"/>
"##,
            sx = neck_x + 8.0 + x_offset,
            sy = top_y + 5.0,
            c1x = neck_x + 20.0 + x_offset,
            c1y = top_y + 35.0,
            c2x = neck_x - 5.0 + x_offset,
            c2y = top_y + 65.0,
            c3x = neck_x + 15.0 + x_offset,
            c3y = top_y + 95.0,
            c4x = neck_x - 12.0 + x_offset,
            c4y = top_y + 125.0,
        ));

        // Ribbon bows at ends
        let bow_x = neck_x - 12.0 + x_offset;
        let bow_y = top_y + 125.0;
        s.push_str(&format!(
            r##"  <path d="M {bow_x} {bow_y} Q {blx} {bly} {bow_x} {bey}"
    fill="none" stroke="{colour}" stroke-width="3" opacity="0.7"/>
  <path d="M {bow_x} {bow_y} Q {brx} {bly} {bow_x} {bey}"
    fill="none" stroke="{colour}" stroke-width="3" opacity="0.7"/>
"##,
            blx = bow_x - 10.0,
            brx = bow_x + 10.0,
            bly = bow_y + 8.0,
            bey = bow_y + 15.0,
        ));
    }

    s
}

// ---------------------------------------------------------------------------
// Mohawk — dramatic upright punk-style mane
// ---------------------------------------------------------------------------

fn mohawk_mane() -> String {
    let neck_x = HORSE_CX + 100.0;
    let top_y = HORSE_CY - 160.0;

    let mut s = String::with_capacity(1024);

    // Spiky upright sections along the neck
    let spikes: [(f32, f32, f32); 6] = [
        (neck_x + 12.0, top_y - 5.0, 25.0),
        (neck_x + 7.0, top_y + 15.0, 30.0),
        (neck_x + 2.0, top_y + 35.0, 28.0),
        (neck_x - 3.0, top_y + 55.0, 25.0),
        (neck_x - 8.0, top_y + 75.0, 22.0),
        (neck_x - 13.0, top_y + 95.0, 18.0),
    ];

    for (sx, sy, height) in &spikes {
        let tip_y = sy - height;
        s.push_str(&format!(
            r##"  <path d="M {lx} {sy} L {sx} {tip_y} L {rx} {sy} Z"
    fill="{MANE_BASE}"/>
  <line x1="{sx}" y1="{tip_y}" x2="{sx}" y2="{mid_y}"
    stroke="{MANE_HIGHLIGHT}" stroke-width="2" opacity="0.5" stroke-linecap="round"/>
"##,
            lx = sx - 8.0,
            rx = sx + 8.0,
            mid_y = sy - height * 0.3,
        ));
    }

    s
}

// ---------------------------------------------------------------------------
// Shared helper
// ---------------------------------------------------------------------------

/// Base flowing shape used by flowers and ribbons styles.
fn flowing_base(neck_x: f32, top_y: f32) -> String {
    format!(
        r##"  <path d="M {sx} {sy}
    Q {c1x} {c1y}, {c2x} {c2y}
    Q {c3x} {c3y}, {c4x} {c4y}
    L {bx} {by}
    Q {c5x} {c5y}, {c6x} {c6y}
    Z"
    fill="{MANE_BASE}"/>
"##,
        sx = neck_x + 10.0,
        sy = top_y,
        c1x = neck_x + 25.0,
        c1y = top_y + 30.0,
        c2x = neck_x + 15.0,
        c2y = top_y + 70.0,
        c3x = neck_x + 5.0,
        c3y = top_y + 100.0,
        c4x = neck_x - 10.0,
        c4y = top_y + 130.0,
        bx = neck_x - 25.0,
        by = top_y + 120.0,
        c5x = neck_x - 15.0,
        c5y = top_y + 60.0,
        c6x = neck_x - 5.0,
        c6y = top_y + 10.0,
    )
}
