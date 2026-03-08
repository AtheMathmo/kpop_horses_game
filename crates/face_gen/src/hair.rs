//! Hair SVG generation — split into back layer (behind face) and front layer (bangs, etc.).

use crate::{FACE_CX, FACE_CY, HairStyle};

/// Hair colour — deep violet with highlights.
const HAIR_BASE: &str = "#3A0066";
const HAIR_HIGHLIGHT: &str = "#6B1FA8";
const HAIR_ACCESSORY: &str = "#FF007F";

/// Hair drawn behind the face shape.
pub fn hair_back_svg(style: HairStyle) -> String {
    match style {
        HairStyle::Long => long_back(),
        HairStyle::Bangs => bangs_back(),
        _ => String::new(),
    }
}

/// Hair drawn in front of the face.
pub fn hair_front_svg(style: HairStyle) -> String {
    match style {
        HairStyle::Short => short_front(),
        HairStyle::Ponytail => ponytail_front(),
        HairStyle::Spiky => spiky_front(),
        HairStyle::Bangs => bangs_front(),
        _ => String::new(),
    }
}

// ---------------------------------------------------------------------------
// Short hair — close-cropped cap
// ---------------------------------------------------------------------------

fn short_front() -> String {
    let top = FACE_CY - 130.0;
    let left = FACE_CX - 100.0;
    let right = FACE_CX + 100.0;
    let side_y = FACE_CY - 40.0;
    let top_peak = top - 15.0;
    let right_in = FACE_CX + 85.0;
    let left_in = FACE_CX - 85.0;
    let side_in_y = FACE_CY - 50.0;
    let inner_top = top + 10.0;
    let hl_left = FACE_CX - 55.0;
    let hl_right = FACE_CX + 40.0;
    let hl_y = top + 10.0;
    let hl_peak = top - 5.0;

    format!(
        r##"  <path d="M {left} {side_y}
    Q {left} {top}, {FACE_CX} {top_peak}
    Q {right} {top}, {right} {side_y}
    L {right_in} {side_in_y}
    Q {FACE_CX} {inner_top}, {left_in} {side_in_y}
    Z"
    fill="{HAIR_BASE}"/>
  <path d="M {hl_left} {hl_y}
    Q {FACE_CX} {hl_peak}, {hl_right} {hl_y}"
    fill="none" stroke="{HAIR_HIGHLIGHT}" stroke-width="3" stroke-linecap="round" opacity="0.5"/>
"##
    )
}

// ---------------------------------------------------------------------------
// Long hair — flows past shoulders
// ---------------------------------------------------------------------------

fn long_back() -> String {
    let top = FACE_CY - 135.0;
    let left = FACE_CX - 110.0;
    let right = FACE_CX + 110.0;
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

// ---------------------------------------------------------------------------
// Ponytail — cap with side tail
// ---------------------------------------------------------------------------

fn ponytail_front() -> String {
    let top = FACE_CY - 130.0;
    let left = FACE_CX - 95.0;
    let right = FACE_CX + 95.0;
    let side_y = FACE_CY - 30.0;
    let top_peak = top - 12.0;
    let right_in = FACE_CX + 82.0;
    let left_in = FACE_CX - 82.0;
    let side_in_y = FACE_CY - 40.0;
    let inner_top = top + 12.0;
    let tail_start_x = right - 15.0;
    let tail_start_y = FACE_CY - 90.0;
    let tail_end_x = FACE_CX + 140.0;
    let tail_end_y = FACE_CY + 100.0;
    let tail_ctrl_x = tail_end_x + 20.0;
    let tail_ctrl_y = tail_start_y + 30.0;

    format!(
        r##"  <path d="M {left} {side_y}
    Q {left} {top}, {FACE_CX} {top_peak}
    Q {right} {top}, {right} {side_y}
    L {right_in} {side_in_y}
    Q {FACE_CX} {inner_top}, {left_in} {side_in_y}
    Z"
    fill="{HAIR_BASE}"/>
  <path d="M {tail_start_x} {tail_start_y}
    Q {tail_ctrl_x} {tail_ctrl_y}, {tail_end_x} {tail_end_y}"
    fill="none" stroke="{HAIR_BASE}" stroke-width="28" stroke-linecap="round"/>
  <path d="M {tail_start_x} {tail_start_y}
    Q {tail_ctrl_x} {tail_ctrl_y}, {tail_end_x} {tail_end_y}"
    fill="none" stroke="{HAIR_HIGHLIGHT}" stroke-width="4" stroke-linecap="round" opacity="0.4"/>
  <circle cx="{tail_start_x}" cy="{tail_start_y}" r="12" fill="{HAIR_BASE}"/>
  <circle cx="{tail_start_x}" cy="{tail_start_y}" r="8" fill="{HAIR_ACCESSORY}" opacity="0.7"/>
"##
    )
}

// ---------------------------------------------------------------------------
// Spiky hair — dramatic pointed spikes
// ---------------------------------------------------------------------------

fn spiky_front() -> String {
    let base_y = FACE_CY - 100.0;

    let spikes: [(f32, f32, f32); 5] = [
        (FACE_CX - 70.0, -60.0, 25.0),
        (FACE_CX - 35.0, -80.0, 22.0),
        (FACE_CX, -90.0, 28.0),
        (FACE_CX + 35.0, -75.0, 22.0),
        (FACE_CX + 70.0, -55.0, 25.0),
    ];

    let mut s = String::with_capacity(512);

    s.push_str(&format!(
        r##"  <ellipse cx="{FACE_CX}" cy="{base_y}" rx="100" ry="35" fill="{HAIR_BASE}"/>
"##
    ));

    for (sx, height, width) in &spikes {
        let tip_y = base_y + height;
        let bl = sx - width;
        let br = sx + width;
        let base_mid = base_y - 5.0;
        s.push_str(&format!(
            r##"  <path d="M {bl} {base_y} L {sx} {tip_y} L {br} {base_y} Z" fill="{HAIR_BASE}"/>
  <line x1="{sx}" y1="{tip_y}" x2="{sx}" y2="{base_mid}" stroke="{HAIR_HIGHLIGHT}" stroke-width="2" opacity="0.4" stroke-linecap="round"/>
"##
        ));
    }

    s
}

// ---------------------------------------------------------------------------
// Bangs — full fringe covering forehead
// ---------------------------------------------------------------------------

fn bangs_back() -> String {
    let top = FACE_CY - 135.0;
    let left = FACE_CX - 105.0;
    let right = FACE_CX + 105.0;
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

fn bangs_front() -> String {
    let top = FACE_CY - 120.0;
    let bang_bottom = FACE_CY - 40.0;
    let left = FACE_CX - 80.0;
    let right = FACE_CX + 65.0;
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
