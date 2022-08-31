use seija_app::ecs::world::World;
use seija_asset::AssetServer;
use seija_template::Template;

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
    assets:AssetCache
}


impl DemoGame {
    pub fn awake(&mut self,server:AssetServer) {
        self.assets.add_template("template/backgroud.xml", &server);
    }

    pub fn start(&mut self,world:&mut World) {
        let template = self.assets.get::<Template>("template/backgroud.xml", world).unwrap();
        Template::instance(template.entity.clone(), world).unwrap();
    }


    pub fn update(&mut self,world:&mut World) {
        match self.state {
            GameState::WaitAsset => {
                if self.assets.update() {
                    self.start(world);
                    self.state = GameState::Update
                }
            },
            _ => {

            },
        }
    }
}