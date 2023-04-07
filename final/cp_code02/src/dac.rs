use crate::point::{dist, pt_default, Distance, Point};

fn closest_pairs(points: &[Point]) -> [Point; 2] {
    let mut sorted_points = points.to_vec();
    sorted_points.sort_by_key(|&(x, _)| x);

    let (p1, p2, _) = cp_helper(&mut sorted_points);
    [p1, p2]
}

fn cp_helper(points: &[Point]) -> (Point, Point, Distance) {
    // base cases
    let n = points.len();
    if n == 1 {
        return (pt_default(), pt_default(), i64::MAX);
    } else if n == 2 {
        return (points[0], points[1], dist(&points[0], &points[1]));
    }

    // step 1
    let (l, r) = points.split_at(n / 2);
    let (lp1, lp2, ld) = cp_helper(l);
    let (rp1, rp2, rd) = cp_helper(r);

    // step 2, 3
    let (mut min_p1, mut min_p2, mut d) = if ld < rd {
        (lp1, lp2, ld)
    } else {
        (rp1, rp2, rd)
    };
    let (x_mid, _) = points[n / 2];

    // step 4, 5
    let mut band: Vec<Point> = points
        .iter()
        .filter(|&(x, _)| ((x_mid - x).abs() as i64) < d)
        .cloned()
        .collect();

    if band.len() <= 1 {
        return (min_p1, min_p2, d);
    } else if band.len() == 2 {
        let (band_p1, band_p2, band_d) = (band[0], band[1], dist(&band[0], &band[1]));
        if band_d < d {
            // band min smaller
            return (band_p1, band_p2, band_d);
        } else {
            // outside of band min smaller
            return (min_p1, min_p2, d);
        }
    }
    band.sort_by_key(|&(_, y)| y);

    for i in 1..band.len().min(7usize) {
        let p1 = band[i - 1];
        let p2 = band[i];

        if dist(&p1, &p2) < d {
            (min_p1, min_p2, d) = (p1, p2, dist(&p1, &p2));
        }
    }

    (min_p1, min_p2, d)
}

mod test {
    #[test]
    fn cp_test() {
        use std::collections::hash_set::HashSet;
        let points = vec![(2, 3), (12, 30), (40, 50), (5, 1), (12, 10), (3, 4)];
        dbg!(super::closest_pairs(&points));
        assert_eq!(
            HashSet::from([(2, 3), (3, 4)]),
            HashSet::from(super::closest_pairs(&points))
        );
    }

    #[test]
    fn big_cp_test() {
        use std::collections::hash_set::HashSet;
        let points = vec![
            (-83, 86),
            (-53, -29),
            (-30, 50),
            (63, 86),
            (-58, 84),
            (-44, -62),
            (85, -57),
            (-5, 83),
            (88, -12),
            (46, 43),
            (-59, -14),
            (31, 79),
            (-46, 25),
            (9, 70),
            (66, -14),
            (-74, 47),
            (36, -91),
            (28, 20),
            (-98, -19),
            (22, 34),
            (-5, 57),
            (68, 3),
            (-30, 86),
            (26, -21),
            (-58, 27),
            (66, 8),
            (35, -81),
            (8, 80),
            (-7, -3),
            (-86, -43),
            (-75, 56),
            (60, 2),
            (44, -37),
            (-67, -75),
            (71, 80),
            (-20, -26),
            (-99, 54),
            (34, 95),
            (21, -91),
            (5, 67),
            (-71, -91),
            (58, 57),
            (-70, -98),
            (78, 100),
            (84, 96),
            (-44, -66),
            (55, -80),
            (-22, 17),
            (-17, -49),
            (80, 59),
            (8, -86),
            (-17, 92),
            (76, -59),
            (-8, 60),
            (82, 31),
            (47, -100),
            (82, 56),
            (-11, -28),
            (58, -87),
            (46, -96),
            (64, 4),
            (66, -58),
            (-5, 32),
            (10, -5),
            (-1, 78),
            (98, 64),
            (-19, 60),
            (10, 7),
            (8, 31),
            (60, 91),
            (95, -87),
            (21, 53),
            (-10, 11),
            (-63, 58),
            (27, -75),
            (-79, -87),
            (-91, 88),
            (-19, -84),
            (-18, -71),
            (22, 19),
            (25, -19),
            (-10, -4),
            (-55, 68),
            (-44, -92),
            (-42, 68),
            (8, 1),
            (63, 33),
            (2, 38),
            (-2, 2),
            (-94, 16),
            (-64, 74),
            (5, -57),
            (16, -63),
            (67, 98),
            (-4, 50),
            (-93, 56),
            (-63, 17),
            (-33, -59),
            (9, -8),
            (-75, -85),
        ];
        assert_eq!(
            HashSet::from([(26, -21), (25, -19)]),
            HashSet::from(super::closest_pairs(&points))
        );
    }
}
