use std::collections::HashMap;

use seija_asset::{LoadingTrack, AssetServer, HandleUntyped, Assets};
use seija_gltf::asset::GltfAsset;
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
        self.loadings.push(("car".into(),asset.load_async::<GltfAsset>("res/model/pony_cartoon/scene.gltf", None).unwrap()));
    }

    pub fn on_asset_ready(&mut self,gltf_assets:&Assets<GltfAsset>) {
        log::error!("on_asset_ready");
        dbg!(self.asset_cache.get("car"));
    }
}