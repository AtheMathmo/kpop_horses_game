//! Horse tail SVG generation — various decorative tail styles.
//!
//! The tail extends from the rump at ≈(355,160) outward and downward.
//! It is rendered as its own layer behind the body so the rump naturally
//! overlaps the tail base.

use crate::TailStyle;

const TAIL_BASE: &str = "#3A0066";
const TAIL_HIGHLIGHT: &str = "#6B1FA8";
const FLOWER_PINK: &str = "#FF69B4";
const FLOWER_YELLOW: &str = "#FFD700";
const RIBBON_PINK: &str = "#FF007F";
const RIBBON_BLUE: &str = "#00BFFF";

/// Draw the horse tail.
pub fn tail_svg(style: TailStyle) -> String {
    match style {
        TailStyle::Plain => plain_tail(),
        TailStyle::Flowers => flowers_tail(),
        TailStyle::Braided => braided_tail(),
        TailStyle::Ribbons => ribbons_tail(),
    }
}

/// The base tail shape used by all styles.
/// Sweeps from the rump outward and down in a flowing arc.
fn tail_base() -> String {
    format!(
        r##"  <path d="M 355,160 C 390,160 410,210 395,270 C 380,260 370,220 360,180 Z"
    fill="{TAIL_BASE}"/>
"##
    )
}

/// Sample a point on the tail's spine bezier at parameter `t` in [0,1].
///
/// The spine follows the centre of the tail from rump to tip:
///   P0=(357,165)  P1=(385,170)  P2=(400,210)  P3=(395,270)
fn tail_spine_point(t: f32) -> (f32, f32) {
    let u = 1.0 - t;
    let x =
        u * u * u * 357.0 + 3.0 * u * u * t * 385.0 + 3.0 * u * t * t * 400.0 + t * t * t * 395.0;
    let y =
        u * u * u * 165.0 + 3.0 * u * u * t * 170.0 + 3.0 * u * t * t * 210.0 + t * t * t * 270.0;
    (x, y)
}

// ---------------------------------------------------------------------------
// Plain — just the base tail shape
// ---------------------------------------------------------------------------

fn plain_tail() -> String {
    tail_base()
}

// ---------------------------------------------------------------------------
// Flowers — flowers woven into the tail
// ---------------------------------------------------------------------------

fn flowers_tail() -> String {
    let mut s = tail_base();

    // Highlight strands through the tail
    s.push_str(&format!(
        r##"  <path d="M 360,170 C 380,175 395,200 393,240"
    fill="none" stroke="{TAIL_HIGHLIGHT}" stroke-width="2" stroke-linecap="round" opacity="0.4"/>
  <path d="M 358,175 C 375,182 388,210 390,255"
    fill="none" stroke="{TAIL_HIGHLIGHT}" stroke-width="2" stroke-linecap="round" opacity="0.4"/>
"##
    ));

    // Flowers placed along the tail spine
    let flower_positions: [(f32, f32, &str); 3] = [
        (
            tail_spine_point(0.25).0,
            tail_spine_point(0.25).1,
            FLOWER_PINK,
        ),
        (
            tail_spine_point(0.50).0,
            tail_spine_point(0.50).1,
            FLOWER_YELLOW,
        ),
        (
            tail_spine_point(0.75).0,
            tail_spine_point(0.75).1,
            FLOWER_PINK,
        ),
    ];

    for (fx, fy, colour) in &flower_positions {
        let r = 3.5_f32;
        for angle_i in 0..5 {
            let angle = angle_i as f32 * std::f32::consts::TAU / 5.0;
            let px = fx + angle.cos() * 4.0;
            let py = fy + angle.sin() * 4.0;
            s.push_str(&format!(
                r#"  <circle cx="{px}" cy="{py}" r="{r}" fill="{colour}" opacity="0.85"/>
"#
            ));
        }
        // Flower center
        s.push_str(&format!(
            r##"  <circle cx="{fx}" cy="{fy}" r="2.5" fill="#FFFFFF"/>
"##
        ));
    }

    s
}

// ---------------------------------------------------------------------------
// Braided — a three-strand braid pattern along the tail
// ---------------------------------------------------------------------------

fn braided_tail() -> String {
    let mut s = tail_base();

    // Braid cross-pattern along the tail spine
    let n = 6;
    for i in 0..n {
        let t = 0.15 + 0.7 * (i as f32) / ((n - 1) as f32);
        let (sx, sy) = tail_spine_point(t);
        let dir = if i % 2 == 0 { 1.0_f32 } else { -1.0 };
        let ox = dir * 4.0;
        let oy = -dir * 3.0;
        s.push_str(&format!(
            r##"  <path d="M {x1} {y1} Q {cx} {cy} {x2} {y2}"
    fill="none" stroke="{TAIL_HIGHLIGHT}" stroke-width="2.5" stroke-linecap="round" opacity="0.5"/>
"##,
            x1 = sx + ox,
            y1 = sy + oy,
            cx = sx + 4.0,
            cy = sy + 2.0,
            x2 = sx + 8.0 - ox,
            y2 = sy + 6.0 - oy,
        ));
    }

    // Braid tie at the tip
    let (tx, ty) = tail_spine_point(0.92);
    s.push_str(&format!(
        r#"  <circle cx="{tx}" cy="{ty}" r="5" fill="{RIBBON_PINK}" opacity="0.8"/>
"#
    ));

    s
}

// ---------------------------------------------------------------------------
// Ribbons — coloured ribbons woven through the tail
// ---------------------------------------------------------------------------

fn ribbons_tail() -> String {
    let mut s = tail_base();

    let ribbons: [(&str, f32); 2] = [(RIBBON_PINK, -3.0), (RIBBON_BLUE, 3.0)];

    for (colour, offset) in &ribbons {
        // Ribbon weaving through the tail, offset sideways from the spine
        let p0 = tail_spine_point(0.1);
        let p1 = tail_spine_point(0.35);
        let p2 = tail_spine_point(0.6);
        let p3 = tail_spine_point(0.85);
        s.push_str(&format!(
            r##"  <path d="M {sx} {sy}
    Q {c1x} {c1y}, {c2x} {c2y}
    Q {c3x} {c3y}, {c4x} {c4y}"
    fill="none" stroke="{colour}" stroke-width="3.5" stroke-linecap="round" opacity="0.8"/>
"##,
            sx = p0.0 + offset,
            sy = p0.1,
            c1x = p1.0 - offset,
            c1y = p1.1,
            c2x = p1.0 + offset,
            c2y = p1.1,
            c3x = p2.0 - offset,
            c3y = p2.1,
            c4x = p3.0 + offset,
            c4y = p3.1,
        ));

        // Small bow at the tip
        let bow_x = p3.0 + offset;
        let bow_y = p3.1;
        s.push_str(&format!(
            r##"  <path d="M {bow_x} {bow_y} Q {blx} {bly} {bex} {bey}"
    fill="none" stroke="{colour}" stroke-width="2.5" opacity="0.7"/>
  <path d="M {bow_x} {bow_y} Q {brx} {bry} {bex} {bey}"
    fill="none" stroke="{colour}" stroke-width="2.5" opacity="0.7"/>
"##,
            blx = bow_x - 6.0,
            bly = bow_y + 8.0,
            brx = bow_x + 6.0,
            bry = bow_y + 8.0,
            bex = bow_x,
            bey = bow_y + 12.0,
        ));
    }

    s
}
