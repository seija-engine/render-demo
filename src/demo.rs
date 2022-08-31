use std::collections::HashMap;

use seija_app::ecs::system::Commands;
use seija_asset::{AssetServer, HandleUntyped, Assets, AssetRequest};
use seija_gltf::{asset::GltfAsset, create_gltf};

#[derive(PartialEq, Eq)]
pub enum GameState {
    None,
    LoadAssets,
    WaitAssets,
    OnStart
}

pub struct DemoGame {
    pub loadings:Vec<(String,AssetRequest)>,
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
  
}