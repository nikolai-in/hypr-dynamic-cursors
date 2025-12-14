/// Renderer utilities for transforming cursor buffers
/// This module provides helper functions for rendering transformed cursors

use crate::CursorTransform;

/// Represents a cursor image buffer
#[derive(Debug, Clone)]
pub struct CursorBuffer {
    pub width: u32,
    pub height: u32,
    pub hotspot_x: i32,
    pub hotspot_y: i32,
    pub data: Vec<u8>, // ARGB8888 format
}

impl CursorBuffer {
    pub fn new(width: u32, height: u32, hotspot_x: i32, hotspot_y: i32) -> Self {
        let size = (width * height * 4) as usize;
        Self {
            width,
            height,
            hotspot_x,
            hotspot_y,
            data: vec![0; size],
        }
    }

    /// Create a transformed version of this cursor buffer
    pub fn transform(&self, transform: &CursorTransform, use_nearest: bool) -> Self {
        let rotation_rad = transform.rotation.to_radians();
        let cos_r = rotation_rad.cos();
        let sin_r = rotation_rad.sin();

        // Calculate bounding box for rotated and scaled cursor
        let corners = [
            (0.0, 0.0),
            (self.width as f64, 0.0),
            (0.0, self.height as f64),
            (self.width as f64, self.height as f64),
        ];

        let mut min_x = f64::MAX;
        let mut max_x = f64::MIN;
        let mut min_y = f64::MAX;
        let mut max_y = f64::MIN;

        for (x, y) in corners {
            let scaled_x = x * transform.scale_x;
            let scaled_y = y * transform.scale_y;
            let rot_x = scaled_x * cos_r - scaled_y * sin_r;
            let rot_y = scaled_x * sin_r + scaled_y * cos_r;

            min_x = min_x.min(rot_x);
            max_x = max_x.max(rot_x);
            min_y = min_y.min(rot_y);
            max_y = max_y.max(rot_y);
        }

        let new_width = (max_x - min_x).ceil() as u32;
        let new_height = (max_y - min_y).ceil() as u32;

        let mut result = CursorBuffer::new(
            new_width,
            new_height,
            self.hotspot_x,
            self.hotspot_y,
        );

        // Center of original image
        let center_x = self.width as f64 / 2.0;
        let center_y = self.height as f64 / 2.0;

        // Center of new image
        let new_center_x = new_width as f64 / 2.0;
        let new_center_y = new_height as f64 / 2.0;

        // Render transformed cursor
        for y in 0..new_height {
            for x in 0..new_width {
                let dx = x as f64 - new_center_x;
                let dy = y as f64 - new_center_y;

                // Inverse transform
                let unrot_x = dx * cos_r + dy * sin_r;
                let unrot_y = -dx * sin_r + dy * cos_r;

                let src_x = (unrot_x / transform.scale_x) + center_x;
                let src_y = (unrot_y / transform.scale_y) + center_y;

                if src_x >= 0.0 && src_x < self.width as f64 && src_y >= 0.0 && src_y < self.height as f64 {
                    let pixel = if use_nearest {
                        self.get_pixel_nearest(src_x, src_y)
                    } else {
                        self.get_pixel_bilinear(src_x, src_y)
                    };

                    result.set_pixel(x, y, pixel);
                }
            }
        }

        result
    }

    fn get_pixel_nearest(&self, x: f64, y: f64) -> [u8; 4] {
        let ix = x.round() as u32;
        let iy = y.round() as u32;

        if ix >= self.width || iy >= self.height {
            return [0, 0, 0, 0];
        }

        let offset = ((iy * self.width + ix) * 4) as usize;
        [
            self.data[offset],
            self.data[offset + 1],
            self.data[offset + 2],
            self.data[offset + 3],
        ]
    }

    fn get_pixel_bilinear(&self, x: f64, y: f64) -> [u8; 4] {
        let x0 = x.floor() as u32;
        let y0 = y.floor() as u32;
        let x1 = (x0 + 1).min(self.width - 1);
        let y1 = (y0 + 1).min(self.height - 1);

        let fx = x - x0 as f64;
        let fy = y - y0 as f64;

        let p00 = self.get_pixel_nearest(x0 as f64, y0 as f64);
        let p01 = self.get_pixel_nearest(x0 as f64, y1 as f64);
        let p10 = self.get_pixel_nearest(x1 as f64, y0 as f64);
        let p11 = self.get_pixel_nearest(x1 as f64, y1 as f64);

        let mut result = [0u8; 4];
        for i in 0..4 {
            let v0 = p00[i] as f64 * (1.0 - fx) + p10[i] as f64 * fx;
            let v1 = p01[i] as f64 * (1.0 - fx) + p11[i] as f64 * fx;
            result[i] = (v0 * (1.0 - fy) + v1 * fy) as u8;
        }

        result
    }

    fn set_pixel(&mut self, x: u32, y: u32, pixel: [u8; 4]) {
        if x >= self.width || y >= self.height {
            return;
        }

        let offset = ((y * self.width + x) * 4) as usize;
        self.data[offset..offset + 4].copy_from_slice(&pixel);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_buffer_creation() {
        let buffer = CursorBuffer::new(32, 32, 16, 16);
        assert_eq!(buffer.width, 32);
        assert_eq!(buffer.height, 32);
        assert_eq!(buffer.data.len(), 32 * 32 * 4);
    }

    #[test]
    fn test_identity_transform() {
        let mut buffer = CursorBuffer::new(32, 32, 16, 16);
        // Set some test data
        buffer.set_pixel(16, 16, [255, 0, 0, 255]);

        let transform = CursorTransform {
            rotation: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            offset_x: 0.0,
            offset_y: 0.0,
        };

        let transformed = buffer.transform(&transform, false);
        assert_eq!(transformed.width, buffer.width);
        assert_eq!(transformed.height, buffer.height);
    }
}
