use specs::prelude::*;
use specs_derive::*;
use rltk::{RGB};
use super::{VisibilitySystem, MonsterAI};

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}
 

#[derive(Component, Debug)]
pub struct Player {}

pub struct State {
    pub ecs: World
}
impl State {
    pub fn run_systems(&mut self) {
        
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);
        
        let mut mob = MonsterAI{};
        mob.run_now(&self.ecs);

        self.ecs.maintain();
    }
}

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>,
    pub range: i32,
    pub dirty: bool
}

#[derive(Component, Debug)]
pub struct Monster {}
