use std::{iter::zip, ops::Neg};

use anyhow::bail;
use aoc::ProblemOutput;
use itertools::Itertools;

use crate::util::{
    coords::{ij, P2},
    err::NoneErr,
};

aoc::register!(solve, 2022, 22);

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let (raw_map, raw_path) = input.split_once("\n\n").ok_or(NoneErr)?;
    let path = parse_path(raw_path)?;
    let faces = World::parse_faces(raw_map);

    let mut world = World::with_flat_edges(faces.clone());
    world.follow_path(&path);
    out.set_part1(world.read_password());

    let mut world = World::with_cube_edges(faces);
    world.follow_path(&path);
    out.set_part2(world.read_password());

    Ok(())
}

fn parse_path(mut raw_path: &str) -> anyhow::Result<Vec<Action>> {
    let mut path = Vec::new();

    raw_path = raw_path.trim();

    while !raw_path.is_empty() {
        let i = raw_path
            .find(|c| c == 'L' || c == 'R')
            .unwrap_or(raw_path.len());
        if i == 0 {
            let turn = if raw_path.starts_with('L') {
                ij::left_turn()
            } else {
                ij::right_turn()
            };
            path.push(Action::Turn(turn));
            raw_path = &raw_path[i + 1..];
        } else {
            let steps = raw_path[..i].parse()?;
            path.push(Action::Move(steps));
            raw_path = &raw_path[i..];
        }
    }

    Ok(path)
}

#[derive(Debug, Clone, Copy)]
enum Action {
    Move(usize),
    Turn(P2<isize>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Facing {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Facing {
    fn try_from_direction(dir: P2<isize>) -> anyhow::Result<Self> {
        Ok(if dir == ij::right() {
            Self::Right
        } else if dir == ij::down() {
            Self::Down
        } else if dir == ij::left() {
            Self::Left
        } else if dir == ij::up() {
            Self::Up
        } else {
            bail!("Direction does not correspond to a facing: {dir:?}")
        })
    }

    fn to_direction(self) -> P2<isize> {
        match self {
            Facing::Right => ij::right(),
            Facing::Down => ij::down(),
            Facing::Left => ij::left(),
            Facing::Up => ij::up(),
        }
    }

    fn get_index(self, pos: P2<isize>, shape: P2<isize>) -> isize {
        // exit indices are counted counterclockwise:
        //     210
        //    0   2
        //    1   1
        //    2   0
        //     012
        let max_row = shape.0 - 1;
        let max_col = shape.1 - 1;
        match self {
            Facing::Left => pos.0,
            Facing::Right => max_row - pos.0,
            Facing::Down => pos.1,
            Facing::Up => max_col - pos.1,
        }
    }

    fn get_position(self, index: isize, shape: P2<isize>) -> P2<isize> {
        // entry indices are counted clockwise, in order to lign up with exit indices
        //     012
        //    2   0
        //    1   1
        //    0   2
        //     210
        let max_row = shape.0 - 1;
        let max_col = shape.1 - 1;
        match self {
            Facing::Left => P2(max_row - index, 0),
            Facing::Right => P2(index, max_col),
            Facing::Down => P2(max_row, max_col - index),
            Facing::Up => P2(0, index),
        }
    }

    fn iter() -> impl Iterator<Item = Self> + Clone {
        [Self::Right, Self::Down, Self::Left, Self::Up].into_iter()
    }

    fn turn(self, dir: P2<isize>) -> Self {
        Self::try_from_direction(self.to_direction() * dir).unwrap()
    }
}

impl Neg for Facing {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Facing::Right => Facing::Left,
            Facing::Down => Facing::Up,
            Facing::Left => Facing::Right,
            Facing::Up => Facing::Down,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Open,
    Wall,
}

#[derive(Debug, Clone)]
struct Face {
    tiles: Vec<Vec<Tile>>,
    edges: [Option<(Facing, P2<isize>)>; 4],
}

impl Face {
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            tiles: vec![vec![Tile::Open; cols]; rows],
            edges: [None; 4],
        }
    }

    fn shape(&self) -> P2<isize> {
        let rows = self.tiles.len() as isize;
        let cols = self.tiles[0].len() as isize;
        P2(rows, cols)
    }
}

struct World {
    faces: Vec<Vec<Option<Face>>>,
    dir: P2<isize>,
    pos: P2<isize>,
    pos_face: P2<isize>,
}

impl World {
    #[allow(clippy::cast_precision_loss)]
    fn parse_faces(raw_map: &str) -> Vec<Vec<Option<Face>>> {
        let total_tiles = raw_map.bytes().filter(|&c| c == b'.' || c == b'#').count();
        let tiles_per_face = total_tiles / 6;
        let side_length = (tiles_per_face as f64).sqrt() as usize;
        assert_eq!(side_length * side_length, tiles_per_face);

        let rows = raw_map.lines().count() / side_length;
        let cols = raw_map.lines().map(str::len).max().unwrap_or(0) / side_length;

        let mut faces = vec![vec![None; cols]; rows];

        for (i, line) in raw_map.lines().enumerate() {
            for (j, val) in line.bytes().enumerate() {
                let tile = match val {
                    b'#' => Tile::Wall,
                    b'.' => Tile::Open,
                    _ => continue,
                };
                let iface = i / side_length;
                let jface = j / side_length;
                let ii = i % side_length;
                let jj = j % side_length;
                faces[iface][jface]
                    .get_or_insert_with(|| Face::new(side_length, side_length))
                    .tiles[ii][jj] = tile;
            }
        }

        faces
    }

    fn new(faces: Vec<Vec<Option<Face>>>) -> Self {
        let initial_col = faces[0].iter().position(Option::is_some).unwrap();
        World {
            faces,
            dir: ij::right(),
            pos: P2(0, 0),
            pos_face: P2(0, initial_col as isize),
        }
    }

    fn with_flat_edges(faces: Vec<Vec<Option<Face>>>) -> Self {
        let mut world = Self::new(faces);

        for i in 0..world.rows() {
            let valid_cols = (0..world.cols())
                .filter(|&j| world.faces[i as usize][j as usize].is_some())
                .collect_vec();
            for (&j1, &j2) in zip(valid_cols.iter(), valid_cols.iter().cycle().skip(1)) {
                world.connect_faces(P2(i, j1), Facing::Right, P2(i, j2), Facing::Right);
            }
        }
        for j in 0..world.cols() {
            let valid_rows = (0..world.rows())
                .filter(|&i| world.faces[i as usize][j as usize].is_some())
                .collect_vec();
            for (&i1, &i2) in zip(valid_rows.iter(), valid_rows.iter().cycle().skip(1)) {
                world.connect_faces(P2(i1, j), Facing::Down, P2(i2, j), Facing::Down);
            }
        }

        world
    }

    fn with_cube_edges(faces: Vec<Vec<Option<Face>>>) -> Self {
        let num_faces = faces.iter().flat_map(|row| row.iter()).flatten().count();
        let mut missing_edges = 2 * num_faces;
        let mut world = Self::new(faces);
        let positions_and_facings = (0..world.rows())
            .cartesian_product(0..world.cols())
            .map(|(i, j)| P2(i, j))
            .filter(|&pos_face| world.get_face(pos_face).is_some())
            .cartesian_product(Facing::iter())
            .collect_vec();

        // Connect adjacent faces
        for &(pos_face_1, facing) in &positions_and_facings {
            let pos_face_2 = pos_face_1 + facing.to_direction();
            if world.connect_faces(pos_face_1, facing, pos_face_2, facing) {
                missing_edges -= 1;
            }
        }

        while missing_edges > 0 {
            for &(pos_face_1, facing1) in &positions_and_facings {
                for turn in [ij::left_turn(), ij::right_turn()] {
                    if world
                        .connect_cube_faces(pos_face_1, facing1, turn)
                        .is_some()
                    {
                        missing_edges -= 1;
                    }
                }
            }
        }
        world
    }

    fn connect_cube_faces(
        &mut self,
        pos_face_1: P2<isize>,
        facing1: Facing,
        turn: P2<isize>,
    ) -> Option<()> {
        // Connect "diagonal" faces:
        // face 1 (left edge) is connected to face 2 (bottom edge)
        //    2X
        //     1
        let face1 = self.get_face(pos_face_1)?;
        let facing_1x_exit = facing1.turn(-turn);
        let (facing_1x_entry, pos_face_x) = face1.edges[facing_1x_exit as usize]?;

        let facex = self.get_face(pos_face_x)?;
        let facing_x2_exit = facing_1x_entry.turn(turn);
        let (facing_x2_entry, pos_face_2) = facex.edges[facing_x2_exit as usize]?;

        let facing2 = -facing_x2_entry.turn(turn);
        self.connect_faces(pos_face_1, facing1, pos_face_2, facing2)
            .then_some(())
    }

    fn rows(&self) -> isize {
        self.faces.len() as isize
    }

    fn cols(&self) -> isize {
        self.faces[0].len() as isize
    }

    fn global_pos(&self) -> P2<isize> {
        let face_shape = self.get_face(self.pos_face).unwrap().shape();
        let i = self.pos_face.0 * face_shape.0 + self.pos.0;
        let j = self.pos_face.1 * face_shape.1 + self.pos.1;
        P2(i, j)
    }

    fn get_face(&self, pos_face: P2<isize>) -> Option<&Face> {
        self.faces
            .get(pos_face.0 as usize)?
            .get(pos_face.1 as usize)?
            .as_ref()
    }

    fn get_face_mut(&mut self, pos_face: P2<isize>) -> Option<&mut Face> {
        self.faces
            .get_mut(pos_face.0 as usize)?
            .get_mut(pos_face.1 as usize)?
            .as_mut()
    }

    fn get_tile(&self, pos_face: P2<isize>, pos: P2<isize>) -> Option<Tile> {
        // This code returns None for negative indices (`pos.X as usize`
        // is larger than the map size).

        self.get_face(pos_face)?
            .tiles
            .get(pos.0 as usize)?
            .get(pos.1 as usize)
            .copied()
    }

    fn connect_faces(
        &mut self,
        pos_face_1: P2<isize>,
        facing1: Facing,
        pos_face_2: P2<isize>,
        facing2: Facing,
    ) -> bool {
        for (pos_face, facing) in [(pos_face_1, facing1), (pos_face_2, -facing2)] {
            if self
                .get_face(pos_face)
                .map_or(true, |face| face.edges[facing as usize].is_some())
            {
                return false;
            }
        }

        let face1 = self.get_face_mut(pos_face_1).unwrap();
        face1.edges[facing1 as usize] = Some((facing2, pos_face_2));

        let face2 = self.get_face_mut(pos_face_2).unwrap();
        face2.edges[(-facing2) as usize] = Some((-facing1, pos_face_1));

        true
    }

    fn turn(&mut self, turn: P2<isize>) {
        self.dir *= turn;
    }

    fn step(&mut self) -> bool {
        let new_pos = self.pos + self.dir;
        if self.get_tile(self.pos_face, new_pos).is_some() {
            return self.step_to(self.pos_face, new_pos);
        }

        let iface = self.pos_face.0 as usize;
        let jface = self.pos_face.1 as usize;
        let face = self.faces[iface][jface].as_ref().unwrap();
        let exit_facing = Facing::try_from_direction(self.dir).unwrap();
        let index = exit_facing.get_index(self.pos, face.shape());

        let Some((entry_facing, pos_face)) =
            self.faces[iface][jface].as_ref().unwrap().edges[exit_facing as usize] else {
                return false;
            };
        let new_pos = (-entry_facing).get_position(index, face.shape());
        if self.step_to(pos_face, new_pos) {
            self.dir = entry_facing.to_direction();
            true
        } else {
            false
        }
    }

    fn step_to(&mut self, pos_face: P2<isize>, pos: P2<isize>) -> bool {
        if let Some(tile) = self.get_tile(pos_face, pos) {
            if tile == Tile::Open {
                self.pos = pos;
                self.pos_face = pos_face;
                return true;
            }
        }
        false
    }

    fn follow_path(&mut self, path: &[Action]) {
        for action in path {
            match *action {
                Action::Turn(t) => self.turn(t),
                Action::Move(steps) => {
                    for _ in 0..steps {
                        if !self.step() {
                            break;
                        }
                    }
                }
            }
        }
    }

    fn read_password(&self) -> isize {
        let P2(i, j) = self.global_pos();
        let facing = Facing::try_from_direction(self.dir).unwrap();
        1000 * (i + 1) + 4 * (j + 1) + (facing as isize)
    }
}
