#![allow(non_snake_case)]
use std::ops::Deref;

mod funcs;
use funcs::Funcs;
mod consts;
use consts::*;
mod ball;
use ball::Ball;
mod paddle;
use paddle::Paddle;

// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::js_sys::Function;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn console_log(s: &str);
}

fn main() {
    // launch the web app
    dioxus_web::launch::launch(App, vec![], vec![]);
}

fn get_context(context_type: String) -> web_sys::CanvasRenderingContext2d {
    let document = web_sys::window().unwrap_throw().document().unwrap_throw();
    let canvas = document.get_element_by_id("gamecanvas").unwrap_throw();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap_throw();
    canvas
        .get_context(&context_type)
        .unwrap_throw()
        .unwrap_throw()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap_throw()
}

fn move_player(player: &mut Paddle, direction: bool) {
    player.move_player(direction);
}

#[derive(Clone)]
pub struct Controller {
    pub w: bool,
    pub s: bool,
    pub au: bool,
    pub ad: bool,
    players: Signal<Vec<Paddle>>,
}

impl Controller {
    pub fn new(players: Signal<Vec<Paddle>>) -> Self {
        Self {
            w: false,
            s: false,
            au: false,
            ad: false,
            players,
        }
    }

    pub fn action(&mut self) {
        if self.w {
            self.players.with_mut(|v| {
                move_player(&mut v[0], true);
            });
        }

        if self.s {
            self.players.with_mut(|v| {
                move_player(&mut v[0], false);
            });
        }

        if self.au {
            self.players.with_mut(|v| {
                move_player(&mut v[1], true);
            });
        }

        if self.ad {
            self.players.with_mut(|v| {
                move_player(&mut v[1], false);
            });
        }
    }
}

// create a component that renders a div with the text "Hello, world!"
fn App() -> Element {
    let players = use_signal(|| vec![false, true]);
    let points = use_signal(|| vec![0, 0]);
    let paddles = use_signal(|| vec![Paddle::new(true), Paddle::new(false)]);
    let controller = use_signal(|| Controller::new(paddles.clone()));
    let ball = use_signal(|| Ball::new(points.clone()));

    let mut listeners = use_signal(Funcs::new);

    let mut hidden = use_signal(|| true);

    let player = |id: usize, player: bool| {
        let mut players = players.clone();
        rsx!( div {
            id: "player{id}",
            class: "w-1/6 select-none hover:select-all",
            span {
                "player {id}: {points.read()[id]}"
            },
            select {
                onchange: move |_: Event<_>| {
                    console_log("change");
                    //let players = players.clone();
                    players.with_mut(|players| players[id] = !players[id]);
                    //console_log("change");
                    //console_log(format!("{:?}", players.read()).as_str());
                },
                option {selected: player, "player"},
                option {selected: !player, "bot"},
            }
        }, )
    };


    let window = web_sys::window().unwrap_throw();
    let players_clone = players.clone();

    // IF NOT PAUSED
    if listeners.read().len() == 0 && !*hidden.read() {
        // REMOVE KEY LISTENERS ON <select>
        [0, 1].iter().for_each(|n| {
            let a = web_sys::window()
                .unwrap_throw()
                .document()
                .unwrap_throw()
                .get_element_by_id(format!("player{n}").as_str());
            if a.is_none() {
                return;
            }
            a.unwrap()
                .add_event_listener_with_callback(
                    "keydown",
                    &Function::new_with_args("e", "e.preventDefault()"),
                    )
                .unwrap_throw();
        });
        // SET KEYDOWN
        {
            let mut controller_clone = controller.clone();
            let closure = Closure::<dyn FnMut(_)>::new(move |e: web_sys::InputEvent| {
                let key = e
                    .deref()
                    .clone()
                    .dyn_into::<web_sys::KeyboardEvent>()
                    .unwrap_throw()
                    .key();
                //console_log(format!("keydown: {:?}", key.as_str()).as_str());

                match key.as_str() {
                    "w" => controller_clone
                        .with_mut(|c| c.w = *players_clone.read().get(0).unwrap_throw()),
                    "s" => controller_clone
                        .with_mut(|c| c.s = *players_clone.read().get(0).unwrap_throw()),
                    "ArrowUp" => controller_clone
                        .with_mut(|c| c.au = *players_clone.read().get(1).unwrap_throw()),
                    "ArrowDown" => controller_clone
                        .with_mut(|c| c.ad = *players_clone.read().get(1).unwrap_throw()),
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

        // SET KEYUP
        {
            let mut controller_clone = controller.clone();
            let closure = Closure::<dyn FnMut(_)>::new(move |e: web_sys::InputEvent| {
                let key = e
                    .deref()
                    .clone()
                    .dyn_into::<web_sys::KeyboardEvent>()
                    .unwrap_throw()
                    .key();
                //console_log(format!("keyup: {:?}", key.as_str()).as_str());

                match key.as_str() {
                    "w" => controller_clone.with_mut(|c| c.w = false),
                    "s" => controller_clone.with_mut(|c| c.s = false),
                    "ArrowUp" => controller_clone.with_mut(|c| c.au = false),
                    "ArrowDown" => controller_clone.with_mut(|c| c.ad = false),
                    _ => {}
                }
            });
            let func = closure.as_ref().unchecked_ref::<Function>().clone();
            window
                .add_event_listener_with_callback("keyup", &func.clone())
                .unwrap_throw();
            listeners.with_mut(|v| {
                v.push_event(func.clone());
                v.clone()
            });
            closure.forget();
        }

        // GAME
        {
            // DRAW
            let mut ball_clone = ball.clone();
            let mut paddles_clone = paddles.clone();
            let mut controller_clone = controller.clone();
            let players_clone = players.clone();
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

                controller_clone.with_mut(|c| {
                    c.action();
                });
                let ball = ball_clone.read();
                players_clone
                    .read()
                    .iter()
                    .enumerate()
                    .filter(|(_, v)| !**v)
                    .for_each(|(i, _)| {
                        paddles_clone.with_mut(|v| {
                            v[i].move_ai(ball.deref());
                        })
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

            // BALL NEXT
            let mut ball_clone = ball.clone();
            let mut paddles_clone = paddles.clone();
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
                            10,
                        )
                        .unwrap_throw(),
                );
                v.clone()
            });
            closure.forget();
        }
    } else if listeners.read().len() > 0 && hidden.read().to_owned() {

        // REMOVE LISTENERS AND INTERVALS IF PAUSED
        //console_log("clear");
        for id in listeners.read().get_intervals() {
            window.clear_interval_with_handle(id);
        }
        for f in listeners.read().get_events() {
            window
                .remove_event_listener_with_callback("keydown", &f)
                .unwrap_throw();
            window
                .remove_event_listener_with_callback("keyup", &f)
                .unwrap_throw();
        }
        listeners.with_mut(|v| {
            v.remove_all();
            v.clone()
        });
    }
    // RENDER HTML
    rsx! {
       div {
        class: "w-fit h-10 mx-auto",
        id: "start",
        button {
            onclick: move |_| {
                //let document = web_sys::window().unwrap_throw().document().unwrap_throw();
                //document.get_element_by_id("start").unwrap_throw().set_attribute("hidden", "").unwrap_throw();
                //document.get_element_by_id("game").unwrap_throw().remove_attribute("hidden").unwrap_throw();
                hidden.with_mut(|v| *v = !*v);
            },
            class: "my-auto",
            if hidden.read().to_owned() { "PLAY" } else { "STOP" }
        }
       }
       div {
        id: "game",
        class: /*if hidden.clone().get().to_owned() { "hidden" } else {*/ "flex flex-row h-[38rem] w-[75rem] mx-auto" /*}*/,
        {player(0, players.read().get(0).unwrap_throw().to_owned())},
        canvas {
            id: "gamecanvas",
            width: "{WIDTH}",
            height: "{HEIGHT}",
            class: "w-4/6 border-2 border-black",
        },
        {player(1, players.read().get(1).unwrap_throw().to_owned())},
       }
    }
}
