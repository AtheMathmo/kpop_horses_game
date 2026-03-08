//! CLI tool to generate face SVGs and PNGs for inspection.
//!
//! Usage:
//!   cargo run -p face_gen              # Generate a default sample + all component variants
//!   cargo run -p face_gen -- --all     # Generate all 3125 combinations (5^5)

use std::fs;
use std::path::Path;

use face_gen::{
    ALL_EYE_STYLES, ALL_FACE_SHAPES, ALL_HAIR_STYLES, ALL_MOUTH_STYLES, ALL_SKIN_TONES, CANVAS_H,
    CANVAS_W, FaceConfig, generate_component_svg, generate_face_svg, rasterize_svg_to_png,
};

const OUTPUT_DIR: &str = "output";
const PNG_SCALE: u32 = 2; // 2x for crisp rendering

fn main() {
    let all_combos = std::env::args().any(|a| a == "--all");

    let base = Path::new(OUTPUT_DIR);
    fs::create_dir_all(base).expect("create output dir");

    if all_combos {
        generate_all_combinations(base);
    } else {
        generate_samples(base);
    }

    println!("Done! Output written to {OUTPUT_DIR}/");
}

/// Generate a curated set of samples: one per variant of each component,
/// plus a few complete face combos.
fn generate_samples(base: &Path) {
    let default_config = FaceConfig {
        face: face_gen::FaceShape::Oval,
        eyes: face_gen::EyeStyle::Round,
        hair: face_gen::HairStyle::Long,
        mouth: face_gen::MouthStyle::Smile,
        skin: face_gen::SkinTone::Light,
    };

    // --- Complete face samples with different skin tones ---
    let samples_dir = base.join("samples");
    fs::create_dir_all(&samples_dir).expect("create samples dir");

    let sample_configs = [
        ("default", default_config),
        (
            "cat_spiky",
            FaceConfig {
                face: face_gen::FaceShape::Heart,
                eyes: face_gen::EyeStyle::Cat,
                hair: face_gen::HairStyle::Spiky,
                mouth: face_gen::MouthStyle::Smirk,
                skin: face_gen::SkinTone::Pale,
            },
        ),
        (
            "wide_bangs",
            FaceConfig {
                face: face_gen::FaceShape::Round,
                eyes: face_gen::EyeStyle::Wide,
                hair: face_gen::HairStyle::Bangs,
                mouth: face_gen::MouthStyle::Open,
                skin: face_gen::SkinTone::Medium,
            },
        ),
        (
            "narrow_ponytail",
            FaceConfig {
                face: face_gen::FaceShape::Long,
                eyes: face_gen::EyeStyle::Narrow,
                hair: face_gen::HairStyle::Ponytail,
                mouth: face_gen::MouthStyle::Neutral,
                skin: face_gen::SkinTone::Dark,
            },
        ),
        (
            "almond_short",
            FaceConfig {
                face: face_gen::FaceShape::Square,
                eyes: face_gen::EyeStyle::Almond,
                hair: face_gen::HairStyle::Short,
                mouth: face_gen::MouthStyle::Pout,
                skin: face_gen::SkinTone::Tan,
            },
        ),
    ];

    for (name, config) in &sample_configs {
        write_face(&samples_dir, name, config);
        println!("  sample: {name}");
    }

    // --- Per-component variant galleries ---
    let components: &[(&str, &[&str])] = &[
        ("face", &["oval", "round", "square", "heart", "long"]),
        ("eyes", &["round", "almond", "cat", "wide", "narrow"]),
        ("hair", &["short", "long", "ponytail", "spiky", "bangs"]),
        ("mouth", &["smile", "neutral", "pout", "open", "smirk"]),
    ];

    for (component, variants) in components {
        let comp_dir = base.join(component);
        fs::create_dir_all(&comp_dir).expect("create component dir");

        for (i, variant_name) in variants.iter().enumerate() {
            let config = match *component {
                "face" => FaceConfig {
                    face: ALL_FACE_SHAPES[i],
                    ..default_config
                },
                "eyes" => FaceConfig {
                    eyes: ALL_EYE_STYLES[i],
                    ..default_config
                },
                "hair" => FaceConfig {
                    hair: ALL_HAIR_STYLES[i],
                    ..default_config
                },
                "mouth" => FaceConfig {
                    mouth: ALL_MOUTH_STYLES[i],
                    ..default_config
                },
                _ => unreachable!(),
            };

            let svg = generate_component_svg(&config, component);
            let svg_path = comp_dir.join(format!("{variant_name}.svg"));
            let png_path = comp_dir.join(format!("{variant_name}.png"));

            fs::write(&svg_path, &svg).expect("write SVG");
            if let Some(png) = rasterize_svg_to_png(
                &svg,
                CANVAS_W as u32 * PNG_SCALE,
                CANVAS_H as u32 * PNG_SCALE,
            ) {
                fs::write(&png_path, png).expect("write PNG");
            }
            println!("  {component}/{variant_name}");
        }
    }

    // --- Skin tone gallery ---
    let skin_dir = base.join("skin");
    fs::create_dir_all(&skin_dir).expect("create skin dir");

    for tone in ALL_SKIN_TONES {
        let config = FaceConfig {
            skin: *tone,
            ..default_config
        };
        let name = tone.label();
        write_face(&skin_dir, name, &config);
        println!("  skin/{name}");
    }
}

/// Generate every possible combination.
fn generate_all_combinations(base: &Path) {
    let all_dir = base.join("all");
    fs::create_dir_all(&all_dir).expect("create all dir");

    let mut count = 0;
    for face in ALL_FACE_SHAPES {
        for eyes in ALL_EYE_STYLES {
            for hair in ALL_HAIR_STYLES {
                for mouth in ALL_MOUTH_STYLES {
                    for skin in ALL_SKIN_TONES {
                        let config = FaceConfig {
                            face: *face,
                            eyes: *eyes,
                            hair: *hair,
                            mouth: *mouth,
                            skin: *skin,
                        };
                        let name = format!(
                            "{}_{}_{}_{}_{}",
                            face.label(),
                            eyes.label(),
                            hair.label(),
                            mouth.label(),
                            skin.label()
                        );
                        write_face(&all_dir, &name, &config);
                        count += 1;
                    }
                }
            }
        }
    }
    println!("Generated {count} face combinations.");
}

fn write_face(dir: &Path, name: &str, config: &FaceConfig) {
    let svg = generate_face_svg(config);
    let svg_path = dir.join(format!("{name}.svg"));
    let png_path = dir.join(format!("{name}.png"));

    fs::write(&svg_path, &svg).expect("write SVG");
    if let Some(png) = rasterize_svg_to_png(
        &svg,
        CANVAS_W as u32 * PNG_SCALE,
        CANVAS_H as u32 * PNG_SCALE,
    ) {
        fs::write(&png_path, png).expect("write PNG");
    }
}
