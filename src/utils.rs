// use web_sys::console::log_1;
use std::cmp;
use crate::game::{
    Coordinate,
    StatusManager,
};
use cfg_if::cfg_if;

cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        pub use self::console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook() {}
    }
}

pub fn coordinate_lerp(start: Coordinate, end: Coordinate, ratio: f64) -> Coordinate {
    if ratio == 0f64 {
        return start;
    }
    if ratio == 1f64 {
        return end;
    }
    let dx = end.x() - start.x();
    let dy = end.y() - start.y();
    Coordinate(start.x() + ratio * dx, start.y() + ratio * dy)
}

pub fn check_collision(status_a: &StatusManager, status_b: &StatusManager) -> bool {
    let (ax1, ay1) = (status_a.coordinate.x(), status_a.coordinate.y());
    let (ax2, ay2) = (ax1 + status_a.width, ay1 + status_a.height);
    let (bx1, by1) = (status_b.coordinate.x(), status_b.coordinate.y());
    let (bx2, by2) = (bx1 + status_b.width, by1 + status_b.height);
    let is_x_collide = cmp::max(ax1 as usize, bx1 as usize) <= cmp::min(ax2 as usize, bx2 as usize);
    let is_y_collide = cmp::max(ay1 as usize, by1 as usize) <= cmp::min(ay2 as usize, by2 as usize);
    is_x_collide && is_y_collide
}

pub fn uuid() -> String {
    let mut result = String::from("");
    let chars = vec!['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z','0','1','2','3','4','5','6','7','8','9'];
    for _ in 0..16 {
        let idx = (js_sys::Math::random() * chars.len() as f64) as usize;
        result.push(chars[idx]);
    }
    result
}