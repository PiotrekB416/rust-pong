#![allow(non_snake_case)]
use std::{
    borrow::Borrow, isize, ops::{Deref, Range, RangeBounds}, ptr::null, usize
};

// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::{
    html::{
        geometry::euclid::{num::Round, Trig},
        option, select, GlobalAttributes,
    },
    prelude::*,
};
use rand::Rng;
use wasm_bindgen::{closure, prelude::*};
use web_sys::js_sys::Function;

const WIDTH: isize = 800;
const HEIGHT: isize = 600;
const BALL_RADIUS: isize = 15;
const PHYSICS_SCALE: isize = 100;

#[derive(Clone, Debug)]
struct Ball {
    pub x: isize,
    pub y: isize,
    speed: f64,
    angle: f64,
    inverted: (bool, bool),
    game_over: fn(),
}

#[derive(Clone)]
struct Paddle {
    pub x: isize,
    pub y: isize,
    pub w: isize,
    pub h: isize,
}

fn ranges_intersect(range1: Range<isize>, range2: Range<isize>) -> bool {
    range1.clone().min() <= range2.clone().max() && range2.min() <= range1.max()
}

impl Paddle {
    fn new(left: bool) -> Self {
        Self {
            x: if left { 50 } else { 750 },
            y: 250,
            w: 30,
            h: 100,
        }
    }

    pub fn collision(&self, ball: &mut Ball) {
        let bx = ball.x / PHYSICS_SCALE;
        let by = ball.y / PHYSICS_SCALE;

        let top = self.y;
        let bottom = self.y + self.h;
        let left = self.x;
        let right = self.x + self.w;

        let by_range = by - BALL_RADIUS..by + BALL_RADIUS;
        let bx_range = bx - BALL_RADIUS..bx + BALL_RADIUS;

        if ranges_intersect(self.x..self.x + self.w, bx_range.clone())
            && (by_range.clone().contains(&top) || by_range.clone().contains(&bottom))
        {
            ball.invert_y();
        }

        if ranges_intersect(self.y..self.y + self.h, by_range.clone())
            && (bx_range.clone().contains(&left) || bx_range.clone().contains(&right)) {
            ball.invert_x();
        }
    }

    pub fn move_player(&mut self, val: isize) {
        self.y += val;

        if self.y < 0 {
            self.y = 0;
        }

        if self.y > HEIGHT - self.h {
            self.y = HEIGHT - self.h;
        }
    }
}

impl Ball {
    fn new(game_over: fn()) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: 385 * PHYSICS_SCALE,
            y: 285 * PHYSICS_SCALE,
            speed: 6.5 * PHYSICS_SCALE as f64,
            angle: rng.gen_range(5.5..6.5) * PHYSICS_SCALE as f64,
            inverted: (rng.gen(), rng.gen()),
            game_over,
        }
    }

    fn next(&mut self) {
        let mut rng = rand::thread_rng();
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
            self.invert_x();
            //self.x += x as isize;
        }

        if invert.1 {
            console_log("invert y");
            self.invert_y();
            //self.y += y as isize;
        }
    }

    pub fn invert_y(&mut self) {
        self.inverted.1 = !self.inverted.1;
    }

    pub fn invert_x(&mut self) {
        self.inverted.0 = !self.inverted.0;
    }
}

#[derive(Clone)]
struct Funcs {
    intervals: Vec<i32>,
    events: Vec<Function>,
}

impl Funcs {
    pub fn new() -> Self {
        Self {
            intervals: Vec::new(),
            events: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.intervals.len() + self.events.len()
    }

    pub fn push_interval(&mut self, id: i32) {
        self.intervals.push(id);
    }

    pub fn get_intervals(&self) -> Vec<i32> {
        self.intervals.clone()
    }

    pub fn remove_all(&mut self) {
        self.intervals.clear();
        self.events.clear();
    }

    pub fn push_event(&mut self, event: Function) {
        self.events.push(event);
    }

    pub fn get_events(&self) -> Vec<Function> {
        self.events.clone()
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn console_log(s: &str);
}

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

fn get_context(context_type: String) -> web_sys::CanvasRenderingContext2d {
    let document = web_sys::window().unwrap_throw().document().unwrap_throw();
    let canvas = document.get_element_by_id("gamecanvas").unwrap_throw();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap_throw();
    return canvas
        .get_context(&context_type)
        .unwrap_throw()
        .unwrap_throw()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap_throw();
}

fn move_player(player: &mut Paddle, direction: bool) {
    player.move_player(if direction {-10} else {10});
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    let players = use_state(cx, || vec![false, true]);
    let paddles = use_ref(cx, || vec![Paddle::new(true), Paddle::new(false)]);
    let ball = use_ref(cx, || Ball::new(|| {}));

    let listeners = use_ref(cx, Funcs::new);

    let hidden = use_state(cx, || true);

    let player = |id: usize, player: bool| {
        rsx!( div {
            id: "player{id}",
            class: "w-1/6",
            span {
                "0"
            },
            select {
                onchange: move |e: Event<_>| {
                    console_log("change");
                    //let players = players.clone();
                    players.modify(move |players| { let mut p = players.clone(); p[id] = !p[id]; p });
                    //console_log("change");
                    console_log(format!("{:?}", players.current()).as_str());
                },
                option {selected: player, "player"},
                option {selected: !player, "bot"},
            }
        }, )
    };

    let window = web_sys::window().unwrap_throw();
    let players_clone = players.clone();

    if listeners.read().len() == 0 && !hidden {
        {
            let paddles_clone = paddles.clone();
            let closure = Closure::<dyn FnMut(_)>::new(move |e: web_sys::InputEvent| {
                let key = e
                    .deref()
                    .clone()
                    .dyn_into::<web_sys::KeyboardEvent>()
                    .unwrap_throw()
                    .key();
                console_log(format!("keydown: {:?}", key.as_str()).as_str());

                match key.as_str() {
                    "w" | "s" => {
                        if !players_clone.get().get(0).unwrap_throw() {
                            console_log(format!("{:?}", players_clone).as_str());
                            return;
                        }

                        paddles_clone.with_mut(|v| {
                            let p = v.get_mut(0).unwrap_throw();
                            move_player(p, key == "w");
                            v.clone()
                        });
                    }
                    "ArrowUp" | "ArrowDown" => {
                        if !players_clone.get().get(1).unwrap_throw() {
                            return;
                        }
                        paddles_clone.with_mut(|v| {
                            let p = v.get_mut(1).unwrap_throw();
                            move_player(p, key == "ArrowUp");
                            v.clone()
                        });
                    }
                    _ => {}
                }
            });
            let func = closure.as_ref().unchecked_ref::<Function>().clone();
            window
                .add_event_listener_with_callback("keydown", &func.clone())
                .unwrap_throw();
            listeners.with_mut(|v| {
                v.push_event(func.clone());
                v.clone()
            });
            closure.forget();
        }
        {
            let ball_clone = ball.clone();
            let paddles_clone = paddles.clone();
            let closure = Closure::<dyn FnMut()>::new(move || {
                //console_log("call");
                let (x, y) = (
                    ball_clone.read().x / PHYSICS_SCALE,
                    ball_clone.read().y / PHYSICS_SCALE,
                );
                let context = get_context("2d".to_string());
                context.clear_rect(0.0, 0.0, 800.0, 600.0);
                context.begin_path();
                //console_log(format!("{}, {}", x, y).as_str());
                context.set_stroke_style(&"#000000".into());
                context
                    .arc(x as f64, y as f64, 15.0, 0.0, std::f64::consts::TAU)
                    .unwrap_throw();
                context.fill();
                context.stroke();

                paddles_clone.with(|vec| {
                    vec.iter().for_each(|p| {
                        context.fill_rect(p.x as f64, p.y as f64, p.w as f64, p.h as f64);
                        context.stroke();
                    });
                });

                //context.fill_rect(x as f64, y as f64, 10.0, 10.0);
                context.close_path();

            });
            listeners.with_mut(|v| {
                v.push_interval(
                    window
                        .set_interval_with_callback_and_timeout_and_arguments_0(
                            closure.as_ref().unchecked_ref(),
                            1,
                        )
                        .unwrap_throw(),
                );
                v.clone()
            });
            closure.forget();

            let ball_clone = ball.clone();
            let paddles_clone = paddles.clone();
            let closure = Closure::<dyn FnMut()>::new(move || {
                let mut ball = ball_clone.read().clone();
                ball.next();
                let paddles = paddles_clone.read().clone();
                paddles.iter().for_each(|e| e.collision(&mut ball));
                //console_log(format!("{:?}", ball).as_str());
                //ball_clone.with_mut(|_| ball);
                ball_clone.set(ball)
            });
            listeners.with_mut(|v| {
                v.push_interval(
                    window
                        .set_interval_with_callback_and_timeout_and_arguments_0(
                            closure.as_ref().unchecked_ref(),
                            30,
                        )
                        .unwrap_throw(),
                );
                v.clone()
            });
            closure.forget();
        }
    } else if listeners.read().len() > 0 && hidden.get().to_owned() {
        //console_log(format!("len: {}", listeners.read().len()).as_str());
        console_log("clear");
        for id in listeners.read().get_intervals() {
            window.clear_interval_with_handle(id);
        }
        for f in listeners.read().get_events() {
            window
                .remove_event_listener_with_callback("keydown", &f)
                .unwrap_throw();
        }
        listeners.with_mut(|v| {
            v.remove_all();
            v.clone()
        });
    }
    cx.render(rsx! {
       div {
        id: "start",
        button {
            onclick: move |_| {
                let document = web_sys::window().unwrap_throw().document().unwrap_throw();
                //document.get_element_by_id("start").unwrap_throw().set_attribute("hidden", "").unwrap_throw();
                //document.get_element_by_id("game").unwrap_throw().remove_attribute("hidden").unwrap_throw();
                hidden.set(!hidden);
            },
            if hidden.get().to_owned() { "PLAY" } else { "STOP" }
        }
       }
       div {
        id: "game",
        class: if hidden.clone().get().to_owned() { "hidden" } else { "flex flex-row h-[38rem] w-[75rem]" },
        player(0, players.get().get(0).unwrap_throw().to_owned()),
        canvas {
            id: "gamecanvas",
            width: "{WIDTH}",
            height: "{HEIGHT}",
            class: "w-4/6 border-2 border-black",
        },
        player(1, players.get().get(1).unwrap_throw().to_owned()),
       }
    })
}
