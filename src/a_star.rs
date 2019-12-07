use crate::direction::Direction;
use crate::matrix::Matrix;
use crate::point::Point;
use crate::snake::Snake;

use std::collections::HashSet;
use std::f32;

fn a_star_path(end: &Point, snake: &Snake, width: usize, height: usize) -> Option<Vec<Point>> {
    let mut snake_pts: Matrix<bool> = Matrix::new(width, height);

    snake.for_each_segment(|pt, _| {
        snake_pts.set_pt(&pt, true);
    });

    let mut came_from: Matrix<Point> = Matrix::new(width, height);
    let mut g_scores: Matrix<f32> = Matrix::new(width, height);
    g_scores.set_pt(snake.get_head(), 0.0);
    let mut f_scores: Matrix<f32> = Matrix::new(width, height);
    f_scores.set_pt(snake.get_head(), distance(snake.get_head(), end));

    let mut open_set: HashSet<Point> = HashSet::new();
    open_set.insert(*snake.get_head());

    while open_set.len() != 0 {
        let current = min_f_score(&open_set, &f_scores).expect("current not found?");

        if &current == end {
            return Some(reconstruct_path(&current, &came_from));
        }

        open_set.remove(&current);

        let current_g_score = g_scores.get_pt(&current).cloned();

        for dir in Direction::all() {
            let neighbor_pt = current.dir_adj(dir);

            if snake_pts.get_pt(&neighbor_pt).is_some() {
                continue;
            }

            let pos_g_score =
                current_g_score.map(|g| g + distance(&current, &neighbor_pt)).unwrap_or(f32::INFINITY);

            let g_score_neighbor = g_scores.get_pt(&neighbor_pt).cloned().unwrap_or(f32::INFINITY);

            if pos_g_score < g_score_neighbor {
                came_from.set_pt(&neighbor_pt, current);
                g_scores.set_pt(&neighbor_pt, pos_g_score);
                f_scores.set_pt(&neighbor_pt, pos_g_score + distance(&neighbor_pt, end));

                open_set.insert(neighbor_pt);
            }
        }
    }

    return None;
}

fn reconstruct_path(current: &Point, came_from: &Matrix<Point>) -> Vec<Point> {
    let mut full_path = vec![*current];

    let mut current: Option<&Point> = Some(current);

    while current.is_some() {
        current = came_from.get_pt(current.unwrap());

        if current.is_some() {
            full_path.push(*current.unwrap());
        }
    }

    return full_path;
}

fn min_f_score(of: &HashSet<Point>, f_scores: &Matrix<f32>) -> Option<Point> {
    let mut min: Option<(&Point, f32)> = None;
    for node in of {
        let node_f_score = *f_scores.get_pt(&node).expect("Node didn't have fscore?");
        if min.is_none() || min.unwrap().1 > node_f_score {
            min = Some((node, node_f_score));
        }
    }
    return min.map(|f| *f.0);
}

fn distance(p1: &Point, p2: &Point) -> f32 {
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;
    let dist = (((dx * dx) + (dy * dy)) as f32).sqrt();
    return dist;
}
