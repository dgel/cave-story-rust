use constants;
use graphics::Graphics;
use sprite::{AnimatedSprite, Sprite, UpdateAndDrawable};
use std::collections::BTreeMap;
use units::Milliseconds;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum MotionType {
    Standing,
    Walking,
    Jumping,
    Falling,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HorizontalFacing {
    Left,
    Right,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum VerticalFacing {
    Up,
    Forward,
    Down,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct SpriteState {
    motion_type: MotionType,
    horizontal_facing: HorizontalFacing,
    vertical_facing: VerticalFacing,
}

impl SpriteState {
    fn new(
        motion_type: MotionType,
        horizontal_facing: HorizontalFacing,
        vertical_facing: VerticalFacing,
    ) -> SpriteState {
        SpriteState {
            motion_type: motion_type,
            horizontal_facing: horizontal_facing,
            vertical_facing: vertical_facing,
        }
    }

    fn default() -> SpriteState {
        SpriteState {
            motion_type: MotionType::Standing,
            horizontal_facing: HorizontalFacing::Left,
            vertical_facing: VerticalFacing::Forward,
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
        if self.active {
            if self.time_remaining > elapsed_time {
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
        let sprite = self.sprites.get(&SpriteState::new(MotionType::Falling, HorizontalFacing::Left, VerticalFacing::Forward)).expect("Lookup of sprite for sprite state");
        sprite.draw(graphics, 320, 200);
        let sprite = self.sprites.get(&SpriteState::new(MotionType::Falling, HorizontalFacing::Left, VerticalFacing::Up)).expect("Lookup of sprite for sprite state");
        sprite.draw(graphics, 320, 240);
        let sprite = self.sprites.get(&SpriteState::new(MotionType::Falling, HorizontalFacing::Left, VerticalFacing::Down)).expect("Lookup of sprite for sprite state");
        sprite.draw(graphics, 320, 280);
        let sprite = self.sprites.get(&self.sprite_state).expect("Lookup of sprite for sprite state");
        sprite.draw(graphics, self.x, self.y);
    }

    pub fn update(&mut self, elapsed_time: Milliseconds) {
        // update jump state
        self.jump.update(elapsed_time);

        // update x position, velocity
        self.x += (self.velocity_x * elapsed_time.value() as f32).round() as i32;
        if self.acceleration_x < 0.0 {
            self.velocity_x = (-constants::MAX_SPEED)
                .max(self.velocity_x + self.acceleration_x * elapsed_time.value() as f32)
        } else if self.acceleration_x > 0.0 {
            self.velocity_x = constants::MAX_SPEED
                .min(self.velocity_x + self.acceleration_x * elapsed_time.value() as f32)
        } else {
            if self.on_ground() {
                self.velocity_x *= constants::SLOW_DOWN;
            }
        }

        // update y position, velocity
        self.y += (self.velocity_y * elapsed_time.value() as f32).round() as i32;
        if !self.jump.active() {
            self.velocity_y = (self.velocity_y
                + (constants::GRAVITY * elapsed_time.value() as f32))
                .min(constants::MAX_SPEED_Y);
        }

        // determine if on ground
        if self.y >= 320 {
            self.y = 320;
            self.velocity_y = 0.0;
            self.is_on_ground = true;
        } else {
            self.is_on_ground = false;
        }

        // update sprite state
        self.update_sprite_state();


        // update the sprite
        if let Some(sprite) = self.sprites.get_mut(&self.sprite_state) {
            sprite.update(elapsed_time);
        }
    }

    fn update_sprite_state(&mut self) {
        if self.acceleration_x < 0.0 {
            self.sprite_state.motion_type = MotionType::Walking;
            self.sprite_state.horizontal_facing = HorizontalFacing::Left;
        } else if self.acceleration_x > 0.0 {
            self.sprite_state.motion_type = MotionType::Walking;
            self.sprite_state.horizontal_facing = HorizontalFacing::Right;
        } else {
            self.sprite_state.motion_type = MotionType::Standing;
        }

        if !self.is_on_ground {
            if self.velocity_y < 0.0 {
                self.sprite_state.motion_type = MotionType::Jumping;
            } else if self.velocity_y > 0.0 {
                self.sprite_state.motion_type = MotionType::Falling;
            }
        }
        //println!("{:?}", self.sprite_state);

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

    pub fn look_horizontal(&mut self) {
        self.sprite_state.vertical_facing = VerticalFacing::Forward;
    }

    pub fn look_up(&mut self) {
        self.sprite_state.vertical_facing = VerticalFacing::Up;
    }

    pub fn look_down(&mut self) {
        self.sprite_state.vertical_facing = VerticalFacing::Down;
    }

    pub fn on_ground(&self) -> bool {
        self.is_on_ground
    }

    pub fn start_jump(&mut self) {
        if self.is_on_ground {
            self.jump.reset();
            self.velocity_y = -constants::JUMP_SPEED;
        } else if self.velocity_y < 0.0 {
            self.jump.reactivate();
        }
    }

    pub fn stop_jump(&mut self) {
        self.jump.deactivate();
    }

    fn create_sprite_map<'b>(
        graphics: &mut Graphics<'b>,
    ) -> BTreeMap<SpriteState, Box<UpdateAndDrawable + 'b>> {
        let mut map: BTreeMap<SpriteState, Box<UpdateAndDrawable + 'b>> = BTreeMap::new();

        // load the 11th character in the sprite sheet
        Player::load_character_sprites(graphics, &mut map, 10); 

        map
    }

    fn load_character_sprites<'b>(
        graphics: &mut Graphics<'b>,
        map: &mut BTreeMap<SpriteState, Box<UpdateAndDrawable + 'b>>,
        nth_character: u32,
    ) {
        Player::load_motion_sprites(graphics, map, 0, (nth_character * 2) as i32, HorizontalFacing::Left, VerticalFacing::Forward);
        Player::load_motion_sprites(graphics, map, 3, (nth_character * 2) as i32, HorizontalFacing::Left, VerticalFacing::Up);
        Player::load_motion_sprites(graphics, map, 6, (nth_character * 2) as i32, HorizontalFacing::Left, VerticalFacing::Down);
        Player::load_motion_sprites(graphics, map, 0, (nth_character * 2 + 1) as i32, HorizontalFacing::Right, VerticalFacing::Forward);
        Player::load_motion_sprites(graphics, map, 3, (nth_character * 2 + 1) as i32, HorizontalFacing::Right, VerticalFacing::Up);
        Player::load_motion_sprites(graphics, map, 6, (nth_character * 2 + 1) as i32, HorizontalFacing::Right, VerticalFacing::Down);
    }

    fn load_motion_sprites<'b>(
        graphics: &mut Graphics<'b>,
        map: &mut BTreeMap<SpriteState, Box<UpdateAndDrawable + 'b>>,
        x_tile_offset: i32,
        y_tile_offset: i32,
        horizontal_facing: HorizontalFacing,
        vertical_facing: VerticalFacing,
    ) {
        map.insert(
            SpriteState::new(MotionType::Walking, horizontal_facing, vertical_facing),
            Box::new(AnimatedSprite::new(
                graphics,
                "content/MyChar.bmp",
                x_tile_offset * constants::TILE_SIZE as i32,
                y_tile_offset * constants::TILE_SIZE as i32,
                constants::TILE_SIZE,
                constants::TILE_SIZE,
                10,
                3,
            )),
        );
        let mut insert_sprite = |motion_type, offset| {
            map.insert(
                SpriteState::new(motion_type, horizontal_facing, vertical_facing),
                Box::new(Sprite::new(
                    graphics,
                    "content/MyChar.bmp",
                    (x_tile_offset + offset) * constants::TILE_SIZE as i32,
                    y_tile_offset * constants::TILE_SIZE as i32,
                    constants::TILE_SIZE,
                    constants::TILE_SIZE,
                )),
            );
        };
        insert_sprite(MotionType::Standing, 0);
        insert_sprite(MotionType::Jumping, 1);
        insert_sprite(MotionType::Falling, 2);
    }
}
