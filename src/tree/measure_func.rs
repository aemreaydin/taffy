//! Measure function type and trait definitions

use crate::geometry::Size;
use crate::style::AvailableSpace;
#[cfg(any(feature = "std", feature = "alloc"))]
use crate::util::sys::Box;

/// A function type that can be used in a [`MeasureFunc`]
///
/// This trait is automatically implemented for all types (including closures) that define a function with the appropriate type signature.
pub trait Measurable: Send + Sync {
    /// Measure node
    fn measure(&self, known_dimensions: Size<Option<f32>>, available_space: Size<AvailableSpace>) -> Size<f32>;
}

/// A function that can be used to compute the intrinsic size of a node
pub enum MeasureFunc {
    /// Stores an unboxed function
    Raw(fn(Size<Option<f32>>, Size<AvailableSpace>) -> Size<f32>),

    /// Stores a boxed function
    #[cfg(any(feature = "std", feature = "alloc"))]
    Boxed(Box<dyn Measurable>),
}

impl Measurable for MeasureFunc {
    /// Call the measure function to measure to the node
    #[inline(always)]
    fn measure(&self, known_dimensions: Size<Option<f32>>, available_space: Size<AvailableSpace>) -> Size<f32> {
        match self {
            Self::Raw(measure) => measure(known_dimensions, available_space),
            #[cfg(any(feature = "std", feature = "alloc"))]
            Self::Boxed(measurable) => measurable.measure(known_dimensions, available_space),
        }
    }
}

#[cfg(test)]
mod test {
    use super::MeasureFunc;

    #[test]
    fn measure_func_is_send_and_sync() {
        fn is_send_and_sync<T: Send + Sync>() {}
        is_send_and_sync::<MeasureFunc>();
    }
}
