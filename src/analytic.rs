// Area of a polygon with integer coordinates
// Assuming that the polygon is closed and the first point does not repeat
// PS : This function returns twice the area
pub fn polygon_area<T: Copy + num::Integer + std::iter::Sum>(polygon: &[(T, T)]) -> T {
    let n = polygon.len();
    (0..n)
        .map(|i| polygon[i].0 * (polygon[(i + 1) % n].1 - polygon[(i + n - 1) % n].1))
        .sum::<T>()
}

// Pick's formula - Number of interior vertices of a polygon given its area and boundary points
#[must_use]
pub fn polygon_inner_vertices(area: usize, num_boundary_points: usize) -> usize {
    area + 1 - num_boundary_points / 2
}

// Solution of a system of linear equations
// a1x + b1y = c1
// a2x + b2y = c2
pub fn linear_equation<T: Copy + num::Integer>(
    a1: T,
    b1: T,
    c1: T,
    a2: T,
    b2: T,
    c2: T,
) -> Option<(T, T)> {
    if a2 * b1 == b2 * a1 {
        return None;
    }
    let den = a2 * b1 - b2 * a1;
    let num_x = b1 * c2 - b2 * c1;
    let num_y = a2 * c1 - a1 * c2;

    if num_x % den != T::zero() || num_y % den != T::zero() {
        return None;
    }

    Some((num_x / den, num_y / den))
}
