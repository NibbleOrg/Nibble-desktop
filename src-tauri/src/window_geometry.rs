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
