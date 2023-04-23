pub mod coordinate {
    #[derive(Clone, Debug)]
    pub struct Coordinate {
        pub x: i16,
        pub y: i16,
    }

    impl Coordinate {
        pub fn new(x: i16, y: i16) -> Self {
            Coordinate { x, y }
        }

        pub fn get_x(&self) -> i16 {
            self.x
        }

        pub fn get_y(&self) -> i16 {
            self.y
        }
    }

    // Implementierung der Default-Trait für Coordinate
    impl Default for Coordinate {
        fn default() -> Self {
            Self { x: -1, y: -1 }
        }
    }
}

pub mod movement_not_possible {
    use std::fmt;

    #[derive(Debug)]
    pub struct MovementNotPossible {
        pub message: String,
    }

    impl MovementNotPossible {
        pub fn new(message: &str) -> Self {
            Self {
                message: message.to_string(),
            }
        }
    }

    impl fmt::Display for MovementNotPossible {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.message)
        }
    }

    impl std::error::Error for MovementNotPossible {}
}

pub mod traffic_area {
    use crate::coordinate::Coordinate;
    use crate::movement_not_possible::MovementNotPossible;

    pub struct TrafficArea {
        area: Vec<Vec<Vec<i16>>>,
    }

    impl TrafficArea {
        pub fn new(max_per_node: usize, max_size_x: usize, max_size_y: usize) -> Self {
            let area = vec![vec![vec![-1; max_per_node]; max_size_y]; max_size_x];
            TrafficArea { area }
        }

        pub fn remove(&mut self, id: i16, from: &Coordinate) -> Result<(), MovementNotPossible> {
            let x = from.get_x() as usize;
            let y = from.get_y() as usize;

            let position = self.area[x][y]
                .iter_mut()
                .position(|client_id| *client_id == id);

            match position {
                Some(pos) => {
                    self.area[x][y][pos] = -1;
                    Ok(())
                }
                None => Err(MovementNotPossible::new("id not found at start")),
            }
        }

        pub fn place(&mut self, id: i16, to: &Coordinate) -> Result<(), MovementNotPossible> {
            let x = to.get_x() as usize;
            let y = to.get_y() as usize;

            let position = self.area[x][y]
                .iter()
                .position(|client_id| *client_id == -1);

            match position {
                Some(pos) => {
                    if self.area[x][y][pos] == id {
                        Err(MovementNotPossible::new(
                            "id already placed at target position",
                        ))
                    } else {
                        self.area[x][y][pos] = id;
                        Ok(())
                    }
                }
                None => Err(MovementNotPossible::new("no empty space left")),
            }
        }

        pub fn get_position(&self, id: i16) -> Option<Coordinate> {
            for x in 0..self.area.len() {
                for y in 0..self.area[x].len() {
                    if let Some(_) = self.area[x][y]
                        .iter()
                        .position(|client_id| *client_id == id)
                    {
                        return Some(Coordinate::new(x as i16, y as i16));
                    }
                }
            }
            None
        }

        pub fn is_free(&self, position: &Coordinate) -> bool {
            let x = position.get_x() as usize;
            let y = position.get_y() as usize;

            self.area[x][y].iter().any(|client_id| *client_id == -1)
        }

        pub fn get_area(&self) -> &Vec<Vec<Vec<i16>>> {
            &self.area
        }

        pub fn set_area(&mut self, area: Vec<Vec<Vec<i16>>>) {
            self.area = area;
        }

        pub fn clear(&mut self) {
            for x in 0..self.area.len() {
                for y in 0..self.area[x].len() {
                    for client_id_pos in 0..self.area[x][y].len() {
                        self.area[x][y][client_id_pos] = -1;
                    }
                }
            }
        }
    }

    impl std::fmt::Display for TrafficArea {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(f, "############## AREA ###############")?;
            for y in 0..self.area[0].len() {
                for x in 0..self.area.len() {
                    let mut to_print = String::new();
                    for i in 0..self.area[x][y].len() {
                        to_print.push_str(&format!(" {}", self.area[x][y][i]));
                    }
                    write!(f, "|{: ^5}|", to_print)?;
                }
                writeln!(f)?;
            }
            writeln!(f, "############## AREA END ###############")
        }
    }
}

pub mod traffic_control_logic {
    use crate::coordinate::Coordinate;
    use crate::movement_not_possible::MovementNotPossible;
    use crate::traffic_area::TrafficArea;

    pub struct TrafficControlLogic {
        traffic_area: TrafficArea,
    }

    impl TrafficControlLogic {
        pub fn new(traffic_area: TrafficArea) -> Self {
            Self { traffic_area }
        }

        pub fn start(&mut self, id: i16) -> Result<Coordinate, MovementNotPossible> {
            let current_position = self.traffic_area.get_position(id);
            if let Some(_position) = current_position {
                Err(MovementNotPossible::new("client already available"))
            } else {
                for y in 0..self.traffic_area.get_area()[0].len() {
                    let pos = Coordinate::new(0, y as i16);
                    if self.traffic_area.is_free(&pos) {
                        self.traffic_area.place(id, &pos)?;
                        return Ok(pos);
                    }
                }
                Err(MovementNotPossible::new("no free position found"))
            }
        }

        pub fn move_to(
            &mut self,
            id: i16,
            target_to_reach: Coordinate,
        ) -> Result<Coordinate, MovementNotPossible> {
            let current_position = self
                .traffic_area
                .get_position(id)
                .ok_or_else(|| MovementNotPossible::new("client not available"))?;

            let mut best_coordinate = current_position.clone();
            let mut distance = Self::get_distance(&best_coordinate, &target_to_reach);

            for x_offset in -1..=1 {
                for y_offset in -1..=1 {
                    let x = (current_position.x + x_offset)
                        .max(0)
                        .min((self.traffic_area.get_area().len() - 1) as i16);
                    let y = (current_position.y + y_offset)
                        .max(0)
                        .min((self.traffic_area.get_area()[0].len() - 1) as i16);

                    let coordinate_to_check = Coordinate::new(x, y);
                    if self.traffic_area.is_free(&coordinate_to_check) {
                        let new_distance =
                            Self::get_distance(&coordinate_to_check, &target_to_reach);
                        if new_distance < distance {
                            distance = new_distance;
                            best_coordinate = coordinate_to_check;
                        }
                    }
                }
            }

            self.traffic_area.remove(id, &current_position)?;
            self.traffic_area.place(id, &best_coordinate)?;

            Ok(best_coordinate)
        }

        fn get_distance(first_coordinate: &Coordinate, second_coordinate: &Coordinate) -> f64 {
            let x1 = first_coordinate.x as f64;
            let x2 = second_coordinate.x as f64;
            let y1 = first_coordinate.y as f64;
            let y2 = second_coordinate.y as f64;

            if x1 == x2 && y1 == y2 {
                0.0
            } else {
                ((x1 - x2).powf(2.0) + (y1 - y2).powf(2.0)).sqrt()
            }
        }

        pub fn get_traffic_area(&self) -> &TrafficArea {
            &self.traffic_area
        }
    }
}
