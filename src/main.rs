mod camera;
mod components;
mod map;
mod map_builder;
mod sound;
mod spawner;
mod systems;
mod turn_state;

mod prelude {
    pub use crate::components::*;
    pub use crate::sound::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::camera::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::turn_state::*;
}

use prelude::*;
use rodio::OutputStream;

struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
}

impl State {
    fn advance_level(&mut self) {
        let player_entity = *<Entity>::query()
            .filter(component::<Player>())
            .iter(&mut self.ecs)
            .next()
            .unwrap();

        use std::collections::HashSet;
        let mut entities_to_keep = HashSet::new();
        entities_to_keep.insert(player_entity);

        <(Entity, &Carried)>::query()
            .iter(&self.ecs)
            .filter(|(_e, carry)| carry.0 == player_entity)
            .map(|(e, _carry)| *e)
            .for_each(|e| {
                entities_to_keep.insert(e);
            });

        let mut cb = CommandBuffer::new(&mut self.ecs);
        for e in Entity::query().iter(&self.ecs) {
            if !entities_to_keep.contains(e) {
                cb.remove(*e);
            }
        }

        cb.flush(&mut self.ecs);

        <&mut FieldOfView>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|fov| fov.is_dirty = true);

        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);

        let mut map_level = 0;
        <(&mut Player, &mut Point)>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|(player, pos)| {
                player.map_level += 1;
                map_level = player.map_level;
                pos.x = map_builder.player_start.x;
                pos.y = map_builder.player_start.y;
            });

        if map_level == 2 {
            spawn_amulet_of_justicialismo(&mut self.ecs, map_builder.amulet_start);
        } else {
            let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
            map_builder.map.tiles[exit_idx] = TileType::Exit;
        }

        spawn_level(
            &mut self.ecs,
            &mut rng,
            map_level as usize,
            &map_builder.monster_spawns,
        );
        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(map_builder.theme);
    }

    fn new() -> Self {
        let mut ecs = World::default();
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);
        let mut resources = Resources::default();
        spawn_player(&mut ecs, map_builder.player_start);
        let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;
        // spawn_amulet_of_justicialismo(&mut ecs, map_builder.amulet_start);
        spawn_level(&mut ecs, &mut rng, 0, &map_builder.monster_spawns);
        // map_builder
        //     .rooms
        //     .iter()
        //     .skip(1)
        //     .map(|r| r.center())
        //     .for_each(|pos| spawn_monster(&mut ecs, &mut rng, pos));
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(TurnState::AwaitingInput);
        resources.insert(map_builder.theme);
        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
        }
    }

    fn reset_game_state(&mut self) {
        self.ecs = World::default();
        self.resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut self.ecs, map_builder.player_start);
        // spawn_amulet_of_justicialismo(&mut self.ecs, map_builder.amulet_start);
        let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;
        spawn_level(&mut self.ecs, &mut rng, 0, &map_builder.monster_spawns);
        // map_builder
        //     .rooms
        //     .iter()
        //     .skip(1)
        //     .map(|r| r.center())
        //     .for_each(|pos| spawn_monster(&mut self.ecs, &mut rng, pos));
        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(map_builder.theme);
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(3);
        ctx.print_color_centered(
            4,
            BLUE,
            BLACK,
            "Tu misión ha terminado. Ganaron los gorilas.",
        );
        ctx.print_color_centered(6, WHITE, BLACK, "Masacrado por una campaña de marketing.");
        ctx.print_color_centered(7, WHITE, BLACK, "El desarrollo nac&pop ha sido detenido.");
        ctx.print_color_centered(
            8,
            YELLOW,
            BLACK,
            "Sin embargo, podrás volver a intentarlo... en 4 años.",
        );
        ctx.print_color_centered(
            9,
            GREEN,
            BLACK,
            "Presioná 1 para comenzar una nueva campaña",
        );

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset_game_state();
        }
    }

    fn victory(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(3);
        ctx.print_color_centered(2, GREEN, BLACK, "Triunfaste.");
        ctx.print_color_centered(4, WHITE, BLACK, "Alcanzaste el dije del justicialismo");
        ctx.print_color_centered(5, WHITE, BLACK, "El país está a salvo... por 4 años.");
        ctx.print_color_centered(7, GREEN, BLACK, "Presioná 1 para retomar la campaña.");
        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset_game_state();
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.resources.insert(ctx.key);
        ctx.set_active_console(0);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(2);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));
        ctx.cls();
        ctx.set_active_console(3);
        ctx.cls();
        let current_state = self.resources.get::<TurnState>().unwrap().clone();
        match current_state {
            TurnState::AwaitingInput => self
                .input_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => {
                self.player_systems
                    .execute(&mut self.ecs, &mut self.resources);
            }
            TurnState::MonsterTurn => self
                .monster_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::GameOver => self.game_over(ctx),
            TurnState::Victory => self.victory(ctx),
            TurnState::NextLevel => self.advance_level(),
        }
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sound = Sound::new(&stream_handle);
    let context = BTermBuilder::simple80x50()
        .with_title("Argentina oxidada")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(SCREEN_WIDTH * 2, SCREEN_HEIGHT * 2, "terminal8x8.png")
        .build()?;

    drop(sound);

    main_loop(context, State::new())
}
