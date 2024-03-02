use std::ops::{Range, RangeFull};

use bevy::prelude::*;
use bevy_xpbd_3d::math::PI;

use crate::drone_plugin::restraint_in_range;

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
            x_pid: PID_Controller::new(0.1, 0., 0.),
            z_pid: PID_Controller::new(0.1, 0., 0.),
            roll_pid: PID_Controller::new(0.6, 0.05, 1.),
            pitch_pid: PID_Controller::new(0.6, 0.05, 1.),
            yaw_pid: PID_Controller::new(0.02, 0., 0.01),
            // h_pid: PID_Controller::new(0.4, 0.05, 1.),
            h_pid: PID_Controller::new(0.1, 0., 0.),
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
        let vec3_drone_target = *target_pos - drone_transform.translation;
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
        let target_roll = -self.x_pid.ctrl(target_z - drone_z, dt);
        let target_roll = restraint_in_range(target_roll, ANGLE_RANGE);
        let target_pitch = -self.z_pid.ctrl(target_x - drone_x, dt);
        let target_pitch = restraint_in_range(target_pitch, ANGLE_RANGE);

        // let target_roll = PI / 3.; //debug
        let roll_cmd = self.roll_pid.ctrl(sub_angle(target_roll, drone_roll), dt);
        let roll_cmd = 0.; // debug

        // let target_pitch = -PI / 6.; //debug
        let pitch_cmd = self
            .pitch_pid
            .ctrl(sub_angle(target_pitch, drone_pitch), dt);
        let pitch_cmd = 0.; // debug

        // let target_yaw = PI / 3.; // debug
        // let yaw_cmd = self.yaw_pid.ctrl(sub_angle(target_yaw, drone_yaw), dt);
        // println!("ty: {} dy: {} yc: {}", target_yaw, drone_yaw, yaw_cmd);
        // let yaw_cmd = 0.; // debug
        let thrust_cmd = self.h_pid.ctrl(target_h - drone_h, dt);
        // if thrust_cmd.abs() > 3. {
        //     println!("???");
        // }
        // println!("tc:{} th:{} dh:{} dt:{}", thrust_cmd, target_h, drone_h, dt);
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
            thrust_cmd + pitch_cmd + roll_cmd, // 右前
            thrust_cmd + pitch_cmd - roll_cmd, // 左前
            thrust_cmd - pitch_cmd + roll_cmd, // 右后
            thrust_cmd - pitch_cmd - roll_cmd, // 左后
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
    prev_e: Option<f32>,
    integration: f32,
}

impl PID_Controller {
    pub fn new(kp: f32, ki: f32, kd: f32) -> Self {
        Self {
            kp,
            ki,
            kd,
            prev_e: None,
            integration: 0.,
        }
    }

    pub fn ctrl(&mut self, error: f32, dt: f32) -> f32 {
        self.integration += error * dt;
        let diff;
        if let Some(prev_e) = self.prev_e {
            diff = (error - prev_e) / dt;
        } else {
            diff = 0.;
        }
        let res = self.kp * error + self.ki * self.integration + self.kd * diff;
        self.prev_e = Some(error);
        res
    }
}

impl Default for PID_Controller {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}
