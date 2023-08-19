// use rand::Rng;
use sio001::*;

fn draw_barrier(mut x: f32, y: f32){
    while x <= 280.{
        draw_rectangle(x, y, 10., 25.);
        x += 30.0;
    };
}

fn main() {
    let mut x_posi_platform = 100.0;
    let mut y_posi_platform = 30.0;
    let x_posi_barrier = 10.0;
    let mut y_posi_barrier = 150.0;
    // Смайл вначале:
    draw_rectangle(145., 100., 8., 25.);
    draw_rectangle(125., 100., 8., 25.);
    draw_rectangle(115., 80., 50., 5.);

    set_event_handler(move |key| {
        y_posi_barrier -= 10.0;
        clear_screen_to_random_color();
        let move_amount = 10.0;
        match key {
            Key::Space => {
                clear_screen_to_color(1.0, 1.0, 1.0, 1.0);
            }
            Key::Left => {
                x_posi_platform -= move_amount;
            },
            Key::Right => {
                x_posi_platform += move_amount
            },
            Key::Up => {
                y_posi_platform += move_amount
            },
            Key::Down => {
                y_posi_platform -= move_amount
            },
        }

        // sio001::clear_screen_to_color(1.0, 1.0, 1.0, 1.0));
        draw_rectangle(x_posi_platform, y_posi_platform, 20., 5.);
        draw_barrier(x_posi_barrier, y_posi_barrier);
        // if x && y_position == y{clear_screen_to_color(1.0, 1.0, 1.0, 1.0)};

        if y_posi_barrier ==0. { y_posi_barrier =150. };
    });
}
