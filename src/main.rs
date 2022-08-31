use std::sync::{Arc};
mod camera_ctrl;
mod tools;
mod demo;
mod asset_cache;
use demo::{DemoGame};
use seija_app::ecs::system::{IntoExclusiveSystem};
use seija_app::ecs::world::World;
use seija_core::bevy_ecs::change_detection::Mut;
use seija_app::{App};
use seija_asset::{AssetModule, AssetServer};

use seija_core::{CoreModule, CoreStage, StartupStage};
use seija_gltf::{GLTFModule};
use seija_input::InputModule;
use seija_render::{RenderModule, RenderConfig, GraphSetting};
use seija_render_template::add_render_templates;
use seija_template::{TemplateModule};
use seija_transform::{TransformModule};
use seija_winit::WinitModule;
use seija_pbr::{create_pbr_plugin};



fn main() {
    env_logger::Builder::new().filter_level(log::LevelFilter::Info).try_init().unwrap();
    let mut app = App::new();
    init_modules(&mut app);
    app.add_system2(CoreStage::Startup ,StartupStage::Startup, on_start.exclusive_system());
    app.add_system(CoreStage::PostUpdate,camera_ctrl::update_camera_trans_system);
   
    app.run();
}

fn init_modules(app:&mut App) {
    app.add_module(CoreModule);
    app.add_module(AssetModule(std::env::current_dir().unwrap().join("assets").into()));
    app.add_module(TemplateModule);
    app.add_module(TransformModule);
    app.add_module(InputModule);
    app.add_module(GLTFModule);
    app.add_module(WinitModule::default());
    let render_config = RenderConfig {
        config_path:".shader".into(),
        script_path:"script/render.clj".into(),
        setting:Arc::new(GraphSetting::default() ),
        plugins:vec![create_pbr_plugin()],
        render_lib_paths:vec!["script".into() ],
    };
    add_render_templates(app);
    app.add_module(RenderModule(Arc::new(render_config)));
    app.init_resource::<DemoGame>();
    app.add_system(CoreStage::Update, on_update.exclusive_system());
    app.start();
}

pub fn on_start(world:&mut World) {
    let server = world.get_resource::<AssetServer>().unwrap().clone();
    let mut game = world.get_resource_mut::<DemoGame>().unwrap();
    game.awake(server);
}

fn on_update(world:&mut World) {
    world.resource_scope(|w:&mut World,mut game:Mut<DemoGame>| {
        game.update(w)
    })
    

}