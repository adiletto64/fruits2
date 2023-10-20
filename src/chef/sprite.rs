use bevy::prelude::*;


pub fn get_sprite(asset_server: &Res<AssetServer>, texture_atlases: &mut ResMut<Assets<TextureAtlas>>) -> SpriteSheetBundle {
    let texture_atlas =TextureAtlas::from_grid(
        asset_server.load("images/chef2.png"), 
        Vec2::new(40.0, 40.0),
        17, 
        1, 
        None,
        None
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    return SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        sprite: TextureAtlasSprite::new(0),
        transform: Transform::from_xyz(0.0, -220.0, 1.0).with_scale(Vec3::splat(5.0)),
        ..default()
    }
}


#[derive(PartialEq, Debug)]
pub enum ChefState {
    Slice,
    Waiting,
    Bag
}


#[derive(Component)]
pub struct AnimationSlice {
    state: ChefState,
    pub index: usize,
    pub timer: Timer,
}

impl AnimationSlice {
    pub fn normal(&mut self) {
        self.state = ChefState::Waiting;
        self.index = 9;
    }

    pub fn trigger_slice(&mut self) {
        self.state = ChefState::Slice;
        self.index = 2;
    }

    pub fn pullout_trash_bag(&mut self) {
        self.state = ChefState::Bag;
        self.index = 8;
    }

    pub fn update(&mut self) {
        match self.state {
            ChefState::Waiting => {
                if self.index == 16 {
                    self.index = 9;
                } else {
                    self.index += 1;
                }
            },
            ChefState::Slice => {
                if self.index == 0 {
                    self.index = 1;
                }
                if self.index < 7 {
                    self.index += 1;
                } else {
                    self.normal();
                } 
            },
            ChefState::Bag => {
                self.index = 8;
            }
        }
    }

    pub fn new() -> Self {
        return Self {
            state: ChefState::Waiting,
            index: 9,
            timer: Timer::from_seconds(0.07, TimerMode::Repeating),
        }
    }

}
