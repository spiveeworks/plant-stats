use piston_window::*;

mod dir;

use self::dir::*;

struct Game {
    player_pos: [f64; 2],
    // is there a piston feature for this?
    view_pos: [f64; 2],

    move_bindings: Dir2<Key>,
    movement: Dir2<bool>,
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
        args: UpdateArgs,
    ) {
        let dt = args.dt;
        let dir = self.movement.dir_vec();
        let speed = 100.0;

        for i in 0..=1 {
            self.player_pos[i] += dt * dir[i] * speed;
        }
    }

    fn on_input(
        self: &mut Self,
        args: ButtonArgs,
    ) {
        match args.button {
            Button::Keyboard(key) => {
                let pressed = args.state == ButtonState::Press;
                self.movement.write_if_eq(&self.move_bindings, &key, &pressed);
            },
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

        move_bindings: Dir2 {
            x: Dir1 {
                pos: Key::D,
                neg: Key::A,
            },
            y: Dir1 {
                pos: Key::S,
                neg: Key::W,
            },
        },
        movement: Default::default(),
    };
    piston_app::run_until_escape(game);
}
