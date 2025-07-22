use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn to_string(&self) -> String {
        format!("{},{}", self.x, self.y)
    }

    pub fn equals(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }

    pub fn add(&self, dx: i32, dy: i32) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    pub fn is_within_bounds(&self, size: i32) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < size && self.y < size
    }

    pub fn from_string(key: &str) -> Self {
        let parts: Vec<&str> = key.split(',').collect();
        let x = parts[0].parse().unwrap();
        let y = parts[1].parse().unwrap();
        Self { x, y }
    }

    pub fn is_point_in_polygon(&self, polygon: &[Point]) -> bool {
        if polygon.iter().any(|v| v.equals(self)) {
            return false;
        }

        let mut inside = false;
        let n = polygon.len();
        let mut j = n - 1;

        for i in 0..n {
            let vi = &polygon[i];
            let vj = &polygon[j];

            if (vi.y > self.y) != (vj.y > self.y) {
                let dx = (vj.x - vi.x) as f64;
                let dy = (vj.y - vi.y) as f64;
                let safe_denom = if dy == 0.0 { f64::EPSILON } else { dy };
                
                let x_intersect = vi.x as f64 + dx * (self.y - vi.y) as f64 / safe_denom;
                if (self.x as f64) < x_intersect {
                    inside = !inside;
                }
            }
            j = i;
        }

        inside
    }
}