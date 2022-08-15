use std::collections::HashMap;

use seija_app::ecs::system::Commands;
use seija_asset::{LoadingTrack, AssetServer, HandleUntyped, Assets};
use seija_gltf::{asset::GltfAsset, create_gltf};
use seija_render::material::MaterialStorage;

use crate::tools::conv_pbr_material;
#[derive(PartialEq, Eq)]
pub enum GameState {
    None,
    LoadAssets,
    WaitAssets,
    OnStart
}

pub struct DemoGame {
    pub loadings:Vec<(String,LoadingTrack)>,
    pub asset_cache:HashMap<String,HandleUntyped>,
    pub state:GameState
}

impl Default for DemoGame {
    fn default() -> DemoGame {
        DemoGame {
            state:GameState::None,
            asset_cache:HashMap::default(),
            loadings:vec![]
        }
    }
}

impl DemoGame {

    pub fn load_assets(&mut self,asset:&AssetServer) {
        self.loadings.push(("car".into(),asset.load_async::<GltfAsset>("model/pony_cartoon/scene.gltf", None).unwrap()));
    }

    pub fn on_asset_ready(&mut self,gltf_assets:&Assets<GltfAsset>,commands:&mut Commands,materials:&MaterialStorage) {
        let car_id = self.asset_cache.get("car").unwrap();
        let car_asset = gltf_assets.get(&car_id.id).unwrap();
       

        create_gltf(car_asset, commands,&|m| { conv_pbr_material(m, materials) });
    }
}