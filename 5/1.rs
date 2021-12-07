use std::cmp;
use std::collections::HashMap;
use std::io::BufRead;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
}

#[derive(Debug)]
struct Segment {
    start: Point,
    end: Point,
}

fn maybe_swap(p: Point) -> Point {
    if p.x > p.y {
        return Point::new(p.y, p.x);
    }
    return p;
}

impl Segment {
    fn intersect_point(&self, other: &Segment) -> Option<Point> {
        let p0_x = self.start.x as f32;
        let p0_y = self.start.y as f32;
        let p1_x = self.end.x as f32;
        let p1_y = self.end.y as f32;
        let p2_x = other.start.x as f32;
        let p2_y = other.start.y as f32;
        let p3_x = other.end.x as f32;
        let p3_y = other.end.y as f32;

        let s1_x = p1_x - p0_x;
        let s1_y = p1_y - p0_y;
        let s2_x = p3_x - p2_x;
        let s2_y = p3_y - p2_y;

        let s = (-s1_y * (p0_x - p2_x) + s1_x * (p0_y - p2_y)) / (-s2_x * s1_y + s1_x * s2_y);
        let t = (s2_x * (p0_y - p2_y) - s2_y * (p0_x - p2_x)) / (-s2_x * s1_y + s1_x * s2_y);
        if s >= 0.0 && s <= 1.0 && t >= 0.0 && t <= 1.0 {
            let i_x = p0_x + (t * s1_x);
            let i_y = p0_y + (t * s1_y);
            return Some(Point::new(i_x.round() as i32, i_y.round() as i32));
        }
        return None;
    }

    fn overlap_points(&self, other: &Segment) -> Vec<Point> {
        let a: Point;
        let b: Point;
        let coeff: i32;
        let horizontal: bool;

        if self.start.x == self.end.x
            && other.start.x == other.end.x
            && self.start.x == other.start.x
        {
            a = maybe_swap(Point::new(self.start.y, self.end.y));
            b = maybe_swap(Point::new(other.start.y, other.end.y));
            coeff = self.start.x;
            horizontal = true;
        } else if self.start.y == self.end.y
            && other.start.y == other.end.y
            && self.start.y == other.start.y
        {
            a = maybe_swap(Point::new(self.start.x, self.end.x));
            b = maybe_swap(Point::new(other.start.x, other.end.x));
            coeff = self.start.y;
            horizontal = false;
        } else {
            return Vec::new();
        }

        if !(a.x <= b.y && a.y >= b.x) {
            return Vec::new();
        }

        let overlap_start = cmp::max(a.x, b.x);
        let overlap_end = cmp::min(a.y, b.y);
        let mut points: Vec<Point> = Vec::new();
        for overlap in overlap_start..=overlap_end {
            points.push(if horizontal {
                Point::new(coeff, overlap)
            } else {
                Point::new(overlap, coeff)
            });
        }
        return points;
    }
}

fn parse_point(s: &str) -> (i32, i32) {
    let s: Vec<i32> = s.split(",").map(|p| p.parse().unwrap()).collect();
    return (s[0], s[1]);
}

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let mut segments: Vec<Segment> = Vec::new();
    while let Some(line) = lines.next() {
        let line_unwrapped = line.unwrap();
        let s: Vec<&str> = line_unwrapped
            .trim()
            .split("->")
            .map(|s| s.trim())
            .collect();

        let (startx, starty) = parse_point(s[0]);
        let (endx, endy) = parse_point(s[1]);

        if !(startx == endx || starty == endy) {
            continue;
        }

        let segment = Segment {
            start: Point::new(startx, starty),
            end: Point::new(endx, endy),
        };
        segments.push(segment)
    }

    let mut intersection_points: HashMap<Point, i32> = HashMap::new();
    for i in 0..segments.len() {
        for j in i + 1..segments.len() {
            let result = segments[i].intersect_point(&segments[j]);
            match result {
                Some(p) => {
                    let p_count = intersection_points.entry(p).or_insert(0);
                    *p_count += 1;
                }
                None => {}
            }
            let overlaps = segments[i].overlap_points(&segments[j]);
            for p in overlaps {
                let p_count = intersection_points.entry(p).or_insert(0);
                *p_count += 1;
            }
        }
    }
    println!("{:?}", intersection_points.len())
}
