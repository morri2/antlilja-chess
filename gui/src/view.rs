use crate::game_controller::GameController;
use chess::{Color, Pos};
use piston_window::{self, clear, ellipse, rectangle, Context, G2d, G2dTexture, Image};
use std::path::Path;

const COLOR_1: [f32; 4] = [0.29, 0.39, 0.54, 1.0];
const COLOR_2: [f32; 4] = [0.82, 0.87, 0.96, 1.0];
const COLOR_3: [f32; 4] = [0.34, 0.31, 0.31, 1.0];
const COLOR_4: [f32; 4] = [0.58, 0.59, 0.63, 1.0];
const COLOR_5: [f32; 4] = [0.15, 0.12, 0.10, 1.0];

pub struct ViewSettings {
    pub board_size: f64,
    pub padding: f64,
    pub background_color: [f32; 4],
    pub border_color: [f32; 4],
    pub white_color: [f32; 4],
    pub black_color: [f32; 4],
    pub move_color: [f32; 4],
}

impl Default for ViewSettings {
    fn default() -> ViewSettings {
        ViewSettings {
            board_size: 640.0,
            padding: 192.0,
            background_color: COLOR_3,
            border_color: COLOR_5,
            white_color: COLOR_1,
            black_color: COLOR_2,
            move_color: COLOR_4,
        }
    }
}

pub struct View {
    pub settings: ViewSettings,
    textures: Vec<G2dTexture>,
}

impl View {
    pub fn new(settings: ViewSettings, textures: Vec<G2dTexture>) -> View {
        View { settings, textures }
    }

    pub fn create_textures(window: &mut piston_window::PistonWindow) -> Vec<G2dTexture> {
        use piston_window::{Texture, TextureSettings};

        let colors = ["white".to_string(), "black".to_string()];
        let piece_types = [
            "pawn".to_string(),
            "rook".to_string(),
            "knight".to_string(),
            "bishop".to_string(),
            "queen".to_string(),
            "king".to_string(),
        ];

        let mut textures: Vec<G2dTexture> = vec![];

        for color in colors.iter() {
            for piece in piece_types.iter() {
                println!(
                    "Fetching texture: {:?}",
                    Path::new(&format!("gui/resources/pieces/{}-{}.png", color, piece))
                );
                let texture: piston_window::G2dTexture = Texture::from_path(
                    &mut window.create_texture_context(),
                    &Path::new(&format!("gui/resources/pieces/{}-{}.png", color, piece)),
                    piston_window::Flip::None,
                    &TextureSettings::new(),
                )
                .unwrap();

                textures.push(texture);
            }
        }

        textures
    }

    pub fn calculate_size(&self, c: &Context) -> (f64, f64, f64) {
        let view_size = c.get_view_size();
        let width: f64 = view_size[0];
        let height: f64 = view_size[1];

        let min_padding = width / 5.0;

        if width - min_padding * 2.0 <= height {
            let board_size: f64 = width - 2.0 * min_padding;

            let x_padding: f64 = min_padding;
            let y_padding: f64 = (height - board_size) / 2.0;

            (board_size, x_padding, y_padding)
        } else {
            let board_size: f64 = height;

            let x_padding: f64 = (width - height) / 2.0;
            let y_padding: f64 = 0.0;

            (board_size, x_padding, y_padding)
        }
    }

    pub fn render(&mut self, controller: &GameController, c: Context, g: &mut G2d) {
        let board_size = self.settings.board_size;
        let padding = self.settings.padding;

        // Draw background
        clear(self.settings.background_color, g);

        rectangle(
            self.settings.border_color,
            [padding, 0.0, board_size, board_size],
            c.transform,
            g,
        );

        // Draw board
        let cell_size = (board_size - 4.0) / 8.0;
        for x in 0..8 {
            for y in 0..8 {
                let mut current_color = if x % 2 != y % 2 {
                    self.settings.black_color
                } else {
                    self.settings.white_color
                };

                if let Some(selected) = controller.selected_square {
                    if selected[0] == x && selected[1] == y {
                        current_color = self.settings.move_color
                    }
                };

                let x_pos = padding + 2.0 + cell_size * x as f64;
                let y_pos = 2.0 + cell_size * (7.0 - y as f64);
                let cell = [x_pos, y_pos, cell_size, cell_size];

                rectangle(current_color, cell, c.transform, g);

                if let Some(moves) = &controller.current_moves {
                    if moves.contains_key(&[x, y]) {
                        ellipse(self.settings.move_color, cell, c.transform, g)
                    }
                };

                let piece = controller.game.at_pos(Pos::new_xy(x as u8, y as u8));

                if !piece.is_empty() {
                    let index = if piece.color() == Color::White {
                        piece.get_type() as u8 - 1
                    } else {
                        piece.get_type() as u8 + 5
                    };
                    let image = Image::new().rect([x_pos, y_pos, cell_size, cell_size]);

                    image.draw(
                        &self.textures[index as usize],
                        &piston_window::DrawState::default(),
                        c.transform,
                        g,
                    )
                }
            }
        }
    }
}
