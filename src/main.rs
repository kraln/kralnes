#[macro_use]
extern crate sciter;

use sciter::{HELEMENT, Element, Value};

static mut GFX_BUFFER: [[u32; 256]; 240] = [[0; 256]; 240];

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
        // unroll gfx_buffer into the appropriate format
        let mut buf = [0u8; 256*240*4+12];

        // this is a sciter format for images. header. bgra data follows
        // the A will always be 0...
        buf[0] = 'B' as u8; buf[1] = 'G' as u8;  buf[2] = 'R' as u8; buf[3] = 'A' as u8;
        buf[4] = 0;   buf[5] = 0;    buf[6] = 0;   buf[7] = 255;
        buf[8] = 0;   buf[9] = 0;   buf[10] = 0;  buf[11] = 240;

        let mut idx: usize = 12;
        for y in 0..239
        {
            for x in 0..254 
            {
                unsafe 
                {
                    let col = GFX_BUFFER[y][x];
                    buf[idx+0] = ((col | 0b00000000000000001111111100000000) >> 8  & 0xff) as u8; // r
                    buf[idx+1] = ((col | 0b00000000111111110000000000000000) >> 16 & 0xff) as u8; // g
                    buf[idx+2] = ((col | 0b11111111000000000000000000000000) >> 24 & 0xff) as u8; // b
                    buf[idx+3] = 0; // a
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

