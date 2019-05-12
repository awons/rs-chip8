use chip8::display::*;
use std::ops;
use wasm_bindgen::{JsCast, JsValue};

const MULTIPLIER_X: f64 = 640.0 / 64.0;
const MULTIPLIER_Y: f64 = 320.0 / 32.0;

pub struct BrowserDisplay {
    context: web_sys::CanvasRenderingContext2d,
    fill_color_white: JsValue,
    fill_color_black: JsValue,
}

impl BrowserDisplay {
    pub fn new() -> BrowserDisplay {
        let canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("game-canvas")
            .unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into().map_err(|_| ()).unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        BrowserDisplay {
            context,
            fill_color_white: JsValue::from_str("#FFFFFF"),
            fill_color_black: JsValue::from_str("#000000"),
        }
    }
}

impl GraphicDisplay for BrowserDisplay {
    fn draw<M>(&mut self, memory: &M)
    where
        M: ops::Index<usize, Output = [u8]>,
    {
        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                if memory[y][x] == 1 {
                    self.context.set_fill_style(&self.fill_color_black);
                } else {
                    self.context.set_fill_style(&self.fill_color_white);
                }

                self.context.fill_rect(
                    x as f64 * MULTIPLIER_X,
                    y as f64 * MULTIPLIER_Y,
                    MULTIPLIER_X,
                    MULTIPLIER_Y,
                );
            }
        }
    }
}
