use macroquad::prelude::*;

const G: f32 = 6.6743E-11;

const MOON_RADIUS: f32 = 1_737.4;
const MOON_MASS: f32 = 7.3477E21;

const EARTH_RADIUS: f32 = 6_378.1;
const EARTH_MASS: f32 = 5.972E23;

const EARTH_MOON_DISTANCE: f32 = 100_000.0;

const KILOMETERS_PER_PIXEL: f32 = 500.0;

struct Circle {
    px: f32,
    py: f32,
    vx: f32,
    vy: f32,
    ax: f32,
    ay: f32,
    radius: f32,
    color: Color,
}

impl Circle {
    fn new(x: f32, y: f32, radius: f32, color: Color) -> Self {
        Circle {
            px: x,
            py: y,
            vx: 0.0,
            vy: 0.0,
            ax: 0.0,
            ay: 0.0,
            radius,
            color,
        }
    }

    fn render(&mut self) {
        self.vx += self.ax;
        self.vy += self.ay;

        self.px += self.vx;
        self.py += self.vy;

        draw_circle(self.px, self.py, self.radius, self.color);
    }

    fn change_pos(&mut self, x: f32, y: f32) {
        self.px += x;
        self.py += y
    }

    fn change_velocity(&mut self, x: f32, y: f32) {
        self.vx += x;
        self.vy += y
    }

    fn change_acceleration(&mut self, x: f32, y: f32) {
        self.ax += x;
        self.ay += y
    }

    fn set_acceleration(&mut self, x: f32, y: f32) {
        self.ax = x;
        self.ay = y
    }
}

struct Planet {
    mass: f32,
    radius: f32,
    texture: Circle,
}

impl Planet {
    fn new(mass: f32, radius: f32, texture: Circle) -> Self {
        Planet {
            mass,
            radius,
            texture,
        }
    }
}

struct SolarSystem {
    planets: Vec<Planet>,
}

impl SolarSystem {
    fn new() -> Self {
        SolarSystem { planets: vec![] }
    }

    fn render(&mut self, fps: f32) {
        for planet in self.planets.into_iter() {
            planet.texture.render();
        }
    }

    fn create_planet(&mut self, planet: Planet) {
        self.planets.push(planet);
    }
}

#[macroquad::main("gravity")]
async fn main() {
    // let mut c = Circle::new(10.0, 10.0, 10.0, RED);

    // let fps: f64 = 60.0;
    // let dt_nanos = 1.0 / fps * 1_000_000_000.0;

    // c.change_acceleration(0.002, 0.0);

    let mut earth = Circle::new(
        screen_width() / 2.0 + 15.0,
        screen_height() / 2.0 + 15.0,
        EARTH_RADIUS / KILOMETERS_PER_PIXEL as f32,
        BLUE,
    );

    let mut moon = Circle::new(
        earth.px,
        earth.py - (earth.radius + EARTH_MOON_DISTANCE / KILOMETERS_PER_PIXEL as f32),
        MOON_RADIUS / KILOMETERS_PER_PIXEL as f32,
        WHITE,
    );

    let fg = (G * MOON_MASS * EARTH_MASS) / EARTH_MOON_DISTANCE.powf(2.0) as f32;
    let l = MOON_MASS * 100.0;

    loop {
        // c.change_pos(
        //     60.0 / get_fps() as f32,
        //     60.0 / get_fps() as f32,
        // );

        let dist = ((earth.px - moon.px).powf(2.0) + (earth.py - moon.py).powf(2.0)).sqrt();
        let fg = (G * MOON_MASS * EARTH_MASS) / dist / 1.0E12;

        moon.set_acceleration(0.0, fg / MOON_MASS / get_fps() as f32);

        earth.render();
        moon.render();

        next_frame().await
    }
}

