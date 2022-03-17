use bevy::prelude::*;

const WINDOW_WIDTH: f32 = 600.;
const WINDOW_HEIGHT: f32 = WINDOW_WIDTH;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            title: String::from("Rust Nonogram"),
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.08, 0.10, 0.32)))
        .add_startup_system(setup)
        .add_system(handle_mouse_clicks)
        .run();
}

#[derive(Component)]
struct MainCamera;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);

    let solution = vec![
        vec![
            true, true, false, false, false, false, false, true, true, false,
        ],
        vec![
            false, false, false, true, false, true, false, true, true, true,
        ],
        vec![
            true, true, true, true, false, false, false, true, true, true,
        ],
        vec![
            false, true, false, false, false, false, false, false, true, true,
        ],
        vec![
            false, false, false, false, false, false, false, false, true, true,
        ],
        vec![
            false, false, false, false, false, false, true, false, true, true,
        ],
        vec![
            false, false, false, false, true, false, true, true, true, true,
        ],
        vec![
            true, true, true, true, true, false, true, false, false, true,
        ],
        vec![
            true, true, true, true, true, true, true, false, false, false,
        ],
        vec![
            true, true, true, true, true, true, true, false, false, false,
        ],
    ];

    let puzzle = Puzzle::new(&mut commands, &asset_server, solution);
    commands.insert_resource(puzzle);
}

fn count_runs(line: Vec<bool>) -> Vec<usize> {
    let mut runs = Vec::new();
    let mut curr_run = 0;
    for cell in line {
        if cell {
            curr_run += 1;
        } else if curr_run > 0 {
            runs.push(curr_run);
            curr_run = 0;
        }
    }

    if curr_run > 0 {
        runs.push(curr_run);
    }

    runs
}

struct Puzzle {
    pub grid: Grid,
    solution: Vec<Vec<bool>>,
    size: usize,
}

impl Puzzle {
    pub fn new(
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        solution: Vec<Vec<bool>>,
    ) -> Self {
        let size = solution.len();
        assert!(
            solution.iter().all(|row| row.len() == size),
            "solution must be square"
        );

        let grid = Grid::new(commands, asset_server.load("textures/cross.png"), size);
        let cell_size = grid.cell_size();

        let row_runs: Vec<Vec<usize>> = solution
            .iter()
            .map(|row| count_runs(row.to_vec()))
            .collect();
        let col_runs: Vec<Vec<usize>> = {
            let transpose = (0..size).map(|i| (0..size).map(|j| solution[j][i]).collect());
            transpose.map(|row| count_runs(row)).collect()
        };

        let font = asset_server.load("fonts/FiraSans-Bold.ttf");
        let text_style = TextStyle {
            font,
            font_size: 16.0,
            color: Color::WHITE,
        };
        let text_alignment = TextAlignment {
            vertical: VerticalAlign::Center,
            horizontal: HorizontalAlign::Center,
        };

        for i in 0..size {
            for (j, run) in row_runs[size - i - 1].iter().rev().enumerate() {
                commands.spawn_bundle(Text2dBundle {
                    text: Text::with_section(run.to_string(), text_style.clone(), text_alignment),
                    transform: Transform {
                        translation: Vec3::new(
                            -GRID_SIZE / 2. - 15. * (j + 1) as f32,
                            -(GRID_SIZE - cell_size) / 2. + cell_size * i as f32,
                            10.,
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                });
            }

            for (j, run) in col_runs[i].iter().rev().enumerate() {
                commands.spawn_bundle(Text2dBundle {
                    text: Text::with_section(run.to_string(), text_style.clone(), text_alignment),
                    transform: Transform {
                        translation: Vec3::new(
                            -(GRID_SIZE - cell_size) / 2. + cell_size * i as f32,
                            GRID_SIZE / 2. + 15. * (j + 1) as f32,
                            10.,
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                });
            }
        }

        Self {
            grid,
            solution,
            size,
        }
    }
}

const GRID_SIZE: f32 = 300.;

#[derive(Copy, Clone, PartialEq)]
enum CellType {
    Filled,
    Cross,
}

#[derive(Component, Copy, Clone)]
struct Cell(CellType);

#[derive(Copy, Clone)]
struct CellEntity {
    pub cell_type: CellType,
    pub entity: Entity,
}

struct Grid {
    size: usize,
    entities: Vec<Vec<Option<CellEntity>>>,
    cells: Vec<Vec<bool>>,

    cross_handle: Handle<Image>,
}

impl Grid {
    pub fn new(commands: &mut Commands, cross_handle: Handle<Image>, size: usize) -> Self {
        let entities = vec![vec![None; size]; size];
        let cells = vec![vec![false; size]; size];

        let grid_thickness = 0.5;

        // Background
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(-grid_thickness / 2., -grid_thickness / 2., 0.),
                scale: Vec3::new(GRID_SIZE - grid_thickness, GRID_SIZE - grid_thickness, 0.),
                ..Default::default()
            },
            ..Default::default()
        });

        // Grid
        let grid_padding = GRID_SIZE / size as f32;
        for i in 0..size - 1 {
            let sprite = Sprite {
                color: Color::rgb(0.08, 0.10, 0.62),
                ..Default::default()
            };
            let offset = grid_padding * (i + 1) as f32 - (GRID_SIZE / 2.);

            let thickness = grid_thickness * if (i + 1) % 5 == 0 { 3. } else { 1. };

            commands.spawn_bundle(SpriteBundle {
                sprite: sprite.clone(),
                transform: Transform {
                    translation: Vec3::new(offset - (thickness / 2.), 0., 10.),
                    scale: Vec3::new(thickness, GRID_SIZE, 0.),
                    ..Default::default()
                },
                ..Default::default()
            });
            commands.spawn_bundle(SpriteBundle {
                sprite: sprite.clone(),
                transform: Transform {
                    translation: Vec3::new(0., offset - (thickness / 2.), 10.),
                    scale: Vec3::new(GRID_SIZE, thickness, 0.),
                    ..Default::default()
                },
                ..Default::default()
            });
        }

        Self {
            size,
            entities,
            cells,
            cross_handle,
        }
    }

    pub fn set_at(
        &mut self,
        commands: &mut Commands,
        row: usize,
        col: usize,
        cell_type: Option<CellType>,
    ) {
        self.cells[row][col] = cell_type.is_some();

        if let Some(cell_type) = cell_type {
            self.spawn_at(commands, row, col, cell_type);
        } else {
            self.despawn_at(commands, row, col);
        }
    }

    // If the cell contains cell_type, set it to empty, else set it to cell_type
    pub fn toggle_at(
        &mut self,
        commands: &mut Commands,
        row: usize,
        col: usize,
        cell_type: CellType,
    ) {
        let new_type = if let Some(entity) = self.entities[row][col] {
            if entity.cell_type == cell_type {
                None
            } else {
                Some(cell_type)
            }
        } else {
            Some(cell_type)
        };
        self.set_at(commands, row, col, new_type);
    }

    fn spawn_at(&mut self, commands: &mut Commands, row: usize, col: usize, cell_type: CellType) {
        self.despawn_at(commands, row, col);

        let grid_thickness = 0.5;
        let x_pos = row as f32 * self.cell_size();
        let y_pos = col as f32 * self.cell_size();
        let mut bundle = SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.1, 0.1, 0.1),
                custom_size: Some(Vec2::new(
                    self.cell_size() - grid_thickness,
                    self.cell_size() - grid_thickness,
                )),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(
                    x_pos - grid_thickness / 2.,
                    y_pos - grid_thickness / 2.,
                    1.,
                ) + self.grid_offset(),
                ..Default::default()
            },
            ..Default::default()
        };
        match cell_type {
            CellType::Filled => {}
            CellType::Cross => {
                bundle.texture = self.cross_handle.clone();
            }
        }
        let entity_id = commands.spawn_bundle(bundle).insert(Cell(cell_type)).id();

        self.entities[row][col] = Some(CellEntity {
            cell_type,
            entity: entity_id,
        });
    }

    fn despawn_at(&mut self, commands: &mut Commands, row: usize, col: usize) {
        if let Some(cell_entity) = self.entities[row][col] {
            commands.entity(cell_entity.entity).despawn();
        }
        self.entities[row][col] = None;
    }

    // If possible, gives the row and col that contains a world pos
    pub fn point_coords(&self, pos: Vec2) -> Option<(usize, usize)> {
        let offset = pos - self.grid_offset().truncate();
        let adjusted = offset + Vec2::new(self.cell_size() / 2., self.cell_size() / 2.);
        if adjusted.x < 0. || adjusted.y < 0. {
            return None;
        }
        let row = (adjusted.x / self.cell_size()) as usize;
        let col = (adjusted.y / self.cell_size()) as usize;
        if row < self.size && col < self.size {
            Some((row, col))
        } else {
            None
        }
    }

    fn cell_size(&self) -> f32 {
        GRID_SIZE / self.size as f32
    }

    // Offset to center grid
    fn grid_offset(&self) -> Vec3 {
        let offset = -(GRID_SIZE - self.cell_size()) / 2.0;
        Vec3::new(offset, offset, 0.0)
    }
}

fn handle_mouse_clicks(
    mut commands: Commands,
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut puzzle: ResMut<Puzzle>,
) {
    let win = windows.get_primary().expect("no primary window");
    let (camera, camera_transform) = camera.single();

    let left_clicked = mouse_input.just_pressed(MouseButton::Left);
    let right_clicked = mouse_input.just_pressed(MouseButton::Right);
    if left_clicked || right_clicked {
        if let Some(click_pos) = win.cursor_position() {
            let window_size = Vec2::new(win.width() as f32, win.height() as f32);
            let ndc = (click_pos / window_size) * 2.0 - Vec2::ONE;
            let ndc_to_world =
                camera_transform.compute_matrix() * camera.projection_matrix.inverse();
            let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0)).truncate();

            if let Some((row, col)) = puzzle.grid.point_coords(world_pos) {
                if left_clicked {
                    puzzle
                        .grid
                        .toggle_at(&mut commands, row, col, CellType::Filled);
                } else if right_clicked {
                    puzzle
                        .grid
                        .toggle_at(&mut commands, row, col, CellType::Cross);
                }
            }
        }
    }
}
