use wasm_bindgen::prelude::*;
use rand::prelude::*;

use web_sys::{console, CanvasRenderingContext2d, HtmlCanvasElement};


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn init_context() -> CanvasRenderingContext2d {
    web_sys::window().unwrap()
        .document().unwrap()
        .get_element_by_id("canvas").unwrap().dyn_into::<HtmlCanvasElement>().unwrap()
        .get_context("2d").unwrap().unwrap().dyn_into::<CanvasRenderingContext2d>().unwrap()
}

const HEIGHT: f64 = 600.0;
const WIDTH: f64 = 300.0; 
const ROWS: u8 = 20;
const COLS: u8 = 10;

const BLOCK_HALF_HEIGHT: f64 = HEIGHT/(ROWS as f64)/2.0;
const BLOCK_HALF_WIDTH: f64 = WIDTH/(COLS as f64)/2.0;

fn draw_block(ctx: &CanvasRenderingContext2d, x: f64, y: f64, style: &JsValue) {
    let h_2 = BLOCK_HALF_HEIGHT - 0.5;
    let w_2 = BLOCK_HALF_WIDTH - 0.5;
    ctx.move_to(x, y);
    ctx.begin_path();
    ctx.line_to(x - w_2, y + h_2);
    ctx.line_to(x + w_2, y + h_2);
    ctx.line_to(x + w_2, y - h_2 );
    ctx.line_to(x - w_2, y - h_2);
    ctx.close_path();
    ctx.set_fill_style(style);
    ctx.stroke();
    ctx.fill();
}

struct Coords {dx:i8, dy:i8 }

impl Coords {
    fn rotate(&self) -> Coords {
        return Coords {dx:-self.dy, dy:self.dx}
    }
    fn add(&self, coords: &Coords) -> Coords {
        return Coords {dx: self.dx + coords.dx, dy: self.dy + coords.dy}
    }
    fn sub(&self, coords: &Coords) -> Coords {
        return Coords {dx: self.dx - coords.dx, dy: self.dy - coords.dy}
    }
    fn div(&self, v: i8) -> Coords {
        return Coords {dx: self.dx / v, dy: self.dy / 2}
    }    
}

struct Figure {
    color: String,
    points: Vec<Coords>,
} 

fn draw_figure(ctx: &CanvasRenderingContext2d, figure: &Figure, x: f64, y: f64) {
    let color = JsValue::from_str(&figure.color);
    let mut vy = 0.0;    
    vy += BLOCK_HALF_HEIGHT*7.0;
    for Coords {dx, dy} in &figure.points {
        draw_block(ctx, x + (*dx as f64)*BLOCK_HALF_WIDTH, vy + y + (*dy as f64)*BLOCK_HALF_HEIGHT, &color);
    }    
}

struct Field {
    slots: [[String; COLS as usize]; ROWS as usize], 
}

impl Field {
    //TODO: make coords_iter kinda private??
    fn coords(at: &Coords, figure: &Figure) -> Vec<Coords> {
        let origin = figure.get_origin();
        figure.points
            .iter()
            .map(|point| point.sub(&origin).div(2).add(at))
            .collect()
    }
    fn can_put(&self, at: &Coords, figure: &Figure) -> bool {
        Field::coords(at, figure).iter().all(|point| 
            point.dx >= 0 && point.dy >= 0 && (point.dx as u8) < COLS && (point.dy as u8) < ROWS &&
            self.slots[point.dy as usize][point.dx as usize] == ""
        )
    }
    fn can_fall(&self, at: &Coords, figure: &Figure) -> bool {
        self.can_put(&Coords {dx: at.dx, dy: at.dy+1}, figure)
    }
    fn put(&mut self, at: &Coords, figure: &Figure) {
        Field::coords(at, figure).iter().for_each(|coords|
            self.slots[coords.dy as usize][coords.dx as usize] = figure.color.clone()
        );
    }
    fn draw(&self, ctx: &CanvasRenderingContext2d, x: f64, y: f64) {
        let mut cur_x = x;
        let mut cur_y = y;
        for row in &self.slots {
            for item in row {
                draw_block(ctx, cur_x, cur_y, &JsValue::from_str(if(item == "") {"white"} else {item}));
                cur_x += BLOCK_HALF_WIDTH*2.0;
            }
            cur_y += BLOCK_HALF_HEIGHT*2.0;
            cur_x = x;            
        }
    }
}

impl Figure {
    fn rotate (&self) -> Figure {
        let points = self.points.iter().map(|point| point.rotate()).collect();
        return Figure {color: self.color.clone(), points}
    }
    fn get_origin (&self) -> Coords {
        self.points.iter().fold(Coords{dx:0, dy:0}, |acc, point| Coords{dx:acc.dx.min(point.dx),dy:acc.dy.min(point.dy)})
    }
}

fn next_random(figures: &[Figure; 7]) -> &Figure {
    let i = rand::thread_rng().gen_range(0..7u8);
    &figures[i as usize]
}

// fn can_put(field: &Field, figure: &Figure, row: u8, col: u8) -> bool {
    // field.slots
// }

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    
    let ctx = init_context();    
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
 
    console::log_1(&JsValue::from_str("Hello world!"));    
    let figures: [Figure; 7] = [
        Figure {
            color: String::from("blue"),
            points: vec![Coords{dx:-1, dy:-1}, Coords{dx:-1, dy:1}, Coords{dx:1, dy:-1}, Coords{dx:1, dy:1}],
        },
        Figure {
            color: String::from("red"),
            points: vec![Coords{dx:-1, dy:-1}, Coords{dx:1, dy:-1}, Coords{dx:1, dy:1}, Coords{dx:1, dy:3}],
        },
        Figure {
            color: String::from("orange"),
            points: vec![Coords{dx:1, dy:-1}, Coords{dx:-1, dy:-1}, Coords{dx:-1, dy:1}, Coords{dx:-1, dy:3}],
        },
        Figure {
            color: String::from("green"),
            points: vec![Coords{dx:-2, dy:0}, Coords{dx:0, dy:0}, Coords{dx:2, dy:0}, Coords{dx:4, dy:0}],
        },
        Figure {
            color: String::from("gray"),
            points: vec![Coords{dx:0, dy:2}, Coords{dx:0, dy:0}, Coords{dx:2, dy:0}, Coords{dx:2, dy:-2}],
        },
        Figure {
            color: String::from("lightblue"),
            points: vec![Coords{dx:0, dy:2}, Coords{dx:0, dy:0}, Coords{dx:-2, dy:0}, Coords{dx:-2, dy:-2}],
        },
        Figure {
            color: String::from("yellow"),
            points: vec![Coords{dx:0, dy:2}, Coords{dx:0, dy:0}, Coords{dx:0, dy:-2}, Coords{dx:2, dy:0}],
        },
    ];
    let fig = figures[1].rotate().rotate().rotate();
    draw_figure(&ctx, &fig, 100.0, 0.0);
    let mut field = Field {
        slots: Default::default()
    };
    console::log_1(&JsValue::from_bool(field.can_put(&Coords {dx: 5, dy: 7}, &fig)));
    field.put(&Coords {dx: 5, dy: 17}, &fig);
    field.put(&Coords {dx: 3, dy: 6}, &figures[2]);   
    field.put(&Coords {dx: 5, dy: 6}, next_random(&figures));
    console::log_1(&JsValue::from_bool(field.can_fall(&Coords {dx: 5, dy: 4}, &fig)));
    console::log_1(&JsValue::from_bool(field.can_fall(&Coords {dx: 5, dy: 5}, &fig)));
    console::log_1(&JsValue::from_bool(field.can_put(&Coords {dx: 5, dy: 6}, &fig)));
    console::log_1(&JsValue::from_bool(field.can_put(&Coords {dx: 5, dy: 7}, &fig)));
    console::log_1(&JsValue::from_bool(field.can_put(&Coords {dx: 5, dy: 8}, &fig)));
    console::log_1(&JsValue::from_bool(field.can_put(&Coords {dx: 5, dy: 9}, &fig)));
    field.draw(&ctx, 15.0, 15.0);
    Ok(())
}
