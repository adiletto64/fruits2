use bevy::prelude::*;
use rand::Rng;


const FRUIT_IMAGES: [&str; 3] = [
    "apple-frames.png", 
    "strawberry.png", 
    "orange.png"
];



#[derive(Resource)]
pub struct FruitTextures {
    textures: Vec<Handle<TextureAtlas>>,
    pineapple_texture: Handle<TextureAtlas>
}

impl FruitTextures {
    fn get_random_image(&self) -> Handle<TextureAtlas> {
        let mut rng = rand::thread_rng();
        let random_index: usize = rng.gen_range(0..self.textures.len());
        return self.textures[random_index].clone();
    }

    pub fn new(asset_server: &Res<AssetServer>, texture_atlases: &mut ResMut<Assets<TextureAtlas>>) -> Self {
        let mut textures: Vec<Handle<TextureAtlas>> = Vec::new();

        for name in FRUIT_IMAGES {
            let image = asset_server.load("images/fruits/".to_owned() + name);
            let texture = get_texture(image);
            let handle = texture_atlases.add(texture);
    
            textures.push(handle);
        }
    
        let pineapple = get_texture(asset_server.load("images/fruits/pineapple.png"));
        let pineapple_texture = texture_atlases.add(pineapple);
    
        return FruitTextures {
            textures: textures,
            pineapple_texture: pineapple_texture
        };
    }
}


pub fn create_sprite(fruit_assets: &Res<FruitTextures>, x: f32, y: f32) -> SpriteSheetBundle {
    let transform = Transform::from_xyz(x, y, 2.).with_scale(Vec3::splat(3.5));

    return SpriteSheetBundle {
        texture_atlas: fruit_assets.get_random_image(),
        sprite: TextureAtlasSprite::new(0),
        transform: transform,
        ..default()
    };
}


pub fn create_pineapple(fruit_assets: &Res<FruitTextures>, x: f32, y: f32) -> SpriteSheetBundle {
    let transform = Transform::from_xyz(x, y, 2.).with_scale(Vec3::splat(5.));

    return SpriteSheetBundle {
        texture_atlas: fruit_assets.pineapple_texture.clone(),
        sprite: TextureAtlasSprite::new(0),
        transform: transform,
        ..default()
    };
}


fn get_texture(image: Handle<Image>) -> TextureAtlas {
    TextureAtlas::from_grid(
        image,
        Vec2::new(60.0, 60.0),
        8,
        1,
        None,
        None
    )
} 
