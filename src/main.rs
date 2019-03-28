use piston_window::*;

struct Game {
    player_pos: [f64; 2],
    // is there a piston feature for this?
    view_pos: [f64; 2],
}


impl piston_app::Draw for Game {
    fn on_draw(
        self: &mut Self,
        mut centre: math::Matrix2d,
        _dt: f64,
        graphics: &mut G2d,
    ) {
        clear([0.0, 0.0, 0.0, 1.0], graphics);

        // think of this as finding the in-game location of the screen centre
        centre = centre.trans(-self.view_pos[0], -self.view_pos[1]);

        let player = centre.trans(self.player_pos[0], self.player_pos[1]);

        let color = [1.0, 0.0, 0.0, 1.0];
        let rect = [-10.0, -10.0, 20.0, 20.0];
        ellipse(color, rect, player, graphics);
    }
}

impl piston_app::App for Game {
    fn on_update(
        self: &mut Self,
        _args: UpdateArgs,
    ) {
    }
    fn on_input(
        self: &mut Self,
        args: ButtonArgs,
    ) {
        match args.button {
            Button::Keyboard(key) => match key {
                Key::W => self.player_pos[1] -= 1.0,
                Key::A => self.player_pos[0] -= 1.0,
                Key::S => self.player_pos[1] += 1.0,
                Key::D => self.player_pos[0] += 1.0,
                _ => (),
            }
            _ => (),
        }
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
    let game = Game {
        player_pos: [0.0, 0.0],
        view_pos: [0.0, 0.0],
    };
    piston_app::run_until_escape(game);
}
