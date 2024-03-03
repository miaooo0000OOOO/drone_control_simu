use std::{fs, ops::Range};

use bevy::prelude::*;
use bevy_xpbd_3d::math::PI;

use crate::drone_plugin::{restraint_in_range, DRONE_THRUST_RANGE};

#[derive(Debug, Component, Clone)]
pub struct Controller {
    x_pid: PID_Controller,
    z_pid: PID_Controller,
    roll_pid: PID_Controller,  // 滚转
    pitch_pid: PID_Controller, // 俯仰
    yaw_pid: PID_Controller,   // 偏航
    h_pid: PID_Controller,     // 高度
}

const ANGLE_RANGE: Range<f32> = (-PI / 6.)..(PI / 6.);
// const ANGLE_RANGE: Range<f32> = -1000000.0..1000000.0;

// angle x - y
fn sub_angle(x: f32, y: f32) -> f32 {
    let mut a = x - y;
    while a < -PI {
        a += 2. * PI;
    }
    while a > PI {
        a -= 2. * PI;
    }
    a
}

impl Controller {
    pub fn new() -> Self {
        Self {
            x_pid: PID_Controller::new(0.05, 0.01, 0.1),
            z_pid: PID_Controller::new(0.05, 0.01, 0.1),
            pitch_pid: PID_Controller::new(0.1, 0., 0.1),
            roll_pid: PID_Controller::new(0.1, 0., 0.1),
            yaw_pid: PID_Controller::new(0.02, 0., 0.01),
            h_pid: PID_Controller::new(8.0, 1., 1.),
        }
    }

    pub fn from_config_file(path: &str) -> Self {
        let config_file = match fs::read_to_string(path) {
            Ok(f) => f,
            Err(e) => panic!("Read controller config file error: {}", e),
        };
        let mut x_pid_p: Option<(f32, f32, f32)> = None;
        let mut z_pid_p: Option<(f32, f32, f32)> = None;
        let mut h_pid_p: Option<(f32, f32, f32)> = None;
        let mut pitch_pid_p: Option<(f32, f32, f32)> = None;
        let mut roll_pid_p: Option<(f32, f32, f32)> = None;
        let mut yaw_pid_p: Option<(f32, f32, f32)> = None;

        for (line_num, line) in config_file.lines().enumerate() {
            let controller_name = line.split(":").collect::<Vec<_>>()[0].trim();
            let lp_index = match line.find('(') {
                Some(i) => i,
                None => panic!(
                    "Controller config file format error in line {}\nline str: {}",
                    line_num, line
                ),
            };
            let rp_index = match line.find(')') {
                Some(i) => i,
                None => panic!(
                    "Controller config file format error in line {}\nline str: {}",
                    line_num, line
                ),
            };
            let params = &line[lp_index + 1..rp_index];
            let params = params
                .split(',')
                .map(str::trim)
                .map(|s| match s.parse::<f32>() {
                    Ok(f) => f,
                    Err(e) => panic!(
                        "Controller config file format error in line {}\nline str: {}\nParse number error {}",
                        line_num, line, e
                    ),
                })
                .collect::<Vec<f32>>();
            assert_eq!(params.len(), 3);
            let params = [params[0], params[1], params[2]];
            match controller_name {
                "x_pid" => x_pid_p = Some(params.into()),
                "z_pid" => z_pid_p = Some(params.into()),
                "h_pid" => h_pid_p = Some(params.into()),
                "pitch_pid" => pitch_pid_p = Some(params.into()),
                "roll_pid"=>roll_pid_p = Some(params.into()),
                "yaw_pid"=>yaw_pid_p = Some(params.into()),
                _=>panic!(
                    "Controller config file error in line {}\nline str: {}\nController name not exist",
                    line_num, line)
            }
        }
        if x_pid_p.is_none()
            || z_pid_p.is_none()
            || h_pid_p.is_none()
            || pitch_pid_p.is_none()
            || roll_pid_p.is_none()
            || yaw_pid_p.is_none()
        {
            panic!("Controller config file error: lack controller param",)
        }
        Self {
            x_pid: PID_Controller::from_tuple(x_pid_p.unwrap()),
            z_pid: PID_Controller::from_tuple(z_pid_p.unwrap()),
            h_pid: PID_Controller::from_tuple(h_pid_p.unwrap()),
            pitch_pid: PID_Controller::from_tuple(pitch_pid_p.unwrap()),
            roll_pid: PID_Controller::from_tuple(roll_pid_p.unwrap()),
            yaw_pid: PID_Controller::from_tuple(yaw_pid_p.unwrap()),
        }
    }

    pub fn ctrl_drone(
        &mut self,
        target_pos: &Vec3,
        drone_transform: &Transform,
        dt: f32,
    ) -> Vec<f32> {
        let (target_x, target_h, target_z) = (target_pos.x, target_pos.y, target_pos.z);
        let (drone_x, drone_h, drone_z) = (
            drone_transform.translation.x,
            drone_transform.translation.y,
            drone_transform.translation.z,
        );
        let (drone_roll, drone_yaw, drone_pitch) = drone_transform.rotation.to_euler(EulerRot::XYZ);
        // let vec3_drone_target = *target_pos - drone_transform.translation;
        // let target_yaw;
        // if vec3_drone_target.length() < 0.000001 || vec3_drone_target.z.abs() < 0.000001 {
        //     target_yaw = 0.;
        // } else if vec3_drone_target.x == 0.0 {
        //     target_yaw = if vec3_drone_target.z > 0. {
        //         -PI / 2.
        //     } else {
        //         PI / 2.
        //     };
        // } else {
        //     target_yaw = (-vec3_drone_target.z / vec3_drone_target.x).atan();
        // }
        let target_roll = self.z_pid.ctrl(target_z - drone_z, dt);
        let target_roll = restraint_in_range(target_roll, ANGLE_RANGE);
        let target_pitch = -self.x_pid.ctrl(target_x - drone_x, dt);
        let target_pitch = restraint_in_range(target_pitch, ANGLE_RANGE);

        // let target_roll = PI / 3.; //debug
        let roll_cmd = self.roll_pid.ctrl(sub_angle(target_roll, drone_roll), dt);
        // let roll_cmd = 0.; // debug

        // let target_pitch = PI / 3.; //debug
        let pitch_cmd = self
            .pitch_pid
            .ctrl(sub_angle(target_pitch, drone_pitch), dt);
        // let pitch_cmd = 0.; // debug

        // let target_yaw = PI / 3.; // debug
        // let yaw_cmd = self.yaw_pid.ctrl(sub_angle(target_yaw, drone_yaw), dt);
        // println!("ty: {} dy: {} yc: {}", target_yaw, drone_yaw, yaw_cmd);
        let yaw_cmd = 0.; // debug
        let thrust_cmd = self.h_pid.ctrl(target_h - drone_h, dt);
        let thrust_cmd = restraint_in_range(
            thrust_cmd,
            DRONE_THRUST_RANGE.start * 0.6..DRONE_THRUST_RANGE.end * 0.6,
        );
        // vec![
        //     thrust_cmd + yaw_cmd + pitch_cmd + roll_cmd, // 右前
        //     thrust_cmd - yaw_cmd + pitch_cmd - roll_cmd, // 左前
        //     thrust_cmd - yaw_cmd - pitch_cmd + roll_cmd, // 右后
        //     thrust_cmd + yaw_cmd - pitch_cmd - roll_cmd, // 左后
        // ]
        // let res = vec![
        //     thrust_cmd - yaw_cmd + pitch_cmd + roll_cmd, // 右前
        //     thrust_cmd + yaw_cmd + pitch_cmd - roll_cmd, // 左前
        //     thrust_cmd + yaw_cmd - pitch_cmd + roll_cmd, // 右后
        //     thrust_cmd - yaw_cmd - pitch_cmd - roll_cmd, // 左后
        // ];
        let res = vec![
            thrust_cmd + yaw_cmd + pitch_cmd - roll_cmd, // 右前
            thrust_cmd - yaw_cmd + pitch_cmd + roll_cmd, // 左前
            thrust_cmd - yaw_cmd - pitch_cmd - roll_cmd, // 右后
            thrust_cmd + yaw_cmd - pitch_cmd + roll_cmd, // 左后
        ];
        res
        // vec![thrust_cmd, thrust_cmd, thrust_cmd, thrust_cmd]
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub struct PID_Controller {
    pub kp: f32,
    pub ki: f32,
    pub kd: f32,
    prev_e: f32,
    integration: f32,
}

impl PID_Controller {
    pub fn new(kp: f32, ki: f32, kd: f32) -> Self {
        Self {
            kp,
            ki,
            kd,
            prev_e: 0.,
            integration: 0.,
        }
    }

    pub fn from_tuple(params: (f32, f32, f32)) -> Self {
        Self::new(params.0, params.1, params.2)
    }

    pub fn ctrl(&mut self, error: f32, dt: f32) -> f32 {
        self.integration += error * dt;
        let diff = (error - self.prev_e) / dt;
        let res = self.kp * error + self.ki * self.integration + self.kd * diff;
        self.prev_e = error;
        res
    }
}

impl Default for PID_Controller {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}
