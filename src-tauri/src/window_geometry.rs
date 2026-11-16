//! Shared physical-pixel constraints; independent of native window APIs.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

pub fn fit(area: Rect, old: Option<Rect>, width: u32, height: u32, gap: i32) -> Rect {
    let width = width.min(area.width);
    let height = height.min(area.height);
    let max_x = area.x + (area.width - width) as i32;
    let max_y = area.y + (area.height - height) as i32;
    let x = old.map_or(area.x + ((area.width - width) / 2) as i32, |r| r.x);
    let y = old.map_or(area.y + gap, |r| r.y);
    Rect {
        x: x.clamp(area.x, max_x),
        y: y.clamp(area.y, max_y),
        width,
        height,
    }
}

/// Center an outer window frame with equal physical-pixel margins and ~80% area.
pub fn workbench_frame(area: Rect) -> Rect {
    let width = f64::from(area.width);
    let height = f64::from(area.height);
    // (w - 2m)(h - 2m) = 0.8wh, using the stable form of the smaller root.
    let margin = (0.2 * width * height
        / (width + height + ((width + height).powi(2) - 0.8 * width * height).sqrt()))
    .round() as u32;
    Rect {
        x: area.x + margin as i32,
        y: area.y + margin as i32,
        width: area.width.saturating_sub(2 * margin),
        height: area.height.saturating_sub(2 * margin),
    }
}

/// Convert a constrained top-left physical rect to AppKit's bottom-left points.
/// Anchor the conversion in the live window so virtual desktop origins and
/// the primary screen's height need not be guessed.
pub fn native_frame(
    old: (f64, f64, f64, f64),
    old_position: (i32, i32),
    target: Rect,
    scale: f64,
) -> (f64, f64, f64, f64) {
    let width = f64::from(target.width) / scale;
    let height = f64::from(target.height) / scale;
    (
        old.0 + (f64::from(target.x) - f64::from(old_position.0)) / scale,
        old.1 + old.3 - (f64::from(target.y) - f64::from(old_position.1)) / scale - height,
        width,
        height,
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn resize_keeps_top_on_negative_origin_retina_monitor() {
        let area = Rect {
            x: -2880,
            y: -180,
            width: 2880,
            height: 1760,
        };
        let old = Rect {
            x: -2090,
            y: -132,
            width: 1300,
            height: 528,
        };
        assert_eq!(
            fit(area, Some(old), 1300, 1200, 48),
            Rect {
                height: 1200,
                ..old
            }
        );
    }
    #[test]
    fn disconnected_window_returns_inside_small_work_area() {
        let area = Rect {
            x: 0,
            y: 25,
            width: 600,
            height: 500,
        };
        let old = Rect {
            x: -2200,
            y: -100,
            width: 650,
            height: 700,
        };
        assert_eq!(
            fit(area, Some(old), 650, 700, 24),
            Rect {
                x: 0,
                y: 25,
                width: 600,
                height: 500
            }
        );
    }
    #[test]
    fn open_centers_on_pointer_monitor_and_reserves_top_gap() {
        let area = Rect {
            x: 1920,
            y: 40,
            width: 1920,
            height: 1000,
        };
        assert_eq!(
            fit(area, None, 650, 264, 24),
            Rect {
                x: 2555,
                y: 64,
                width: 650,
                height: 264
            }
        );
    }
}

#[cfg(test)]
mod native_frame_tests {
    use super::*;
    #[test]
    fn places_frame_before_show_with_retina_pixel_to_point_conversion() {
        let result = native_frame(
            (1179.0, 1048.0, 650.0, 264.0),
            (2358, 880),
            Rect {
                x: 2358,
                y: 228,
                width: 1300,
                height: 528,
            },
            2.0,
        );
        assert_eq!(result, (1179.0, 1374.0, 650.0, 264.0));
    }
    #[test]
    fn resize_keeps_native_top_edge_on_negative_origin_display() {
        let result = native_frame(
            (-1200.0, 100.0, 650.0, 264.0),
            (-2400, -180),
            Rect {
                x: -2400,
                y: -180,
                width: 1300,
                height: 1240,
            },
            2.0,
        );
        assert_eq!(result, (-1200.0, -256.0, 650.0, 620.0));
        assert_eq!(result.1 + result.3, 364.0);
    }
}

#[cfg(test)]
mod workbench_tests {
    use super::*;
    #[test]
    fn equal_margins_cover_four_fifths_on_wide_and_portrait_displays() {
        for (width, height) in [(1920, 1040), (1080, 1880), (2880, 1760), (600, 400)] {
            let area = Rect {
                x: -2880,
                y: -180,
                width,
                height,
            };
            let frame = workbench_frame(area);
            let left = frame.x - area.x;
            let top = frame.y - area.y;
            assert_eq!(left, top);
            assert_eq!(left as u32, area.width - frame.width - left as u32);
            assert_eq!(top as u32, area.height - frame.height - top as u32);
            let ratio = f64::from(frame.width) * f64::from(frame.height)
                / (f64::from(width) * f64::from(height));
            assert!((ratio - 0.8).abs() < 0.006, "{ratio}");
        }
    }
}
