use num::Zero;

use alga::linear::Translation;
use na::{self, Real, Unit};
use shape::{self, SupportMap};
use query::algorithms::{CSOPoint, gjk, gjk::GJKResult};
use query::algorithms::{Simplex, VoronoiSimplex};
use query::ClosestPoints;
use math::{Isometry, Point, Vector};

/// Closest points between support-mapped shapes (`Cuboid`, `ConvexHull`, etc.)
pub fn support_map_against_support_map<N, G1: ?Sized, G2: ?Sized>(
    m1: &Isometry<N>,
    g1: &G1,
    m2: &Isometry<N>,
    g2: &G2,
    prediction: N,
) -> ClosestPoints<N>
where
    N: Real,
    G1: SupportMap<N>,
    G2: SupportMap<N>,
{
    match support_map_against_support_map_with_params(
        m1,
        g1,
        m2,
        g2,
        prediction,
        &mut VoronoiSimplex::new(),
        None,
    ) {
        GJKResult::ClosestPoints(pt1, pt2, _) => ClosestPoints::WithinMargin(pt1, pt2),
        GJKResult::NoIntersection(_) => ClosestPoints::Disjoint,
        GJKResult::Intersection => ClosestPoints::Intersecting,
        GJKResult::Proximity(_) => unreachable!(),
    }
}

/// Closest points between support-mapped shapes (`Cuboid`, `ConvexHull`, etc.)
///
/// This allows a more fine grained control other the underlying GJK algorigtm.
pub fn support_map_against_support_map_with_params<N, S, G1: ?Sized, G2: ?Sized>(
    m1: &Isometry<N>,
    g1: &G1,
    m2: &Isometry<N>,
    g2: &G2,
    prediction: N,
    simplex: &mut S,
    init_dir: Option<Vector<N>>,
) -> GJKResult<N>
where
    N: Real,
    S: Simplex<N>,
    G1: SupportMap<N>,
    G2: SupportMap<N>,
{
    let dir = match init_dir {
        // FIXME: or m2.translation - m1.translation ?
        None => m1.translation.vector - m2.translation.vector,
        Some(dir) => dir,
    };

    if let Some(dir) = Unit::try_new(dir, N::default_epsilon()) {
        simplex.reset(CSOPoint::from_shapes(m1, g1, m2, g2, &dir));
    } else {
        simplex.reset(CSOPoint::from_shapes(m1, g1, m2, g2, &Vector::x_axis()));
    }

    gjk::closest_points(m1, g1, m2, g2, prediction, true, simplex)
}
