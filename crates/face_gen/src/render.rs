//! SVG → PNG rasterization using resvg.

use resvg::tiny_skia;
use resvg::usvg;

/// Rasterize an SVG string to RGBA pixel data at the given dimensions.
/// Returns `(width, height, rgba_pixels)`.
pub fn rasterize_svg(svg_str: &str, width: u32, height: u32) -> Option<(u32, u32, Vec<u8>)> {
    let options = usvg::Options::default();
    let tree = usvg::Tree::from_str(svg_str, &options).ok()?;

    let mut pixmap = tiny_skia::Pixmap::new(width, height)?;

    // Scale the SVG to fit the target dimensions
    let svg_size = tree.size();
    let sx = width as f32 / svg_size.width();
    let sy = height as f32 / svg_size.height();

    resvg::render(
        &tree,
        tiny_skia::Transform::from_scale(sx, sy),
        &mut pixmap.as_mut(),
    );

    Some((width, height, pixmap.data().to_vec()))
}

/// Rasterize an SVG string and encode as PNG bytes.
pub fn rasterize_svg_to_png(svg_str: &str, width: u32, height: u32) -> Option<Vec<u8>> {
    let options = usvg::Options::default();
    let tree = usvg::Tree::from_str(svg_str, &options).ok()?;

    let mut pixmap = tiny_skia::Pixmap::new(width, height)?;

    let svg_size = tree.size();
    let sx = width as f32 / svg_size.width();
    let sy = height as f32 / svg_size.height();

    resvg::render(
        &tree,
        tiny_skia::Transform::from_scale(sx, sy),
        &mut pixmap.as_mut(),
    );

    pixmap.encode_png().ok()
}
