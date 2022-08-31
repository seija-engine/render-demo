use std::collections::HashMap;

use seija_app::ecs::world::World;
use seija_asset::{AssetRequest, Asset, AssetServer, HandleUntyped, Assets};
use seija_core::smol_str::SmolStr;
use seija_template::Template;


#[derive(Default)]
pub(crate) struct AssetCache {
    loadings:Vec<(SmolStr,AssetRequest)>,
    assets:HashMap<SmolStr,HandleUntyped>
}

impl AssetCache {
    pub fn add_res<T:Asset>(&mut self,path:&str,server:&AssetServer) {
        let req = server.load_async::<T>(path, None).unwrap();
        self.loadings.push((SmolStr::new(path),req));
    }

    pub fn add_template(&mut self,path:&str,server:&AssetServer) {
        self.add_res::<Template>(path, server);
    }

    pub fn get<'a,T:Asset>(&self,path:&str,world:&'a World) -> Option<&'a T> {
        let handle = self.assets.get(path)?;
        let assets = world.get_resource::<Assets<T>>()?;
        assets.get(&handle.id)
    } 

    pub fn update(&mut self) -> bool {
        for (_,req) in self.loadings.iter() {
            if !req.is_finish() {
                return false;
            }
        }
        for (p,req) in self.loadings.drain(..) {
            self.assets.insert(p, req.make_handle());
        }
        true
    }
}