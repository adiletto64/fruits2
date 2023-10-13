use bevy::prelude::*;
use rand::Rng;


#[derive(Resource)]
pub struct FruitAssets {
    images: Vec<Handle<TextureAtlas>>,
    pineapple: Handle<TextureAtlas>
}

impl FruitAssets {
    fn get_random_image(&self) -> Handle<TextureAtlas> {
        let mut rng = rand::thread_rng();
        let random_index: usize = rng.gen_range(0..self.images.len());
        return self.images[random_index].clone();
    }
}


pub fn get_sprite(fruit_assets: &Res<FruitAssets>, x: f32, y: f32) -> SpriteSheetBundle {
    let transform = Transform::from_xyz(x, y, 1.).with_scale(Vec3::splat(3.5));

    return SpriteSheetBundle {
        texture_atlas: fruit_assets.get_random_image(),
        sprite: TextureAtlasSprite::new(0),
        transform: transform,
        ..default()
    };
}


pub fn get_pineapple(fruit_assets: &Res<FruitAssets>, x: f32, y: f32) -> SpriteSheetBundle {
    let transform = Transform::from_xyz(x, y, 1.).with_scale(Vec3::splat(5.));

    return SpriteSheetBundle {
        texture_atlas: fruit_assets.pineapple.clone(),
        sprite: TextureAtlasSprite::new(0),
        transform: transform,
        ..default()
    };
}



pub fn get_fruit_assets(asset_server: Res<AssetServer>, texture_atlases: &mut ResMut<Assets<TextureAtlas>>) -> FruitAssets {
    let image_names = ["apple-frames.png", "strawberry.png", "orange.png"];
    let mut images: Vec<Handle<TextureAtlas>> = Vec::new();

    for name in image_names {
        let image = asset_server.load(name);
        let texture = get_texture(image);
        let handle = texture_atlases.add(texture);

        images.push(handle);
    }

    let pineapple_texture = get_texture(asset_server.load("pineapple.png"));
    let pineapple_handle = texture_atlases.add(pineapple_texture);

    return FruitAssets {
        images: images,
        pineapple: pineapple_handle
    };
}


fn get_texture(image: Handle<Image>) -> TextureAtlas {
    TextureAtlas::from_grid(
        image,
        Vec2::new(40.0, 40.0),
        6,
        1,
        None,
        None
    )
} 
