mod from_text_davinci {
    use crate::{Point2D, Point3D};
    use num_traits::Bounded;

    // Point2D

    #[test]
    fn point2d_add() {
        let p1 = Point2D::from([1, 2]);
        let p2 = Point2D::from([3, 4]);
        let p3 = p1 + p2;
        assert_eq!(p3, Point2D::from([4, 6]));
    }

    #[test]
    fn point2d_add_assign() {
        let mut p1 = Point2D::from([1, 2]);
        let p2 = Point2D::from([3, 4]);
        p1 += p2;
        assert_eq!(p1, Point2D::from([4, 6]));
    }

    #[test]
    fn point2d_sub() {
        let p1 = Point2D::from([1, 2]);
        let p2 = Point2D::from([3, 4]);
        let p3 = p2 - p1;
        assert_eq!(p3, Point2D::from([2, 2]));
    }

    #[test]
    fn point2d_sub_assign() {
        let p1 = Point2D::from([1, 2]);
        let mut p2 = Point2D::from([3, 4]);
        p2 -= p1;
        assert_eq!(p2, Point2D::from([2, 2]));
    }

    #[test]
    fn point2d_neg() {
        let p1 = Point2D::from([1, -2]);
        let p2 = -p1;
        assert_eq!(p2, Point2D::from([-1, 2]));
    }

    #[test]
    fn point2d_display() {
        let p1 = Point2D::from([1, -2]);
        assert_eq!(format!("{}", p1), "( 1, -2 )");
    }

    #[test]
    fn point2d_eq() {
        let p1 = Point2D::from([1, 2]);
        let p2 = Point2D::from([3, 4]);
        let p3 = Point2D::from([2, 2]);
        assert_ne!(p1, p2);
        assert_eq!(p1 + (1, 0).into(), p3);
    }

    #[test]
    fn point2d_from_arr() {
        let p1 = Point2D::from([1, -2]);
        let arr = [1, -2];
        assert_eq!(p1, Point2D::from(arr));
    }

    #[test]
    fn point2d_into_arr() {
        let p1 = Point2D { x: 1, y: -2 };
        let lhs_arr = [1, -2];
        let rhs_arr: [i32; 2] = p1.into();
        assert_eq!(lhs_arr, rhs_arr);
    }

    #[test]
    fn point2d_from_tuple() {
        let p1 = Point2D::from((1, -2));
        let tup = (1, -2);
        assert_eq!(p1, Point2D::from(tup));
    }

    #[test]
    fn point2d_into_tuple() {
        let p1 = Point2D::from((1, -2));
        let tup = (1, -2);
        assert_eq!(tup, p1.into());
    }

    #[test]
    fn point2d_bounds() {
        let p1 = Point2D::min_value();
        let p2 = Point2D::max_value();
        assert_eq!(p1, Point2D::from([i32::MIN, i32::MIN]));
        assert_eq!(p2, Point2D::from([f64::MAX, f64::MAX]));
    }

    // Point3D

    #[test]
    fn point3d_add() {
        let p1 = Point3D::from([1, 2, 3]);
        let p2 = Point3D::from([3, 4, 5]);
        let p3 = p1 + p2;
        assert_eq!(p3, Point3D::from([4, 6, 8]));
    }

    #[test]
    fn point3d_add_assign() {
        let mut p1 = Point3D::from([1, 2, 3]);
        let p2 = Point3D::from([3, 4, 5]);
        p1 += p2;
        assert_eq!(p1, Point3D::from([4, 6, 8]));
    }

    #[test]
    fn point3d_sub() {
        let p1 = Point3D::from([1, 2, 3]);
        let p2 = Point3D::from([3, 4, 5]);
        let p3 = p2 - p1;
        assert_eq!(p3, Point3D::from([2, 2, 2]));
    }

    #[test]
    fn point3d_sub_assign() {
        let p1 = Point3D::from([1, 2, 3]);
        let mut p2 = Point3D::from([3, 4, 5]);
        p2 -= p1;
        assert_eq!(p2, Point3D::from([2, 2, 2]));
    }

    #[test]
    fn point3d_neg() {
        let p1 = Point3D::from([1, -2, 0]);
        let p2 = -p1;
        assert_eq!(p2, Point3D::from([-1, 2, 0]));
    }

    #[test]
    fn point3d_display() {
        let p1 = Point3D::from([1, -2, 0]);
        assert_eq!(format!("{}", p1), "( 1, -2, 0 )");
    }

    #[test]
    fn point3d_eq() {
        let p1 = Point3D::from([1, 2, 3]);
        let p2 = Point3D::from([3, 4, 5]);
        let p3 = Point3D::from([1, 2, 3]);
        assert_ne!(p1, p2);
        assert_eq!(p1, p3);
    }

    #[test]
    fn point3d_from_arr() {
        let p1 = Point3D::from([1, -2, 3]);
        let arr = [1, -2, 3];
        assert_eq!(p1, Point3D::from(arr));
    }

    #[test]
    fn point3d_into_arr() {
        let p1 = Point3D {
            xy: [1, -2].into(),
            z: 3,
        };
        let lhs_arr = [1, -2, 3];
        let rhs_arr: [i32; 3] = p1.into();
        assert_eq!(lhs_arr, rhs_arr);
    }

    #[test]
    fn point3d_from_tuple() {
        let p1 = Point3D::from((1, -2, 3));
        let tup = (1, -2, 3);
        assert_eq!(p1, Point3D::from(tup));
    }

    #[test]
    fn point3d_into_tuple() {
        let p1 = Point3D::from((1, -2, 3));
        let tup = (1, -2, 3);
        assert_eq!(tup, p1.into());
    }
}
