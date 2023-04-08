use std::collections::HashMap;

use rand::seq::SliceRandom;

use crate::point::{dist, Distance, Point};

pub struct Grid {
    r: i64,
    p1: Point,
    p2: Point,
    bins: HashMap<Point, Vec<Point>>,
}

impl Grid {
    fn new(p: &Point, q: &Point) -> Grid {
        let mut grid = Grid {
            r: dist(p, q),
            p1: p.clone(),
            p2: q.clone(),
            bins: HashMap::new(),
        };

        grid.insert(p);
        grid.insert(q);

        grid
    }

    fn boxify(&self, p: &Point) -> Point {
        let (x, y) = p;
        (x / self.r as i32, y / self.r as i32)
    }

    fn insert(&mut self, p: &Point) {
        match self.lookup(p) {
            None => {
                self.bins.entry(self.boxify(p)).or_insert(vec![]).push(*p);
            }
            Some((rp, min_q)) => {
                // updates r'
                self.r = rp;

                let (i, j) = self.boxify(p);
                let bins_to_cmp = [
                    (i - 1, j - 1),
                    (i - 1, j),
                    (i - 1, j + 1),
                    (i, j - 1),
                    (i, j),
                    (i, j + 1),
                    (i + 1, j - 1),
                    (i + 1, j),
                    (i + 1, j + 1),
                ];

                let existing_points: Vec<Point> = bins_to_cmp
                    .into_iter()
                    .map(|bin_key| self.bins.get(&bin_key))
                    .filter(|e| e.is_some())
                    .flat_map(|e| e.unwrap())
                    .cloned()
                    .collect();

                // let existing_points = self.bins.values().flatten();
                let mut new_bins: HashMap<Point, Vec<Point>> = HashMap::new();

                // rehashes every existing point
                existing_points.iter().for_each(|point| {
                    new_bins
                        .entry(self.boxify(point))
                        .or_insert(vec![])
                        .push(*point);
                });

                // added the new point
                new_bins.entry(self.boxify(p)).or_insert(vec![]).push(*p);

                self.bins = new_bins;
                self.p1 = min_q;
                self.p2 = *p;
            }
        }
    }

    fn lookup(&self, p: &Point) -> Option<(Distance, Point)> {
        let (i, j) = self.boxify(p);
        let bins_to_cmp = [
            (i - 1, j - 1),
            (i - 1, j),
            (i - 1, j + 1),
            (i, j - 1),
            (i, j),
            (i, j + 1),
            (i + 1, j - 1),
            (i + 1, j),
            (i + 1, j + 1),
        ];

        let points_to_cmp: Vec<Point> = bins_to_cmp
            .into_iter()
            .map(|bin_key| self.bins.get(&bin_key))
            .filter(|e| e.is_some())
            .flat_map(|e| e.unwrap())
            .cloned()
            .collect();

        if points_to_cmp.is_empty() {
            return None;
        }

        let (min_dist, min_pair) = points_to_cmp
            .iter()
            .map(|q| (dist(p, q), q))
            .reduce(|dp1, dp2| {
                let (dist1, q1) = dp1;
                let (dist2, q2) = dp2;

                if dist1 < dist2 {
                    (dist1, q1)
                } else {
                    (dist2, q2)
                }
            })
            .unwrap();

        if min_dist < self.r {
            Some((min_dist, *min_pair))
        } else {
            None
        }
    }
}

pub fn lin_closest_pair(points: &[Point]) -> [Point; 2] {
    use rand::thread_rng;
    let mut rng = thread_rng();
    let mut points_cp = points.to_vec();
    points_cp.shuffle(&mut rng);

    let mut grid = Grid::new(&points_cp[0], &points_cp[1]);
    points_cp.iter().skip(2).for_each(|e| grid.insert(e));

    [grid.p1, grid.p2]
}

mod test {
    #[test]
    fn cp_test() {
        use std::collections::hash_set::HashSet;
        let points = vec![(2, 3), (12, 30), (40, 50), (5, 1), (12, 10), (3, 4)];
        dbg!(super::lin_closest_pair(&points));
        assert_eq!(
            HashSet::from([(2, 3), (3, 4)]),
            HashSet::from(super::lin_closest_pair(&points))
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
            HashSet::from(super::lin_closest_pair(&points))
        );
    }
}
