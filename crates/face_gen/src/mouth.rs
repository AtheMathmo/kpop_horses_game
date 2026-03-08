//! Mouth SVG generation.

use crate::{FACE_CX, FACE_CY, MouthStyle};

/// Vertical position of the mouth.
const MOUTH_Y: f32 = FACE_CY + 45.0;

pub fn mouth_svg(style: MouthStyle) -> String {
    match style {
        MouthStyle::Smile => smile(),
        MouthStyle::Neutral => neutral(),
        MouthStyle::Pout => pout(),
        MouthStyle::Open => open(),
        MouthStyle::Smirk => smirk(),
    }
}

fn smile() -> String {
    let cx = FACE_CX;
    let left = cx - 25.0;
    let right = cx + 25.0;
    let curve_y = MOUTH_Y + 14.0;
    let fill_y = MOUTH_Y + 10.0;
    let lip_stroke = "#CC2255";
    let lip_fill = "#DD4477";

    format!(
        r##"  <path d="M {left} {MOUTH_Y} Q {cx} {curve_y} {right} {MOUTH_Y}"
    fill="none" stroke="{lip_stroke}" stroke-width="3" stroke-linecap="round"/>
  <path d="M {left} {MOUTH_Y} Q {cx} {fill_y} {right} {MOUTH_Y} Z"
    fill="{lip_fill}" opacity="0.5"/>
"##
    )
}

fn neutral() -> String {
    let left = FACE_CX - 18.0;
    let right = FACE_CX + 18.0;
    let stroke = "#AA3355";

    format!(
        r##"  <line x1="{left}" y1="{MOUTH_Y}" x2="{right}" y2="{MOUTH_Y}"
    stroke="{stroke}" stroke-width="3" stroke-linecap="round"/>
"##
    )
}

fn pout() -> String {
    let cx = FACE_CX;
    let top = MOUTH_Y - 5.0;
    let bot = MOUTH_Y + 7.0;
    let left = cx - 14.0;
    let right = cx + 14.0;
    let fill_upper = "#DD4477";
    let fill_lower = "#EE5588";
    let stroke = "#AA2244";

    format!(
        r##"  <path d="M {left} {MOUTH_Y} Q {cx} {top} {right} {MOUTH_Y}" fill="{fill_upper}" stroke="{stroke}" stroke-width="1.5"/>
  <path d="M {left} {MOUTH_Y} Q {cx} {bot} {right} {MOUTH_Y}" fill="{fill_lower}" stroke="{stroke}" stroke-width="1.5"/>
"##
    )
}

fn open() -> String {
    let cx = FACE_CX;
    let inner_y = MOUTH_Y + 3.0;
    let fill_outer = "#331122";
    let stroke = "#AA2244";
    let fill_inner = "#CC3366";

    format!(
        r##"  <ellipse cx="{cx}" cy="{MOUTH_Y}" rx="14" ry="11" fill="{fill_outer}" stroke="{stroke}" stroke-width="2"/>
  <ellipse cx="{cx}" cy="{inner_y}" rx="10" ry="5" fill="{fill_inner}" opacity="0.6"/>
"##
    )
}

fn smirk() -> String {
    let cx = FACE_CX;
    let left = cx - 22.0;
    let right = cx + 15.0;
    let right_y = MOUTH_Y - 6.0;
    let curve_y = MOUTH_Y + 5.0;
    let stroke = "#CC2255";

    format!(
        r##"  <path d="M {left} {MOUTH_Y} Q {cx} {curve_y} {right} {right_y}"
    fill="none" stroke="{stroke}" stroke-width="3" stroke-linecap="round"/>
"##
    )
}
