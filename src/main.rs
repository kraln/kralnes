#[macro_use]
extern crate sciter;

use sciter::{HELEMENT, Element, Value};

const GFX_WIDTH: u8 = 255;
const GFX_HEIGHT: u8 = 239;

static mut inc: u8 = 0;
fn main()
{
    let html = include_bytes!("ui/main.htm");
    let mut frame = sciter::Window::new();
    let handler = Handler { root: None };

    frame.load_html(html, None);
    frame.event_handler(handler);
    frame.run_app();
}

struct Handler
{
    root: Option<Element>,
}

impl Handler
{
    // GBRA size + header + width + height
    fn get_gfx_buffer(&mut self) -> Value
    {

        let mut gfx_buffer = [[0; GFX_WIDTH as usize]; GFX_HEIGHT as usize];

        // unroll gfx_buffer into the appropriate format
        let mut buf = [0u8; ((GFX_WIDTH as usize)*(GFX_HEIGHT as usize)*4+12)];

        // this is a sciter format for images. header. bgra data follows
        // the A will always be 0...
        buf[0] = 'B' as u8; buf[1] = 'G' as u8;  buf[2] = 'R' as u8; buf[3] = 'A' as u8;
        buf[4] = 0;   buf[5] = 0;    buf[6] = 0;   buf[7] = GFX_WIDTH;
        buf[8] = 0;   buf[9] = 0;   buf[10] = 0;  buf[11] = GFX_HEIGHT;

        let mut idx: usize = 12;
        for y in 0..GFX_HEIGHT as usize
        {
            for x in 0..GFX_WIDTH as usize
            {
                let col = gfx_buffer[y][x];
                unsafe {
                buf[idx+0] = inc;// ((col | 0b00000000000000001111111100000000) >> 8  & 0xff) as u8; // r
                buf[idx+1] = inc;// ((col | 0b00000000111111110000000000000000) >> 16 & 0xff) as u8; // g
                buf[idx+2] = inc;// ((col | 0b11111111000000000000000000000000) >> 24 & 0xff) as u8; // b
                buf[idx+3] = inc;// 255; // a
                inc = inc.wrapping_add(1);
                }
                idx+=4;
            }
        }

        let res: Value = Value::from(&buf[..]);
        res
    }

    fn tick(&mut self, ) -> Value
    {
        // CPU run a frame
        let mut res = Value::new();


        // stuff

        res
    }
}

impl sciter::EventHandler for Handler
{

    fn attached(&mut self, root: HELEMENT)
    {
        self.root = Some(Element::from(root));
    }

    dispatch_script_call!
    {
        fn tick();
        fn get_gfx_buffer();
    }
}

