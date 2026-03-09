//! Face shape SVG generation.

use crate::{FACE_CX, FACE_CY, FaceShape, SkinTone};

pub fn face_svg(shape: FaceShape, skin: SkinTone) -> String {
    let mut s = String::with_capacity(2048);

    // Face outline with gradient fill
    s.push_str(&face_outline(shape));

    // Nose highlight + jaw shadow, clipped to face shape
    s.push_str(&face_overlays(shape));

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
                r#"  <ellipse cx="{FACE_CX}" cy="{FACE_CY}" rx="85" ry="115"
    fill="url(#face-grad)" stroke="none" filter="url(#soft-shadow)"/>
"#
            )
        }
        FaceShape::Round => {
            // Slightly elliptical so it's not a perfect circle — more natural
            format!(
                r#"  <ellipse cx="{FACE_CX}" cy="{FACE_CY}" rx="90" ry="100"
    fill="url(#face-grad)" stroke="none" filter="url(#soft-shadow)"/>
"#
            )
        }
        FaceShape::Square => {
            let half_w = 85.0;
            let half_h = 110.0;
            let x = FACE_CX - half_w;
            let y = FACE_CY - half_h;
            format!(
                r#"  <rect x="{x}" y="{y}" width="{w}" height="{h}" rx="32" ry="32"
    fill="url(#face-grad)" stroke="none" filter="url(#soft-shadow)"/>
"#,
                w = half_w * 2.0,
                h = half_h * 2.0,
            )
        }
        FaceShape::Heart => {
            // Wider at forehead/cheeks, tapering to a rounded chin
            format!(
                r##"  <path d="{path}"
    fill="url(#face-grad)" stroke="none" filter="url(#soft-shadow)"/>
"##,
                path = heart_path(),
            )
        }
        FaceShape::Long => {
            format!(
                r#"  <ellipse cx="{FACE_CX}" cy="{FACE_CY}" rx="78" ry="125"
    fill="url(#face-grad)" stroke="none" filter="url(#soft-shadow)"/>
"#
            )
        }
    }
}

/// Heart face path as a reusable string (used for outline, clip, and overlays).
fn heart_path() -> String {
    let cx = FACE_CX;
    let top_y = FACE_CY - 105.0;
    let chin_y = FACE_CY + 108.0; // pulled up slightly for rounder chin
    let wide = 88.0;
    format!(
        "M {cx} {chin_y} \
         C {chin_ctrl_lx} {chin_ctrl_y}, {left_wide} {cheek_y}, {left_wide} {upper_y} \
         C {left_wide} {top_ctrl_y}, {left_forehead} {top_y}, {cx} {top_y} \
         C {right_forehead} {top_y}, {right_wide} {top_ctrl_y}, {right_wide} {upper_y} \
         C {right_wide} {cheek_y}, {chin_ctrl_rx} {chin_ctrl_y}, {cx} {chin_y} \
         Z",
        left_wide = cx - wide,
        right_wide = cx + wide,
        upper_y = FACE_CY - 40.0,
        cheek_y = FACE_CY + 20.0,
        // Wider chin control points → rounder chin curve
        chin_ctrl_lx = cx - 50.0,
        chin_ctrl_rx = cx + 50.0,
        chin_ctrl_y = FACE_CY + 70.0,
        top_ctrl_y = FACE_CY - 85.0,
        left_forehead = cx - 50.0,
        right_forehead = cx + 50.0,
    )
}

/// Nose highlight + jaw shadow overlays, clipped to the face shape so gradients
/// don't bleed outside non-elliptical faces (e.g. heart).
fn face_overlays(shape: FaceShape) -> String {
    let clip_shape = match shape {
        FaceShape::Oval => format!(r#"<ellipse cx="{FACE_CX}" cy="{FACE_CY}" rx="85" ry="115"/>"#),
        FaceShape::Round => format!(r#"<ellipse cx="{FACE_CX}" cy="{FACE_CY}" rx="90" ry="100"/>"#),
        FaceShape::Square => {
            let x = FACE_CX - 85.0;
            let y = FACE_CY - 110.0;
            format!(r#"<rect x="{x}" y="{y}" width="170" height="220" rx="32" ry="32"/>"#)
        }
        FaceShape::Heart => format!(r#"<path d="{}"/>"#, heart_path()),
        FaceShape::Long => format!(r#"<ellipse cx="{FACE_CX}" cy="{FACE_CY}" rx="78" ry="125"/>"#),
    };

    // Use a bounding rect for the overlay fills — the clip path constrains them
    let (rx, ry) = match shape {
        FaceShape::Oval => (85.0, 115.0),
        FaceShape::Round => (90.0, 100.0),
        FaceShape::Square => (85.0, 110.0),
        FaceShape::Heart => (88.0, 110.0),
        FaceShape::Long => (78.0, 125.0),
    };

    format!(
        r##"  <clipPath id="face-clip">{clip_shape}</clipPath>
  <g clip-path="url(#face-clip)">
    <ellipse cx="{FACE_CX}" cy="{FACE_CY}" rx="{rx}" ry="{ry}"
      fill="url(#nose-highlight)" stroke="none"/>
    <ellipse cx="{FACE_CX}" cy="{FACE_CY}" rx="{rx}" ry="{ry}"
      fill="url(#jaw-shadow)" stroke="none"/>
  </g>
"##
    )
}

fn cheeks(shape: FaceShape, skin: SkinTone) -> String {
    let (rx, offset_y) = match shape {
        FaceShape::Oval => (85.0, 20.0),
        FaceShape::Round => (90.0, 15.0),
        FaceShape::Square => (85.0, 15.0),
        FaceShape::Heart => (78.0, 10.0),
        FaceShape::Long => (74.0, 22.0),
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
        FaceShape::Oval => (85.0, FACE_CY - 10.0),
        FaceShape::Round => (90.0, FACE_CY - 5.0),
        FaceShape::Square => (85.0, FACE_CY - 15.0),
        FaceShape::Heart => (88.0, FACE_CY - 10.0),
        FaceShape::Long => (78.0, FACE_CY - 10.0),
    };

    let left_x = FACE_CX - rx + 5.0;
    let right_x = FACE_CX + rx - 5.0;

    format!(
        r#"  <ellipse cx="{left_x}" cy="{ear_y}" rx="10" ry="18" fill="{color}" stroke="{shadow}" stroke-width="1.5"/>
  <ellipse cx="{right_x}" cy="{ear_y}" rx="10" ry="18" fill="{color}" stroke="{shadow}" stroke-width="1.5"/>
"#
    )
}
