use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

const INPUT_FILENAME: &'static str = "input";

type CoordSpace = HashMap<String, Space>;

#[derive(Clone, Debug)]
enum Space {
    PathOne,
    PathTwo,
    Intersection,
}

fn main() -> Result<()> {
    let file = File::open(INPUT_FILENAME)?;
    let mut buf = BufReader::new(file);
    let mut line_one = String::new();
    let mut line_two = String::new();
    buf.read_line(&mut line_one)?;
    buf.read_line(&mut line_two)?;
    let path_one: Vec<&str> = line_one.split(",").collect::<Vec<&str>>();
    let path_two: Vec<&str> = line_two.split(",").collect::<Vec<&str>>();
    println!(
        "Part one: {:?}",
        part_one(path_one.clone(), path_two.clone())
    );
    println!("Part two: {:?}", part_two(path_one, path_two));
    Ok(())
}

fn part_one(path_one: Vec<&str>, path_two: Vec<&str>) -> Option<usize> {
    let coord_space = gen_coordinate_space(path_one, path_two);
    return find_shortest_intersection(coord_space);
}

fn part_two(path_one: Vec<&str>, path_two: Vec<&str>) -> Option<usize> {
    let coord_space = gen_coordinate_space(path_one.clone(), path_two.clone());
    let all_intersections = coord_space
        .iter()
        .filter(|(_c, space_type)| match space_type {
            Space::Intersection => true,
            _ => false,
        })
        .collect::<Vec<(&String, &Space)>>();

    let mut shortest = None;
    for (coord, _intersection_type) in all_intersections.iter() {
        let (end_x, end_y) = coord_from_str(coord.to_string());
        let steps_path_one = walk_until(&path_one, end_x, end_y);
        let steps_path_two = walk_until(&path_two, end_x, end_y);
        let total_steps = steps_path_one + steps_path_two;
        match shortest {
            None => {
                shortest = Some(total_steps);
            }
            Some(s2) => {
                if total_steps < s2 {
                    shortest = Some(total_steps);
                }
            }
        }
    }
    return shortest;
}

fn gen_coordinate_space(
    path_one: Vec<&str>,
    path_two: Vec<&str>,
) -> CoordSpace {
    let mut coordinate_space: CoordSpace = HashMap::new();
    walk(path_one, &mut coordinate_space, Space::PathOne);
    walk(path_two, &mut coordinate_space, Space::PathTwo);
    return coordinate_space;
}

fn find_shortest_intersection(coord_space: CoordSpace) -> Option<usize> {
    let mut shortest: Option<usize> = None;
    for (coord, space_type) in coord_space.iter() {
        match space_type {
            Space::Intersection => {
                let dist = manhatten_dist(coord.clone());
                match shortest {
                    None => {
                        shortest = Some(dist);
                    }
                    Some(d2) => {
                        if dist <= d2 {
                            shortest = Some(dist);
                        }
                    }
                }
            }
            _ => {}
        }
    }
    return shortest;
}

fn manhatten_dist(coord: String) -> usize {
    let (x, y) = coord_from_str(coord);
    (x.abs() + y.abs()) as usize
}

fn walk_until(path: &Vec<&str>, end_x: isize, end_y: isize) -> usize {
    const START_X: isize = 0;
    const START_Y: isize = 0;
    let mut pos_x: isize = START_X;
    let mut pos_y: isize = START_Y;
    let mut steps: usize = 1; // lol
    for instr in path {
        let (dir_x, dir_y) = parse_path_instruction(instr.to_string());
        if dir_x != 0 {
            for _ in 0..dir_x.abs() {
                if dir_x >= 0 {
                    pos_x += 1;
                } else {
                    pos_x -= 1;
                }

                if pos_x == end_x && pos_y == end_y {
                    return steps;
                } else {
                    steps += 1;
                }
            }
        }
        if dir_y != 0 {
            for _ in 0..dir_y.abs() {
                if dir_y >= 0 {
                    pos_y += 1;
                } else {
                    pos_y -= 1;
                }

                if pos_x == end_x && pos_y == end_y {
                    return steps;
                } else {
                    steps += 1;
                }
            }
        }
    }
    return steps;
}

fn walk(path: Vec<&str>, coordinate_space: &mut CoordSpace, space_type: Space) {
    const START_X: isize = 0;
    const START_Y: isize = 0;
    let mut pos_x: isize = START_X;
    let mut pos_y: isize = START_Y;
    for instr in path {
        let (dir_x, dir_y) = parse_path_instruction(instr.to_string());
        if dir_x != 0 {
            for _ in 0..dir_x.abs() {
                if dir_x >= 0 {
                    pos_x += 1;
                } else {
                    pos_x -= 1;
                }
                insert_path_space(
                    coordinate_space,
                    space_type.clone(),
                    pos_x,
                    pos_y,
                );
            }
        }
        if dir_y != 0 {
            for _ in 0..dir_y.abs() {
                if dir_y >= 0 {
                    pos_y += 1;
                } else {
                    pos_y -= 1;
                }
                insert_path_space(
                    coordinate_space,
                    space_type.clone(),
                    pos_x,
                    pos_y,
                );
            }
        }
    }
}

fn insert_path_space(
    coordinate_space: &mut CoordSpace,
    current_space_type: Space,
    pos_x: isize,
    pos_y: isize,
) {
    let current_val: Option<&Space> =
        coordinate_space.get(&coord_to_str(pos_x, pos_y));

    let coord = coord_to_str(pos_x, pos_y);

    match (current_val, current_space_type.clone()) {
        (None, _) => {
            let _ = coordinate_space.insert(coord, current_space_type.clone());
        }
        (Some(Space::PathOne), Space::PathTwo) => {
            let _ = coordinate_space.insert(coord, Space::Intersection);
        }
        (Some(Space::PathTwo), Space::PathOne) => {
            let _ = coordinate_space.insert(coord, Space::Intersection);
        }
        _ => {
            () // nothing to do, this is the same path backtracing
        }
    }
}

fn coord_from_str(coord_str: String) -> (isize, isize) {
    let num_strs: Vec<&str> = coord_str.split(",").collect();
    (
        num_strs[0].parse::<isize>().unwrap(),
        num_strs[1].parse::<isize>().unwrap(),
    )
}

fn coord_to_str(pos_x: isize, pos_y: isize) -> String {
    return format!("{},{}", pos_x, pos_y);
}

fn parse_path_instruction(instr: String) -> (isize, isize) {
    let mut chars = instr.chars();
    let dir = chars.next().unwrap();
    let dist_str: String = chars.filter(|c| *c != '\n').collect();
    let dist: isize = dist_str.parse::<isize>().unwrap();

    match dir {
        'U' => (0, dist),
        'D' => (0, -dist),
        'L' => (-dist, 0),
        'R' => (dist, 0),
        _ => panic!("unrecognized direction!"),
    }
}

#[test]
fn example_one() {
    let line_one = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
    let line_two = "U62,R66,U55,R34,D71,R55,D58,R83";
    let path_one: Vec<&str> = line_one.split(",").collect::<Vec<&str>>();
    let path_two: Vec<&str> = line_two.split(",").collect::<Vec<&str>>();
    let shortest = part_one(path_one.clone(), path_two.clone());
    let least_steps = part_two(path_one, path_two);
    assert_eq!(shortest, Some(159));
    assert_eq!(least_steps, Some(610));
}

#[test]
fn example_two() {
    let line_one = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51";
    let line_two = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
    let path_one: Vec<&str> = line_one.split(",").collect::<Vec<&str>>();
    let path_two: Vec<&str> = line_two.split(",").collect::<Vec<&str>>();
    let shortest = part_one(path_one.clone(), path_two.clone());
    let least_steps = part_two(path_one, path_two);
    assert_eq!(shortest, Some(135));
    assert_eq!(least_steps, Some(410));
}
