use bevy::prelude::*;
use rand::Rng;


#[derive(Resource)]
pub struct FruitAssets {
    images: Vec<Handle<TextureAtlas>>
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


pub fn get_fruit_assets(asset_server: Res<AssetServer>, texture_atlases: &mut ResMut<Assets<TextureAtlas>>) -> FruitAssets {
    let image_names = ["apple-frames.png", "strawberry.png", "orange.png"];
    let mut images: Vec<Handle<TextureAtlas>> = Vec::new();

    for name in image_names {
        let texture = TextureAtlas::from_grid(
            asset_server.load(name),
            Vec2::new(40.0, 40.0),
            4,
            1,
            None,
            None
        );

        let handle = texture_atlases.add(texture);

        images.push(handle);
    }

    return FruitAssets {
        images: images
    };
}


fn randint(min: i32, max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(min..max);
}

