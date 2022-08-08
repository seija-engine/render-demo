use seija_asset::{Handle, Assets};
use seija_render::resource::{Texture, TextureDescInfo};

pub fn load_texture(textures:&mut Assets<Texture>,path:&str) -> Handle<Texture> {
    
    let texture = Texture::from_image_bytes(&std::fs::read(path).unwrap(),TextureDescInfo::default()).unwrap();
    textures.add(texture)
}