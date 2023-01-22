//! This library provides a set of data structures for representing points in two and three dimensional space.
//!
//! The structures are named `Point2D` and `Point3D` respectively.
//! They provide intuitive operations such as addition, subtraction, negation and comparison.

use derive_more::{Add, AddAssign, Deref, DerefMut, From, Into, Neg, Sub, SubAssign};
use num_traits::{Bounded, Num, CheckedMul, CheckedAdd};
use std::array;
use std::fmt::{Display, Formatter};

macro_rules! derive_universal_traits {
    ($struct_def: item) => {
        // Constructors
        #[derive(Copy, Clone, Default)]
        // std::ops – unary
        #[derive(Neg)]
        // std::ops – binary
        #[derive(Add, AddAssign, Sub, SubAssign)]
        // Comparisons
        #[derive(Hash,Eq,PartialEq)]
        // Displayers
        #[derive(Debug)]
        $struct_def
    };
}
macro_rules! impl_hypot_sq{
    ($struct_id: ident, $first_operand: ident $(, $other: ident)*)=> {
        impl<'l,N> $struct_id<N>  where N: Num +CheckedMul +CheckedAdd {
            /// Returns the sum of squares of each coordinate.
            ///
            /// Performs computation using `N` type and its checked operations
            /// defined by `CheckedMul` and `CheckedAdd`.
            /// If `None` gets returned by any checked operation, then it is propagated.
            /// Otherwise `Some(the_sum)` is returned.
            ///
            /// # Examples
            ///
            #[doc= concat!(
                "```\n",
                "use siiir_points::", stringify!($struct_id), ";\n",
                "\n",
                "let p= ", stringify!($struct_id), "::from( std::array::from_fn(|idx| idx) );\n",
                "assert_eq!( ",
                    "p.hypot_sq(), ",
                    "Some( ",
                        stringify!(p.$first_operand), "*", stringify!(p.$first_operand),
                        $( " + ", stringify!(p.$other), "*", stringify!(p.$other), )*
                    " )",
                " );\n",
                "```\n",
            )]
            ///
            /// # Panics
            ///
            /// Propagates panic caused by any checked operation it uses.
            pub fn hypot_sq(&'l self)-> Option<N>{
                let mut sum: N= self.$first_operand.checked_mul( &self.$first_operand )?;
                $(
                    sum= sum.checked_add( &self.$other.checked_mul( &self.$other )? )?;
                )*
                Some(sum)
            }
        }
    };
}
macro_rules! impl_bounds {
    ($id: ident) => {
        impl<N: Num + Bounded> Bounded for $id<N> {
            fn min_value() -> Self {
                array::from_fn(|_| N::min_value()).into()
            }

            fn max_value() -> Self {
                array::from_fn(|_| N::max_value()).into()
            }
        }
    };
}

// Point2D
derive_universal_traits! {
    // Conversions
    #[derive(From, Into)]
    /// `Point2D` structure represents a point in two dimensional space.
    ///
    /// It is a generic structure, so it can be used with any type that implements the `num::Num` trait.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use siiir_points::Point2D;
    ///
    /// let p1 = Point2D::<f32>::from((2.0, 3.0));
    /// assert_eq!(&p1, &p1);
    ///
    /// let p2: Point2D::<f32> = (4.0, 6.0).into();
    /// assert_eq!(p2, p2);
    ///
    /// let p3 = p1 + p2;
    /// assert_eq!( p3, Point2D::from((6.0, 9.0) ) );
    ///
    /// let p4 = p3 - p1;
    /// assert_eq!( p4, Point2D::from((4.0, 6.0)) );
    ///
    /// let p5 = -p4;
    /// assert_eq!( p5, Point2D::from((-4.0, -6.0)) );
    /// ```
    pub struct Point2D<N: Num>{
        pub x: N,
        pub y: N,
    }
}
impl_hypot_sq!(Point2D, x, y);
impl<N: Num> From<[N; 2]> for Point2D<N> {
    fn from([x, y]: [N; 2]) -> Self {
        Self { x, y }
    }
}
impl<N: Num> From<Point2D<N>> for [N; 2] {
    fn from(Point2D { x, y }: Point2D<N>) -> Self {
        [x, y]
    }
}
impl<N: Num + Display> Display for Point2D<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "( {}, {} )", self.x, self.y)
    }
}
impl_bounds!(Point2D);

// Point3D
derive_universal_traits! {
    // Dereference
    #[derive(Deref,DerefMut)]
    /// `Point3D` structure represents a point in three dimensional space.
    ///
    /// It is also a generic structure and can be used with any type that implements the `num::Num` trait.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use siiir_points::Point3D;
    ///
    /// let p1: Point3D<_> = (2.0, 3.0, 4.0).into();
    /// assert_eq!(&p1, &p1);
    ///
    /// let p2: Point3D<_> = (4.0, 6.0, 8.0).into();
    /// assert_eq!(&p2, &p2);
    ///
    /// let p3 = p1 + p2;
    /// assert_eq!(p3, Point3D::from((6.0, 9.0, 12.0)));
    ///
    /// let p4 = p3 - p1;
    /// assert_eq!(p4, Point3D::from((4.0, 6.0, 8.0)));
    ///
    /// let p5 = -p4;
    /// assert_eq!(p5, Point3D::from((-4.0, -6.0, -8.0)));
    /// ```
    pub struct Point3D<N: Num>{
        #[deref]
        #[deref_mut]
        pub xy: Point2D<N>,
        pub z: N,
    }
}
impl_hypot_sq!(Point3D, x, y, z);
impl<N: Num> From<[N; 3]> for Point3D<N> {
    fn from([x, y, z]: [N; 3]) -> Self {
        Self {
            xy: [x, y].into(),
            z,
        }
    }
}
impl<N: Num> From<Point3D<N>> for [N; 3] {
    fn from(value: Point3D<N>) -> Self {
        let Point3D {
            xy: Point2D { x, y },
            z,
        } = value;
        [x, y, z]
    }
}
impl<N: Num> From<(N, N, N)> for Point3D<N> {
    fn from((x, y, z): (N, N, N)) -> Self {
        [x, y, z].into()
    }
}
impl<N: Num> From<Point3D<N>> for (N, N, N) {
    fn from(value: Point3D<N>) -> Self {
        let [x, y, z]: [N; 3] = value.into();
        (x, y, z)
    }
}
impl<N: Num + Display> Display for Point3D<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "( {}, {}, {} )", self.x, self.y, self.z)
    }
}
impl_bounds!(Point3D);

#[cfg(test)]
mod test;