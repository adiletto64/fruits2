use bevy::prelude::*;


fn animate(
    time: Time,
    mut timer: Timer,
    mut sprite: TextureAtlasSprite
) {
    timer.tick(time.delta());

    if timer.finished() {
        sprite.index += 1;
    }

}

