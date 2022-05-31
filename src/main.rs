use rltk::{GameState, Rltk, RGB};
use::specs::*;

mod monster_ai_system;
pub use monster_ai_system::*;

mod visibility_system;
pub use visibility_system::*;

mod rect;
pub use rect::*;

mod components;
pub use components::*;

mod player;
pub use player::*;

mod map;
pub use map::*;



impl GameState for State {
    // Основной игровой цикл
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        for (pos, render) in (&positions, &renderables).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            
            if map.visible_tiles[idx]{
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }
        }
    }
}


fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("First game in Rust i guess")
        .build()?;
    let mut gs = State {
        ecs: World::new()
    };
    // Регистрируем в ECS необходимые компоненты
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();

    let map = map::Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();
    
    let mut rng = rltk::RandomNumberGenerator::new();

    for room in map.rooms.iter().skip(1) {
        let (x, y) = room.center();
        
        let glyph: rltk::FontCharType;
        let roll = rng.roll_dice(1, 2);
        
        match roll {
            1 => {glyph = rltk::to_cp437('e')}
            _ => {glyph = rltk::to_cp437('a')}
        }

        gs.ecs.create_entity()
            .with(Position{x, y})
            .with(Renderable{
                glyph: glyph,
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK)
            })
            .with(Viewshed{ visible_tiles: Vec::new(), range: 8, dirty: true})
            .with(Monster{})
            .build();
    }
    gs.ecs.insert(map);
    // Создаем игрока
    gs.ecs
        .create_entity()
        .with(Position { x: player_x, y: player_y })
        .with(Renderable {
            glyph: rltk::to_cp437('h'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .with(Viewshed{ visible_tiles: Vec::new(), range: 8, dirty: true})
        .build();


    rltk::main_loop(context, gs)
}
