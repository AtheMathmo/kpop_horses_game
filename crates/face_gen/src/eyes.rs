//! Eye SVG generation — iris, pupil, highlight, and optional eyelashes.

use crate::{EyeStyle, FACE_CX, FACE_CY, SkinTone};

/// Vertical center of the eyes relative to face center.
const EYE_Y: f32 = FACE_CY - 15.0;
/// Horizontal distance from center to each eye.
const EYE_SPREAD: f32 = 38.0;

pub fn eyes_svg(style: EyeStyle, skin: SkinTone) -> String {
    let left_x = FACE_CX - EYE_SPREAD;
    let right_x = FACE_CX + EYE_SPREAD;

    let mut s = String::with_capacity(1024);
    s.push_str(&single_eye(style, left_x, EYE_Y, false, skin));
    s.push_str(&single_eye(style, right_x, EYE_Y, true, skin));

    // Eyebrows
    s.push_str(&eyebrows(style, left_x, right_x));

    s
}

fn single_eye(style: EyeStyle, cx: f32, cy: f32, is_right: bool, skin: SkinTone) -> String {
    let mut s = String::new();
    s.push_str(&sclera(style, cx, cy, is_right));
    s.push_str(&iris(style, cx, cy, skin));
    s.push_str(&pupil(style, cx, cy));
    s.push_str(&highlight(style, cx, cy, is_right));
    s.push_str(&eyelid(style, cx, cy, is_right));
    s
}

fn sclera(style: EyeStyle, cx: f32, cy: f32, is_right: bool) -> String {
    let stroke = "#555555";
    match style {
        EyeStyle::Round => {
            format!(
                r##"  <ellipse cx="{cx}" cy="{cy}" rx="16" ry="15" fill="white" stroke="{stroke}" stroke-width="1"/>
"##
            )
        }
        EyeStyle::Almond => {
            let left = cx - 18.0;
            let right = cx + 18.0;
            let top = cy - 12.0;
            let bot = cy + 10.0;
            format!(
                r##"  <path d="M {left} {cy} Q {cx} {top} {right} {cy} Q {cx} {bot} {left} {cy} Z"
    fill="white" stroke="{stroke}" stroke-width="1"/>
"##
            )
        }
        EyeStyle::Cat => {
            // Outer corner lifts up and extends further; mirror for left vs right eye
            let (inner, outer, outer_y) = if is_right {
                (cx - 17.0, cx + 19.0, cy - 5.0)
            } else {
                (cx + 17.0, cx - 19.0, cy - 5.0)
            };
            let top = cy - 13.0;
            let bot = cy + 9.0;
            format!(
                r##"  <path d="M {inner} {cy} Q {cx} {top} {outer} {outer_y} Q {cx} {bot} {inner} {cy} Z"
    fill="white" stroke="{stroke}" stroke-width="1"/>
"##
            )
        }
        EyeStyle::Wide => {
            format!(
                r##"  <ellipse cx="{cx}" cy="{cy}" rx="20" ry="18" fill="white" stroke="{stroke}" stroke-width="1"/>
"##
            )
        }
        EyeStyle::Narrow => {
            let left = cx - 17.0;
            let right = cx + 17.0;
            let top = cy - 6.0;
            let bot = cy + 5.0;
            format!(
                r##"  <path d="M {left} {cy} Q {cx} {top} {right} {cy} Q {cx} {bot} {left} {cy} Z"
    fill="white" stroke="{stroke}" stroke-width="1"/>
"##
            )
        }
    }
}

fn iris(style: EyeStyle, cx: f32, cy: f32, skin: SkinTone) -> String {
    // Use lighter iris on dark skin for better contrast
    let iris_color = match skin {
        SkinTone::Dark => "#7B3EB8",
        _ => "#4B0082",
    };
    let r = match style {
        EyeStyle::Round => 9.0,
        EyeStyle::Almond => 8.0,
        EyeStyle::Cat => 8.5,
        EyeStyle::Wide => 11.0,
        EyeStyle::Narrow => 5.5,
    };

    format!(
        r##"  <circle cx="{cx}" cy="{cy}" r="{r}" fill="{iris_color}"/>
"##
    )
}

fn pupil(style: EyeStyle, cx: f32, cy: f32) -> String {
    let pupil_color = "#111111";
    let r = match style {
        EyeStyle::Round => 4.5,
        EyeStyle::Almond => 4.0,
        EyeStyle::Cat => 3.0,
        EyeStyle::Wide => 5.5,
        EyeStyle::Narrow => 3.0,
    };

    format!(
        r##"  <circle cx="{cx}" cy="{cy}" r="{r}" fill="{pupil_color}"/>
"##
    )
}

fn highlight(style: EyeStyle, cx: f32, cy: f32, is_right: bool) -> String {
    let (dx, dy, r) = match style {
        EyeStyle::Wide => (4.0, -4.0, 3.5),
        _ => (3.0, -3.0, 2.5),
    };
    // Highlight on the outer-upper side of each eye (mirrored)
    let dx = if is_right { dx } else { -dx };
    format!(
        r##"  <circle cx="{}" cy="{}" r="{r}" fill="white" opacity="0.9"/>
"##,
        cx + dx,
        cy + dy,
    )
}

fn eyelid(style: EyeStyle, cx: f32, cy: f32, is_right: bool) -> String {
    let line_color = "#333333";
    match style {
        EyeStyle::Round => {
            let left = cx - 16.0;
            let right = cx + 16.0;
            let top = cy - 15.0;
            format!(
                r##"  <path d="M {left} {cy} Q {cx} {top} {right} {cy}" fill="none" stroke="{line_color}" stroke-width="2" stroke-linecap="round"/>
"##
            )
        }
        EyeStyle::Almond => {
            let left = cx - 18.0;
            let right = cx + 18.0;
            let top = cy - 13.0;
            format!(
                r##"  <path d="M {left} {cy} Q {cx} {top} {right} {cy}" fill="none" stroke="{line_color}" stroke-width="2" stroke-linecap="round"/>
"##
            )
        }
        EyeStyle::Cat => {
            // Mirror the eyelid shape for left vs right eye
            let (inner, outer, outer_y) = if is_right {
                (cx - 17.0, cx + 19.0, cy - 5.0)
            } else {
                (cx + 17.0, cx - 19.0, cy - 5.0)
            };
            let top = cy - 14.0;
            let flick_x = if is_right { outer + 4.0 } else { outer - 4.0 };
            let flick_y = outer_y - 4.0;
            format!(
                r##"  <path d="M {inner} {cy} Q {cx} {top} {outer} {outer_y} L {flick_x} {flick_y}" fill="none" stroke="{line_color}" stroke-width="2" stroke-linecap="round"/>
"##
            )
        }
        EyeStyle::Wide => {
            let left = cx - 20.0;
            let right = cx + 20.0;
            let top = cy - 19.0;
            format!(
                r##"  <path d="M {left} {cy} Q {cx} {top} {right} {cy}" fill="none" stroke="{line_color}" stroke-width="1.5" stroke-linecap="round"/>
"##
            )
        }
        EyeStyle::Narrow => {
            let left = cx - 17.0;
            let right = cx + 17.0;
            let top = cy - 7.0;
            let bot = cy + 6.0;
            format!(
                r##"  <path d="M {left} {cy} Q {cx} {top} {right} {cy}" fill="none" stroke="{line_color}" stroke-width="2.5" stroke-linecap="round"/>
  <path d="M {left} {cy} Q {cx} {bot} {right} {cy}" fill="none" stroke="{line_color}" stroke-width="1.5" stroke-linecap="round"/>
"##
            )
        }
    }
}

fn eyebrows(style: EyeStyle, left_x: f32, right_x: f32) -> String {
    let brow_color = "#333333";
    let brow_y = EYE_Y - 24.0;

    let (w, thickness, arch) = match style {
        EyeStyle::Round => (18.0, 2.5, 6.0),
        EyeStyle::Almond => (20.0, 2.0, 5.0),
        EyeStyle::Cat => (20.0, 2.0, 8.0),
        EyeStyle::Wide => (22.0, 2.0, 4.0),
        EyeStyle::Narrow => (18.0, 3.0, 2.0),
    };

    let mut s = String::new();
    for &bx in &[left_x, right_x] {
        let x1 = bx - w;
        let x2 = bx + w;
        let peak_y = brow_y - arch;
        s.push_str(&format!(
            r##"  <path d="M {x1} {brow_y} Q {bx} {peak_y} {x2} {brow_y}" fill="none" stroke="{brow_color}" stroke-width="{thickness}" stroke-linecap="round"/>
"##
        ));
    }
    s
}
