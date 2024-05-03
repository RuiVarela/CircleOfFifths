mod utils;

use std::f64;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;


const BASE_SPEED: f64 = 0.4;
const HIT_SHOW_DURATION: f64 = 0.5; // how long does the color fae out after a hit
const HIT_THRESHOLD: f64 = (2.0 * f64::consts::PI) * 0.003; // angle difference in radians to consider a hit
const TRIGGER_TIME: f64 = 0.25; // minimum time since last hit to trigger a new one


//
// CircleOfFifths
//

struct Key {
    name: String,
    color: String,
    angle: f64,
    hit_time: f64,
}


#[wasm_bindgen]
pub struct CircleOfFifths {
    keys: Vec<Key>,
    colors: Vec<String>,

    window: web_sys::Window,
    canvas: web_sys::HtmlCanvasElement,
    context: web_sys::CanvasRenderingContext2d,

    last_time: f64,

    positions : Vec<f64>,
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl CircleOfFifths {
    pub fn new() -> CircleOfFifths {
        set_panic_hook();

        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let canvas = document.get_element_by_id("canvas").expect("no canvas element found")
                                                .dyn_into::<web_sys::HtmlCanvasElement>().expect("canvas element is not a HtmlCanvasElement");

        let context = canvas.get_context("2d")
            .expect("unable to get 2d cxB   ontext").expect("unable to get 2d context")
            .dyn_into::<web_sys::CanvasRenderingContext2d>().expect("unable to get 2d context");
        
        let colors: Vec<String> = vec![
            "#008DDA", "#41C9E2", "#ACE2E1", "#7F6A93", "#FEFF86", "#B0DAFF",
            "#5E503F", "#FFC94A", "#C40C0C", "#FF6500", "#FF8A08", "#FFC100",
            "#453F78", "#795458", "#C08B5C", "#49111C"]
            .iter().map(|s| s.to_string()).collect();

        let mut keys = Vec::<Key>::new();
        let keynames = vec!["C", "G", "D", "A", "E", "B", "Gb", "Db", "Ab", "Eb", "Bb", "F"];

        for (i, name) in keynames .iter().enumerate() {

            let color = colors[i % colors.len()].clone();

            let pi = f64::consts::PI;
            let angle_step = pi * 2.0 / keynames.len() as f64;
            let angle = i as f64 * angle_step;
   
            keys.push(Key { name: name.to_string(), color, angle, hit_time: -1.0f64 });
        }


        CircleOfFifths {
            keys, colors,
            window, canvas, context,
            last_time: -1.0f64, 
            positions: Vec::<f64>::new()
        }
    }

    pub fn tick(&mut self, time: f64, sides: u32, speed: f64) {

        // log!("tick time={} sides={} speed={}", time, sides, speed);
        //log!("canvas size {}x{}", width, height);

        let delta: f64;
        if self.last_time < 0.0 {
            delta = 0.0;
        } else {
            delta = time - self.last_time;
        }
        self.last_time = time;

        //
        // setup positions
        //
        if self.positions.len() != sides as usize {
            self.positions.clear();
            let angle_delta = 2.0 * f64::consts::PI / sides as f64;

            for i in 0..sides {
                let angle = i as f64 * angle_delta;
                self.positions.push(angle);
            }
        }

        //
        // move circle positions
        //
        for pos in self.positions.iter_mut() {
            *pos += delta * speed * BASE_SPEED;
        }

        //
        // check for hits
        //
        let mut triggered = Vec::<usize>::new();

        for pos in self.positions.iter() {
            for (index, key) in self.keys.iter_mut().enumerate() {
                let mut pos_angle = pos % (2.0 * f64::consts::PI);
                if pos_angle < 0.0 {
                    pos_angle += 2.0 * f64::consts::PI;
                }

                let delta_angle = (pos_angle - key.angle).abs();
                if delta_angle < HIT_THRESHOLD {
                    let delta_time = time - key.hit_time;
                    key.hit_time = time;

                    if delta_time > TRIGGER_TIME {
                        triggered.push(index);
                        //log!("hit {} {}", key.name, delta_time);
                    }
                } 
            }
        }
        
        // play sounds
        for key in triggered.iter() {
            self.play_sound(&self.keys[*key]);
        }

        //log!("conts {counts}");

        // ensure canvas fills screen
        let width = self.window.inner_width().unwrap().as_f64().unwrap() as f64;
        let height = self.window.inner_height().unwrap().as_f64().unwrap() as f64;

        let center_x = width as f64 / 2.0;
        let center_y = height as f64 / 2.0;

        // update cavas size
        self.canvas.set_width(width as u32);
        self.canvas.set_height(height as u32);

        self.render_frame(center_x, center_y, width, height);
    }
}

impl CircleOfFifths {

    fn render_frame(&self, center_x: f64, center_y: f64, width: f64, height: f64) {
        let min_dim = width.min(height) as f64;
        let font_size = min_dim * 0.05;
        let circle_radius = min_dim * 0.5 - font_size * 1.5;

        // clear
        {
            self.context.set_fill_style(&JsValue::from_str("black")); 
            self.context.fill_rect(0.0, 0.0, width, height);
        }

        self.context.set_line_width(font_size * 0.07);

        {
            self.context.set_stroke_style(&JsValue::from_str("white"));
            self.render_base_circle(center_x, center_y, circle_radius);

            let value = JsValue::from_str(self.colors[self.positions.len() as usize % self.colors.len()].as_str());
            self.context.set_stroke_style(&value);  
            if self.positions.len() > 2 {
                self.render_polygon(center_x, center_y, circle_radius);
            } else {
                self.render_positions_with_lines(center_x, center_y, circle_radius);
            }

            self.render_notes_names(center_x, center_y, circle_radius, font_size);
        }

    }

    fn render_base_circle(&self, center_x: f64, center_y: f64, radius: f64) {
        let pi = f64::consts::PI;

        self.context.begin_path();
        self.context.arc(center_x, center_y, radius, 0.0, pi * 2.0).expect("unable to draw circle");
        self.context.stroke();
    }
        
    fn render_notes_names(&self, center_x: f64, center_y: f64, radius: f64, font_size: f64) {
        let pi = f64::consts::PI;
        for key in self.keys.iter() {
            let angle = key.angle - pi / 2.0;

            let px = center_x + radius * angle.cos();
            let py = center_y + radius * angle.sin();

            let tx = center_x + (radius + font_size * 0.8) * angle.cos();
            let ty = center_y + (radius + font_size * 0.8) * angle.sin();

            for j in 0..2 {
                self.context.begin_path();
                self.context.arc(px, py, font_size * 0.1, 0.0, pi * 2.0).expect("unable to draw circle");
                self.context.fill();

                if j == 0 {
                    self.context.set_fill_style(&JsValue::from_str("white"));  
                    self.context.set_global_alpha(1.0f64);
                } else if j == 1 {
                    let elapsed = self.last_time - key.hit_time;
                    let alpha = 1.0 - (elapsed / HIT_SHOW_DURATION).clamp(0.0, 1.0);

                    if alpha > 0.0 {
                        let value = JsValue::from_str(&key.color);
                        self.context.set_global_alpha(alpha);
                        self.context.set_fill_style(&value);  
                    }
                }
 
                let font = format!("{}px Arial", font_size as u32);
                self.context.set_font(&font);
                self.context.set_text_align("center");
                self.context.set_text_baseline("middle");
                self.context.fill_text(&key.name, tx, ty).expect("unable to draw text");
         
            }
        }
        self.context.set_global_alpha(1.0f64);
    }

    fn render_positions_with_lines(&self, center_x: f64, center_y: f64, radius: f64) {
        let pi = f64::consts::PI;
        for pos in self.positions.iter() {
            let angle = pos - pi / 2.0;
            let px = center_x + radius * angle.cos();
            let py = center_y + radius * angle.sin();

            self.context.begin_path();
            self.context.move_to(center_x, center_y);
            self.context.line_to(px, py);
            self.context.stroke();
        }
    }

    fn render_polygon(&self, center_x: f64, center_y: f64, radius: f64) {
        let pi = f64::consts::PI;
        self.context.begin_path();
        for (i, current) in self.positions.iter().enumerate() {
            let angle = current - pi / 2.0;
            let px = center_x + radius * angle.cos();
            let py = center_y + radius * angle.sin();

            if i == 0 {
                self.context.move_to(px, py);
            } else {
                self.context.line_to(px, py);
            }
        }
        self.context.close_path();
        self.context.stroke();
    }

    fn play_sound(&self, key: &Key) {
        let note = format!("notes/{}.mp3", key.name);
        let audio = web_sys::HtmlAudioElement::new_with_src(&note).expect("unable to create audio element");
        audio.set_volume(1.0);
        let _ = audio.play().expect("unable to play audio");
    }

}