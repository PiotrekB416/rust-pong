#![allow(non_snake_case)]
use std::{isize, ops::Deref, usize};

// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::{html::{geometry::euclid::{num::Round, Trig}, option, select, GlobalAttributes}, prelude::*};
use rand::Rng;
use wasm_bindgen::{closure, prelude::*};
use web_sys::js_sys::Function;

#[derive(Clone, Debug)]
struct Ball {
    pub x: isize,
    pub y: isize,
    angle: usize,
    speed: usize,
    inverted: (bool, bool),
    game_over: fn (),
}

impl Ball {
    fn new(game_over: fn()) -> Self {
        let mut rng = rand::thread_rng();
        Self { x: 3850, y: 2850, angle: (rng.gen::<f64>().round() as usize * 80 + 50) * (rng.gen::<f64>().round() as usize * 3 + 1),
            speed: 30, inverted: (rng.gen(), rng.gen()), game_over}
    }

    fn next(&mut self) {
        let mut t: f64 = (self.angle as f64).to_radians().tan();

        console_log(format!("{}", t).as_str());

        let mult_x = if self.inverted.0 {-1} else {1};
        let mult_y = if self.inverted.1 {-1} else {1};


        if t < 1.0 {
            self.y += self.speed as isize * mult_y;
            self.x += (self.speed as f64 * t).ceil() as isize * mult_x;
        } else {
            self.x += self.speed as isize * mult_x;
            self.y += (self.speed as f64 * t).ceil() as isize * mult_y;
        }

        let mut invert = (false, false);

        if self.x < 150 {
            invert.0 = true;
            self.x = 300 - self.x;
        }

        if self.x > 7850 {
            invert.0 = true;
            self.x = 7850 - (self.x - 7850);
        }

        if self.y < 150 {
            invert.1 = true;
            self.y = 300 - self.y;
        }

        if self.y > 5850 {
            invert.1 = true;
            self.y = 5850 - (self.y - 5850);
        }

        if invert.0 {
            console_log("invert x");
            self.invert_x();
        }

        if invert.1 {
            console_log("invert y");
            self.invert_y();
        }

    }

    fn invert_y(&mut self) {
        self.inverted.1 = !self.inverted.1;
    }

    fn invert_x(&mut self) {
        self.inverted.0 = !self.inverted.0;
    }
}

struct Funcs<'a> {
    intervals: Vec<i32>,
    events: Vec<&'a Function>
}

impl<'a> Funcs<'a> {
    pub fn new() -> Self {
        Self {intervals: Vec::new(), events: Vec::new()}
    }

    pub fn len(&self) -> usize {
        self.intervals.len() +
        self.events.len()
    }

    pub fn push_interval(&mut self, id: i32) {
        self.intervals.push(id);
    }

    pub fn get_intervals(&self) -> Vec<i32> {
        self.intervals
    }

    pub fn push_event(&mut self, event: &'a Function) {
        self.events.push(event);
    }

    pub fn get_events(&self) -> Vec<&Function> {
        self.events
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
    return canvas.get_context(&context_type).unwrap_throw().unwrap_throw().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap_throw();
}

fn move_player(id: usize, direction: bool) {

}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {

    let players = use_state(cx, || vec![false, true]);
    let ball = use_ref(cx, || Ball::new(|| {

    }));

    let listeners = use_ref(cx, Funcs::new);

    let hidden = use_state(cx, || true);

    let player = |id: usize, player: bool| rsx!( div {
            id: "player{id}",
            class: "w-1/6",
            span {
                "0"
            },
            select {
                onchange: move |e: Event<_>| {
                    console_log("change");
                    let players = players.clone();
                    players.modify(move |players| { let mut p = players.clone(); p[id] = !p[id]; p });
                    //console_log("change");
                    console_log(format!("{:?}", players.current()).as_str());
                },
                option {selected: player, "player"},
                option {selected: !player, "bot"},
            }
        }, );

    let window = web_sys::window().unwrap_throw();
        let players_clone = players.clone();

    if listeners.read().len() == 0 && !hidden
    {
        {
            let closure = Closure::<dyn FnMut(_)>::new( move |e: web_sys::InputEvent| {
            let key = e.deref().clone().dyn_into::<web_sys::KeyboardEvent>().unwrap_throw().key();
            //console_log(format!("keydown: {:?}", key).as_str());

                match key.as_str() {
                    "w" | "s" => {
                        if !players_clone.get().get(0).unwrap_throw() { return; }

                        move_player(0, key == "w");
                    },
                    "ArrowUp" | "ArrowDown" => {
                        if !players_clone.get().get(1).unwrap_throw() { return; }

                        move_player(1, key == "ArrowUp");
                    },
                    _ => {}
                }
            } );
            let func = closure.as_ref().unchecked_ref();
            window.add_event_listener_with_callback("keydown", func).unwrap_throw();
            listeners.with_mut(|v| {v.push_event(func); v});
            closure.forget();
        }
        {
            let ball_clone = ball.clone();
            let closure = Closure::<dyn FnMut()>::new(move || {
                //console_log("call");
                let (x, y) = ( ball_clone.read().x / 10, ball_clone.read().y / 10 );
                let context = get_context("2d".to_string());
                context.clear_rect(0.0, 0.0, 800.0, 600.0);
                context.begin_path();
                //console_log(format!("{}, {}", x, y).as_str());
                context.set_stroke_style(&"#000000".into());
                context.arc(x as f64, y as f64, 15.0, 0.0, std::f64::consts::TAU).unwrap_throw();
                context.fill();
                context.stroke();
                //context.fill_rect(x as f64, y as f64, 10.0, 10.0);
                context.close_path();

            });
            listeners.with_mut(|v| {
                window.set_interval_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), 1).unwrap_throw();
            v });
            closure.forget();

            let ball_clone = ball.clone();
            let closure = Closure::<dyn FnMut()>::new(move || {
                let mut ball = ball_clone.read().clone();
                ball.next();
                console_log(format!("{:?}", ball).as_str());
                //ball_clone.with_mut(|_| ball);
                ball_clone.set(ball)
            });
            listeners.with_mut(|v| {
            window.set_interval_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), 30).unwrap_throw();
            v });
            closure.forget();
        }
    } else if listeners.read().len() > 0 && hidden.get().to_owned() {
        console_log("clear");
        for id in listeners.read().get_intervals() {
            window.clear_interval_with_handle(id);
        }

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
            "PLAY"
        }
       }
       div {
        id: "game",
        hidden: hidden.get().to_owned(),
        class: "flex flex-row h-[38rem] w-[75rem]",
        player(0, players.get().get(0).unwrap_throw().to_owned()),
        canvas {
            id: "gamecanvas",
            width: "800",
            height: "600",
            class: "w-4/6 border-2 border-black",
        },
        player(1, players.get().get(1).unwrap_throw().to_owned()),
       }
    })
}
