use piston_window::*;

struct Game {
}


impl piston_app::App for Game {
    fn on_draw(
        self: &mut Self,
        _centre: Context,
        _graphics: &mut G2d,
        _args: RenderArgs,
    ) {
    }
    fn on_update(
        self: &mut Self,
        _args: UpdateArgs,
    ) {
    }
    fn on_input(
        self: &mut Self,
        _args: ButtonArgs,
    ) {
    }
    fn on_mouse_move(
        self: &mut Self,
        _mouse: [f64; 2],
    ) {
    }

    fn window_name() -> &'static str {
        "Homestead Game"
    }
    fn window_starting_size() -> [u32; 2] {
        [600, 600]
    }
}



fn main() {
    let game = Game {};
    piston_app::run_until_escape(game);
}
