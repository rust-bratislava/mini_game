extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

pub struct Ball {
    x: u32,
    y: u32,
}

pub struct App {
    gl: GlGraphics,
    rng: rand::ThreadRng,
    run: bool,
    rotation: f64,
    position: u32,
    fall_speed: f64,
    lives: u8,
}

pub struct Entities {
    red_balls: Vec<Ball>,
    green_balls: Vec<Ball>,
}

impl App {
    fn render_balls(balls: &[Ball], context: &graphics::context::Context, gl: &mut GlGraphics, color: graphics::types::Color) {
        use graphics::*;

        let circle = ellipse::circle(0.0, 0.0, 20.0);

        for ball in balls {
            let transform = context.transform.trans(ball.x as f64, ball.y as f64)
                                       .trans(-10.0, -10.0);

            ellipse(color, circle, transform, gl);
        }
    }

    fn render(&mut self, args: &RenderArgs, entities: &Entities) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLUE:  [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        const BLACK:  [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = (self.position as f64,
                      args.height as f64);

        let small_square = rectangle::square(0.0, 0.0, 10.0);
        let lives = self.lives;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLUE, gl);

            let transform = c.transform.trans(x, y)
                                       .rot_rad(rotation)
                                       .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);

            Self::render_balls(&entities.red_balls, &c, gl, RED);
            Self::render_balls(&entities.green_balls, &c, gl, GREEN);

            for i in 0..lives {
                let transform = c.transform.trans(15.0 + 15.0 * i as f64, 5.0)
                                           .trans(-5.0, -5.0);

                rectangle(BLACK, small_square, transform, gl);
            }
        });
    }

    fn update_balls<F: FnMut()>(&mut self, balls: &mut Vec<Ball>, probability: f64, mut collision: F) {
        use rand::Rng;

        let rnd: f64 = self.rng.gen();
        if rnd < probability {
            let x: f64 = self.rng.gen();
            balls.push(Ball {
                x: (x * 800.0) as u32,
                y: 0,
            });
        }

        for ball in &mut *balls {
            ball.y += (10.0 * self.fall_speed) as u32;
        }

        balls.retain(|ball| {
            if ball.y > 600 - 35 && ball.x > self.position - 35 && ball.x < self.position + 35 {
                collision();
                false
            } else {
                ball.y < 600
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs, entities: &mut Entities) {
        let mut lives = self.lives;
        let mut run = true;

        self.update_balls(&mut entities.red_balls, 0.005, || { lives -= 1; if lives == 0 { run = false; }});
        self.update_balls(&mut entities.green_balls, 0.0005, || { if lives < 3 { lives += 1; }});

        self.lives = lives;
        self.run = run;

        self.fall_speed += args.dt * 0.01;
    }

    fn move_left(&mut self) {
        if self.position > 35 {
            self.position -= 10;
        }
    }

    fn move_right(&mut self) {
        if self.position < 800 - 35 {
            self.position += 10;
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let width = 800;
    let height = 600;

    let mut window: Window = WindowSettings::new(
            "spinning-square",
            [width, height]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        rng: rand::thread_rng(),
        run: true,
        rotation: 0.0,
        position: width / 2,
        fall_speed: 0.2,
        lives: 3,
    };

    let mut entities = Entities {
        red_balls: Default::default(),
        green_balls: Default::default(),
    };

    let mut events = Events::new(EventSettings::new());

    while let (true, Some(e)) = (app.run, events.next(&mut window)) {
        match e {
            Input::Render(r) => app.render(&r, &entities),
            Input::Update(u) => app.update(&u, &mut entities),
            Input::Press(Button::Keyboard(Key::Right)) => app.move_right(),
            Input::Press(Button::Keyboard(Key::Left)) => app.move_left(),
            _ => (),
        }
    }
}
