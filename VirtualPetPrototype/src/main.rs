use minifb::{Key, Window, WindowOptions};

fn main() {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 600;

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Virtual Pet",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Handle user input here

        // Update pet state and draw to buffer here

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
