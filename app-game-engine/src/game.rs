use glam::Vec2;
use sdl2::{
    EventPump, Sdl, event::Event, image::LoadTexture, keyboard::Keycode, pixels::Color, rect::Rect,
    render::Canvas, video::Window,
};

pub const LIMIT_FPS: bool = false;
pub const FPS: i32 = 60;
pub const MILLISECS_PER_FRAME: i32 = 1000 / FPS;

#[derive(Default)]
pub struct Game {
    is_running: bool,
    canvas: Option<Canvas<Window>>,
    event_bump: Option<EventPump>,
    sdl_context: Option<Sdl>,
    player_position: Vec2,
    player_velocity: Vec2,
    milisecs_previous_frame: i32,
}

impl Game {
    pub fn initialize(&mut self) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let display_mode = video_subsystem.display_mode(0, 0).unwrap();

        let window = video_subsystem
            .window("game", display_mode.w as u32, display_mode.h as u32)
            .fullscreen()
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        self.sdl_context = Some(sdl_context);
        self.canvas = Some(canvas);
        self.event_bump = Some(event_pump);
        self.milisecs_previous_frame = 0;
        self.is_running = true;
    }

    pub fn run(&mut self) {
        self.setup();
        'running: loop {
            if !self.is_running {
                break 'running;
            }

            self.process_input();
            self.update();
            self.render();
        }
    }

    fn setup(&mut self) {
        self.player_position = Vec2 { x: 10.0, y: 20.0 };
        self.player_velocity = Vec2 { x: 100.0, y: 0.0 };
    }

    fn process_input(&mut self) {
        for event in self.event_bump.as_mut().unwrap().poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.is_running = false,
                _ => {}
            }
        }
    }

    fn update(&mut self) {
        let sdl_context = self.sdl_context.as_ref().unwrap();

        if LIMIT_FPS {
            let milisecs_since_init = sdl_context.timer().unwrap().ticks() as i32;
            let time_to_wait =
                MILLISECS_PER_FRAME - (milisecs_since_init - self.milisecs_previous_frame);

            if time_to_wait > 0 && time_to_wait <= MILLISECS_PER_FRAME {
                sdl_context.timer().unwrap().delay(time_to_wait as u32);
            }
        }

        let delta_time = (sdl_context.timer().unwrap().ticks() as i32 - self.milisecs_previous_frame) as f32 / 1000.0;

        self.milisecs_previous_frame = sdl_context.timer().unwrap().ticks() as i32;

        self.player_position.x += self.player_velocity.x * delta_time;
        self.player_position.y += self.player_velocity.y * delta_time;
    }

    fn render(&mut self) {
        let canvas = self.canvas.as_mut().unwrap();

        canvas.set_draw_color(Color::RGBA(21, 21, 21, 255));
        canvas.clear();

        let texture_creator = canvas.texture_creator();
        let texture = texture_creator
            .load_texture("./assets/images/tank-tiger-right.png")
            .unwrap();

        let dst_rect = Rect::new(
            self.player_position.x as i32,
            self.player_position.y as i32,
            32,
            32,
        );

        canvas.copy(&texture, None, dst_rect).unwrap();
        canvas.present();
    }
}
