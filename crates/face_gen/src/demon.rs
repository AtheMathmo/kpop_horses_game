//! Cute demon sprite SVG generation for the Demon Hunt mini-game.
//!
//! Each demon is a 200x200 SVG with a silly, kid-friendly design:
//! big googly eyes, round shapes, tiny horns, bright colours.

/// Canvas dimensions for demon sprites.
pub const DEMON_W: f32 = 200.0;
pub const DEMON_H: f32 = 200.0;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum DemonType {
    /// Round pink blob with stubby horns and a goofy smile.
    #[default]
    Blob,
    /// Small purple bat-winged critter with big round eyes.
    Bat,
    /// Lanky magenta triangle body with a mischievous grin.
    Imp,
    /// Fluffy lavender cloud shape with tiny star-shaped horns.
    Puff,
}

pub const ALL_DEMON_TYPES: &[DemonType] = &[
    DemonType::Blob,
    DemonType::Bat,
    DemonType::Imp,
    DemonType::Puff,
];

impl DemonType {
    pub fn label(self) -> &'static str {
        match self {
            Self::Blob => "blob",
            Self::Bat => "bat",
            Self::Imp => "imp",
            Self::Puff => "puff",
        }
    }

    pub fn display(self) -> &'static str {
        match self {
            Self::Blob => "Blob",
            Self::Bat => "Bat",
            Self::Imp => "Imp",
            Self::Puff => "Puff",
        }
    }

    /// Primary body colour (from palette).
    pub fn hex(self) -> &'static str {
        match self {
            Self::Blob => "#FF007F", // NeonPink
            Self::Bat => "#9932CC",  // ElectricPurple
            Self::Imp => "#FF1DCE",  // HotMagenta
            Self::Puff => "#C8B4E6", // SoftLavender
        }
    }

    /// Darker shade for depth/shadows.
    pub fn shadow_hex(self) -> &'static str {
        match self {
            Self::Blob => "#CC0066",
            Self::Bat => "#7A28A3",
            Self::Imp => "#CC17A5",
            Self::Puff => "#A090C0",
        }
    }

    /// Lighter highlight.
    pub fn highlight_hex(self) -> &'static str {
        match self {
            Self::Blob => "#FF66B2",
            Self::Bat => "#BB77DD",
            Self::Imp => "#FF77DD",
            Self::Puff => "#E0D4F0",
        }
    }
}

/// Generate a complete SVG document for a demon sprite.
pub fn generate_demon_svg(demon_type: DemonType) -> String {
    let body = match demon_type {
        DemonType::Blob => blob_svg(demon_type),
        DemonType::Bat => bat_svg(demon_type),
        DemonType::Imp => imp_svg(demon_type),
        DemonType::Puff => puff_svg(demon_type),
    };

    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {DEMON_W} {DEMON_H}" width="{DEMON_W}" height="{DEMON_H}">
{body}
</svg>"#
    )
}

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

/// Big googly eyes — the same cute style on every demon.
fn googly_eyes(cx: f32, cy: f32, spacing: f32, size: f32) -> String {
    let lx = cx - spacing;
    let rx = cx + spacing;
    let sr = size * 1.1;
    let lpx = lx + 2.0;
    let lpy = cy + 1.0;
    let rpx = rx - 2.0;
    let rpy = cy + 1.0;
    let pr = size * 0.45;
    let lhx = lx + 4.0;
    let lhy = cy - 3.0;
    let rhx = rx;
    let rhy = cy - 3.0;
    let hr = size * 0.2;

    format!(
        r##"  <ellipse cx="{lx}" cy="{cy}" rx="{size}" ry="{sr}" fill="white" stroke="#333" stroke-width="1.5"/>
  <circle cx="{lpx}" cy="{lpy}" r="{pr}" fill="#333"/>
  <circle cx="{lhx}" cy="{lhy}" r="{hr}" fill="white"/>
  <ellipse cx="{rx}" cy="{cy}" rx="{size}" ry="{sr}" fill="white" stroke="#333" stroke-width="1.5"/>
  <circle cx="{rpx}" cy="{rpy}" r="{pr}" fill="#333"/>
  <circle cx="{rhx}" cy="{rhy}" r="{hr}" fill="white"/>
"##
    )
}

/// A pair of small stubby horns.
fn stubby_horns(cx: f32, top_y: f32, spread: f32, colour: &str) -> String {
    let lx = cx - spread;
    let rx = cx + spread;
    let lmx = lx + 8.0;
    let ltx = lx + 16.0;
    let rmx = rx - 8.0;
    let rtx = rx - 16.0;
    let tip_y = top_y - 22.0;

    format!(
        r##"  <path d="M {lx},{top_y} Q {lmx},{tip_y} {ltx},{top_y}" fill="{colour}" stroke="#333" stroke-width="1"/>
  <path d="M {rx},{top_y} Q {rmx},{tip_y} {rtx},{top_y}" fill="{colour}" stroke="#333" stroke-width="1"/>
"##
    )
}

// ---------------------------------------------------------------------------
// Blob — round pink blob
// ---------------------------------------------------------------------------

fn blob_svg(dt: DemonType) -> String {
    let fill = dt.hex();
    let shadow = dt.shadow_hex();
    let highlight = dt.highlight_hex();
    let cx = 100.0_f32;
    let cy = 110.0_f32;

    let mut s = String::with_capacity(2048);

    // Defs
    s.push_str(&format!(
        r##"<defs>
  <radialGradient id="blob-grad" cx="40%" cy="35%" r="60%">
    <stop offset="0%" stop-color="{highlight}"/>
    <stop offset="60%" stop-color="{fill}"/>
    <stop offset="100%" stop-color="{shadow}"/>
  </radialGradient>
</defs>
"##
    ));

    // Horns (behind body — base overlaps the ellipse top)
    s.push_str(&stubby_horns(cx, 56.0, 25.0, shadow));

    // Body — big round blob
    s.push_str(&format!(
        r##"  <ellipse cx="{cx}" cy="{cy}" rx="70" ry="65" fill="url(#blob-grad)" stroke="{shadow}" stroke-width="2"/>
"##
    ));

    // Cheek blush
    s.push_str(
        r##"  <ellipse cx="65" cy="125" rx="14" ry="9" fill="#FF8888" opacity="0.4"/>
  <ellipse cx="135" cy="125" rx="14" ry="9" fill="#FF8888" opacity="0.4"/>
"##,
    );

    // Eyes
    s.push_str(&googly_eyes(cx, 100.0, 22.0, 13.0));

    // Goofy smile — wide, slightly crooked
    s.push_str(
        r##"  <path d="M 75,132 Q 100,155 125,132" fill="none" stroke="#333" stroke-width="2.5" stroke-linecap="round"/>
"##,
    );

    // Tiny feet
    s.push_str(&format!(
        r##"  <ellipse cx="78" cy="175" rx="14" ry="8" fill="{shadow}"/>
  <ellipse cx="122" cy="175" rx="14" ry="8" fill="{shadow}"/>
"##
    ));

    s
}

// ---------------------------------------------------------------------------
// Bat — bat-winged critter
// ---------------------------------------------------------------------------

fn bat_svg(dt: DemonType) -> String {
    let fill = dt.hex();
    let shadow = dt.shadow_hex();
    let highlight = dt.highlight_hex();
    let cx = 100.0_f32;
    let cy = 110.0_f32;

    let mut s = String::with_capacity(2048);

    // Defs
    s.push_str(&format!(
        r##"<defs>
  <radialGradient id="bat-grad" cx="45%" cy="35%" r="60%">
    <stop offset="0%" stop-color="{highlight}"/>
    <stop offset="50%" stop-color="{fill}"/>
    <stop offset="100%" stop-color="{shadow}"/>
  </radialGradient>
</defs>
"##
    ));

    // Wings (behind body — bases overlap body sides)
    s.push_str(&format!(
        r##"  <path d="M 50,100 Q 10,70 15,105 Q 18,120 30,115 Q 35,130 55,115 Z" fill="{shadow}" stroke="#333" stroke-width="1.5"/>
  <path d="M 150,100 Q 190,70 185,105 Q 182,120 170,115 Q 165,130 145,115 Z" fill="{shadow}" stroke="#333" stroke-width="1.5"/>
"##
    ));

    // Pointy ears (bases overlap top of body ellipse)
    s.push_str(&format!(
        r##"  <path d="M 65,78 L 50,30 L 80,72 Z" fill="{fill}" stroke="#333" stroke-width="1.5"/>
  <path d="M 135,78 L 150,30 L 120,72 Z" fill="{fill}" stroke="#333" stroke-width="1.5"/>
"##
    ));

    // Body — slightly oval
    s.push_str(&format!(
        r##"  <ellipse cx="{cx}" cy="{cy}" rx="55" ry="55" fill="url(#bat-grad)" stroke="{shadow}" stroke-width="2"/>
"##
    ));

    // Eyes — extra big for cuteness
    s.push_str(&googly_eyes(cx, 100.0, 20.0, 15.0));

    // Little fangs (tiny, not scary — like a kitten)
    s.push_str(
        r##"  <path d="M 90,130 Q 100,148 110,130" fill="none" stroke="#333" stroke-width="2" stroke-linecap="round"/>
  <line x1="92" y1="131" x2="94" y2="138" stroke="white" stroke-width="2" stroke-linecap="round"/>
  <line x1="108" y1="131" x2="106" y2="138" stroke="white" stroke-width="2" stroke-linecap="round"/>
"##,
    );

    // Tiny feet
    s.push_str(&format!(
        r##"  <ellipse cx="82" cy="165" rx="12" ry="7" fill="{shadow}"/>
  <ellipse cx="118" cy="165" rx="12" ry="7" fill="{shadow}"/>
"##
    ));

    s
}

// ---------------------------------------------------------------------------
// Imp — lanky triangle body
// ---------------------------------------------------------------------------

fn imp_svg(dt: DemonType) -> String {
    let fill = dt.hex();
    let shadow = dt.shadow_hex();
    let highlight = dt.highlight_hex();
    let cx = 100.0_f32;

    let mut s = String::with_capacity(2048);

    // Defs
    s.push_str(&format!(
        r##"<defs>
  <linearGradient id="imp-grad" x1="0" y1="0" x2="0" y2="1">
    <stop offset="0%" stop-color="{highlight}"/>
    <stop offset="50%" stop-color="{fill}"/>
    <stop offset="100%" stop-color="{shadow}"/>
  </linearGradient>
</defs>
"##
    ));

    // Horns — curvy (bases overlap the top of the pear body)
    s.push_str(&format!(
        r##"  <path d="M 88,55 Q 70,20 65,15 Q 78,30 92,50" fill="{shadow}" stroke="#333" stroke-width="1.5"/>
  <path d="M 112,55 Q 130,20 135,15 Q 122,30 108,50" fill="{shadow}" stroke="#333" stroke-width="1.5"/>
"##
    ));

    // Body — rounded triangle / pear shape
    s.push_str(&format!(
        r##"  <path d="M {cx},45 Q 145,80 140,140 Q 135,180 100,180 Q 65,180 60,140 Q 55,80 {cx},45 Z"
    fill="url(#imp-grad)" stroke="{shadow}" stroke-width="2"/>
"##
    ));

    // Eyes
    s.push_str(&googly_eyes(cx, 88.0, 18.0, 11.0));

    // Mischievous grin — asymmetric
    s.push_str(
        r##"  <path d="M 78,112 Q 95,128 122,110" fill="none" stroke="#333" stroke-width="2.5" stroke-linecap="round"/>
"##,
    );

    // Tiny tail (starts from left body edge at hip level)
    s.push_str(&format!(
        r##"  <path d="M 66,165 Q 45,160 38,148 Q 35,140 40,136" fill="none" stroke="{shadow}" stroke-width="3" stroke-linecap="round"/>
  <circle cx="40" cy="135" r="4" fill="{shadow}"/>
"##
    ));

    // Tiny feet
    s.push_str(&format!(
        r##"  <ellipse cx="82" cy="182" rx="12" ry="6" fill="{shadow}"/>
  <ellipse cx="118" cy="182" rx="12" ry="6" fill="{shadow}"/>
"##
    ));

    s
}

// ---------------------------------------------------------------------------
// Puff — fluffy cloud shape
// ---------------------------------------------------------------------------

fn puff_svg(dt: DemonType) -> String {
    let fill = dt.hex();
    let shadow = dt.shadow_hex();
    let highlight = dt.highlight_hex();
    let cx = 100.0_f32;
    let cy = 110.0_f32;

    let mut s = String::with_capacity(2048);

    // Defs
    s.push_str(&format!(
        r##"<defs>
  <radialGradient id="puff-grad" cx="45%" cy="30%" r="65%">
    <stop offset="0%" stop-color="{highlight}"/>
    <stop offset="50%" stop-color="{fill}"/>
    <stop offset="100%" stop-color="{shadow}"/>
  </radialGradient>
</defs>
"##
    ));

    // Star horns — tiny five-pointed stars (bottom points overlap cloud top)
    s.push_str(&format!(
        r##"  <polygon points="70,45 73,55 83,55 75,61 78,71 70,65 62,71 65,61 57,55 67,55" fill="{highlight}" stroke="#333" stroke-width="1"/>
  <polygon points="130,45 133,55 143,55 135,61 138,71 130,65 122,71 125,61 117,55 127,55" fill="{highlight}" stroke="#333" stroke-width="1"/>
"##
    ));

    // Cloud body — overlapping circles for fluffy look
    s.push_str(&format!(
        r##"  <circle cx="70" cy="{cy}" r="40" fill="url(#puff-grad)"/>
  <circle cx="130" cy="{cy}" r="40" fill="url(#puff-grad)"/>
  <circle cx="{cx}" cy="95" r="42" fill="url(#puff-grad)"/>
  <circle cx="{cx}" cy="125" r="40" fill="url(#puff-grad)"/>
  <circle cx="80" cy="90" r="30" fill="url(#puff-grad)"/>
  <circle cx="120" cy="90" r="30" fill="url(#puff-grad)"/>
"##
    ));

    // Eyes — on the main centre circle
    s.push_str(&googly_eyes(cx, 95.0, 20.0, 12.0));

    // Happy little "o" mouth
    s.push_str(
        r##"  <ellipse cx="100" cy="120" rx="8" ry="10" fill="#333"/>
  <ellipse cx="100" cy="118" rx="5" ry="6" fill="#555"/>
"##,
    );

    // Cheek sparkles (little stars)
    s.push_str(
        r##"  <text x="58" y="115" font-size="12" fill="#FFD700" opacity="0.7">&#9733;</text>
  <text x="132" y="115" font-size="12" fill="#FFD700" opacity="0.7">&#9733;</text>
"##,
    );

    s
}
