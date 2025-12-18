use aoc::runner::{Runner, output};

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    rooms: [Room; 4],
}

impl AocDay {
    pub fn new<S: Into<String>>(input: S) -> Self {
        Self {
            input: input.into(),
            ..Default::default()
        }
    }
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2021, 23)
    }

    fn parse(&mut self) {
        for line in puzlib::read_lines(&self.input) {
            let mut idx = 0;
            for ch in line.chars() {
                if let Some(amph) = match ch {
                    'A' => Some(RoomState::Occupied(Amphipod::Amber)),
                    'B' => Some(RoomState::Occupied(Amphipod::Bronze)),
                    'C' => Some(RoomState::Occupied(Amphipod::Copper)),
                    'D' => Some(RoomState::Occupied(Amphipod::Desert)),
                    _ => None,
                } {
                    self.rooms[idx].occupants.push(amph);
                    idx += 1;
                };
            }
        }
    }

    fn part1(&mut self) -> String {
        let energy = exit_energy(&self.rooms) + entry_energy(2) + solve(&self.rooms);
        output(energy)
    }

    fn part2(&mut self) -> String {
        output("Unsolved")
    }
}

fn exit_energy(rooms: &[Room]) -> usize {
    // Total energy needed to get to the hallway.
    rooms
        .iter()
        .flat_map(|room| {
            room.occupants.iter().enumerate().map(|(depth, amph)| {
                (depth + 1)
                    * if let RoomState::Occupied(a) = amph {
                        a.energy()
                    } else {
                        0
                    }
            })
        })
        .sum()
}

fn entry_energy(depth: usize) -> usize {
    // Total energy to get from the hallway into the rooms
    (depth * (depth + 1) / 2)
        * (Amphipod::Amber.energy()
            + Amphipod::Bronze.energy()
            + Amphipod::Copper.energy()
            + Amphipod::Desert.energy())
}

fn solve(rooms: &[Room; 4]) -> usize {
    let mut stack = vec![(Burrow::new(rooms), 0)];
    let mut min_energy = usize::MAX;

    while let Some((burrow, cost)) = stack.pop() {
        for t in (0..4).rev() {
            if let Some((amph, target, moves)) = burrow.moves(t) {
                for (mask, price) in moves {
                    let mut burrow = burrow.clone();
                    let cost = cost + price;
                    if cost < min_energy {
                        burrow.commit(amph, target, mask);
                        if burrow.hallway.is_empty() {
                            min_energy = cost;
                        } else {
                            stack.push((burrow, cost));
                        }
                    }
                }
            }
        }
    }
    min_energy
}

#[derive(Debug, Default, Copy, Clone)]
struct Hallway {
    amber: u8,
    bronze: u8,
    copper: u8,
    desert: u8,
}

impl Hallway {
    fn flatten(&self) -> u8 {
        self.amber | self.bronze | self.copper | self.desert
    }

    fn is_empty(&self) -> bool {
        self.flatten() == 0
    }
}

#[derive(Debug, Default, Clone)]
struct Burrow {
    rooms: [Room; 4],
    hallway: Hallway,
}

impl Burrow {
    fn new(rooms: &[Room; 4]) -> Self {
        Self {
            rooms: rooms.clone(),
            ..Default::default()
        }
    }
    fn has_path(&self, start: usize, dist: usize) -> bool {
        if dist == 0 {
            true
        } else {
            let mask = ((2 << (dist - 1)) - 1) << (5 - dist - start);
            self.hallway.flatten() & mask == 0
        }
    }

    fn commit(&mut self, amph: Amphipod, target: usize, mask: u8) {
        self.rooms[target].step_out();

        match amph {
            Amphipod::Amber => {
                self.hallway.amber |= mask;
            }
            Amphipod::Bronze => {
                self.hallway.bronze |= mask;
            }
            Amphipod::Copper => {
                self.hallway.copper |= mask;
            }
            Amphipod::Desert => {
                self.hallway.desert |= mask;
            }
        }

        if self.rooms[0].is_empty() {
            self.hallway.amber = 0;
        }
        if self.rooms[1].is_empty() {
            self.hallway.bronze = 0;
        }
        if self.rooms[2].is_empty() {
            self.hallway.copper = 0;
        }
        if self.rooms[3].is_empty() {
            self.hallway.desert = 0;
        }
    }

    fn moves(&self, pos: usize) -> Option<State> {
        let amph = self.rooms[pos].peek()?;

        const COSTS: [usize; 7] = [2, 2, 4, 4, 4, 2, 2];
        let mut moves = Vec::with_capacity(7);
        let hallway = self.hallway.flatten();
        let energy = amph.energy();
        let target = amph as usize;
        let (left, right) = if pos < target {
            (pos, target)
        } else {
            (target, pos)
        };

        let dist = right - left;
        let base_cost = dist * 2;
        let path_available = self.has_path(left, dist);

        if path_available && self.rooms[target].is_empty() {
            moves.push((0, base_cost * energy));
        } else {
            if path_available || pos < target {
                moves.extend(
                    (0..left + 2)
                        .rev()
                        .map(|offset| (1 << (6 - offset), COSTS[offset]))
                        .scan(0, |acc, (mask, weight)| {
                            *acc += weight;
                            let cost = if *acc == weight { 2 } else { *acc };
                            (mask & hallway == 0).then(|| (mask, (base_cost + cost) * energy))
                        }),
                );
            }
            if path_available || pos > target {
                moves.extend(
                    (right + 2..7)
                        .map(|offset| (1 << (6 - offset), COSTS[offset]))
                        .scan(0, |acc, (mask, weight)| {
                            *acc += weight;
                            let cost = if *acc == weight { 2 } else { *acc };
                            (mask & hallway == 0).then(|| (mask, (base_cost + cost) * energy))
                        }),
                );
            }
        };
        Some((amph, pos, moves))
    }
}

type State = (Amphipod, usize, Vec<(u8, usize)>);

#[derive(Default, Debug, Copy, Clone)]
enum RoomState {
    Occupied(Amphipod),
    #[default]
    Empty,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Amphipod {
    fn energy(&self) -> usize {
        10_usize.pow(*self as u32)
    }
}

#[derive(Debug, Clone, Default)]
struct Room {
    occupants: Vec<RoomState>,
    position: usize,
}

impl Room {
    fn peek(&self) -> Option<Amphipod> {
        if self.position < self.occupants.len()
            && let RoomState::Occupied(amph) = self.occupants[self.position]
        {
            Some(amph)
        } else {
            None
        }
    }

    fn step_out(&mut self) {
        if self.position < self.occupants.len() {
            self.occupants[self.position] = RoomState::Empty;
            self.position += 1;
        }
    }

    fn is_empty(&self) -> bool {
        self.peek().is_none()
    }
}
