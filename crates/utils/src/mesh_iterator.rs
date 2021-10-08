use crate::maths::{One, Zero};
use glm::{Scalar, TVec3};
use std::ops::{AddAssign, Neg};

/// An iterator that iterates all voxels in a box with the lower corner at origin.
/// Iterates the x-dimension last.
/// The upper bound is exclusive.
pub struct MeshIterator<T>
where
    T: Scalar + PartialOrd + AddAssign + Neg + Zero + One,
{
    cur_loc: TVec3<T>,
    max_loc: TVec3<T>,
}

impl<T> MeshIterator<T>
where
    T: Scalar + PartialOrd + AddAssign + Neg<Output = T> + Zero + One,
{
    /// Creates a MeshIterator for the box with inclusive lower corner at origin and exclusive upper corner at `max_loc`.
    ///
    /// # Panics
    ///
    /// Panics if any coordinates of the given vector are non-positive.
    /// Zero coordinates are not allowed.
    /// For some purposes you can use any value below one instead.
    pub fn create(max_loc: TVec3<T>) -> MeshIterator<T> {
        for i in 0..3 {
            if max_loc[i] <= T::zero() {
                panic!(
                    "Argument must have all positive coordinates. Coordinate {} is {:?}",
                    i, max_loc[i]
                );
            }
        }
        MeshIterator {
            cur_loc: TVec3::new(T::zero(), T::zero(), -T::one()),
            max_loc,
        }
    }
}

impl<T> Iterator for MeshIterator<T>
where
    T: Scalar + PartialOrd + AddAssign + Neg + Zero + One,
{
    type Item = TVec3<T>;

    fn next(&mut self) -> Option<TVec3<T>> {
        for i in (0..3).rev() {
            self.cur_loc[i] += T::one();
            if self.cur_loc[i] < self.max_loc[i] {
                return Some(self.cur_loc.clone());
            }
            self.cur_loc[i] = T::zero();
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use glm::{IVec3, Vec3};

    #[test]
    fn simple_f32() {
        let mut iterator = MeshIterator::create(Vec3::new(5., 5., 5.));
        assert_eq!(iterator.next(), Some(Vec3::new(0., 0., 0.)));
        for _ in 1..7 {
            iterator.next().unwrap();
        }
        assert_eq!(iterator.next(), Some(Vec3::new(0., 1., 2.)));
        for _ in 8..32 {
            iterator.next().unwrap();
        }
        assert_eq!(iterator.next(), Some(Vec3::new(1., 1., 2.)));
        for _ in 33..123 {
            iterator.next().unwrap();
        }
        assert_eq!(iterator.next(), Some(Vec3::new(4., 4., 3.)));
        assert_eq!(iterator.next(), Some(Vec3::new(4., 4., 4.)));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn simple_i32() {
        let mut iterator = MeshIterator::create(IVec3::new(5, 5, 5));
        assert_eq!(iterator.next(), Some(IVec3::new(0, 0, 0)));
        for _ in 1..7 {
            iterator.next().unwrap();
        }
        assert_eq!(iterator.next(), Some(IVec3::new(0, 1, 2)));
        for _ in 8..32 {
            iterator.next().unwrap();
        }
        assert_eq!(iterator.next(), Some(IVec3::new(1, 1, 2)));
        for _ in 33..123 {
            iterator.next().unwrap();
        }
        assert_eq!(iterator.next(), Some(IVec3::new(4, 4, 3)));
        assert_eq!(iterator.next(), Some(IVec3::new(4, 4, 4)));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    #[should_panic]
    fn negative_vec() {
        MeshIterator::create(Vec3::new(4., -5., 2.));
    }

    #[test]
    #[should_panic]
    fn zero_coordinate() {
        MeshIterator::create(Vec3::new(0., 5., 2.));
    }
}
