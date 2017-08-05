use std::fs::File;
use std::io::Read;

#[macro_use]
extern crate sciter;

use sciter::{HELEMENT, Element, Value};

const GFX_WIDTH: usize = 256;
const GFX_HEIGHT: usize = 240;

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

        let mut gfx_buffer = [[0; GFX_WIDTH]; GFX_HEIGHT];

        // unroll gfx_buffer into the appropriate format
        let mut buf = [0u8; ((GFX_WIDTH)*(GFX_HEIGHT)*4+12)];

        // this is a sciter format for images. header. bgra data follows
        // the A will always be 0...
        buf[0] = 'B' as u8; buf[1] = 'G' as u8;  buf[2] = 'R' as u8; buf[3] = 'A' as u8;
        buf[4] = 0;   buf[5] = 0;    buf[6] = ((GFX_WIDTH & 0xff00) >> 8) as u8;   buf[7] = (GFX_WIDTH & 0xff) as u8;
        buf[8] = 0;   buf[9] = 0;   buf[10] = ((GFX_HEIGHT & 0xff00) >> 8) as u8;  buf[11] = (GFX_HEIGHT & 0xff) as u8;

        let mut idx: usize = 12;
        for y in 0..GFX_HEIGHT
        {
            for x in 0..GFX_WIDTH
            {
                let col = gfx_buffer[y][x];
                unsafe {
                buf[idx+0] = inc;// ((col | 0b00000000000000001111111100000000) >> 8  & 0xff) as u8; // r
                buf[idx+1] = inc;// ((col | 0b00000000111111110000000000000000) >> 16 & 0xff) as u8; // g
                buf[idx+2] = inc;// ((col | 0b11111111000000000000000000000000) >> 24 & 0xff) as u8; // b
                buf[idx+3] = inc;// 255; // a
                inc = inc.wrapping_add(1);
                if(inc > 254) {inc = 0};
                }
                idx+=4;
            }
        }
        Value::from(&buf[..])
    }

    fn tick(&mut self, ) -> Value
    {
        // CPU run a frame
        Value::new()
    }

    fn load_rom(&mut self, mut filename: String) -> Value
    {
        let mut data = Vec::new();
        println!("Asked to open file {:?}", filename);
        let actual_filename = filename.split_off(7);
        let mut f = File::open(actual_filename).expect("Unable to open file");
        f.read_to_end(&mut data).expect("Unable to read data");
        println!("{}", data.len());
        Value::from(&data[..])
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
        fn load_rom(String);
    }
}

