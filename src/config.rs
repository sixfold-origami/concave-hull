use crate::Real;

/// Configuration options when searching for candidate points to add to the concave hull
///
/// If you are unsure of how to set this, [`PointSearchConfig::default()`] provides reasonable defaults.
/// You can also use the constructors of this type for other common configuration options.
/// Lastly, you can set the values manually for fine-tuning.
///
/// When constructing the concave hull, the algorithm attempts to split long edges by inserting points in the middle.
/// Points are considered to be "better" if the angle that the new edges make with the existing edge is smaller.
/// (There are actually two angles, one for each new edge, but the larger angle is used.)
///
/// Points are searched by first checking points in a square window around the existing edge,
/// and then increasing the size of that window until a "suitable" point is found.
/// This config object can fine-tune how the window is grown, and what counts as "suitable".
///
/// In general, there is a tradeoff between speed and quality.
/// If you check every possible point, then you can guarantee that the point you add will have the minimum angle.
/// But checking every point is slow.
/// If you only check points which are very close by, you can save time, but you might miss more optional solutions.
#[derive(Debug, Clone, Copy)]
pub struct PointSearchConfig {
    /// How much the window is grown at each iteration?
    ///
    /// Ranges between `[0,1]`.
    /// The window will be loosened (in all extents) by this fraction of the longest extent of the point cloud.
    /// So, a value of `0.05` will result in a window which grows by 1/20
    /// of the length of the longest size of the point cloud's AABB.
    pub window_step_factor: Real,

    /// What fraction of the total point cloud is searched before terminating early?
    ///
    /// Ranges between `[0,1]`.
    /// The window will keep growing and checking points until either a suitable point is found,
    /// or the window grows to be larger than this fraction of the logest extent of the point cloud.
    /// So, a value of `0.6` will result in a window which will grow until it covers 60% of the
    /// length of the longest side of the point cloud's AABB.
    ///
    /// At this point, the iterations will terminate, and the best point found will be used.
    ///
    /// Note: if an edge is longer than this threshold, and there are no points inside its AABB,
    /// then it will not be split.
    /// Be careful when setting this low: "cresent shaped" point clouds with long edges in the convex hull
    /// will be completely skipped if the max size is less than the length of the cresent's interior.
    pub window_max_size_factor: Real,

    /// Size threshold for what angle is considered "suitable"
    ///
    /// Ranges between `[0,pi]`
    /// After a candidate point is found, the angle between its new edges and the original edge is calculated.
    /// (And the maximum of these two angles is used.)
    /// If that angle is larger than or equal to this threshold, then we will keep searching for a better point.
    /// If it is smaller than this threshold, then we will stop searching and use this point.
    /// So, a value of `pi/2` means that only angles which are acute are considered "suitable".
    ///
    /// Note: this is not a hard threshold.
    /// If the only available candidate point is above this angle threshold, it will still be used.
    /// This only determines whether we should keep searching or terminate early.
    pub bad_angle_threshold: Real,
}

impl Default for PointSearchConfig {
    fn default() -> Self {
        Self {
            window_step_factor: 0.05,    // Add 1/20 of the point cloud each step
            window_max_size_factor: 0.6, // Search at most 60% of the total point cloud extents
            bad_angle_threshold: crate::FRAC_PI_2, // Right angle
        }
    }
}

impl PointSearchConfig {
    /// Configuration preset that searches the whole point cloud every time
    ///
    /// This is very slow!
    /// But it will always return maximally optimal results.
    pub const SEARCH_ALL_POINTS: Self = Self {
        window_step_factor: 1.,     // Just check the whole thing immediately
        window_max_size_factor: 1., // Keep going for the whole point cloud
        bad_angle_threshold: 0.,    // Only an angle of zero is perfect, so we check everything
    };

    /// Configuration preset that searches the full extents of the point cloud,
    /// but with small steps to improve performance
    ///
    /// Strikes a decent balance between being fast and being thorough.
    /// Slower than the default settings, but will always split long edges.
    /// If you are having problems with "cresent shaped" point clouds,
    /// this is a good preset to try.
    pub const BALANCED_FULL_EXTENTS: Self = Self {
        window_step_factor: 0.05,
        window_max_size_factor: 1.,
        bad_angle_threshold: crate::FRAC_PI_2,
    };

    /// Configuration preset that only searches a small area around each edge
    ///
    /// This will be fast, but it may miss points or edges in some cases.
    /// In particular, long edges will be skipped because [`Self::window_max_size_factor`] is quite low
    /// It is best suited to point clouds which are very uniform, and roughly "blobby".
    /// Shapes with long, shallow concave regions, such as a crescent moon, will get bad results.
    pub const TIGHT_LOCAL_SEARCH: Self = Self {
        window_step_factor: 0.01,              // Check 1% more each time
        window_max_size_factor: 0.1,           // But only check 10% max
        bad_angle_threshold: crate::FRAC_PI_2, // Acute angles are good
    };
}
