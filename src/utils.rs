// use web_sys::console::log_1;
// use crate::game::constants::TOTAL_TILES;
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
    // Exclude corner-only collision
    if (ax1 == bx2 && ay1 == by2) || (ax2 == bx1 && ay2 == by1) || (ax2 == bx1 && by2 == ay1) || (ax1 == bx2 && ay2 == by1) {
        return false;
    }
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

pub fn get_object_coverage(parent_status: &StatusManager, object_status: &StatusManager) -> f64 {
    let (ax1, ay1) = (parent_status.coordinate.x(), parent_status.coordinate.y());
    let (ax2, ay2) = (ax1 + parent_status.width, ay1 + parent_status.height);
    let (bx1, by1) = (object_status.coordinate.x(), object_status.coordinate.y());
    let (bx2, by2) = (bx1 + object_status.width, by1 + object_status.height);
    let x_diff = cmp::min(ax2 as usize, bx2 as usize) as f64 - cmp::max(ax1 as usize, bx1 as usize) as f64;
    let y_diff = cmp::min(ay2 as usize, by2 as usize) as f64 - cmp::max(ay1 as usize, by1 as usize) as f64;
    if x_diff < 0f64 || y_diff < 0f64 {
        0f64
    } else {
        (x_diff * y_diff) / ((bx2 - bx1) * (by2 - by1))
    }
}

// pub fn terrain_generator() -> Vec<&'static str> {
//     let mut result = vec![];
//     for _ in 0..TOTAL_TILES {
//         let idx = js_sys::Math::random();
//         if idx < 0.4 {
//             result.push("G3");
//         } else if idx < 0.8 {
//             result.push("G4");
//         } else if idx < 0.9 {
//             result.push("G2");
//         } else {
//             result.push("G1");
//         }
//     }
//     result
// }