use nannou::prelude::*;

pub enum TaskType {
    PairMovement,
    SingleMovement,
    PairMovementShift,
    Wiggle,
    Spin,
    GetClose,
    GetAway,
    TargetAngle,
    TargetAngles,
    Wait,
}

pub struct ToioTask {
    pub what: TaskType,
    pub duration: Option<u64>,
    pub targets: Option<(Vector2, Vector2)>,
    pub target: Option<Vector2>,
    pub target_angle: Option<f32>,
    pub target_angles: Option<(f32, f32)>,
    pub power: Option<f32>,
    pub distance: Option<f32>,
    pub start_time: u64,
}
impl ToioTask {
    pub fn new_pair_move(x0: f32, y0: f32, x1: f32, y1: f32, d: f32) -> Self {
        ToioTask {
            what: TaskType::PairMovement,
            duration: None,
            targets: Some((vec2(x0, y0), vec2(x1, y1))),
            target: None,
            target_angle: None,
            target_angles: None,
            power: None,
            distance: Some(d),
            start_time: 0,
        }
    }
    pub fn new_pair_move_shift(x0: f32, y0: f32, d: f32) -> Self {
        ToioTask {
            what: TaskType::PairMovementShift,
            duration: None,
            targets: None,
            target: Some(vec2(x0, y0)),
            target_angle: None,
            target_angles: None,
            power: None,
            distance: Some(d),
            start_time: 0,
        }
    }
    pub fn new_single_move(x0: f32, y0: f32, d: f32) -> Self {
        ToioTask {
            what: TaskType::SingleMovement,
            duration: None,
            targets: None,
            target: Some(vec2(x0, y0)),
            target_angle: None,
            target_angles: None,
            power: None,
            distance: Some(d),
            start_time: 0,
        }
    }
    pub fn new_wait(t: u64) -> Self {
        ToioTask {
            what: TaskType::Wait,
            duration: Some(t),
            targets: None,
            target: None,
            target_angle: None,
            target_angles: None,
            power: None,
            distance: None,
            start_time: 0,
        }
    }
    pub fn new_wiggle(t: u64, power: f32) -> Self {
        ToioTask {
            what: TaskType::Wiggle,
            duration: Some(t),
            targets: None,
            target: None,
            target_angle: None,
            target_angles: None,
            power: Some(power),
            distance: None,
            start_time: 0,
        }
    }
    pub fn new_spin(t: u64, power: f32) -> Self {
        ToioTask {
            what: TaskType::Spin,
            duration: Some(t),
            targets: None,
            target: None,
            target_angle: None,
            target_angles: None,
            power: Some(power),
            distance: None,
            start_time: 0,
        }
    }
    pub fn new_get_close(d: f32) -> Self {
        ToioTask {
            what: TaskType::GetClose,
            duration: None,
            targets: None,
            target: None,
            target_angle: None,
            target_angles: None,
            power: None,
            distance: Some(d),
            start_time: 0,
        }
    }
    pub fn new_get_away(d: f32) -> Self {
        ToioTask {
            what: TaskType::GetAway,
            duration: None,
            targets: None,
            target: None,
            target_angle: None,
            target_angles: None,
            power: None,
            distance: Some(d),
            start_time: 0,
        }
    }
    pub fn new_target_angle(a0: f32, d: f32) -> Self {
        ToioTask {
            what: TaskType::TargetAngle,
            duration: None,
            targets: None,
            target: None,
            target_angle: Some(a0),
            target_angles: None,
            power: None,
            distance: Some(d),
            start_time: 0,
        }
    }
    pub fn new_target_angles(a0: f32, a1: f32, d: f32) -> Self {
        ToioTask {
            what: TaskType::TargetAngles,
            duration: None,
            targets: None,
            target: None,
            target_angle: None,
            target_angles: Some((a0, a1)),
            power: None,
            distance: Some(d),
            start_time: 0,
        }
    }
    pub fn start(&mut self, now: u64) {
        self.start_time = now;
    }
    pub fn is_done(&self, now: u64, data: Vec<(f32, f32, f32)>) -> bool {
        //x0:f32, y0:f32, angle0:f32,x1:f32, y1:f32, angle1:f32, x2:f32, y2) -> bool {
        let (mut x0, mut y0, mut angle0) = data[0];
        let (mut x1, mut y1, mut angle1) = data[0];
        let (mut x2, mut y2, mut angle2) = data[0];
        if data.len() > 1 {
            x1 = data[1].0;
            y1 = data[1].1;
            angle1 = data[1].2;
        }
        if data.len() > 2 {
            x2 = data[2].0;
            y2 = data[2].1;
            angle2 = data[2].2;
        }

        match self.what {
            //PensUp, //>=102
            //PensDown, //<=98
            TaskType::Wiggle | TaskType::Spin | TaskType::Wait => {
                let time_needed = self.duration.unwrap();
                now > self.start_time + time_needed
            }
            TaskType::PairMovement => {
                let d_wanted = self.distance.unwrap();
                let (t0, t1) = self.targets.unwrap();
                //are we far away from our targets?
                let d0 = (t0 - vec2(x0, y0)).magnitude();
                let d1 = (t1 - vec2(x1, y1)).magnitude();
                d0.max(d1) < d_wanted
            }
            TaskType::PairMovementShift => {
                let d_wanted = self.distance.unwrap();
                let t0 = self.target.unwrap();
                //are we far away from our targets?
                let d0 = (t0 - vec2(x0 + x1, y0 + y1) * 0.5).magnitude();
                d0 < d_wanted
            }
            TaskType::SingleMovement => {
                let d_wanted = self.distance.unwrap();
                let t0 = self.target.unwrap();
                //are we far away from our targets?
                let d0 = (t0 - vec2(x0, y0)).magnitude();
                d0 < d_wanted
            }
            TaskType::GetClose => {
                let d = vec2(x1 - x0, y1 - y0).magnitude();
                let d_wanted = self.distance.unwrap();
                d < d_wanted
            }
            TaskType::GetAway => {
                let d = vec2(x1 - x0, y1 - y0).magnitude();
                let d_wanted = self.distance.unwrap();
                d > d_wanted
            }
            TaskType::TargetAngle => {
                let t0 = self.target_angle.unwrap();
                let mut dangle = angle0 - t0;
                while dangle < -PI {
                    dangle += TAU;
                }
                while dangle > PI {
                    dangle -= TAU;
                }
                let d_wanted = self.distance.unwrap();
                dangle.abs() < d_wanted
            }
            TaskType::TargetAngles => {
                let (t0, t1) = self.target_angles.unwrap();
                let mut dangle0 = angle0 - t0;
                while dangle0 < -PI {
                    dangle0 += TAU;
                }
                while dangle0 > PI {
                    dangle0 -= TAU;
                }
                let mut dangle1 = angle1 - t1;
                while dangle1 < -PI {
                    dangle1 += TAU;
                }
                while dangle1 > PI {
                    dangle1 -= TAU;
                }
                let d_wanted = self.distance.unwrap();
                dangle0.abs().max(dangle1.abs()) < d_wanted
            }
            _ => true,
        }
    }
}
