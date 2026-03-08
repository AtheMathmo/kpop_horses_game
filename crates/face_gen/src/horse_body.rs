//! Horse body and coat marking SVG generation.

use crate::{CoatColour, CoatStyle, HORSE_CX, HORSE_CY};

/// Muzzle colour for dark vs light coats.
fn muzzle_hex(colour: CoatColour) -> &'static str {
    match colour {
        CoatColour::Black => "#333333",
        CoatColour::White => "#EECCC0",
        _ => "#C49880",
    }
}

/// Draw the base horse body (side profile facing right).
pub fn body_svg(colour: CoatColour) -> String {
    let mut s = String::with_capacity(2048);

    // -- Torso: large ellipse --
    let torso_cx = HORSE_CX;
    let torso_cy = HORSE_CY;
    s.push_str(&format!(
        r#"  <ellipse cx="{torso_cx}" cy="{torso_cy}" rx="120" ry="75"
    fill="url(#coat-grad)" filter="url(#soft-shadow)"/>
"#
    ));

    // -- Neck: angled upward to the right --
    let neck_base_x = torso_cx + 80.0;
    let neck_base_y = torso_cy - 40.0;
    let head_x = torso_cx + 145.0;
    let head_y = torso_cy - 130.0;
    s.push_str(&format!(
        r##"  <path d="M {nbx_l} {nby}
    Q {mid_x} {mid_y}, {hx} {hy}
    L {hx_r} {hy_r}
    Q {mid_x_r} {mid_y_r}, {nbx_r} {nby_r}
    Z"
    fill="url(#coat-grad)"/>
"##,
        nbx_l = neck_base_x - 20.0,
        nby = neck_base_y,
        mid_x = neck_base_x + 15.0,
        mid_y = neck_base_y - 60.0,
        hx = head_x - 15.0,
        hy = head_y + 10.0,
        hx_r = head_x + 15.0,
        hy_r = head_y + 10.0,
        mid_x_r = neck_base_x + 50.0,
        mid_y_r = neck_base_y - 50.0,
        nbx_r = neck_base_x + 20.0,
        nby_r = neck_base_y,
    ));

    // -- Head: elongated ellipse --
    s.push_str(&format!(
        r#"  <ellipse cx="{head_x}" cy="{head_y}" rx="35" ry="50"
    fill="url(#coat-grad)" transform="rotate(15 {head_x} {head_y})" filter="url(#soft-shadow)"/>
"#
    ));

    // -- Muzzle --
    let muzzle_x = head_x + 12.0;
    let muzzle_y = head_y + 38.0;
    let muzzle = muzzle_hex(colour);
    s.push_str(&format!(
        r#"  <ellipse cx="{muzzle_x}" cy="{muzzle_y}" rx="20" ry="16"
    fill="{muzzle}" transform="rotate(15 {muzzle_x} {muzzle_y})"/>
"#
    ));

    // -- Nostrils --
    s.push_str(&format!(
        r##"  <ellipse cx="{nx1}" cy="{ny}" rx="4" ry="3" fill="#222"/>
  <ellipse cx="{nx2}" cy="{ny}" rx="4" ry="3" fill="#222"/>
"##,
        nx1 = muzzle_x - 6.0,
        nx2 = muzzle_x + 6.0,
        ny = muzzle_y + 4.0,
    ));

    // -- Eye --
    let eye_x = head_x + 10.0;
    let eye_y = head_y - 15.0;
    s.push_str(&format!(
        r##"  <ellipse cx="{eye_x}" cy="{eye_y}" rx="7" ry="9" fill="#1A1A1A"/>
  <ellipse cx="{ehx}" cy="{ehy}" rx="2.5" ry="3" fill="#FFFFFF" opacity="0.7"/>
"##,
        ehx = eye_x + 2.0,
        ehy = eye_y - 3.0,
    ));

    // -- Ear --
    let ear_x = head_x + 5.0;
    let ear_y = head_y - 48.0;
    s.push_str(&format!(
        r##"  <path d="M {elx} {eby} L {ear_x} {ear_y} L {erx} {eby} Z"
    fill="url(#coat-grad)"/>
"##,
        elx = ear_x - 8.0,
        erx = ear_x + 8.0,
        eby = ear_y + 20.0,
    ));

    // -- Legs (4) --
    let leg_positions = [
        (torso_cx - 70.0, "front-left"),
        (torso_cx - 40.0, "front-right"),
        (torso_cx + 50.0, "back-left"),
        (torso_cx + 80.0, "back-right"),
    ];

    for (lx, _name) in &leg_positions {
        let top_y = torso_cy + 50.0;
        let bottom_y = HORSE_CY + 150.0;
        let hoof_y = bottom_y + 10.0;
        s.push_str(&format!(
            r##"  <rect x="{rx}" y="{top_y}" width="18" height="{h}" rx="5" ry="5"
    fill="url(#coat-grad)"/>
  <rect x="{rx}" y="{hy}" width="18" height="12" rx="3" ry="3"
    fill="#333"/>
"##,
            rx = lx - 9.0,
            h = bottom_y - top_y,
            hy = hoof_y - 2.0,
        ));
    }

    // -- Tail --
    let tail_x = torso_cx - 115.0;
    let tail_y = torso_cy - 20.0;
    let tail_end_y = torso_cy + 60.0;
    s.push_str(&format!(
        r##"  <path d="M {tail_x} {tail_y}
    Q {tcx} {tcy}, {tex} {tail_end_y}"
    fill="none" stroke="url(#coat-grad)" stroke-width="14" stroke-linecap="round"/>
"##,
        tcx = tail_x - 30.0,
        tcy = tail_y + 50.0,
        tex = tail_x - 15.0,
    ));

    s
}

/// Draw coat markings on top of the base body.
pub fn markings_svg(style: CoatStyle, colour: CoatColour) -> String {
    match style {
        CoatStyle::Plain => String::new(),
        CoatStyle::Socks => socks_svg(colour),
        CoatStyle::Blaze => blaze_svg(),
        CoatStyle::Painted => painted_svg(colour),
        CoatStyle::Starry => starry_svg(),
    }
}

fn socks_svg(_colour: CoatColour) -> String {
    let torso_cy = HORSE_CY;
    let mut s = String::with_capacity(512);

    let leg_xs = [
        HORSE_CX - 70.0,
        HORSE_CX - 40.0,
        HORSE_CX + 50.0,
        HORSE_CX + 80.0,
    ];

    for lx in &leg_xs {
        let sock_top = torso_cy + 110.0;
        let sock_bottom = HORSE_CY + 158.0;
        s.push_str(&format!(
            r##"  <rect x="{rx}" y="{sock_top}" width="18" height="{h}" rx="5" ry="5"
    fill="#FFFFFF" opacity="0.85"/>
"##,
            rx = lx - 9.0,
            h = sock_bottom - sock_top,
        ));
    }
    s
}

fn blaze_svg() -> String {
    let head_x = HORSE_CX + 145.0;
    let head_y = HORSE_CY - 130.0;

    format!(
        r##"  <path d="M {tx} {ty}
    Q {mx} {my}, {bx} {by}"
    fill="none" stroke="#FFFFFF" stroke-width="12" stroke-linecap="round" opacity="0.85"/>
"##,
        tx = head_x + 5.0,
        ty = head_y - 35.0,
        mx = head_x + 8.0,
        my = head_y,
        bx = head_x + 10.0,
        by = head_y + 30.0,
    )
}

fn painted_svg(colour: CoatColour) -> String {
    let torso_cx = HORSE_CX;
    let torso_cy = HORSE_CY;

    // Contrasting patch colour
    let patch = match colour {
        CoatColour::Black => "#FFFFFF",
        CoatColour::White => "#8B4513",
        _ => "#FFFFFF",
    };

    let mut s = String::with_capacity(512);

    // Large patches on torso
    s.push_str(&format!(
        r#"  <ellipse cx="{px1}" cy="{py1}" rx="40" ry="30" fill="{patch}" opacity="0.7"/>
  <ellipse cx="{px2}" cy="{py2}" rx="30" ry="25" fill="{patch}" opacity="0.6"/>
  <ellipse cx="{px3}" cy="{py3}" rx="25" ry="20" fill="{patch}" opacity="0.5"/>
"#,
        px1 = torso_cx - 30.0,
        py1 = torso_cy - 15.0,
        px2 = torso_cx + 40.0,
        py2 = torso_cy + 10.0,
        px3 = torso_cx - 60.0,
        py3 = torso_cy + 25.0,
    ));

    s
}

fn starry_svg() -> String {
    let torso_cx = HORSE_CX;
    let torso_cy = HORSE_CY;

    let mut s = String::with_capacity(512);

    // Scatter small star/sparkle dots
    let stars = [
        (torso_cx - 50.0, torso_cy - 30.0, 4.0),
        (torso_cx - 20.0, torso_cy + 20.0, 3.0),
        (torso_cx + 30.0, torso_cy - 10.0, 5.0),
        (torso_cx + 60.0, torso_cy + 15.0, 3.5),
        (torso_cx - 70.0, torso_cy + 10.0, 3.0),
        (torso_cx + 10.0, torso_cy - 40.0, 4.5),
        (torso_cx + 80.0, torso_cy - 25.0, 3.0),
        (torso_cx - 30.0, torso_cy + 40.0, 3.5),
    ];

    for (sx, sy, r) in &stars {
        // Four-pointed star using a path
        let r = *r;
        let ir = r * 0.4; // inner radius
        s.push_str(&format!(
            r##"  <path d="M {sx} {ty} L {irx} {iry} L {rx} {sy} L {irx2} {iry2} L {sx} {by} L {ilx2} {ily2} L {lx} {sy} L {ilx} {ily} Z"
    fill="#FFFFFF" opacity="0.7"/>
"##,
            ty = sy - r,
            rx = sx + r,
            by = sy + r,
            lx = sx - r,
            irx = sx + ir,
            iry = sy - ir,
            irx2 = sx + ir,
            iry2 = sy + ir,
            ilx = sx - ir,
            ily = sy - ir,
            ilx2 = sx - ir,
            ily2 = sy + ir,
        ));
    }

    s
}
