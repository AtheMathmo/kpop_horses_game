//! Hair SVG generation — split into back layer (behind face) and front layer (bangs, etc.).

use crate::{FACE_CX, FACE_CY, HairMetrics, HairStyle};

/// Hair colour — deep violet with highlights.
const HAIR_BASE: &str = "#3A0066";
const HAIR_HIGHLIGHT: &str = "#6B1FA8";
const HAIR_ACCESSORY: &str = "#FF007F";

/// Hair drawn behind the face shape.
pub fn hair_back_svg(style: HairStyle, m: HairMetrics) -> String {
    match style {
        HairStyle::Long => long_back(m),
        HairStyle::Bangs => bangs_back(m),
        _ => String::new(),
    }
}

/// Hair drawn in front of the face.
pub fn hair_front_svg(style: HairStyle, m: HairMetrics) -> String {
    match style {
        HairStyle::Short => short_front(m),
        HairStyle::Long => long_front(m),
        HairStyle::Ponytail => ponytail_front(m),
        HairStyle::Spiky => spiky_front(m),
        HairStyle::Bangs => bangs_front(m),
    }
}

// ---------------------------------------------------------------------------
// Shared hair cap — close-fitting skull cap using elliptical arcs
// ---------------------------------------------------------------------------

/// Generates a skull-hugging hair cap path string using SVG elliptical arcs.
/// Returns only the SVG elements (no wrapping group). Used by short and ponytail.
fn hair_cap(m: HairMetrics) -> String {
    let pad = 6.0;
    let crown_pad = 8.0; // extra padding above crown

    // Outer dimensions
    let outer_side_x = m.side_x + pad;
    let outer_crown_rx = m.crown_rx + pad;
    let crown_y = m.crown_y - crown_pad;
    let side_y = m.side_y;

    // Quarter-arc radii: the arc connects the side to the crown edge
    let arc_rx = outer_side_x - outer_crown_rx;
    let arc_ry = side_y - crown_y;

    let outer_left = FACE_CX - outer_side_x;
    let outer_right = FACE_CX + outer_side_x;
    let crown_left = FACE_CX - outer_crown_rx;
    let crown_right = FACE_CX + outer_crown_rx;

    // Inner cutout: smaller arc exposing the forehead
    let inner_inset_x = 22.0;
    let inner_inset_y_frac = 0.45;
    let inner_side_x = m.side_x - inner_inset_x;
    let inner_crown_rx = (m.crown_rx - inner_inset_x).max(0.0);
    let inner_arc_rx = inner_side_x - inner_crown_rx;
    let inner_arc_ry = (arc_ry * inner_inset_y_frac).max(18.0);

    let inner_left = FACE_CX - inner_side_x;
    let inner_right = FACE_CX + inner_side_x;
    let inner_crown_left = FACE_CX - inner_crown_rx;
    let inner_crown_right = FACE_CX + inner_crown_rx;
    let inner_crown_y = side_y - inner_arc_ry;

    // Highlight
    let hl_left = FACE_CX - 50.0;
    let hl_right = FACE_CX + 35.0;
    let hl_y = crown_y + 20.0;
    let hl_peak = crown_y + 2.0;

    // Outer path: left-side → quarter-arc up → flat crown → quarter-arc down → right-side
    // Inner path (reverse): right-inner → quarter-arc up → flat crown → quarter-arc down → left-inner
    format!(
        r##"  <path d="M {outer_left} {side_y}
    A {arc_rx} {arc_ry} 0 0 1 {crown_left} {crown_y}
    L {crown_right} {crown_y}
    A {arc_rx} {arc_ry} 0 0 1 {outer_right} {side_y}
    L {inner_right} {side_y}
    A {inner_arc_rx} {inner_arc_ry} 0 0 0 {inner_crown_right} {inner_crown_y}
    L {inner_crown_left} {inner_crown_y}
    A {inner_arc_rx} {inner_arc_ry} 0 0 0 {inner_left} {side_y}
    Z"
    fill="{HAIR_BASE}"/>
  <path d="M {hl_left} {hl_y}
    Q {FACE_CX} {hl_peak}, {hl_right} {hl_y}"
    fill="none" stroke="{HAIR_HIGHLIGHT}" stroke-width="3" stroke-linecap="round" opacity="0.5"/>
"##
    )
}

// ---------------------------------------------------------------------------
// Short hair — close-cropped cap
// ---------------------------------------------------------------------------

fn short_front(m: HairMetrics) -> String {
    hair_cap(m)
}

// ---------------------------------------------------------------------------
// Long hair — flows past shoulders
// ---------------------------------------------------------------------------

fn long_back(m: HairMetrics) -> String {
    let pad = 22.0;
    let top = m.crown_y - 20.0;
    let left = FACE_CX - m.side_x - pad;
    let right = FACE_CX + m.side_x + pad;
    let bottom = FACE_CY + 160.0;
    let top_peak = top - 15.0;
    let upper = FACE_CY - 60.0;
    let mid = FACE_CY + 40.0;
    let left_ctrl = left - 10.0;
    let right_ctrl = right + 10.0;
    let bottom_curve = bottom + 20.0;
    let s1_x = FACE_CX - 60.0;
    let s1_y = top + 15.0;
    let s1_cx = FACE_CX - 70.0;
    let s1_cy = FACE_CY;
    let s1_ex = FACE_CX - 80.0;
    let s1_ey = bottom - 30.0;
    let s2_x = FACE_CX + 40.0;
    let s2_y = top + 20.0;
    let s2_cx = FACE_CX + 55.0;
    let s2_cy = FACE_CY + 20.0;
    let s2_ex = FACE_CX + 70.0;
    let s2_ey = bottom - 40.0;

    format!(
        r##"  <path d="M {left} {bottom}
    Q {left_ctrl} {mid}, {left} {upper}
    Q {left} {top}, {FACE_CX} {top_peak}
    Q {right} {top}, {right} {upper}
    Q {right_ctrl} {mid}, {right} {bottom}
    Q {FACE_CX} {bottom_curve}, {left} {bottom}
    Z"
    fill="{HAIR_BASE}"/>
  <path d="M {s1_x} {s1_y} Q {s1_cx} {s1_cy} {s1_ex} {s1_ey}"
    fill="none" stroke="{HAIR_HIGHLIGHT}" stroke-width="2.5" stroke-linecap="round" opacity="0.4"/>
  <path d="M {s2_x} {s2_y} Q {s2_cx} {s2_cy} {s2_ex} {s2_ey}"
    fill="none" stroke="{HAIR_HIGHLIGHT}" stroke-width="2" stroke-linecap="round" opacity="0.35"/>
"##
    )
}

/// Thin fringe covering the hairline seam for long hair.
fn long_front(m: HairMetrics) -> String {
    let pad = 10.0;
    let top = m.crown_y - 18.0;
    let left = FACE_CX - m.side_x - pad;
    let right = FACE_CX + m.side_x + pad;
    let top_peak = top - 12.0;
    // Fringe comes down just past the hairline
    let fringe_bottom = m.crown_y + 20.0;
    let left_in = FACE_CX - m.side_x + 5.0;
    let right_in = FACE_CX + m.side_x - 5.0;

    format!(
        r##"  <path d="M {left} {fringe_bottom}
    Q {left} {top}, {FACE_CX} {top_peak}
    Q {right} {top}, {right} {fringe_bottom}
    L {right_in} {fringe_bottom}
    Q {FACE_CX} {inner_top}, {left_in} {fringe_bottom}
    Z"
    fill="{HAIR_BASE}"/>
"##,
        inner_top = top + 8.0,
    )
}

// ---------------------------------------------------------------------------
// Ponytail — reuses hair cap + side tail
// ---------------------------------------------------------------------------

fn ponytail_front(m: HairMetrics) -> String {
    let mut s = hair_cap(m);

    // Compute the cap's outer edge position at the knot Y level,
    // so the knot sits exactly on the hair cap edge.
    let pad = 6.0;
    let crown_pad = 8.0;
    let outer_side_x = m.side_x + pad;
    let outer_crown_rx = m.crown_rx + pad;
    let crown_y = m.crown_y - crown_pad;
    let side_y = m.side_y;
    let arc_rx = outer_side_x - outer_crown_rx;
    let arc_ry = side_y - crown_y;

    // Place the knot about 1/3 down from the crown on the right side
    let knot_y = crown_y + arc_ry * 0.3;
    // Find the x on the outer arc at this y
    let dy = (knot_y - side_y) / arc_ry;
    let knot_x = FACE_CX + outer_crown_rx + arc_rx * (1.0 - dy * dy).sqrt();

    let tail_end_x = FACE_CX + 140.0;
    let tail_end_y = FACE_CY + 100.0;
    let tail_ctrl_x = tail_end_x + 20.0;
    let tail_ctrl_y = knot_y + 30.0;

    s.push_str(&format!(
        r##"  <path d="M {knot_x} {knot_y}
    Q {tail_ctrl_x} {tail_ctrl_y}, {tail_end_x} {tail_end_y}"
    fill="none" stroke="{HAIR_BASE}" stroke-width="28" stroke-linecap="round"/>
  <path d="M {knot_x} {knot_y}
    Q {tail_ctrl_x} {tail_ctrl_y}, {tail_end_x} {tail_end_y}"
    fill="none" stroke="{HAIR_HIGHLIGHT}" stroke-width="4" stroke-linecap="round" opacity="0.4"/>
  <circle cx="{knot_x}" cy="{knot_y}" r="12" fill="{HAIR_BASE}"/>
  <circle cx="{knot_x}" cy="{knot_y}" r="8" fill="{HAIR_ACCESSORY}" opacity="0.7"/>
"##
    ));

    s
}

// ---------------------------------------------------------------------------
// Spiky hair — dramatic pointed spikes on a skull-following base
// ---------------------------------------------------------------------------

fn spiky_front(m: HairMetrics) -> String {
    let mut s = String::with_capacity(1024);

    // Base band using the same quarter-arc + flat-crown approach as hair_cap
    let pad = 2.0;
    let crown_pad = 6.0;
    let outer_side_x = m.side_x + pad;
    let outer_crown_rx = m.crown_rx + pad;
    let crown_y = m.crown_y - crown_pad;
    let side_y = m.side_y;
    let arc_rx = outer_side_x - outer_crown_rx;
    let arc_ry = side_y - crown_y;

    let outer_left = FACE_CX - outer_side_x;
    let outer_right = FACE_CX + outer_side_x;
    let crown_left = FACE_CX - outer_crown_rx;
    let crown_right = FACE_CX + outer_crown_rx;

    let inner_inset = 10.0;
    let inner_side_x = m.side_x - inner_inset;
    let inner_crown_rx = (m.crown_rx - inner_inset).max(0.0);
    let inner_arc_rx = inner_side_x - inner_crown_rx;
    let inner_arc_ry = (arc_ry - inner_inset).max(12.0);
    let inner_left = FACE_CX - inner_side_x;
    let inner_right = FACE_CX + inner_side_x;
    let inner_crown_left = FACE_CX - inner_crown_rx;
    let inner_crown_right = FACE_CX + inner_crown_rx;
    let inner_crown_y = side_y - inner_arc_ry;

    s.push_str(&format!(
        r##"  <path d="M {outer_left} {side_y}
    A {arc_rx} {arc_ry} 0 0 1 {crown_left} {crown_y}
    L {crown_right} {crown_y}
    A {arc_rx} {arc_ry} 0 0 1 {outer_right} {side_y}
    L {inner_right} {side_y}
    A {inner_arc_rx} {inner_arc_ry} 0 0 0 {inner_crown_right} {inner_crown_y}
    L {inner_crown_left} {inner_crown_y}
    A {inner_arc_rx} {inner_arc_ry} 0 0 0 {inner_left} {side_y}
    Z"
    fill="{HAIR_BASE}"/>
"##
    ));

    // Spikes rise from the outer skull curve.
    // Outer spikes reach near the skull edges to avoid exposed skin at the corners.
    let spread = m.side_x * 0.92;
    let half_spread = spread * 0.5;
    let spike_w = (m.side_x * 0.22).min(20.0);

    let spike_defs: [(f32, f32, f32); 5] = [
        (-spread, 45.0, spike_w + 2.0),
        (-half_spread, 70.0, spike_w),
        (0.0, 80.0, spike_w + 3.0),
        (half_spread, 65.0, spike_w),
        (spread, 40.0, spike_w + 2.0),
    ];

    for (x_off, height, half_w) in &spike_defs {
        let cx_spike = FACE_CX + x_off;

        // Find Y on the outer skull curve at this X position.
        // The curve is: quarter-arc on each side + flat crown in between.
        let base_y = skull_y_at(
            x_off.abs(),
            outer_side_x,
            outer_crown_rx,
            arc_rx,
            arc_ry,
            side_y,
        );
        let tip_y = base_y - height;

        // Clamp corner positions so they don't extend beyond the base band
        let bl_x = (cx_spike - half_w).max(FACE_CX - outer_side_x);
        let br_x = (cx_spike + half_w).min(FACE_CX + outer_side_x);
        let bl_y = skull_y_at(
            (bl_x - FACE_CX).abs(),
            outer_side_x,
            outer_crown_rx,
            arc_rx,
            arc_ry,
            side_y,
        );
        let br_y = skull_y_at(
            (br_x - FACE_CX).abs(),
            outer_side_x,
            outer_crown_rx,
            arc_rx,
            arc_ry,
            side_y,
        );

        let hl_base_y = base_y + 5.0;

        s.push_str(&format!(
            r##"  <path d="M {bl_x} {bl_y} L {cx_spike} {tip_y} L {br_x} {br_y} Z" fill="{HAIR_BASE}"/>
  <line x1="{cx_spike}" y1="{tip_y}" x2="{cx_spike}" y2="{hl_base_y}" stroke="{HAIR_HIGHLIGHT}" stroke-width="2" opacity="0.4" stroke-linecap="round"/>
"##
        ));
    }

    s
}

/// Find the Y coordinate on the skull curve at a given |x_offset| from center.
/// The curve is: flat crown for |x| <= crown_rx, then quarter-ellipse arcs on each side.
fn skull_y_at(
    abs_x: f32,
    _side_x: f32,
    crown_rx: f32,
    arc_rx: f32,
    arc_ry: f32,
    side_y: f32,
) -> f32 {
    if abs_x <= crown_rx {
        // On the flat crown section
        side_y - arc_ry
    } else if arc_rx > 0.0 {
        // On the quarter-ellipse arc section
        let t = ((abs_x - crown_rx) / arc_rx).clamp(0.0, 1.0);
        side_y - arc_ry * (1.0 - t * t).sqrt()
    } else {
        side_y
    }
}

// ---------------------------------------------------------------------------
// Bangs — full fringe covering forehead
// ---------------------------------------------------------------------------

fn bangs_back(m: HairMetrics) -> String {
    let pad = 18.0;
    let top = m.crown_y - 20.0;
    let left = FACE_CX - m.side_x - pad;
    let right = FACE_CX + m.side_x + pad;
    let bottom = FACE_CY + 60.0;
    let top_peak = top - 15.0;
    let upper = FACE_CY - 80.0;
    let mid = FACE_CY - 20.0;
    let bottom_curve = bottom + 10.0;

    format!(
        r##"  <path d="M {left} {bottom}
    Q {left} {mid}, {left} {upper}
    Q {left} {top}, {FACE_CX} {top_peak}
    Q {right} {top}, {right} {upper}
    Q {right} {mid}, {right} {bottom}
    Q {FACE_CX} {bottom_curve}, {left} {bottom}
    Z"
    fill="{HAIR_BASE}"/>
"##
    )
}

fn bangs_front(m: HairMetrics) -> String {
    // Top of bangs extends above the crown so no face peeks through
    let top = m.crown_y - 5.0;
    let bang_bottom = FACE_CY - 40.0;
    let left = FACE_CX - m.side_x - 2.0;
    let right = FACE_CX + m.side_x - 12.0;
    let mid_left = FACE_CX - 50.0;
    let mid_right = FACE_CX + 35.0;
    let top_peak = top - 15.0;

    let mut s = format!(
        r##"  <path d="M {left} {top}
    Q {left} {bang_bottom}, {mid_left} {bang_bottom}
    L {mid_right} {bang_bottom}
    Q {right} {bang_bottom}, {right} {top}
    Q {FACE_CX} {top_peak}, {left} {top}
    Z"
    fill="{HAIR_BASE}"/>
"##
    );

    for x in [FACE_CX - 45.0, FACE_CX - 15.0, FACE_CX + 15.0] {
        let y1 = top + 5.0;
        let y2 = bang_bottom - 5.0;
        let x2 = x + 3.0;
        s.push_str(&format!(
            r##"  <line x1="{x}" y1="{y1}" x2="{x2}" y2="{y2}" stroke="{HAIR_HIGHLIGHT}" stroke-width="1.5" opacity="0.3" stroke-linecap="round"/>
"##
        ));
    }

    s
}
