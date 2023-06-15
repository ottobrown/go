use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn star_points(w: usize, h: usize) -> Vec<(usize, usize)> {
    let mut points = Vec::new();

    // if the board has an exact center
    if w % 2 == 1 && h % 2 == 1 {
        // add a center star point
        points.push((w / 2, h / 2));
    }

    if w < 9 || h < 9 {
        return points;
    }

    // 3-3 points
    if w < 13 || h < 13 {
        points.push((2, 2));
        points.push((2, h - 3));
        points.push((w - 3, 2));
        points.push((w - 3, h - 3));

        return points;
    }

    // sides
    if w > 13 {
        if h % 2 == 1 {
            points.push((3, h / 2));
            points.push((w - 4, h / 2));
        }

        if w % 2 == 1 {
            points.push((w / 2, 3));
            points.push((w / 2, h - 4));
        }
    }

    // 4-4 points
    points.push((3, 3));
    points.push((3, h - 4));
    points.push((w - 4, 3));
    points.push((w - 4, h - 4));

    return points;
}
