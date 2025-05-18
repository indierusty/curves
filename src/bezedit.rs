use kurbo::{BezPath, ParamCurve, PathEl, Point};
use macroquad::prelude::*;

pub struct BezEditor {
    bezpath: Vec<BezPath>,
    selected: Option<usize>,
}

impl BezEditor {
    pub fn new() -> Self {
        Self {
            bezpath: Vec::new(),
            selected: None,
        }
    }

    pub fn update(&mut self) {
        let is_last_close_elm = |bezpath: &BezPath| {
            bezpath
                .elements()
                .last()
                .map_or(false, |elm| matches!(*elm, PathEl::ClosePath))
        };

        if is_key_pressed(KeyCode::S) {
            let (x, y) = mouse_position();
            let mouse = Point::new(x as f64, y as f64);
            self.selected = None;

            for i in 0..self.bezpath.len() {
                let bezpath = &self.bezpath[i];
                for element in bezpath.elements() {
                    let points = match element {
                        PathEl::MoveTo(point) => [point].to_vec(),
                        PathEl::LineTo(point) => [point].to_vec(),
                        PathEl::QuadTo(point, point1) => [point, point1].to_vec(),
                        PathEl::CurveTo(point, point1, point2) => [point, point1, point2].to_vec(),
                        PathEl::ClosePath => [].to_vec(),
                    };
                    for point in points {
                        if point.distance(mouse) < 5. {
                            self.selected = Some(i);
                        }
                    }
                }
            }
        }

        if is_key_pressed(KeyCode::I) {
            let bezpath = if self.selected.is_none()
                || self.bezpath[self.selected.unwrap()]
                    .elements()
                    .last()
                    .map_or(false, |elm| *elm == PathEl::ClosePath)
            {
                self.selected = Some(self.bezpath.len());
                self.bezpath.push(BezPath::new());
                self.bezpath.last_mut().unwrap()
            } else {
                self.bezpath.get_mut(self.selected.unwrap()).unwrap()
            };

            let (x, y) = mouse_position();
            let point = Point::new(x as f64, y as f64);
            if bezpath.elements().is_empty() {
                bezpath.push(PathEl::MoveTo(point));
                println!("inseting moveto");
            } else {
                bezpath.push(PathEl::LineTo(point));
                println!("inseting lineto");
            }
        }

        if is_key_pressed(KeyCode::J) {
            let bezpath = if let Some(selected) = self.selected {
                &mut self.bezpath[selected]
            } else {
                self.bezpath.push(BezPath::new());
                self.bezpath.last_mut().unwrap()
            };

            if !bezpath.is_empty() && !is_last_close_elm(&bezpath) {
                bezpath.line_to(bezpath.elements().first().unwrap().end_point().unwrap());
                bezpath.close_path();
            }
        }
    }

    pub fn draw(&self) {
        for bezpath in &self.bezpath {
            for segment in bezpath.segments() {
                match segment {
                    kurbo::PathSeg::Line(line) => {
                        draw_circle(line.p0.x as f32, line.p0.y as f32, 3., RED);
                        draw_circle(line.p1.x as f32, line.p1.y as f32, 3., RED);
                    }
                    kurbo::PathSeg::Quad(quad_bez) => {
                        draw_circle(quad_bez.p0.x as f32, quad_bez.p0.y as f32, 3., RED);
                        draw_circle(quad_bez.p1.x as f32, quad_bez.p1.y as f32, 3., RED);
                        draw_circle(quad_bez.p2.x as f32, quad_bez.p2.y as f32, 3., RED);
                    }
                    kurbo::PathSeg::Cubic(cubic_bez) => {
                        draw_circle(cubic_bez.p0.x as f32, cubic_bez.p0.y as f32, 3., RED);
                        draw_circle(cubic_bez.p1.x as f32, cubic_bez.p1.y as f32, 3., RED);
                        draw_circle(cubic_bez.p2.x as f32, cubic_bez.p2.y as f32, 3., RED);
                        draw_circle(cubic_bez.p3.x as f32, cubic_bez.p3.y as f32, 3., RED);
                    }
                }

                let mut last_point: Option<Point> = None;
                let mut t = 0.;
                while t <= 1. {
                    let point = segment.eval(t);
                    if let Some(last_point) = last_point {
                        draw_line(
                            last_point.x as f32,
                            last_point.y as f32,
                            point.x as f32,
                            point.y as f32,
                            2.,
                            DARKGRAY,
                        );
                    }
                    last_point = Some(point);
                    t += 0.1;
                }
            }
        }
    }
}
