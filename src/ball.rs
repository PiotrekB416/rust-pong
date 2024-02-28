use std::ptr::null;

use crate::{console_log, consts::*};
use dioxus::hooks::UseState;
use rand::Rng;

#[derive(Clone, Debug)]
pub struct Ball {
    pub x: isize,
    pub y: isize,
    speed: f64,
    angle: f64,
    pub(crate) inverted: (bool, bool),
    score: UseState<Vec<usize>>,
}

impl Ball {
    pub fn new(score: UseState<Vec<usize>>) -> Self {
        let mut s = Self {
            x: 0,
            y: 0,
            speed: 3.5 * PHYSICS_SCALE as f64,
            angle: 0.0,
            inverted: (false, false),
            score,
        };
        s.setup();
        s
    }

    pub fn next(&mut self) {
        //let mut rng = rand::thread_rng();
        let mult_x = if self.inverted.0 { -1 } else { 1 } as f64;
        let mult_y = if self.inverted.1 { -1 } else { 1 } as f64;

        //let v = 15.0;

        let (x, y) = (
            self.angle * mult_x,
            ((self.speed * self.speed) - (self.angle * self.angle)).sqrt() * mult_y,
        );

        let mut invert = (false, false);

        self.x += x as isize;
        self.y += y as isize;

        if self.x < BALL_RADIUS * PHYSICS_SCALE {
            invert.0 = true;
            self.x = BALL_RADIUS * 2 * PHYSICS_SCALE - self.x;
        }

        if self.x > (WIDTH - BALL_RADIUS) * PHYSICS_SCALE {
            invert.0 = true;
            self.x = (WIDTH - BALL_RADIUS) * PHYSICS_SCALE * 2 - self.x;
        }

        if self.y < BALL_RADIUS * PHYSICS_SCALE {
            invert.1 = true;
            self.y = BALL_RADIUS * 2 * PHYSICS_SCALE - self.y;
        }

        if self.y > (HEIGHT - BALL_RADIUS) * PHYSICS_SCALE {
            invert.1 = true;
            self.y = (HEIGHT - BALL_RADIUS) * PHYSICS_SCALE * 2 - self.y;
        }

        if invert.0 {
            console_log("invert x");
            //self.invert_x();
            //self.x += x as isize;
            let id = if self.x > (WIDTH / 2 * PHYSICS_SCALE) {
                0
            } else {
                1
            };
            self.score.with_mut(|v| v[id] += 1);
            self.setup();
        }

        if invert.1 {
            console_log("invert y");
            self.invert_y();

            //self.y += y as isize;
        }
    }

    fn setup(&mut self) {
        let mut rng = rand::thread_rng();
        self.x = WIDTH / 2 * PHYSICS_SCALE;
        self.y = HEIGHT / 2 * PHYSICS_SCALE;
        self.inverted = (rng.gen(), rng.gen());
        self.angle = rng.gen_range(2.7..3.5) * PHYSICS_SCALE as f64;
    }

    pub fn invert_y(&mut self) {
        self.inverted.1 = !self.inverted.1;
    }

    pub fn invert_x(&mut self) {
        self.inverted.0 = !self.inverted.0;
    }
}
