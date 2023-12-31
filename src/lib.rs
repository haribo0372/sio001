pub enum Key {
    Left,
    Right,
    Up,
    Down,
    Space
}


extern "C" {
    fn js_clear_screen_to_color(red: f32, green: f32, blue: f32, alpha: f32);
    fn js_draw_rectangle(x: f32, y: f32, width: f32, height: f32);
    fn js_clear_screen_to_random_color();
}

pub fn clear_screen_to_color(red: f32, green: f32, blue: f32, alpha: f32) {
    unsafe {
        js_clear_screen_to_color(red, green, blue, alpha)
    }
}
pub fn draw_rectangle(x: f32, y: f32, width: f32, height: f32) {
    unsafe {
        js_draw_rectangle(x, y, width, height);
    }
}
pub fn clear_screen_to_random_color(){
    unsafe {
        js_clear_screen_to_random_color();
    }
}

#[no_mangle]
pub extern "C" fn key_press(value: usize) {
    let key = match value {
        1 => Key::Left,
        2 => Key::Right,
        3 => Key::Up,
        4 => Key::Down,
        5 => Key::Space,
        _ => return,
    };

    EVENT_HANDLER.with(|event_handler| (event_handler.borrow_mut())(key))
}

thread_local! {
    pub static EVENT_HANDLER: std::cell::RefCell<Box<dyn FnMut(Key)>> = std::cell::RefCell::new(Box::new(|_|{}));
}

pub fn set_event_handler(function: impl FnMut(Key) + 'static) {
    EVENT_HANDLER.with(|event_handler| {
        *event_handler.borrow_mut() = Box::new(function);
    });
}