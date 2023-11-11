use bevy::prelude::*;
use crate::utils::random::{randint, probably};
use super::fruit::FruitType;


const FRUIT_IMAGES: [&str; 5] = [
    "apple-frames.png", 
    "strawberry.png", 
    "watermelon.png",
    "orange.png",
    "banana.png"
];



#[derive(Resource)]
pub struct FruitTextures {
    textures: Vec<Handle<TextureAtlas>>,
    pineapple_texture: Handle<TextureAtlas>,
}

impl FruitTextures {
    fn get_random_fruit(&self) -> (Handle<TextureAtlas>, FruitType) {
        if probably(0.1) {
            return (self.textures[2].clone(), FruitType::WATERMELON)
        }

        let random_index = randint(1, 5);

        match random_index {
            1 => (self.textures[0].clone(), FruitType::APPLE),
            2 => (self.textures[1].clone(), FruitType::STRAWBERRY),
            3 => (self.textures[4].clone(), FruitType::BANANA),
            _ => (self.textures[3].clone(), FruitType::ORANGE)
        }
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
    
        FruitTextures { textures, pineapple_texture }
    }
}


pub fn create_sprite(fruit_assets: &Res<FruitTextures>, x: f32, y: f32, z: f32) -> (SpriteSheetBundle, FruitType) {
    let transform = Transform::from_xyz(x, y, z).with_scale(Vec3::splat(3.5));
    let (texture, fruit_type) = fruit_assets.get_random_fruit();
    let sprite = SpriteSheetBundle {
        texture_atlas: texture,
        sprite: TextureAtlasSprite::new(0),
        transform,
        ..default()
    };

    (sprite, fruit_type)
}


pub fn create_pineapple(fruit_assets: &Res<FruitTextures>, x: f32, y: f32) -> SpriteSheetBundle {
    let transform = Transform::from_xyz(x, y, 2.).with_scale(Vec3::splat(5.));

    SpriteSheetBundle {
        texture_atlas: fruit_assets.pineapple_texture.clone(),
        sprite: TextureAtlasSprite::new(0),
        transform,
        ..default()
    }
}


pub enum SplashColor {
    Red,
    Orange,
    Yellow
}

impl SplashColor {
    fn color(&self) -> Color {
        match self {
            Self::Orange => Color::rgba_u8(245, 155, 66, 200),
            Self::Yellow => Color::rgba_u8(255, 230, 41, 200),
            Self::Red    => Color::rgba_u8(247, 54, 32, 200)
        }
    }
}


pub fn create_splash(
    asset_server: &Res<AssetServer>, 
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>, 
    x: f32, 
    y: f32,
    color: SplashColor
) -> SpriteSheetBundle {
    let transform = Transform::
        from_xyz(x, y - 30., 1.)
        .with_scale(Vec3::splat(4.))
        .with_rotation(Quat::from_rotation_z(randint(0, 360) as f32))
    ;

    let texture = TextureAtlas::from_grid(
        asset_server.load("images/splash.png"),
        Vec2::new(50., 50.),
        6,
        1,
        None,
        None
    );

    let texture_atlas = texture_atlases.add(texture);

    SpriteSheetBundle { 
        sprite: TextureAtlasSprite {
            color: color.color(),
            ..default()
        },
        texture_atlas, 
        transform, 
        ..default()
    }
}


pub fn create_wave(
    asset_server: &Res<AssetServer>, 
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>, 
    x: f32 
) -> SpriteSheetBundle {
    let transform = Transform::from_xyz(x, -220., 1.).with_scale(Vec3::splat(4.));

    let texture = TextureAtlas::from_grid(
        asset_server.load("images/wave.png"),
        Vec2::new(60., 80.),
        6,
        1,
        None,
        None
    );

    let texture_atlas = texture_atlases.add(texture);

    SpriteSheetBundle { 
        texture_atlas, 
        transform, 
        ..default()
    }
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
