use std::sync::Arc;

use seija_app::App;
use seija_asset::AssetModule;
use seija_core::CoreModule;
use seija_input::InputModule;
use seija_render::{RenderModule, RenderConfig, GraphSetting};
use seija_transform::TransformModule;
use seija_winit::WinitModule;
use seija_pbr::{create_pbr_plugin};
fn main() {
    env_logger::init();
    let mut app = App::new();
    init_modules(&mut app);
    app.run();
}

fn init_modules(app:&mut App) {
    app.add_module(CoreModule);
    app.add_module(TransformModule);
    app.add_module(InputModule);
    app.add_module(AssetModule);
    app.add_module(WinitModule::default());
    let render_config = RenderConfig {
        config_path:".render".into(),
        script_name:"render.clj".into(),
        setting:Arc::new(GraphSetting::default() ),
        plugins:vec![create_pbr_plugin()],
        render_lib_paths:vec![],
    };
    app.add_module(RenderModule(Arc::new(render_config)));
}