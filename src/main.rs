use std::collections::HashMap;

use kurbo::{Line, ParamCurve, Shape};
use macroquad::prelude::*;

mod bezedit;

use bezedit::*;

fn conf() -> Conf {
    Conf {
        window_title: "Bézier curves".to_string(),
        window_width: 900,
        window_height: 600,
        fullscreen: false,
        ..Default::default()
    }
}

const GRID_SIZE: usize = 5;

#[macroquad::main(conf)]
async fn main() {
    let mut bez_editor = BezEditor::new();
    loop {
        bez_editor.update();

        let mut bezpaths = bez_editor.bezpaths().to_vec();

        let mut segments = bezpaths
            .iter_mut()
            .map(|path| {
                path.close_path();
                path
            })
            .flat_map(|path| path.segments())
            .map(|segment| (segment, segment.bounding_box().expand()))
            .fold(Vec::new(), |mut acc, (segment, bbox)| {
                acc.push((bbox, segment));
                acc
            });

        let first_bezpath = bez_editor.bezpaths().first();

        // sort the bbox based on min x-axis
        segments.sort_by(|a, b| (a.0.x0 as isize).cmp(&(b.0.x0 as isize)));

        let mut xys = HashMap::<isize, Vec<isize>>::new();

        if let Some(bezpath) = first_bezpath {
            let bbox = bezpath.bounding_box().expand();
            let mut active_segments = HashMap::new();
            let mut i = 0;

            let start_x = bbox.x0 as isize / GRID_SIZE as isize;
            let end_x = bbox.x1 as isize / GRID_SIZE as isize + 1;

            for x in start_x..=end_x {
                let x = x * GRID_SIZE as isize;
                // push segments which are need to be checked for intersection.
                while i < segments.len() && segments[i].0.x0 as isize <= x {
                    let (min_x, max_x, segment) = (
                        segments[i].0.x0 as isize,
                        segments[i].0.x1 as isize,
                        segments[i].1,
                    );
                    active_segments.insert((min_x, max_x), segment);
                    i += 1;
                }

                let line = Line::new((x as f64, bbox.y0), (x as f64, bbox.y1));
                for (_, seg) in &active_segments {
                    for inter in seg.intersect_line(line) {
                        let intersection = line.eval(inter.line_t);
                        let (x, y) = (
                            intersection.x as isize,
                            (intersection.y / GRID_SIZE as f64).floor() as isize
                                * GRID_SIZE as isize,
                        );
                        xys.entry(x).and_modify(|ys| ys.push(y)).or_insert(vec![y]);
                    }
                }

                // pop segments whose bouding boxed are behind current x intersection checking
                let mut deactives: Vec<(isize, isize)> = Vec::new();
                for (active, _) in &active_segments {
                    if active.1 < x {
                        deactives.push(*active);
                    }
                }
                for i in 0..deactives.len() {
                    let d = deactives[i];
                    active_segments.remove(&d);
                }
            }
        }

        // sort the bbox based on min x-axis
        segments.sort_by(|a, b| (a.0.y0 as isize).cmp(&(b.0.y0 as isize)));

        let mut yxs = HashMap::<isize, Vec<isize>>::new();

        if let Some(bezpath) = first_bezpath {
            let bbox = bezpath.bounding_box().expand();
            let mut active_segments = HashMap::new();
            let mut i = 0;

            let start_y = bbox.y0 as isize / GRID_SIZE as isize;
            let end_y = bbox.y1 as isize / GRID_SIZE as isize + 1;

            for y in start_y..end_y {
                let y = y * GRID_SIZE as isize;
                // push segments which are need to be checked for intersection.
                while i < segments.len() && segments[i].0.y0 as isize <= y {
                    let (min_y, max_y, segment) = (
                        segments[i].0.y0 as isize,
                        segments[i].0.y1 as isize,
                        segments[i].1,
                    );
                    active_segments.insert((min_y, max_y), segment);
                    i += 1;
                }

                let line = Line::new((bbox.x0, y as f64), (bbox.x1, y as f64));
                for (_, seg) in &active_segments {
                    for inter in seg.intersect_line(line) {
                        let intersection = line.eval(inter.line_t);
                        let (x, y) = (
                            (intersection.x / GRID_SIZE as f64).floor() as isize
                                * GRID_SIZE as isize,
                            intersection.y as isize,
                        );
                        yxs.entry(x).and_modify(|ys| ys.push(y)).or_insert(vec![y]);
                    }
                }

                // pop segments whose bouding boxed are behind current x intersection checking
                let mut deactives: Vec<(isize, isize)> = Vec::new();
                for (active, _) in &active_segments {
                    if active.1 < y {
                        deactives.push(*active);
                    }
                }
                for i in 0..deactives.len() {
                    let d = deactives[i];
                    active_segments.remove(&d);
                }
            }
        }

        clear_background(WHITE);
        bez_editor.draw();
        for xy in xys {
            let x = xy.0;
            for y in xy.1 {
                draw_rectangle_lines(
                    x as f32,
                    y as f32,
                    GRID_SIZE as f32,
                    GRID_SIZE as f32,
                    2.,
                    RED,
                );
            }
        }
        for xy in yxs {
            let x = xy.0;
            for y in xy.1 {
                draw_rectangle_lines(
                    x as f32,
                    y as f32,
                    GRID_SIZE as f32,
                    GRID_SIZE as f32,
                    2.,
                    RED,
                );
            }
        }
        next_frame().await
    }
}
