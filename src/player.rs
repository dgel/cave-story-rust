use constants;
use graphics::Graphics;
use sprite::{Sprite, AnimatedSprite, UpdateAndDrawable};
use units::Milliseconds;
use std::collections::BTreeMap;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum MotionType {
    Standing,
    Walking,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HorizontalFacing {
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct SpriteState {
    motion_type: MotionType,
    horizontal_facing: HorizontalFacing,
}

impl SpriteState {
    fn new(motion_type: MotionType, horizontal_facing: HorizontalFacing) -> SpriteState {
        SpriteState {
            motion_type: motion_type,
            horizontal_facing: horizontal_facing,
        }
    }

    fn default() -> SpriteState {
        SpriteState {
            motion_type: MotionType::Standing,
            horizontal_facing: HorizontalFacing::Left,
        }
    }
}

struct Jump {
    time_remaining: Milliseconds,
    active: bool,
}

impl Jump {
    fn new() -> Jump {
        Jump {
            time_remaining: Milliseconds::new(0),
            active: false,
        }
    }

    fn update(&mut self, elapsed_time: Milliseconds) {
        if (self.active) {
            if (self.time_remaining > elapsed_time) {
                self.time_remaining -= elapsed_time;
            } else {
                self.time_remaining = Milliseconds::new(0);
                self.active = false;
            }
        }
    }

     fn active(&self) -> bool {
         self.active
     }

    fn reset(&mut self) {
        self.time_remaining = Milliseconds::new(constants::JUMP_TIME);
        self.reactivate();
    }

    fn reactivate(&mut self) {
        self.active = self.time_remaining > Milliseconds::new(0);
    }

    fn deactivate(&mut self) {
        self.active = false;
    }
}

pub struct Player<'a> {
    sprites: BTreeMap<SpriteState, Box<UpdateAndDrawable + 'a>>,
    sprite_state: SpriteState,
    x: i32,
    y: i32,
    velocity_x: f32,
    acceleration_x: f32,
    velocity_y: f32,
    is_on_ground: bool,
    jump: Jump,
}

impl<'a> Player<'a> {
    pub fn new(graphics: &mut Graphics<'a>, x: i32, y: i32) -> Player<'a> {
        Player {
            sprites: Player::create_sprite_map(graphics),
            sprite_state: SpriteState::default(),
            x: x,
            y: y,
            velocity_x: 0.0,
            acceleration_x: 0.0,
            velocity_y: 0.0,
            is_on_ground: true,
            jump: Jump::new(),
        }
    }

    pub fn draw(&self, graphics: &mut Graphics) {
        if let Some(sprite) = self.sprites.get(&self.sprite_state) {
            sprite.draw(graphics, self.x, self.y);
        }
    }

    pub fn update(&mut self, elapsed_time: Milliseconds) {

        self.jump.update(elapsed_time);

        self.x += (self.velocity_x * elapsed_time.value() as f32).round() as i32;
        if self.acceleration_x < 0.0 {
            self.sprite_state.motion_type = MotionType::Walking;
            self.sprite_state.horizontal_facing = HorizontalFacing::Left;
            self.velocity_x =
                (-constants::MAX_SPEED).max(self.velocity_x + self.acceleration_x * elapsed_time.value() as f32)
        } else if self.acceleration_x > 0.0 {
            self.sprite_state.motion_type = MotionType::Walking;
            self.sprite_state.horizontal_facing = HorizontalFacing::Right;
            self.velocity_x =
                constants::MAX_SPEED.min(self.velocity_x + self.acceleration_x * elapsed_time.value() as f32)
        } else {
            self.sprite_state.motion_type = MotionType::Standing;
            if (self.on_ground()) {
                self.velocity_x *= constants::SLOW_DOWN;
            }
        }

        self.y += (self.velocity_y * elapsed_time.value() as f32).round() as i32;
        if !self.jump.active() {
            self.velocity_y = (self.velocity_y + (constants::GRAVITY * elapsed_time.value() as f32)).min(constants::MAX_SPEED_Y);
        }

        if (self.y >= 320) {
            self.y = 320;
            self.velocity_y = 0.0;
            self.is_on_ground = true;
        }
        else {
            self.is_on_ground = false;
        }

        if let Some(sprite) = self.sprites.get_mut(&self.sprite_state) {
            sprite.update(elapsed_time);
        }
    }

    pub fn start_moving_left(&mut self) {
        self.acceleration_x = -constants::WALKING_ACCELERATION;
    }

    pub fn start_moving_right(&mut self) {
        self.acceleration_x = constants::WALKING_ACCELERATION;
    }

    pub fn stop_moving(&mut self) {
        self.acceleration_x = 0.0;
    }

    pub fn on_ground(&self) -> bool {
        self.is_on_ground
    }

    pub fn start_jump(&mut self) {
        println!("start jump");
        if self.is_on_ground {
            println!("on ground?");
            self.jump.reset();
            self.velocity_y = -constants::JUMP_SPEED;
        } else if (self.velocity_y < 0.0) {
            self.jump.reactivate();
        }
    }

    pub fn stop_jump(&mut self) {
        println!("stop jump");
        self.jump.deactivate();
    }

    fn create_sprite_map<'b>(graphics: &mut Graphics<'b>) -> BTreeMap<SpriteState, Box<UpdateAndDrawable + 'b>> {
        let mut map: BTreeMap<SpriteState, Box<UpdateAndDrawable + 'b>> = BTreeMap::new();
        map.insert(SpriteState::new( MotionType::Walking, HorizontalFacing::Left ),
                   Box::new(AnimatedSprite::new(
                        graphics,
                        "content/MyChar.bmp",
                        0,
                        20 * constants::TILE_SIZE as i32,
                        constants::TILE_SIZE,
                        constants::TILE_SIZE,
                        10,
                        3,
                       )));
        map.insert(SpriteState::new( MotionType::Standing, HorizontalFacing::Left ),
                   Box::new(Sprite::new(
                        graphics,
                        "content/MyChar.bmp",
                        0,
                        20 * constants::TILE_SIZE as i32,
                        constants::TILE_SIZE,
                        constants::TILE_SIZE,
                       )));
        map.insert(SpriteState::new( MotionType::Walking, HorizontalFacing::Right ),
                   Box::new(AnimatedSprite::new(
                        graphics,
                        "content/MyChar.bmp",
                        0,
                        21* constants::TILE_SIZE as i32,
                        constants::TILE_SIZE,
                        constants::TILE_SIZE,
                        10,
                        3,
                       )));
        map.insert(SpriteState::new( MotionType::Standing, HorizontalFacing::Right ),
                   Box::new(Sprite::new(
                        graphics,
                        "content/MyChar.bmp",
                        0,
                        21 * constants::TILE_SIZE as i32,
                        constants::TILE_SIZE,
                        constants::TILE_SIZE,
                       )));
        map
    }
}
