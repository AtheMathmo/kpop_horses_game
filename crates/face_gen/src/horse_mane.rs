//! Horse mane SVG generation — various decorative mane styles.
//!
//! The horse faces left. The mane grows from the crest of the neck and
//! arcs upward/rightward from the poll (ears) to the withers (body junction).
//! Neck ridge runs from ≈(115,80) at the ears to ≈(185,148) at the withers.

use crate::ManeStyle;

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
// Flowing — long wavy mane cascading along the neck
// ---------------------------------------------------------------------------

fn flowing_mane() -> String {
    let mut s = flowing_base();

    // Highlight strands curving through the mane (upper-left → lower-right)
    s.push_str(&format!(
        r##"  <path d="M 118,72 C 130,52 150,58 170,108"
    fill="none" stroke="{MANE_HIGHLIGHT}" stroke-width="2.5" stroke-linecap="round" opacity="0.4"/>
  <path d="M 124,80 C 138,60 155,66 178,122"
    fill="none" stroke="{MANE_HIGHLIGHT}" stroke-width="2.5" stroke-linecap="round" opacity="0.4"/>
  <path d="M 130,88 C 145,70 160,78 182,135"
    fill="none" stroke="{MANE_HIGHLIGHT}" stroke-width="2.5" stroke-linecap="round" opacity="0.4"/>
"##
    ));

    s
}

// ---------------------------------------------------------------------------
// Braided — three-strand braid along the neck crest curve
// ---------------------------------------------------------------------------

fn braided_mane() -> String {
    let mut s = String::with_capacity(1024);

    // The braid follows the neck curve (cubic bezier from ears to withers).
    // Outer edge: offset ~8px above the neck curve.
    // Inner edge: sits on the neck curve itself.
    //
    // Neck curve: (120,70) C (150,80) (170,110) (190,140)
    // Offset curve (≈8px outward/upward):
    //   (118,63) C (146,72) (165,102) (188,133)
    s.push_str(&format!(
        r##"  <path d="M 118,63
    C 146,72 165,102 188,133
    L 190,140
    C 170,110 150,80 120,70
    Z"
    fill="{MANE_BASE}"/>
"##
    ));

    // Braid cross-pattern along the curve
    // Sample 6 points on the neck curve for cross-strand positions
    let segments = neck_curve_points(6);

    for (i, (sx, sy)) in segments.iter().enumerate() {
        let dir = if i % 2 == 0 { 1.0_f32 } else { -1.0 };
        // Perpendicular to the local curve tangent — approximate as across the braid
        let ox = dir * 5.0;
        let oy = -dir * 4.0;
        s.push_str(&format!(
            r##"  <path d="M {x1} {y1} Q {cx} {cy} {x2} {y2}"
    fill="none" stroke="{MANE_HIGHLIGHT}" stroke-width="3" stroke-linecap="round" opacity="0.5"/>
"##,
            x1 = sx + ox,
            y1 = sy + oy,
            cx = sx + 5.0,
            cy = sy + 3.0,
            x2 = sx + 10.0 - ox,
            y2 = sy + 8.0 - oy,
        ));
    }

    // Braid tie at the bottom (near withers)
    let (tx, ty) = neck_curve_point(0.95);
    s.push_str(&format!(
        r#"  <circle cx="{tx}" cy="{ty}" r="6" fill="{RIBBON_PINK}" opacity="0.8"/>
"#
    ));

    s
}

// ---------------------------------------------------------------------------
// Flowers — mane decorated with flowers
// ---------------------------------------------------------------------------

fn flowers_mane() -> String {
    let mut s = flowing_base();

    // Flowers placed within the mane shape (above the neck)
    let flower_positions: [(f32, f32, &str); 4] = [
        (124.0, 60.0, FLOWER_PINK),
        (142.0, 68.0, FLOWER_YELLOW),
        (160.0, 88.0, FLOWER_PINK),
        (176.0, 122.0, FLOWER_YELLOW),
    ];

    for (fx, fy, colour) in &flower_positions {
        let r = 4.5_f32;
        for angle_i in 0..5 {
            let angle = angle_i as f32 * std::f32::consts::TAU / 5.0;
            let px = fx + angle.cos() * 5.0;
            let py = fy + angle.sin() * 5.0;
            s.push_str(&format!(
                r#"  <circle cx="{px}" cy="{py}" r="{r}" fill="{colour}" opacity="0.85"/>
"#
            ));
        }
        // Flower center
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
    let mut s = flowing_base();

    let ribbons: [(&str, f32); 2] = [(RIBBON_PINK, 0.0), (RIBBON_BLUE, 5.0)];

    for (colour, offset) in &ribbons {
        // Ribbon weaving through the mane — offset perpendicular to the ridge
        // Perpendicular to 45° ridge → shift along (1,-1) direction
        let ox = *offset;
        let oy = -offset;
        s.push_str(&format!(
            r##"  <path d="M {sx} {sy}
    Q {c1x} {c1y}, {c2x} {c2y}
    Q {c3x} {c3y}, {c4x} {c4y}"
    fill="none" stroke="{colour}" stroke-width="4" stroke-linecap="round" opacity="0.8"/>
"##,
            sx = 118.0 + ox,
            sy = 70.0 + oy,
            c1x = 140.0 + ox,
            c1y = 50.0 + oy,
            c2x = 148.0 + ox,
            c2y = 72.0 + oy,
            c3x = 162.0 + ox,
            c3y = 85.0 + oy,
            c4x = 185.0 + ox,
            c4y = 140.0 + oy,
        ));

        // Bow at the bottom end (near withers)
        let bow_x = 185.0 + ox;
        let bow_y = 140.0 + oy;
        s.push_str(&format!(
            r##"  <path d="M {bow_x} {bow_y} Q {blx} {bly} {bex} {bey}"
    fill="none" stroke="{colour}" stroke-width="3" opacity="0.7"/>
  <path d="M {bow_x} {bow_y} Q {brx} {bry} {bex} {bey}"
    fill="none" stroke="{colour}" stroke-width="3" opacity="0.7"/>
"##,
            blx = bow_x - 8.0,
            bly = bow_y + 10.0,
            brx = bow_x + 8.0,
            bry = bow_y + 10.0,
            bex = bow_x,
            bey = bow_y + 16.0,
        ));
    }

    s
}

// ---------------------------------------------------------------------------
// Mohawk — dramatic upright spikes along the neck crest
// ---------------------------------------------------------------------------

fn mohawk_mane() -> String {
    let mut s = String::with_capacity(1024);

    // Spike heights — taller in the middle, shorter at extremes
    let spike_heights: [f32; 6] = [22.0, 28.0, 30.0, 26.0, 22.0, 16.0];

    // Sample 6 points along the neck curve for spike base positions
    let bases = neck_curve_points(6);

    for ((sx, sy), height) in bases.iter().zip(spike_heights.iter()) {
        // Spike tip points straight up (−Y) from the curve
        let tip_x = *sx;
        let tip_y = sy - height;
        // Base extends along the local curve direction
        let base_lx = sx - 4.5;
        let base_ly = sy - 2.0;
        let base_rx = sx + 4.5;
        let base_ry = sy + 2.0;
        s.push_str(&format!(
            r##"  <path d="M {base_lx} {base_ly} L {tip_x} {tip_y} L {base_rx} {base_ry} Z"
    fill="{MANE_BASE}"/>
  <line x1="{tip_x}" y1="{tip_y}" x2="{tip_x}" y2="{mid_y}"
    stroke="{MANE_HIGHLIGHT}" stroke-width="2" opacity="0.5" stroke-linecap="round"/>
"##,
            mid_y = sy - height * 0.3,
        ));
    }

    s
}

// ---------------------------------------------------------------------------
// Shared helper
// ---------------------------------------------------------------------------

/// Sample a point on the neck-top cubic bezier at parameter `t` ∈ [0,1].
///
/// The curve is defined by the body path segment from the ears to the withers:
///   P0=(120,70)  P1=(150,80)  P2=(170,110)  P3=(190,140)
fn neck_curve_point(t: f32) -> (f32, f32) {
    let u = 1.0 - t;
    let x =
        u * u * u * 120.0 + 3.0 * u * u * t * 150.0 + 3.0 * u * t * t * 170.0 + t * t * t * 190.0;
    let y = u * u * u * 70.0 + 3.0 * u * u * t * 80.0 + 3.0 * u * t * t * 110.0 + t * t * t * 140.0;
    (x, y)
}

/// Sample `n` evenly-spaced points along the neck curve (t = 0.1 … 0.9).
fn neck_curve_points(n: usize) -> Vec<(f32, f32)> {
    (0..n)
        .map(|i| {
            let t = 0.1 + 0.8 * (i as f32) / ((n - 1) as f32);
            neck_curve_point(t)
        })
        .collect()
}

/// Base flowing mane shape used by flowing, flowers, and ribbons styles.
///
/// Arcs upward from the poll (ears) along the top of the neck crest,
/// then sweeps down to the withers (body junction). Matches the template
/// mane direction: outer edge above the neck, inner edge on the ridge.
fn flowing_base() -> String {
    format!(
        r##"  <path d="M 112,68
    C 130,42 160,52 192,142
    L 185,148
    C 155,108 132,88 115,80
    Z"
    fill="{MANE_BASE}"/>
"##
    )
}
