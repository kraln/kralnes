extern crate sciter;

fn main() {
  let html = include_bytes!("ui/main.htm");
  let mut frame = sciter::Window::new();
  frame.load_html(html, None);
  frame.run_app();
}
