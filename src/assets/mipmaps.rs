use bevy::{
    prelude::*,
    render::render_resource::{TextureDataOrder, TextureFormat},
};
use std::collections::HashSet;

pub struct MipmapPlugin;

impl Plugin for MipmapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MipmapTargets>()
            .add_systems(Update, generate_mipmaps);
    }
}

/// Tracks which image assets should have mipmaps generated.
#[derive(Resource, Default)]
pub struct MipmapTargets {
    targets: HashSet<AssetId<Image>>,
    processed: HashSet<AssetId<Image>>,
}

impl MipmapTargets {
    pub fn add(&mut self, handle: &Handle<Image>) {
        self.targets.insert(handle.id());
    }
}

fn generate_mipmaps(
    mut asset_events: MessageReader<AssetEvent<Image>>,
    mut images: ResMut<Assets<Image>>,
    mut targets: ResMut<MipmapTargets>,
) {
    let mut to_process = Vec::new();

    for event in asset_events.read() {
        let (AssetEvent::Added { id } | AssetEvent::Modified { id }) = event else {
            continue;
        };
        if targets.targets.contains(id) && !targets.processed.contains(id) {
            to_process.push(*id);
        }
    }

    for id in to_process {
        let Some(image) = images.get_mut(id) else {
            continue;
        };

        if image.texture_descriptor.format != TextureFormat::Rgba8UnormSrgb {
            warn!(
                "Skipping mipmap generation for {:?}: unsupported format {:?}",
                id, image.texture_descriptor.format
            );
            targets.processed.insert(id);
            continue;
        }

        let width = image.texture_descriptor.size.width;
        let height = image.texture_descriptor.size.height;
        let Some(data) = image.data.as_ref() else {
            continue;
        };
        // RGBA8 = 4 bytes per pixel
        let expected = (width * height * 4) as usize;
        if data.len() < expected {
            warn!("Skipping mipmap generation for {:?}: data too small", id);
            targets.processed.insert(id);
            continue;
        }

        let base_data = data[..expected].to_vec();
        let (mip_data, mip_count) = generate_mip_chain(&base_data, width, height);

        image.data = Some(mip_data);
        image.texture_descriptor.mip_level_count = mip_count;
        image.data_order = TextureDataOrder::MipMajor;

        targets.processed.insert(id);
    }
}

/// Generates a full mip chain from base-level RGBA8 pixel data.
///
/// Returns concatenated mip data (base level + all generated levels) and the total level count.
fn generate_mip_chain(base_data: &[u8], width: u32, height: u32) -> (Vec<u8>, u32) {
    let mut result = base_data.to_vec();
    let mut current_w = width;
    let mut current_h = height;
    let mut level_count = 1u32;

    while current_w > 1 || current_h > 1 {
        let next_w = (current_w / 2).max(1);
        let next_h = (current_h / 2).max(1);

        let src_offset = result.len() - (current_w * current_h * 4) as usize;
        let src = &result[src_offset..];
        let downsampled = box_filter_downsample(src, current_w, current_h, next_w, next_h);

        result.extend_from_slice(&downsampled);
        current_w = next_w;
        current_h = next_h;
        level_count += 1;
    }

    (result, level_count)
}

/// Downsamples RGBA8 sRGB pixel data by averaging 2x2 blocks.
///
/// Converts sRGB to linear before averaging, then back to sRGB.
/// Alpha is averaged directly (already linear).
fn box_filter_downsample(src: &[u8], src_w: u32, src_h: u32, dst_w: u32, dst_h: u32) -> Vec<u8> {
    let mut dst = vec![0u8; (dst_w * dst_h * 4) as usize];

    for dy in 0..dst_h {
        for dx in 0..dst_w {
            let sx = dx * 2;
            let sy = dy * 2;

            // Gather the 2x2 block, clamping to source bounds
            let sx1 = (sx + 1).min(src_w - 1);
            let sy1 = (sy + 1).min(src_h - 1);

            let offsets = [
                ((sy * src_w + sx) * 4) as usize,
                ((sy * src_w + sx1) * 4) as usize,
                ((sy1 * src_w + sx) * 4) as usize,
                ((sy1 * src_w + sx1) * 4) as usize,
            ];

            // Average in linear space for RGB
            let mut linear_r = 0.0f32;
            let mut linear_g = 0.0f32;
            let mut linear_b = 0.0f32;
            let mut alpha = 0.0f32;

            for &off in &offsets {
                linear_r += srgb_to_linear(src[off]);
                linear_g += srgb_to_linear(src[off + 1]);
                linear_b += srgb_to_linear(src[off + 2]);
                alpha += src[off + 3] as f32;
            }

            linear_r *= 0.25;
            linear_g *= 0.25;
            linear_b *= 0.25;
            alpha *= 0.25;

            let dst_off = ((dy * dst_w + dx) * 4) as usize;
            dst[dst_off] = linear_to_srgb(linear_r);
            dst[dst_off + 1] = linear_to_srgb(linear_g);
            dst[dst_off + 2] = linear_to_srgb(linear_b);
            dst[dst_off + 3] = (alpha + 0.5) as u8;
        }
    }

    dst
}

/// Convert an sRGB byte value to linear float.
fn srgb_to_linear(s: u8) -> f32 {
    let s = s as f32 / 255.0;
    if s <= 0.04045 {
        s / 12.92
    } else {
        ((s + 0.055) / 1.055).powf(2.4)
    }
}

/// Convert a linear float to sRGB byte value.
fn linear_to_srgb(l: f32) -> u8 {
    let s = if l <= 0.0031308 {
        l * 12.92
    } else {
        1.055 * l.powf(1.0 / 2.4) - 0.055
    };
    (s.clamp(0.0, 1.0) * 255.0 + 0.5) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mip_chain_4x4_produces_correct_levels() {
        // 4x4 solid white image
        let data = vec![255u8; 4 * 4 * 4];
        let (result, levels) = generate_mip_chain(&data, 4, 4);

        assert_eq!(levels, 3); // 4x4 -> 2x2 -> 1x1
        let expected_size = (4 * 4 + 2 * 2 + 1 * 1) * 4;
        assert_eq!(result.len(), expected_size);
    }

    #[test]
    fn mip_chain_non_power_of_two() {
        // 6x4 image
        let data = vec![128u8; 6 * 4 * 4];
        let (result, levels) = generate_mip_chain(&data, 6, 4);

        // 6x4 -> 3x2 -> 1x1
        assert_eq!(levels, 3);
        let expected_size = (6 * 4 + 3 * 2 + 1 * 1) * 4;
        assert_eq!(result.len(), expected_size);
    }

    #[test]
    fn mip_chain_1x1_is_noop() {
        let data = vec![100, 150, 200, 255];
        let (result, levels) = generate_mip_chain(&data, 1, 1);

        assert_eq!(levels, 1);
        assert_eq!(result, data);
    }

    #[test]
    fn box_filter_solid_color_preserves_value() {
        // 4x4 solid red
        let mut src = vec![0u8; 4 * 4 * 4];
        for pixel in src.chunks_exact_mut(4) {
            pixel[0] = 200; // R
            pixel[1] = 0; // G
            pixel[2] = 0; // B
            pixel[3] = 255; // A
        }

        let dst = box_filter_downsample(&src, 4, 4, 2, 2);

        // All pixels in the 2x2 output should be the same solid red
        for pixel in dst.chunks_exact(4) {
            assert_eq!(pixel[0], 200);
            assert_eq!(pixel[1], 0);
            assert_eq!(pixel[2], 0);
            assert_eq!(pixel[3], 255);
        }
    }

    #[test]
    fn srgb_roundtrip() {
        for v in 0..=255u8 {
            let linear = srgb_to_linear(v);
            let back = linear_to_srgb(linear);
            assert!(
                (v as i16 - back as i16).unsigned_abs() <= 1,
                "sRGB roundtrip failed for {v}: got {back}"
            );
        }
    }
}
