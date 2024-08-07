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
pub fn polygon_inner_vertices(area: usize, num_boundary_points: usize) -> usize {
    area + 1 - num_boundary_points / 2
}
