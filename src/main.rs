use nannou::prelude::*;
use nannou_osc as osc;
use nannou_osc::Type;
use std::collections::HashMap;

mod toiotask;
use toiotask::TaskType;
use toiotask::ToioTask;


const CENTER_X: i32 = 250;
const CENTER_Y: i32 = 250;

const PORT: u16 = 3333;
const TARGET_PORT: u16 = 3334;
const DIST_CLOSE: f32 = 52.0;
const DIST_AWAY: f32 = 56.0;
fn main() {
    nannou::app(model).update(update).run();
}

struct CubeData {
    x: i32,
    y: i32,
    angle: i32,
    last: u64,
}
struct Model {
    panic: bool,
    panic_time: u64,
    receiver: osc::Receiver,
    sender: osc::Sender<osc::Connected>,
    toio: HashMap<usize, CubeData>,
    auto_turn: bool,
    go: bool,
    aim_close: bool,
    aim_away: bool,
    aim_target: bool,
    wiggle: bool,
    start_wiggle: u64,
    shift0: Vector2,
    shift1: Vector2,
    tasks: Vec<ToioTask>,
    should_stay_away: bool,
    indices: [usize; 2],
}

fn target_address_string() -> String {
    format!("{}:{}", "127.0.0.1", TARGET_PORT)
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .view(view)
        .size(1024, 1024)
        .event(window_event)
        .build()
        .unwrap();
    let receiver = osc::receiver(PORT).unwrap();
    let target_addr = target_address_string();
    let sender = osc::sender().unwrap().connect(target_addr).unwrap();

    let toio = HashMap::new();

    let shift0 = vec2(0.0, 0.0);
    let shift1 = vec2(0.0, 0.0);

    let mut tasks = Vec::new();

    tasks.push(ToioTask::new_get_close(DIST_CLOSE));
    for _k in 0..1 {
        let ni = 4;
        let nj = 6;
        for i in 0..ni {
            let fi = i as f32 / ni as f32;
            for j in 0..nj {
                let fj = j as f32 / nj as f32;
                let angle = fj * TAU;
                let r = (0.4 + 0.6 * (1.0 - fi)) * 80.0;
                tasks.push(ToioTask::new_pair_move_shift(
                    angle.cos() * r,
                    angle.sin() * r,
                    15.0,
                ));
                if i + j == 0 {
                    tasks.push(ToioTask::new_get_away(DIST_AWAY));
                }

            }
        }
        tasks.push(ToioTask::new_get_close(DIST_CLOSE));
    }

    let indices = [0, 1];
    Model {
        panic: false,
        panic_time: 0,
        receiver,

        sender,
        toio,
        auto_turn: false,
        go: false,
        aim_close: false,
        aim_away: false,
        aim_target: false,
        wiggle: false,
        start_wiggle: 0,
        shift0,
        shift1,
        tasks,
        should_stay_away: false,
        indices,
    }
}
fn window_event(app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(key) => {
            println!("{}", app.elapsed_frames());
            if let Key::C = key {
                model.aim_close = !model.aim_close;
                model.aim_away = false;
            }
            if let Key::D = key {
                model.aim_away = !model.aim_away;
                model.aim_close = false;
            }
            if let Key::T = key {
                model.aim_target = !model.aim_target;
                model.aim_close = false;
                model.aim_away = false;
            }
            if let Key::W = key {
                model.wiggle = !model.wiggle;
                model.start_wiggle = app.duration.since_start.as_millis() as u64;
            }

            let addr = "/motor";
            if let Key::Q = key {
                model.panic = true;
                model.panic_time = app.duration.since_start.as_millis() as u64;
                let args = vec![
                    Type::Int(0),
                    Type::Int(100),
                    Type::Int(-100),
                    Type::Int(500),
                ];
                model.sender.send((addr, args)).ok();
            }
            if let Key::I = key {
                let args = vec![Type::Int(1), Type::Int(20), Type::Int(20), Type::Int(20)];
                model.sender.send((addr, args)).ok();
            }
            if let Key::K = key {
                let args = vec![Type::Int(1), Type::Int(-20), Type::Int(-20), Type::Int(20)];
                model.sender.send((addr, args)).ok();
            }
            if let Key::O = key {
                let args = vec![Type::Int(2), Type::Int(20), Type::Int(20), Type::Int(20)];
                model.sender.send((addr, args)).ok();
            }
            if let Key::L = key {
                let args = vec![Type::Int(2), Type::Int(-20), Type::Int(-20), Type::Int(20)];
                model.sender.send((addr, args)).ok();
            }
            if let Key::W = key {
                let args = vec![Type::Int(0), Type::Int(30), Type::Int(30), Type::Int(50)];
                model.sender.send((addr, args)).ok();
            }
            if let Key::S = key {
                let args = vec![Type::Int(0), Type::Int(-30), Type::Int(-30), Type::Int(50)];
                model.sender.send((addr, args)).ok();
            }
            if let Key::A = key {
                let args = vec![Type::Int(0), Type::Int(-20), Type::Int(20), Type::Int(50)];
                model.sender.send((addr, args)).ok();
            }
            if let Key::D = key {
                let args = vec![Type::Int(0), Type::Int(20), Type::Int(-20), Type::Int(50)];
                model.sender.send((addr, args)).ok();
            }
            if let Key::A = key {
                model.auto_turn = !model.auto_turn;
            }
        }
        KeyReleased(_key) => {}
        MouseMoved(_pos) => {}
        MousePressed(_button) => {
            model.aim_away = false;
            model.aim_close = false;
            model.go = true;
        }
        MouseReleased(_button) => {
            model.go = false;
        }
        MouseEntered => {}
        MouseExited => {}
        MouseWheel(_amount, _phase) => {}
        Moved(_pos) => {}
        Resized(_size) => {}
        Touch(_touch) => {}
        TouchPressure(_pressure) => {}
        HoveredFile(_path) => {}
        DroppedFile(_path) => {}
        HoveredFileCancelled => {}
        Focused => {}
        Unfocused => {}
        Closed => {}
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    for (packet, addr) in model.receiver.try_iter() {
        //println!("{:?}", packet);
        for message in packet.into_msgs().iter() {
            let mut marg = [0; 7];
            match message.addr.as_ref() {
                "/position" => {
                    if let Some(args) = &message.args {
                        if args.len() == 7 {
                            for k in 0..7 {
                                if let nannou_osc::Type::Int(i) = args[k] {
                                    marg[k] = i;
                                }
                            }
                            //update info in our cube map
                            //host id, cube id, x,y,angle, real x, real y, real angle
                            //do we have this cube already?
                            let now = app.duration.since_start.as_millis() as u64;
                            let toio_index = marg[1] as usize;
                            let index = model.indices[toio_index];
                            match model.toio.get_mut(&index) {
                                Some(toio) => {
                                    //println!("{} {}", marg[2], marg[3]);
                                    let mut new_x = marg[2] as i32 - CENTER_X;
                                    let mut new_y = CENTER_Y - marg[3] as i32;

                                    let new_angle = -marg[4] as i32;
                                    toio.angle = new_angle;
                                    toio.x = new_x;
                                    toio.y = new_y;
                                    toio.last = now;
                                }
                                None => {
                                    //insert
                                    model.toio.insert(
                                        index,
                                        CubeData {
                                            x: marg[2] as i32 - CENTER_X,
                                            y: CENTER_Y - marg[3] as i32,
                                            angle: -marg[4] as i32,
                                            last: now,
                                        },
                                    );
                                }
                            }
                        }
                    }
                }
                "/button" => {
                    if let Some(args) = &message.args {
                        if args.len() == 3 {
                            for k in 0..3 {
                                if let nannou_osc::Type::Int(i) = args[k] {
                                    marg[k] = i;
                                }
                            }
                            let index = marg[1] as usize;
                            let button_pressed = marg[2] as usize;
                            //do something with index of button

                        }
                    }
                }
                _ => { }
            }
        }
    }

    //the logic
    let now = app.duration.since_start.as_millis() as u64;

    let cube0o = model.toio.get(&0);
    let cube1o = model.toio.get(&1);
    let cube2o = model.toio.get(&2);

    if let Some(cube0) = cube0o {
        let angle0 = cube0.angle as f32 / 360.0 * TAU;
        let x0 = cube0.x as f32;
        let y0 = cube0.y as f32;
        let mut last = cube0.last;
        if let Some(cube1) = cube1o {
            let angle1 = cube1.angle as f32 / 360.0 * TAU;
            let x1 = cube1.x as f32;
            let y1 = cube1.y as f32;
            last = cube1.last.max(last);
            let cx = (x0 + x1) * 0.5;
            let cy = (y0 + y1) * 0.5;
            //face the other
            let mut dangle = angle0 - (angle1 + PI);
            while dangle > PI {
                dangle -= TAU;
            }
            while dangle < -PI {
                dangle += TAU;
            }
            //println!("angle {}",dangle);

            //get the task at hand
            if model.tasks.len() > 0 {
                let task = &model.tasks[0];
                let mut data = Vec::new();
                data.push((x0, y0, angle0));
                data.push((x1, y1, angle1));
                let mut x2 = 0.0;
                let mut y2 = 0.0;
                let mut angle2 = 0.0;
                if let Some(cube2) = cube2o {
                    angle2 = cube2.angle as f32 / 360.0 * TAU;
                    x2 = cube2.x as f32;
                    y2 = cube2.y as f32;
                    last = cube2.last.max(last);
                    data.push((x2, y2, angle2));
                }

                let d = ((x1 - x0) * (x1 - x0) + (y1 - y0) * (y1 - y0)).sqrt();
                let type_now = &task.what;
                if model.should_stay_away == true
                    && d < DIST_AWAY - 3.0
                    && matches!(TaskType::GetAway, type_now) == false
                {
                    model
                        .tasks
                        .insert(0, ToioTask::new_get_away(DIST_AWAY - 3.0));
                } else {
                    if task.is_done(now, data) {
                        // x0,y0, angle0, x1,y1,angle1) {
                        println!("Task done!");
                        model.tasks.remove(0);
                        //go to the next task and start it
                        if model.tasks.len() > 0 {
                            print!("Starting new task -> ");
                            model.tasks[0].start(now);
                        }
                    } else {
                        let start = task.start_time;
                        match task.what {
                            TaskType::Spin => {
                                let power = task.power.unwrap();
                                let s = (power * 100.0).round() as i32;
                                //wiggle around the center
                                let addr = "/motor";
                                let index = model.indices[0] as i32;
                                let args = vec![
                                    Type::Int(index),
                                    Type::Int(s),
                                    Type::Int(-s),
                                    Type::Int(50),
                                ];
                                model.sender.send((addr, args)).ok();
                            }
                            TaskType::TargetAngles => {
                                let targets = task.target_angles.unwrap();
                                let (t0, t1) = targets;
                                let addr = "/motor";
                                let what0 = aim_angle(x0, y0, angle0, t0);
                                let index = model.indices[0] as i32;
                                let args = vec![
                                    Type::Int(index),
                                    Type::Int(what0[0]),
                                    Type::Int(what0[1]),
                                    Type::Int(50),
                                ];
                                model.sender.send((addr, args)).ok();
                                let angle1_to_center = (cy - y1).atan2(cx - x1);
                                let what1 = aim_angle(x1, y1, angle1, t1);
                                let index = model.indices[1] as i32;
                                let args = vec![
                                    Type::Int(index),
                                    Type::Int(what1[0]),
                                    Type::Int(what1[1]),
                                    Type::Int(50),
                                ];
                                model.sender.send((addr, args)).ok();
                            }

                            TaskType::Wiggle => {
                                let power = task.power.unwrap();
                                //wiggle around the center
                                let angle0_to_center = (cy - y0).atan2(cx - x0);
                                let wiggle_time =
                                    app.duration.since_start.as_millis() as u64 - start;
                                let phase = (wiggle_time as f32 / 1000.0 * 3.0 * TAU).cos();
                                let addr = "/motor";
                                let what0 =
                                    aim_angle(x0, y0, angle0, angle0_to_center + phase * power);
                                let index = model.indices[0] as i32;
                                let args = vec![
                                    Type::Int(index),
                                    Type::Int(what0[0]),
                                    Type::Int(what0[1]),
                                    Type::Int(50),
                                ];
                                model.sender.send((addr, args)).ok();
                                let angle1_to_center = (cy - y1).atan2(cx - x1);
                                let addr = "/motor";
                                let what1 =
                                    aim_angle(x1, y1, angle1, angle1_to_center - phase * power);
                                let index = model.indices[1] as i32;
                                let args = vec![
                                    Type::Int(index),
                                    Type::Int(what1[0]),
                                    Type::Int(what1[1]),
                                    Type::Int(50),
                                ];
                                model.sender.send((addr, args)).ok();
                            }
                            TaskType::PairMovement => {
                                let targets = task.targets.unwrap();
                                let (t0, t1) = targets;
                                //println!("move {:?} {:?}",t0,t1);
                                let what = aimany(x0, y0, angle0, t0.x, t0.y);
                                let addr = "/motor";
                                let index = model.indices[0] as i32;
                                let args = vec![
                                    Type::Int(index),
                                    Type::Int(what[0]),
                                    Type::Int(what[1]),
                                    Type::Int(50),
                                ];
                                model.sender.send((addr, args)).ok();
                                let what = aimany(x1, y1, angle1, t1.x, t1.y);
                                let addr = "/motor";
                                let index = model.indices[1] as i32;
                                let args = vec![
                                    Type::Int(index),
                                    Type::Int(what[0]),
                                    Type::Int(what[1]),
                                    Type::Int(50),
                                ];
                                model.sender.send((addr, args)).ok();
                            }
                            TaskType::SingleMovement => {
                                let target = task.target.unwrap();
                                let t0 = target;
                                //println!("move {:?} {:?}",t0,t1);
                                let what = aimany(x0, y0, angle0, t0.x, t0.y);
                                let addr = "/motor";
                                let index = model.indices[0] as i32;
                                let args = vec![
                                    Type::Int(index),
                                    Type::Int(what[0]),
                                    Type::Int(what[1]),
                                    Type::Int(50),
                                ];
                                model.sender.send((addr, args)).ok();
                            }
                            TaskType::PairMovementShift => {
                                let target = task.target.unwrap();
                                let t0 = target;
                                //println!("move {:?} {:?}",t0,t1);
                                let what = aimany(
                                    x0,
                                    y0,
                                    angle0,
                                    t0.x + model.shift0.x,
                                    t0.y + model.shift0.y,
                                );
                                let addr = "/motor";
                                let index = model.indices[0] as i32;
                                let args = vec![
                                    Type::Int(index),
                                    Type::Int(what[0]),
                                    Type::Int(what[1]),
                                    Type::Int(50),
                                ];
                                model.sender.send((addr, args)).ok();
                                let what = aimany(
                                    x1,
                                    y1,
                                    angle1,
                                    t0.x + model.shift1.x,
                                    t0.y + model.shift1.y,
                                );
                                let addr = "/motor";
                                let index = model.indices[1] as i32;
                                let args = vec![
                                    Type::Int(index),
                                    Type::Int(what[0]),
                                    Type::Int(what[1]),
                                    Type::Int(50),
                                ];
                                model.sender.send((addr, args)).ok();
                            }
                            TaskType::GetClose => {
                                let addr = "/motor";
                                let what0 = aimany(x0, y0, angle0, x1, y1);
                                let index = model.indices[0] as i32;
                                let args = vec![
                                    Type::Int(index),
                                    Type::Int(what0[0]),
                                    Type::Int(what0[1]),
                                    Type::Int(50),
                                ];
                                model.sender.send((addr, args)).ok();
                                let what1 = aimany(x1, y1, angle1, x0, y0);
                                let index = model.indices[1] as i32;
                                let args = vec![
                                    Type::Int(index),
                                    Type::Int(what1[0]),
                                    Type::Int(what1[1]),
                                    Type::Int(50),
                                ];
                                model.sender.send((addr, args)).ok();
                                model.shift0 = vec2(x0 - cx, y0 - cy);
                                model.shift1 = vec2(x1 - cx, y1 - cy);
                                model.should_stay_away = false;
                            }
                            TaskType::GetAway => {
                                //where to target
                                let angle = (y1 - y0).atan2(x1 - x0);
                                let addr = "/motor";
                                let what0 = aimany(
                                    x0,
                                    y0,
                                    angle0,
                                    x0 - angle.cos() * DIST_AWAY,
                                    y0 - angle.sin() * DIST_AWAY,
                                );
                                let index = model.indices[0] as i32;
                                let args = vec![
                                    Type::Int(index),
                                    Type::Int(what0[0]),
                                    Type::Int(what0[1]),
                                    Type::Int(50),
                                ];
                                model.sender.send((addr, args)).ok();
                                let what1 = aimany(
                                    x1,
                                    y1,
                                    angle1,
                                    x1 + angle.cos() * DIST_AWAY,
                                    y1 + angle.sin() * DIST_AWAY,
                                );
                                let index = model.indices[1] as i32;
                                let args = vec![
                                    Type::Int(index),
                                    Type::Int(what1[0]),
                                    Type::Int(what1[1]),
                                    Type::Int(50),
                                ];
                                model.sender.send((addr, args)).ok();

                                model.shift0 = vec2(x0 - cx, y0 - cy);
                                model.shift1 = vec2(x1 - cx, y1 - cy);
                                //model.should_stay_away=true;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        if last + 10000 < now {
            //it's been 10 seconds since our last info
            //maybe it's time to kill the tasks
            if model.tasks.len() > 0 {
                println!("CUBES LOST, RESET");
                model.tasks.clear();
            }
        }
    }
}

fn aim_angle(x: f32, y: f32, angle: f32, target_angle: f32) -> [i32; 2] {
    //find the angle
    let mut angle_to_target = target_angle - angle;
    while angle_to_target > PI {
        angle_to_target -= TAU;
    }
    while angle_to_target < -PI {
        angle_to_target += TAU;
    }

    let d = (angle_to_target.abs() / PI * 90.0).floor() as i32 + 2;
    if angle_to_target > 0.0 {
        [-d, d]
    } else {
        [d, -d]
    }
}

fn aim_back(x: f32, y: f32, angle: f32, tx: f32, ty: f32) -> [i32; 2] {
    //find the angle
    let mut angle_to_target = (ty - y).atan2(tx - x) - (angle);
    while angle_to_target > PI {
        angle_to_target -= TAU;
    }
    while angle_to_target < -PI {
        angle_to_target += TAU;
    }
    let d = ((tx - x) * (tx - x) + (ty - y) * (ty - y)).sqrt();
    if d > 13.0 {
        if angle_to_target.abs() > PI * 0.2 {
            if angle_to_target > 0.0 {
                [-20, 20]
            } else {
                [20, -20]
            }
        } else {
            let ds = 20 - (20.0 * angle_to_target.abs() / (PI * 0.2)).floor() as i32;
            if angle_to_target < 0.0 {
                [-ds, -20]
            } else {
                [-20, -ds]
            }
        }
    } else {
        [0, 0]
    }
}

fn aimany(x: f32, y: f32, angle: f32, tx: f32, ty: f32) -> [i32; 2] {
    //find the angle
    let mut angle_to_target = (ty - y).atan2(tx - x) - angle;
    while angle_to_target > PI {
        angle_to_target -= TAU;
    }
    while angle_to_target < -PI {
        angle_to_target += TAU;
    }
    let d = ((tx - x) * (tx - x) + (ty - y) * (ty - y)).sqrt();
    if d > 10.0 {
        if angle_to_target.abs() > PI * 0.5 {
            if angle_to_target.abs() < PI * 0.8 {
                if angle_to_target > 0.0 {
                    [20, -20]
                } else {
                    [-20, 20]
                }
            } else {
                let ds = 20 - (20.0 * (PI - angle_to_target.abs()) / (PI * 0.2)).floor() as i32;
                if angle_to_target > 0.0 {
                    [-ds, -20]
                } else {
                    [-20, -ds]
                }
            }
        } else {
            if angle_to_target.abs() > PI * 0.2 {
                if angle_to_target > 0.0 {
                    [-20, 20]
                } else {
                    [20, -20]
                }
            } else {
                let ds = 20 - (20.0 * angle_to_target.abs() / (PI * 0.2)).floor() as i32;
                if angle_to_target > 0.0 {
                    [ds, 20]
                } else {
                    [20, ds]
                }
            }
        }
    } else {
        [0, 0]
    }
}
fn facetarget(x: f32, y: f32, angle: f32, tx: f32, ty: f32) -> [i32; 2] {
    let scale = 20;
    //find the angle
    let mut angle_to_target = (ty - y).atan2(tx - x) - angle;
    while angle_to_target > PI {
        angle_to_target -= TAU;
    }
    while angle_to_target < -PI {
        angle_to_target += TAU;
    }

    if angle_to_target.abs() > PI * 0.15 {
        if angle_to_target > 0.0 {
            [-20, 20]
        } else {
            [20, -20]
        }
    } else {
        [0, 0]
    }
}
fn aimstable(x: f32, y: f32, angle: f32, tx: f32, ty: f32) -> [i32; 2] {
    let scale = 20;
    //find the angle
    let mut angle_to_target = (ty - y).atan2(tx - x) - angle;
    while angle_to_target > PI {
        angle_to_target -= TAU;
    }
    while angle_to_target < -PI {
        angle_to_target += TAU;
    }
    let d = ((tx - x) * (tx - x) + (ty - y) * (ty - y)).sqrt();
    if d > 15.0 {
        if angle_to_target.abs() > PI * 0.2 {
            if angle_to_target > 0.0 {
                [-20, 20]
            } else {
                [20, -20]
            }
        } else {
            let ds = scale - (scale as f32 * angle_to_target.abs() / (PI * 0.2)).floor() as i32;
            if angle_to_target > 0.0 {
                [ds, scale]
            } else {
                [scale, ds]
            }
        }
    } else {
        [0, 0]
    }
}
fn aim(x: f32, y: f32, angle: f32, tx: f32, ty: f32) -> [i32; 2] {
    let d = ((tx - x) * (tx - x) + (ty - y) * (ty - y)).sqrt() * 0.5;
    let scale = 10 + d.min(50.0).floor() as i32;
    //find the angle
    let mut angle_to_target = (ty - y).atan2(tx - x) - angle;
    while angle_to_target > PI {
        angle_to_target -= TAU;
    }
    while angle_to_target < -PI {
        angle_to_target += TAU;
    }
    let d = ((tx - x) * (tx - x) + (ty - y) * (ty - y)).sqrt();
    if d > 15.0 {
        if angle_to_target.abs() > PI * 0.2 {
            if angle_to_target > 0.0 {
                [-20, 20]
            } else {
                [20, -20]
            }
        } else {
            let ds = scale - (scale as f32 * angle_to_target.abs() / (PI * 0.2)).floor() as i32;
            if angle_to_target > 0.0 {
                [ds, scale]
            } else {
                [scale, ds]
            }
        }
    } else {
        [0, 0]
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    let keys = model.toio.keys();
    for i in keys {
        let main_cube = model.toio.get(&i);
        if let Some(cube_data) = main_cube {
            let angle = cube_data.angle as f32 / 360.0 * TAU;
            draw.rect()
                .x_y(cube_data.x as f32, cube_data.y as f32)
                .rotate(angle)
                .w_h(20.0, 20.0)
                .color(WHITE);
            let c = if *i == 0 {
                srgb(1.0, 0.0, 0.0)
            } else {
                srgb(0.0, 1.0, 0.0)
            };
            draw.rect()
                .x_y(
                    cube_data.x as f32 + angle.cos() * 5.0,
                    cube_data.y as f32 + angle.sin() * 5.0,
                )
                .rotate(angle)
                .w_h(10.0, 10.0)
                .color(c);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
