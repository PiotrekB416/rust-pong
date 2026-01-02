use std::ops::{Range, RangeBounds};

use crate::{console_log, consts::*, Ball};

#[derive(Clone)]
pub struct Paddle {
    pub x: isize,
    pub y: isize,
    pub w: isize,
    pub h: isize,
}

impl Paddle {
    pub fn new(left: bool) -> Self {
        Self {
            x: if left { 50 } else { WIDTH - 50 },
            y: 250,
            w: 30,
            h: 100,
        }
    }
    // check ball collision
    pub fn collision(&self, ball: &mut Ball) {
        fn ranges_intersect(range1: Range<isize>, range2: Range<isize>) -> bool {
            range1.clone().min() <= range2.clone().max() && range2.min() <= range1.max()
        }

        let bx = ball.x / PHYSICS_SCALE;
        let by = ball.y / PHYSICS_SCALE;

        let top = self.y;
        let bottom = self.y + self.h;
        let left = self.x;
        let right = self.x + self.w;

        let by_range = by - BALL_RADIUS..by + BALL_RADIUS;
        let bx_range = bx - BALL_RADIUS..bx + BALL_RADIUS;

        //console_log(format!("{by_range:?}, {top}, {bottom}").as_str());

        if (left..=right).contains(&bx) {
            let mult = if bx < (right - left) / 2 { -1 } else { 1 };
            ball.x += self.w * mult;
        }

        if ranges_intersect(self.x..self.x + self.w, bx_range.clone())
            && (by_range.clone().contains(&top) || by_range.clone().contains(&bottom))
        {
            //console_log("hit");
            ball.invert_y();
        }

        if ranges_intersect(self.y..self.y + self.h, by_range.clone())
            && (bx_range.clone().contains(&left) || bx_range.clone().contains(&right))
            && (ball.inverted.0 == (self.x < WIDTH / 2))
        {
            ball.invert_x();

            ball.random_angle();
        }
    }
    // player code
    pub fn move_player(&mut self, dir: bool) {
        self.y += if dir { -1 } else { 1 };

        if self.y < 0 {
            self.y = 0;
        }

        if self.y > HEIGHT - self.h {
            self.y = HEIGHT - self.h;
        }
    }
    // bot code
    pub fn move_ai(&mut self, ball: &Ball) {
        let range = self.y + BALL_RADIUS * 2..self.y + self.h - BALL_RADIUS * 2;
        //console_log(format!("{range:?}|{}", ball.y / PHYSICS_SCALE).as_str());
        if !range.contains(&(ball.y / PHYSICS_SCALE)) && ((self.x < WIDTH / 2) ^ !ball.inverted.0) {
            self.move_player(ball.y / PHYSICS_SCALE < range.start);
        }
    }
}
