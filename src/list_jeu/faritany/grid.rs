use std::collections::{HashMap, HashSet, VecDeque};
use super::direction::{ALL_DIRECTIONS, DIRECTIONS4};
use super::player::Player;
use super::point::Point;
#[derive(Debug,Clone)]
pub struct Grid {
    occupied_cells: HashMap<String, Option<Player>>,
    polygon_loops: HashMap<Vec<Point>, Player>,
    disabled_points: HashSet<String>,
    size: i32,
}

pub struct PlaceStoneResult {
    pub closed_loop: Vec<Point>,
    pub enclosed_points: usize,
    pub is_closer: bool,
}

impl Grid {
    pub fn new(size: i32) -> Self {
        Self {
            occupied_cells: HashMap::new(),
            polygon_loops: HashMap::new(),
            disabled_points: HashSet::new(),
            size,
        }
    }

    pub fn is_cell_empty(&self, point: &Point) -> bool {
        !self.occupied_cells.contains_key(&point.to_string())
    }

    pub fn place_stone(&mut self, point: Point, player: Player) -> Option<PlaceStoneResult> {
        let key = point.to_string();
        self.occupied_cells.insert(key.clone(), Some(player));

        let connected_stones = self.find_connected_stones(&point, player);
        let closed_loop = self.find_closed_loop(&connected_stones);
        let (enclosed_points, perimeter_points) = self.find_enclosed_points(&closed_loop, player);

        if enclosed_points.is_empty() {
            for (poly, owner) in self.polygon_loops.clone().iter() {
                if *owner != player && point.is_point_in_polygon(poly) {
                    let poly_clone = poly.clone();
                    self.polygon_loops.remove(&poly_clone);
                    let (enclosed_points, perimeter_points) = self.find_enclosed_points(&poly_clone, *owner);
                    return Some(PlaceStoneResult {
                        closed_loop: perimeter_points,
                        enclosed_points: enclosed_points.len(),
                        is_closer: true,
                    });
                }
            }
            return None;
        }

        Some(PlaceStoneResult {
            closed_loop: perimeter_points,
            enclosed_points: enclosed_points.len(),
            is_closer: false,
        })
    }

    pub fn get_player_at(&self, point: &Point) -> Option<Player> {
        self.occupied_cells
            .get(&point.to_string())
            .and_then(|p| *p)
    }

    fn find_connected_stones(&self, start_point: &Point, player: Player) -> Vec<Point> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(*start_point);
        let mut connected_stones = Vec::new();

        while let Some(current_point) = queue.pop_front() {
            let key = current_point.to_string();
            if visited.contains(&key) || self.disabled_points.contains(&key) {
                continue;
            }
            visited.insert(key);

            if self.get_player_at(&current_point) == Some(player) {
                connected_stones.push(current_point);

                for (dx, dy) in ALL_DIRECTIONS.iter() {
                    let neighbor = current_point.add(*dx, *dy);
                    if neighbor.is_within_bounds(self.size) && !visited.contains(&neighbor.to_string()) {
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        connected_stones
    }

    fn find_closed_loop(&self, points: &[Point]) -> Vec<Point> {
        let mut loop_points = points.to_vec();
        let mut changed = true;

        while changed {
            changed = false;
            let mut points_to_remove = HashSet::new();

            for point in &loop_points {
                let mut neighbor_count = 0;
                for (dx, dy) in ALL_DIRECTIONS.iter() {
                    let neighbor = point.add(*dx, *dy);
                    if loop_points.iter().any(|p| p.equals(&neighbor)) {
                        neighbor_count += 1;
                    }
                }

                if neighbor_count == 1 {
                    points_to_remove.insert(point.to_string());
                }
            }

            if !points_to_remove.is_empty() {
                changed = true;
                loop_points.retain(|p| !points_to_remove.contains(&p.to_string()));
            }
        }

        if loop_points.len() >= 4 {
            self.reorder_polygon(&loop_points)
        } else {
            Vec::new()
        }
    }

    fn reorder_polygon(&self, points: &[Point]) -> Vec<Point> {
        if points.is_empty() {
            return Vec::new();
        }

        let point_set: HashSet<String> = points.iter().map(|p| p.to_string()).collect();
        let is_in_set = |p: &Point| point_set.contains(&p.to_string());

        let mut sorted_points = points.to_vec();
        sorted_points.sort_by(|a, b| {
            a.y.cmp(&b.y)
                .then_with(|| a.x.cmp(&b.x))
        });
        let start = sorted_points[0];

        let mut contour = Vec::new();
        let mut current = start;
        let mut dir_index = 0;
        let start_key = start.to_string();
        let mut first_loop = true;

        loop {
            contour.push(current);
            let mut found = false;
            let mut next_dir_index = 0;

            for i in 0..ALL_DIRECTIONS.len() {
                let idx = (dir_index + i) % ALL_DIRECTIONS.len();
                let (dx, dy) = ALL_DIRECTIONS[idx];
                let neighbor = current.add(dx, dy);

                if is_in_set(&neighbor) {
                    current = neighbor;
                    next_dir_index = (idx + 6) % 8;
                    found = true;
                    break;
                }
            }

            if !found {
                break;
            }

            dir_index = next_dir_index;

            if !first_loop && current.to_string() == start_key {
                break;
            }

            first_loop = false;
        }

        contour
    }

    fn find_enclosed_points(&mut self, loop_points: &[Point], player: Player) -> (Vec<Point>, Vec<Point>) {
        if loop_points.is_empty() {
            return (Vec::new(), Vec::new());
        }

        let min_x = loop_points.iter().map(|p| p.x).min().unwrap();
        let max_x = loop_points.iter().map(|p| p.x).max().unwrap();
        let min_y = loop_points.iter().map(|p| p.y).min().unwrap();
        let max_y = loop_points.iter().map(|p| p.y).max().unwrap();

        let mut enclosed_points = Vec::new();
        let mut point_capture = Vec::new();
        let mut perimeter_set = HashSet::new();

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let point = Point::new(x, y);
                if !point.is_point_in_polygon(loop_points) {
                    continue;
                }

                point_capture.push(point);
                let key = point.to_string();

                if !self.disabled_points.contains(&key) {
                    if let Some(owner) = self.get_player_at(&point) {
                        if owner != player {
                            enclosed_points.push(point);
                            self.disabled_points.insert(key);
                        }
                    }
                }

                for (dx, dy) in DIRECTIONS4.iter() {
                    let neighbor = point.add(*dx, *dy);
                    if !neighbor.is_point_in_polygon(loop_points) {
                        perimeter_set.insert(neighbor);
                    }
                }
            }
        }

        if !enclosed_points.is_empty() {
            for p in &point_capture {
                if self.get_player_at(p).is_none() {
                    self.occupied_cells.insert(p.to_string(), None);
                }
            }
        }

        let perimeter_points: Vec<Point> = perimeter_set.into_iter().collect();
        let perimeter_points = self.reorder_polygon(&perimeter_points);

        if enclosed_points.is_empty() && !point_capture.is_empty() {
            self.polygon_loops.insert(perimeter_points.clone(), player);
        }

        (enclosed_points, perimeter_points)
    }
}