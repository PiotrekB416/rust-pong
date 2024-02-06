#![allow(non_snake_case)]
use std::{usize, ops::Deref};

// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::{html::{geometry::euclid::{num::Round, Trig}, option, select, GlobalAttributes}, prelude::*};
use rand::Rng;
use wasm_bindgen::prelude::*;

#[derive(Clone)]
struct Ball {
    pub x: usize,
    pub y: usize,
    angle: usize,
    speed: usize,
}

impl Ball {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self { x: 2950, y: 3950, angle: (rng.gen::<f64>().round() as usize * 80 + 50) * (rng.gen::<f64>().round() as usize * 3), speed: 1}
    }

    fn next(&mut self) {
        let mut t: f64 = (self.angle as f64).to_radians().tan();

        if t < 1.0 {
            self.y += (self.speed as f64 / t) as usize;
            self.x += self.speed;
        } else {
            self.y += self.speed;
            self.x += (self.speed as f64 / t) as usize;
        }

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
    let ball = use_ref(cx, Ball::new);

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
        window.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref()).unwrap_throw();
        closure.forget();
    }
    {
        let ball_clone = ball.clone();
        let closure = Closure::<dyn FnMut()>::new(move || {
            //console_log("call");
            let (x, y) = ( ball_clone.read().x, ball_clone.read().y );
            let mut ball = ball_clone.read().deref().clone();
            ball.next();
            ball_clone.with_mut(|_| ball);

            let context = get_context("2d".to_string());

            context.begin_path();
            context.arc(x as f64, y as f64, 10.0, 0.0, std::f64::consts::TAU).unwrap_throw();
            context.close_path();

        });
        window.set_interval_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), 100).unwrap_throw();
        closure.forget();
    }



    cx.render(rsx! {
       div {
        id: "start",
        button {
            onclick: move |_| {
                let document = web_sys::window().unwrap_throw().document().unwrap_throw();
                document.get_element_by_id("start").unwrap_throw().set_attribute("hidden", "").unwrap_throw();
                document.get_element_by_id("game").unwrap_throw().remove_attribute("hidden").unwrap_throw();
            },
            "PLAY"
        }
       }
       div {
        id: "game",
        //hidden: true,
        class: "flex flex-row h-[38rem] w-[75rem]",
        player(0, players.get().get(0).unwrap_throw().to_owned()),
        canvas {
            id: "gamecanvas",
            class: "w-4/6",
        },
        player(1, players.get().get(1).unwrap_throw().to_owned()),
       }
    })
}
