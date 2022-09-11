use seija_app::ecs::world::World;
use seija_asset::{AssetServer, Handle};
use seija_core::{math::Quat, info::EInfo};
use seija_render::resource::Mesh;
use seija_template::Template;
use seija_transform::Transform;

use crate::asset_cache::AssetCache;
pub enum GameState {
    WaitAsset,
    Update
}

impl Default for GameState {
    fn default() -> Self { GameState::WaitAsset }
}


#[derive(Default)]
pub struct DemoGame {
    state:GameState,
    assets:AssetCache,

    r:f32
}


impl DemoGame {
    pub fn awake(&mut self,server:AssetServer) {
        self.assets.add_template("template/backgroud.xml", &server);
    }

    pub fn start(&mut self,world:&mut World) {
        let template = self.assets.get::<Template>("template/backgroud.xml", world).unwrap().clone();
        template.instance(world).unwrap();
       
    }


    pub fn update(&mut self,world:&mut World) {
        match self.state {
            GameState::WaitAsset => {
                if self.assets.update() {
                    self.start(world);
                    self.state = GameState::Update
                }
            },
            _ => { self.on_update(world) },
        }
    }

    fn on_update(&mut self,world:&mut World) {
        let mut all_meshs = world.query::<(&mut Transform,Option<&EInfo>)>();
        if self.r >= 360f32 {
            self.r = 0f32;
        }
        for (mut t,info) in all_meshs.iter_mut(world) {
            if let Some(info_name) = info.and_then(|v| v.name.as_ref()).map(|v| v.as_str()) {
                
                if info_name == "Test" {

                    t.local.rotation = Quat::from_euler(Default::default(), self.r.to_radians(), 0f32, 0f32);
                }
            }
            
           
        }
        self.r += 0.2f32;
    }
}