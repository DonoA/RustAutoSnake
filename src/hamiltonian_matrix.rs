use crate::direction::Direction;
use crate::matrix::Matrix;
use crate::point::Point;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

use rand::seq::SliceRandom;
use rand::thread_rng;

struct GridWeightNode {
    right: u8,
    down: u8,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Connection {
    first: Point,
    second: Point,

    direction: Direction,

    weight: u8,
}

impl Ord for Connection {
    fn cmp(&self, other: &Connection) -> Ordering {
        other.weight.cmp(&self.weight)
    }
}

impl PartialOrd for Connection {
    fn partial_cmp(&self, other: &Connection) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct PrimTreeNode {
    up: bool,
    right: bool,
    down: bool,
    left: bool,

    in_tree: bool,
}

fn generate_rand_weights(width: usize, height: usize) -> Matrix<GridWeightNode> {
    let mut matrix = Matrix::new(width, height);

    for x in 0..width {
        for y in 0..height {
            matrix.set(
                x,
                y,
                GridWeightNode {
                    right: rand::random(),
                    down: rand::random(),
                },
            );
        }
    }

    return matrix;
}

fn generate_empty_prim_graph(width: usize, height: usize) -> Matrix<PrimTreeNode> {
    let mut matrix = Matrix::new(width, height);

    for x in 0..width {
        for y in 0..height {
            matrix.set(
                x,
                y,
                PrimTreeNode {
                    up: false,
                    right: false,
                    down: false,
                    left: false,

                    in_tree: false,
                },
            );
        }
    }

    return matrix;
}

fn print_prim_graph(graph: &Matrix<PrimTreeNode>) {
    for y in 0..graph.get_height() {
        // print tops
        for x in 0..graph.get_width() {
            let pt = graph.get(x, y).expect("graph node not found");
            print!(" ");
            if pt.up {
                print!("#");
            } else {
                print!(" ");
            }
            print!(" ");
        }

        println!();

        // print l r self
        for x in 0..graph.get_width() {
            let pt = graph.get(x, y).expect("graph node not found");
            if pt.left {
                print!("#");
            } else {
                print!(" ");
            }
            print!("#");
            if pt.right {
                print!("#");
            } else {
                print!(" ");
            }
        }

        println!();

        // print tops
        for x in 0..graph.get_width() {
            let pt = graph.get(x, y).expect("graph node not found");
            print!(" ");
            if pt.down {
                print!("#");
            } else {
                print!(" ");
            }
            print!(" ");
        }

        println!();
    }
}

fn add_all_dirs(
    current: &Point,
    weights: &Matrix<GridWeightNode>,
    node_queue: &mut BinaryHeap<Connection>,
) {
    // add up
    let up_pos = current.add(0, -1);
    if let Some(up_node) = weights.get(up_pos.x, up_pos.y) {
        node_queue.push(Connection {
            first: *current,
            second: up_pos,
            direction: Direction::UP,
            weight: up_node.down,
        });
    }

    // add left
    let left_pos = current.add(-1, 0);
    if let Some(up_node) = weights.get(left_pos.x, left_pos.y) {
        node_queue.push(Connection {
            first: *current,
            second: left_pos,
            direction: Direction::LEFT,
            weight: up_node.right,
        });
    }

    if let Some(own_node) = weights.get(current.x, current.y) {
        // add down
        node_queue.push(Connection {
            first: *current,
            second: current.add(0, 1),
            direction: Direction::DOWN,
            weight: own_node.down,
        });

        // add right
        node_queue.push(Connection {
            first: *current,
            second: current.add(1, 0),
            direction: Direction::RIGHT,
            weight: own_node.right,
        });
    }
}

fn create_rand_prim_tree(width: usize, height: usize) -> Matrix<PrimTreeNode> {
    let weights = generate_rand_weights(width, height);
    let mut prim_nodes = generate_empty_prim_graph(width, height);
    let mut current = Point::new(0, 0);
    let mut node_queue = BinaryHeap::new();

    add_all_dirs(&current, &weights, &mut node_queue);

    loop {
        let next_conn = node_queue.pop();
        if next_conn.is_none() {
            break;
        }
        let next_conn = next_conn.unwrap();

        // first node must be in graph
        let second_node = prim_nodes.get(next_conn.second.x, next_conn.second.y);

        // discard stale connections
        if second_node.is_none() || second_node.unwrap().in_tree {
            continue;
        }
        {
            let first_node_mut = prim_nodes
                .get_mut(next_conn.first.x, next_conn.first.y)
                .unwrap();
            match next_conn.direction {
                Direction::UP => first_node_mut.up = true,
                Direction::RIGHT => first_node_mut.right = true,
                Direction::DOWN => first_node_mut.down = true,
                Direction::LEFT => first_node_mut.left = true,
            }
        }

        {
            let second_node_mut = prim_nodes
                .get_mut(next_conn.second.x, next_conn.second.y)
                .unwrap();
            match next_conn.direction {
                Direction::UP => second_node_mut.down = true,
                Direction::RIGHT => second_node_mut.left = true,
                Direction::DOWN => second_node_mut.up = true,
                Direction::LEFT => second_node_mut.right = true,
            }

            second_node_mut.in_tree = true;
        }

        add_all_dirs(&next_conn.second, &weights, &mut node_queue);
    }

    return prim_nodes;
}

fn hamilton_from_prim_nodes(prim_nodes: &Matrix<PrimTreeNode>) -> Matrix<u32> {
    let mut ham_mat: Matrix<u32> = Matrix::new(prim_nodes.get_width() * 2, prim_nodes.get_height() * 2);
    let mut ham_loc = Point::new(0, 0);
    let mut og_loc = Point::new(0, 0);
    let mut id = 0;
    let mut curr_dir = Direction::RIGHT;
    loop {
        ham_mat.set(ham_loc.x, ham_loc.y, id);
        let og_node = prim_nodes.get(og_loc.x, og_loc.y).unwrap();
        match curr_dir {
            Direction::UP => {
                if og_node.left {
                    curr_dir = Direction::LEFT;
                } else {
                    ham_loc = ham_loc.add(0, -1);
                    id += 1;
                    ham_mat.set(ham_loc.x, ham_loc.y, id);
                }
            }
            Direction::RIGHT => {
                if og_node.up {
                    curr_dir = Direction::UP;
                } else {
                    ham_loc = ham_loc.add(1, 0);
                    id += 1;
                    ham_mat.set(ham_loc.x, ham_loc.y, id);
                }
            }
            Direction::DOWN => {
                if og_node.right {
                    curr_dir = Direction::RIGHT;
                } else {
                    ham_loc = ham_loc.add(0, 1);
                    id += 1;
                    ham_mat.set(ham_loc.x, ham_loc.y, id);
                }
            }
            Direction::LEFT => {
                if og_node.down {
                    curr_dir = Direction::DOWN;
                } else {
                    ham_loc = ham_loc.add(-1, 0);
                    id += 1;
                    ham_mat.set(ham_loc.x, ham_loc.y, id);
                }
            }
        }

        match curr_dir {
            Direction::UP => {
                if og_node.up {
                    id += 1;
                    og_loc = og_loc.add(0, -1);
                    ham_loc = ham_loc.add(0, -1);
                } else {
                    curr_dir = Direction::RIGHT;
                }
            }
            Direction::RIGHT => {
                if og_node.right {
                    id += 1;
                    og_loc = og_loc.add(1, 0);
                    ham_loc = ham_loc.add(1, 0);
                } else {
                    curr_dir = Direction::DOWN;
                }
            }
            Direction::DOWN => {
                if og_node.down {   
                    id += 1;
                    og_loc = og_loc.add(0, 1);
                    ham_loc = ham_loc.add(0, 1);
                } else {
                    curr_dir = Direction::LEFT;
                }
            }
            Direction::LEFT => {
                if og_node.left {
                    id += 1;
                    og_loc = og_loc.add(-1, 0);
                    ham_loc = ham_loc.add(-1, 0);
                } else {
                    curr_dir = Direction::UP;
                }
            }
        }

        if ham_loc == Point::new(0, 0) {
            break;
        }
    }

    return ham_mat;
}

pub type HamiltonMatrix = Matrix<u32>;

impl HamiltonMatrix {
    pub fn new_filled(width: usize, height: usize) -> HamiltonMatrix {
        let prim_nodes = create_rand_prim_tree(width/2, height/2);
        // print_prim_graph(&prim_nodes);
        let hamilton = hamilton_from_prim_nodes(&prim_nodes);
        // hamilton.print_matrix();

        return hamilton;
    }
}
