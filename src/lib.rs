#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Idle,
    MovingUp,
    MovingDown,
    DoorsOpen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElevatorError {
    CannotMoveDoorsOpen,
    InvalidFloor(i32),
    CannotOpenWhileMoving,
    DoorsAlreadyClosed,
    EmptyQueue,
}

pub struct Elevator {
    floor: i32,
    queue: Vec<i32>,
    state: State,
}

pub struct Status {
    pub floor: i32,
    pub queue: Vec<i32>,
    pub state: State,
}

impl Elevator {
    pub fn new(nfloor: i32) -> Result<Elevator, String> {
        if !(0..=5).contains(&nfloor) {
            Err("invalid start floor".to_string())
        } else {
            Ok(Self {
                floor: nfloor,
                queue: Vec::new(),
                state: State::Idle,
            })
        }
    }

    pub fn call(&mut self, nextfloor: i32) -> Result<String, ElevatorError> {
        match nextfloor {
            d if !(0..=5).contains(&d) => Err(ElevatorError::InvalidFloor(nextfloor)),
            val if val == self.floor => Ok("same floor accepter".to_string()),
            d if d < self.floor => {
                self.state = State::MovingDown;
                if Some(&nextfloor) != self.queue.last() {
                    self.queue.push(nextfloor)
                };
                Ok("call accepted".to_string())
            }
            _ => {
                self.state = State::MovingUp;
                if Some(&nextfloor) != self.queue.last() {
                    self.queue.push(nextfloor)
                };
                Ok("call accepted".to_string())
            }
        }
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn floor(&self) -> i32 {
        self.floor
    }

    pub fn queue(&self) -> Vec<i32> {
        self.queue.clone()
    }

    pub fn open_doors(&mut self) -> Result<String, ElevatorError> {
        if self.state == State::Idle {
            self.state = State::DoorsOpen;
            Ok("doors open".to_string())
        } else {
            Err(ElevatorError::CannotOpenWhileMoving)
        }
    }

    pub fn close_doors(&mut self) -> Result<String, ElevatorError> {
        if self.state == State::DoorsOpen {
            if self.queue.is_empty() {
                self.state = State::Idle;
            } else {
                if self.queue.first() < Some(&self.floor) {
                    self.state = State::MovingDown;
                    dbg!("Descente");
                } else {
                    self.state = State::MovingUp;
                }
            }
            Ok("close doors".to_string())
        } else {
            Err(ElevatorError::DoorsAlreadyClosed)
        }
    }

    pub fn status(&self) -> Status {
        Status {
            floor: self.floor,
            state: self.state,
            queue: self.queue.clone(),
        }
    }

    pub fn step(&mut self) -> Result<String, ElevatorError> {
        if self.state != State::DoorsOpen {
            if !self.queue.is_empty() {
                if self.state == State::MovingUp {
                    self.floor += 1;
                } else {
                    self.floor -= 1;
                }

                if Some(self.floor) == self.queue.first().copied() {
                    self.state = State::DoorsOpen;
                    self.queue.remove(0);
                }
                Ok("step".to_string())

            } else {
                    Err(ElevatorError::EmptyQueue)
            }
        } else {
            Err(ElevatorError::CannotMoveDoorsOpen)
        }
    }
}
