use crate::Edge;

/// Checks if the two provided edges are intersecting
///
/// Assumes that distinct indices point to distinct points.
/// i.e. if two indices are different, then the points are in different places.
pub fn edges_intersect(e1: &Edge, e2: &Edge) -> bool {
    // Edges are mirrors of each other
    debug_assert!(!(e1.i == e2.j && e2.i == e1.j), "Found mirrored edges");
    // Only possible if the winding gets messed up
    debug_assert!(
        !(e1.i == e2.i && e1.j != e2.j),
        "Found V edges with shared i"
    );
    debug_assert!(
        !(e1.j == e2.j && e1.i != e2.i),
        "Found V edges with shared j"
    );

    if e1 == e2 {
        // These edges are duplicates
        true
    } else if e1.i == e2.j || e2.i == e1.j {
        // These edges are connected at one endpoint, which doesn't count for our purposes
        // Assuming no degeneracies (see debug asserts), these are not the same, and therefore not intersecting
        false
    } else {
        // https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_points_on_each_line_segment

        let t = ((e1.segment.a.x - e2.segment.a.x) * (e2.segment.a.y - e2.segment.b.y)
            - (e1.segment.a.y - e2.segment.a.y) * (e2.segment.a.x - e2.segment.b.x))
            / ((e1.segment.a.x - e1.segment.b.x) * (e2.segment.a.y - e2.segment.b.y)
                - (e1.segment.a.y - e1.segment.b.y) * (e2.segment.a.x - e2.segment.b.x));

        let u = -1.
            * ((e1.segment.a.x - e1.segment.b.x) * (e1.segment.a.y - e2.segment.a.y)
                - (e1.segment.a.y - e1.segment.b.y) * (e1.segment.a.x - e2.segment.a.x))
            / ((e1.segment.a.x - e1.segment.b.x) * (e2.segment.a.y - e2.segment.b.y)
                - (e1.segment.a.y - e1.segment.b.y) * (e2.segment.a.x - e2.segment.b.x));

        debug_assert!(!t.is_nan());
        debug_assert!(!u.is_nan());

        t >= 0. && t <= 1. && u >= 0. && u <= 1.
    }
}

#[cfg(test)]
mod tests {
    use crate::Point;

    use super::*;

    /// An array of points in a numpad grid, in numpad order
    ///
    /// 7 8 9
    /// 4 5 6
    /// 1 2 3
    /// 0
    const POINTS: [Point; 10] = [
        Point::new(0., 0.),
        Point::new(0., 1.),
        Point::new(1., 1.),
        Point::new(2., 1.),
        Point::new(0., 2.),
        Point::new(1., 2.),
        Point::new(2., 2.),
        Point::new(0., 3.),
        Point::new(1., 3.),
        Point::new(2., 3.),
    ];

    #[test]
    fn duplicate_edge() {
        let e1 = Edge::new(0, 1, &POINTS);
        let e2 = Edge::new(0, 1, &POINTS);

        assert!(edges_intersect(&e1, &e2));
    }

    #[test]
    fn connected_edges_e1_first() {
        let e1 = Edge::new(0, 1, &POINTS);
        let e2 = Edge::new(1, 4, &POINTS);

        assert!(!edges_intersect(&e1, &e2));
    }

    #[test]
    fn connected_edges_e2_first() {
        let e1 = Edge::new(1, 4, &POINTS);
        let e2 = Edge::new(0, 1, &POINTS);

        assert!(!edges_intersect(&e1, &e2));
    }

    #[test]
    fn intersection_plus() {
        let e1 = Edge::new(2, 8, &POINTS);
        let e2 = Edge::new(4, 6, &POINTS);

        assert!(edges_intersect(&e1, &e2));
        assert!(edges_intersect(&e2, &e1));
    }

    #[test]
    fn intersection_x() {
        let e1 = Edge::new(1, 9, &POINTS);
        let e2 = Edge::new(3, 7, &POINTS);

        assert!(edges_intersect(&e1, &e2));
        assert!(edges_intersect(&e2, &e1));
    }

    #[test]
    fn intersection_t() {
        let e1 = Edge::new(1, 7, &POINTS);
        let e2 = Edge::new(4, 6, &POINTS);

        assert!(edges_intersect(&e1, &e2));
        assert!(edges_intersect(&e2, &e1));
    }

    #[test]
    fn parallel_horizontal() {
        let e1 = Edge::new(1, 3, &POINTS);
        let e2 = Edge::new(4, 6, &POINTS);

        assert!(!edges_intersect(&e1, &e2));
        assert!(!edges_intersect(&e2, &e1));
    }

    #[test]
    fn parallel_vertical() {
        let e1 = Edge::new(1, 4, &POINTS);
        let e2 = Edge::new(3, 9, &POINTS);

        assert!(!edges_intersect(&e1, &e2));
        assert!(!edges_intersect(&e2, &e1));
    }
}
