use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

#[derive(Debug, Component)]
pub struct Controller;

#[allow(non_camel_case_types)]
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

    fn ctrl(mut self, error: f32, dt: f32) -> f32 {
        self.integration += error * dt;
        let diff = (error - self.prev_e) / dt;
        let res = self.kp * error + self.ki * self.integration + self.kd * diff;
        self.prev_e = error;
        res
    }
}
