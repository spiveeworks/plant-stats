use piston_window::*;

mod dir;
mod water;
mod crop;

use self::dir::*;

struct Game {
    player_pos: [f64; 2],
    // is there a piston feature for this?
    view_pos: [f64; 2],

    move_bindings: Dir2<Key>,
    movement: Dir2<bool>,

    water: water::WaterMap,
    crops: crop::CropMap,
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

        {
            let tile_size = 20.0;
            let corner = centre.trans(-16.0 * tile_size, -16.0 * tile_size);
            let rect = [0.0, 0.0, tile_size, tile_size];
            // @DebugPerformance
            // this loop seems very very slow in debug,
            // removing matrix transformations doesn't help
            // piston is just not made for debug mode
            for i in 0..32 {
                for j in 0..32 {
                    {
                        let val = self.water[i][j] as f32 / 200.0;
                        let color = [val, val, val, 1.0];

                        let tile = corner.trans(
                            tile_size * i as f64,
                            tile_size * j as f64
                        );
                        rectangle(color, rect, tile, graphics);
                    }
                    if let Some(crop) = self.crops[i][j] {
                        let mut size = crop.growth / crop.genome_derived.fibre_time;
                        if size > 1.0 {
                            size = 1.0;
                        }
                        let color = [0.0, (crop.health / crop.genome_derived.max_health) as f32, 0.0, 1.0];
                        let tile = corner.trans(
                            tile_size * (i as f64 + 0.5),
                            tile_size * (j as f64 + 0.5),
                        );
                        let screen_size = size * tile_size;
                        let rect = [-screen_size / 2.0, -screen_size / 2.0, screen_size, screen_size];
                        ellipse(color, rect, tile, graphics);
                    }
                }
            }
        }

        {
            let player = centre.trans(self.player_pos[0], self.player_pos[1]);

            let color = [1.0, 0.0, 0.0, 1.0];
            let rect = [-10.0, -10.0, 20.0, 20.0];
            ellipse(color, rect, player, graphics);
        }
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
                if pressed && key == Key::P {
                    crop::update_crops(&mut self.crops, &mut self.water);
                    water::diffuse_water(&mut self.water);
                    self.water[16][16] = 200.0;
                }
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
    let mut water = [[10.0; 32]; 32];
    water[16][16] = 200.0;
    let mut crops = [[None; 32]; 32];
    for i in 12..22 {
        crops[i][14] = Some(crop::SeedData {
            species: crop::Crop::Root,
            richness: 0.5,
            volume: 0.5,
        }.crop());
        crops[i][15] = Some(crop::SeedData {
            species: crop::Crop::Bean,
            richness: 0.5,
            volume: 0.5,
        }.crop());
        crops[i][16] = Some(crop::SeedData {
            species: crop::Crop::Gourd,
            richness: 0.5,
            volume: 0.5,
        }.crop());
        crops[i][17] = Some(crop::SeedData {
            species: crop::Crop::Grass,
            richness: 0.5,
            volume: 0.5,
        }.crop());
    }
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
        water,
        crops,
    };
    piston_app::run_until_escape(game);
}
