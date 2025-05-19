use kurbo::{BezPath, ParamCurve, PathEl, Point};
use macroquad::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Selection {
    bezpath_index: usize,
    element_index: usize,
    point_index: usize,
}

impl Selection {
    fn new(bezpath_index: usize, element_index: usize, point_index: usize) -> Self {
        Self {
            bezpath_index,
            element_index,
            point_index,
        }
    }
}

pub struct BezEditor {
    bezpath: Vec<BezPath>,
    selected: Option<Selection>,
}

impl BezEditor {
    pub fn new() -> Self {
        Self {
            bezpath: Vec::new(),
            selected: None,
        }
    }

    pub fn update(&mut self) {
        let is_bezpath_closed = |bezpath: &BezPath| {
            bezpath
                .elements()
                .last()
                .map_or(false, |elm| matches!(*elm, PathEl::ClosePath))
        };

        // Select the bezpath if any of its points is close enough to the mouse position and 'S' is pressed.
        if is_key_pressed(KeyCode::S) {
            let (x, y) = mouse_position();
            let mouse = Point::new(x as f64, y as f64);
            self.selected = None;

            'a: for bez_index in 0..self.bezpath.len() {
                let bezpath = &self.bezpath[bez_index];
                for ele_index in 0..bezpath.elements().len() {
                    let element = bezpath.elements()[ele_index];
                    let points = match element {
                        PathEl::MoveTo(point) => [point].to_vec(),
                        PathEl::LineTo(point) => [point].to_vec(),
                        PathEl::QuadTo(point, point1) => [point, point1].to_vec(),
                        PathEl::CurveTo(point, point1, point2) => [point, point1, point2].to_vec(),
                        PathEl::ClosePath => [].to_vec(),
                    };
                    for point_index in 0..points.len() {
                        let point = points[point_index];
                        if point.distance(mouse) < 5. {
                            self.selected = Some(Selection::new(bez_index, ele_index, point_index));
                            break 'a;
                        }
                    }
                }
            }
        }

        if is_key_down(KeyCode::C) && self.selected.is_some() {
            let Selection {
                bezpath_index,
                element_index,
                point_index: _,
            } = self.selected.unwrap();

            let bezpath = &mut self.bezpath[bezpath_index];
            // if element index in point at MoveTo element then increment it to point next element / segment.
            let element_index = element_index.min(1);

            if element_index <= bezpath.segments().count() {
                let segment = bezpath.get_seg(element_index).unwrap();
                let cubic = segment.to_cubic();
                *bezpath.elements_mut().get_mut(element_index).unwrap() =
                    PathEl::CurveTo(cubic.p1, cubic.p2, cubic.p3);
            }
        }

        // Move selected point to the mouse position if there is any selection.
        if is_key_down(KeyCode::Space) {
            if let Some(selection) = self.selected {
                let Selection {
                    bezpath_index: bezpath,
                    element_index: element,
                    point_index: point,
                } = selection;

                let (x, y) = mouse_position();
                let mouse = Point::new(x as f64, y as f64);

                let element = &mut self.bezpath[bezpath].elements_mut()[element];
                match element {
                    PathEl::MoveTo(point) => *point = mouse,
                    PathEl::LineTo(point) => *point = mouse,
                    PathEl::QuadTo(point0, point1) => {
                        let points = [point0, point1];
                        *points[point] = mouse;
                    }
                    PathEl::CurveTo(point0, point1, point2) => {
                        let points = [point0, point1, point2];
                        *points[point] = mouse;
                    }
                    PathEl::ClosePath => {}
                }
            }
        }

        // Insert a new segment in the currently selected bezpath.
        // If no bezpath is selected or the selected bezpath is closed then create a new one insert a point.
        if is_key_pressed(KeyCode::I) {
            let bezpath = if self.selected.is_none()
                || is_bezpath_closed(&self.bezpath[self.selected.unwrap().bezpath_index])
            {
                self.selected = Some(Selection::new(self.bezpath.len(), 0, 0));
                self.bezpath.push(BezPath::new());
                self.bezpath.last_mut().unwrap()
            } else {
                self.bezpath
                    .get_mut(self.selected.unwrap().bezpath_index)
                    .unwrap()
            };

            let (x, y) = mouse_position();
            let point = Point::new(x as f64, y as f64);

            if bezpath.elements().is_empty() {
                bezpath.push(PathEl::MoveTo(point));
            } else {
                bezpath.push(PathEl::LineTo(point));
            }
        }

        // Close the selected bezpath.
        if is_key_pressed(KeyCode::J) {
            if let Some(selected) = &self.selected {
                let bezpath = &mut self.bezpath[selected.bezpath_index];
                bezpath.close_path();
            }
        }
    }

    pub fn draw(&self) {
        for (bi, bezpath) in self.bezpath.iter().enumerate() {
            if let Some(point) = bezpath.elements()[0].end_point() {
                draw_circle(point.x as f32, point.y as f32, 3., RED);
            }
            for (ei, elements) in bezpath.elements().iter().enumerate() {
                let points = match elements {
                    PathEl::MoveTo(p1) => [p1].to_vec(),
                    PathEl::LineTo(p1) => [p1].to_vec(),
                    PathEl::QuadTo(p1, p2) => [p1, p2].to_vec(),
                    PathEl::CurveTo(p1, p2, p3) => [p1, p2, p3].to_vec(),
                    PathEl::ClosePath => [].to_vec(),
                };
                for (pi, point) in points.iter().enumerate() {
                    let active = self
                        .selected
                        .is_some_and(|s| s == Selection::new(bi, ei, pi));

                    if pi + 1 == points.len() {
                        let color = if active { GREEN } else { BLUE };
                        draw_circle(point.x as f32, point.y as f32, 3., color);
                    } else {
                        let color = if active { GREEN } else { SKYBLUE };
                        draw_circle_lines(point.x as f32, point.y as f32, 3., 2., color);
                    }
                }
            }
            for segment in bezpath.segments() {
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
