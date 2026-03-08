//! Face shape SVG generation.

use crate::{FACE_CX, FACE_CY, FaceShape, SkinTone};

pub fn face_svg(shape: FaceShape, skin: SkinTone) -> String {
    let mut s = String::with_capacity(1024);

    // Face outline with gradient fill
    s.push_str(&face_outline(shape));

    // Subtle cheek blush (reduced on dark skin)
    s.push_str(&cheeks(shape, skin));

    // Ears
    s.push_str(&ears(shape, skin));

    s
}

fn face_outline(shape: FaceShape) -> String {
    match shape {
        FaceShape::Oval => {
            format!(
                r#"  <ellipse cx="{FACE_CX}" cy="{FACE_CY}" rx="88" ry="118"
    fill="url(#face-grad)" stroke="none" filter="url(#soft-shadow)"/>
"#
            )
        }
        FaceShape::Round => {
            format!(
                r#"  <circle cx="{FACE_CX}" cy="{FACE_CY}" r="105"
    fill="url(#face-grad)" stroke="none" filter="url(#soft-shadow)"/>
"#
            )
        }
        FaceShape::Square => {
            let x = FACE_CX - 95.0;
            let y = FACE_CY - 110.0;
            format!(
                r#"  <rect x="{x}" y="{y}" width="190" height="220" rx="22" ry="22"
    fill="url(#face-grad)" stroke="none" filter="url(#soft-shadow)"/>
"#
            )
        }
        FaceShape::Heart => {
            // Wider at forehead/cheeks, tapering to a narrow chin
            let cx = FACE_CX;
            let top_y = FACE_CY - 105.0;
            let chin_y = FACE_CY + 120.0;
            let wide = 95.0; // max half-width at cheekbone
            format!(
                r##"  <path d="M {cx} {chin_y}
    C {chin_ctrl_lx} {chin_ctrl_y}, {left_wide} {cheek_y}, {left_wide} {upper_y}
    C {left_wide} {top_ctrl_y}, {left_forehead} {top_y}, {cx} {top_y}
    C {right_forehead} {top_y}, {right_wide} {top_ctrl_y}, {right_wide} {upper_y}
    C {right_wide} {cheek_y}, {chin_ctrl_rx} {chin_ctrl_y}, {cx} {chin_y}
    Z"
    fill="url(#face-grad)" stroke="none" filter="url(#soft-shadow)"/>
"##,
                left_wide = cx - wide,
                right_wide = cx + wide,
                upper_y = FACE_CY - 40.0,
                cheek_y = FACE_CY + 20.0,
                chin_ctrl_lx = cx - 35.0,
                chin_ctrl_rx = cx + 35.0,
                chin_ctrl_y = FACE_CY + 80.0,
                top_ctrl_y = FACE_CY - 85.0,
                left_forehead = cx - 55.0,
                right_forehead = cx + 55.0,
            )
        }
        FaceShape::Long => {
            format!(
                r#"  <ellipse cx="{FACE_CX}" cy="{FACE_CY}" rx="75" ry="135"
    fill="url(#face-grad)" stroke="none" filter="url(#soft-shadow)"/>
"#
            )
        }
    }
}

fn cheeks(shape: FaceShape, skin: SkinTone) -> String {
    let (rx, offset_y) = match shape {
        FaceShape::Oval => (88.0, 20.0),
        FaceShape::Round => (105.0, 15.0),
        FaceShape::Square => (85.0, 15.0),
        FaceShape::Heart => (80.0, 10.0),
        FaceShape::Long => (70.0, 25.0),
    };

    // Reduce blush visibility on darker skin tones
    let opacity = match skin {
        SkinTone::Dark => 0.15,
        SkinTone::Tan => 0.25,
        _ => 0.35,
    };

    let cheek_y = FACE_CY + offset_y;
    let left_x = FACE_CX - rx * 0.55;
    let right_x = FACE_CX + rx * 0.55;

    format!(
        r##"  <ellipse cx="{left_x}" cy="{cheek_y}" rx="28" ry="20" fill="#FF8888" opacity="{opacity}"/>
  <ellipse cx="{right_x}" cy="{cheek_y}" rx="28" ry="20" fill="#FF8888" opacity="{opacity}"/>
"##
    )
}

fn ears(shape: FaceShape, skin: SkinTone) -> String {
    let color = skin.hex();
    let shadow = skin.shadow_hex();

    let (rx, ear_y) = match shape {
        FaceShape::Oval => (88.0, FACE_CY - 10.0),
        FaceShape::Round => (105.0, FACE_CY - 5.0),
        FaceShape::Square => (95.0, FACE_CY - 15.0),
        FaceShape::Heart => (90.0, FACE_CY - 10.0),
        FaceShape::Long => (75.0, FACE_CY - 10.0),
    };

    let left_x = FACE_CX - rx + 5.0;
    let right_x = FACE_CX + rx - 5.0;

    format!(
        r#"  <ellipse cx="{left_x}" cy="{ear_y}" rx="10" ry="18" fill="{color}" stroke="{shadow}" stroke-width="1.5"/>
  <ellipse cx="{right_x}" cy="{ear_y}" rx="10" ry="18" fill="{color}" stroke="{shadow}" stroke-width="1.5"/>
"#
    )
}
