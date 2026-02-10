

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
        
        if nfloor < 0 || nfloor > 5
        {
            return Err("invalid start floor".to_string());
        }
        else {
            Ok(Self { floor: nfloor , queue: Vec::new(), state: State::DoorsOpen})
        }
    }

    
    pub fn call(&mut self, nextfloor: i32) -> Result<String, ElevatorError>{
        match nextfloor {
            d if d < 0 || d > 5 => return Err(ElevatorError::InvalidFloor(nextfloor)),
            val if val == self.floor => return Err(ElevatorError::InvalidFloor(nextfloor)),
            d if d < self.floor => {self.state = State::MovingDown; return Ok("call accepted".to_string())},
            _ => {self.state = State::MovingUp; if Some(nextfloor) != self.queue.last().copied() {self.queue.push(nextfloor)}; return Ok("call accepted".to_string())}
        }
    }

    pub fn state(&self) -> State {
        return self.state;
    }

    pub fn floor(&self) -> i32 {
        return self.floor;
    }

    pub fn queue(&self) -> Vec<i32> {
        return self.queue.clone();
    }

    pub fn open_doors(&self) -> Result<String, ElevatorError> {
        if self.state == State::DoorsOpen {
            return Ok("doors open".to_string());
        }
        else {
            return Err(ElevatorError::CannotOpenWhileMoving);
        }
    }

    pub fn close_doors(&mut self) -> Result<String, ElevatorError> {
        if self.state != State::DoorsOpen {
            return Ok("doors closed".to_string());
        }
        else {
            self.state = State::Idle;
            return Err(ElevatorError::DoorsAlreadyClosed);
        }
    }

    pub fn status(&self) -> Status {
        return Status {floor: self.floor, state: self.state, queue: self.queue.clone()};
    }

    pub fn step(&mut self) -> Result<String, ElevatorError> {
        if self.state != State::Idle && self.state != State::DoorsOpen {
            if self.state == State::MovingUp {
                self.floor += 1;
            }
            else {
                self.floor -= 1;
            }

            if Some(self.floor) == self.queue.first().copied() {
                self.state = State::DoorsOpen;
                self.queue.remove(0);
            }
            return Ok("step".to_string());
        }
        else {
            return Err(ElevatorError::CannotMoveDoorsOpen);
        }
    }
    
}
