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

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);

    let grid = Grid::new(&mut commands, 10);
    commands.insert_resource(grid);
}

const GRID_SIZE: f32 = 300.;

#[derive(Copy, Clone)]
enum CellType {
    Filled,
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
}

impl Grid {
    pub fn new(commands: &mut Commands, size: usize) -> Self {
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
                // color: Color::rgb(0.0, 0.0, 0.0),
                color: Color::rgb(0.08, 0.10, 0.62),
                ..Default::default()
            };
            let offset = grid_padding * (i + 1) as f32 - (GRID_SIZE / 2.);

            let thickness = grid_thickness * if (i + 1) % 5 == 0 { 3. } else { 1. };

            commands.spawn_bundle(SpriteBundle {
                sprite: sprite.clone(),
                transform: Transform {
                    translation: Vec3::new(offset - (thickness / 2.), 0., 0.),
                    scale: Vec3::new(thickness, GRID_SIZE, 0.),
                    ..Default::default()
                },
                ..Default::default()
            });
            commands.spawn_bundle(SpriteBundle {
                sprite: sprite.clone(),
                transform: Transform {
                    translation: Vec3::new(0., offset - (thickness / 2.), 0.),
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
        }
    }

    pub fn set_at(&mut self, commands: &mut Commands, row: usize, col: usize, val: bool) {
        self.cells[row][col] = val;

        if val {
            self.spawn_at(commands, row, col);
        } else {
            self.despawn_at(commands, row, col);
        }
    }

    pub fn toggle_at(&mut self, commands: &mut Commands, row: usize, col: usize) {
        self.set_at(commands, row, col, !self.cells[row][col]);
    }

    fn spawn_at(&mut self, commands: &mut Commands, row: usize, col: usize) {
        self.despawn_at(commands, row, col);
        let cell_type = CellType::Filled;

        let grid_thickness = 0.5;
        let x_pos = row as f32 * self.cell_size();
        let y_pos = col as f32 * self.cell_size();
        let cell_scale = Vec3::new(
            self.cell_size() - grid_thickness,
            self.cell_size() - grid_thickness,
            0.,
        );
        let entity_id = commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.1, 0.1, 0.1),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(
                        x_pos - grid_thickness / 2.,
                        y_pos - grid_thickness / 2.,
                        0.,
                    ) + self.grid_offset(),
                    scale: cell_scale,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Cell(cell_type))
            .id();

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
        println!("{:?}", adjusted);
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
    mut grid: ResMut<Grid>,
) {
    let win = windows.get_primary().expect("no primary window");
    let (camera, camera_transform) = camera.single();

    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some(click_pos) = win.cursor_position() {
            let window_size = Vec2::new(win.width() as f32, win.height() as f32);
            let ndc = (click_pos / window_size) * 2.0 - Vec2::ONE;
            let ndc_to_world =
                camera_transform.compute_matrix() * camera.projection_matrix.inverse();
            let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0)).truncate();

            if let Some((row, col)) = grid.point_coords(world_pos) {
                grid.toggle_at(&mut commands, row, col);
            }
        }
    }
}
