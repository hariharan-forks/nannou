// The Nature of Code
// Daniel Shiffman
// http://natureofcode.com
//
// Example 2-6: Attraction
extern crate nannou;

use nannou::prelude::*;

fn main() {
    nannou::app(model, event, view).run();
}

struct Model {
    mover: Mover,
    attractor: Attractor,
}

struct Mover {
    position: Point2<f32>,
    velocity: Vector2<f32>,
    acceleration: Vector2<f32>,
    mass: f32,
}

// A type for a draggable attractive body in our world
struct Attractor {
    mass: f32,                 // Maxx, tied to size
    position: Point2<f32>,     // position
    dragging: bool,            // Is the object being dragged?
    roll_over: bool,           // Is the mouse over the ellipse?
    drag_offset: Vector2<f32>, // holds the offset for when the object is clicked on
}

impl Attractor {
    const G: f32 = 1.0; // Gravitational Constant
    fn new(rect: Rect<f32>) -> Self {
        let position = rect.xy();
        let mass = 20.0;
        let drag_offset = vec2(0.0, 0.0);
        let dragging = false;
        let roll_over = false;
        Attractor {
            position,
            mass,
            drag_offset,
            dragging,
            roll_over,
        }
    }

    fn attract(&self, m: &Mover) -> Vector2<f32> {
        let mut force = self.position - m.position; // Calculate direction of force
        let mut d = force.magnitude(); // Distance between objects
        d = d.max(5.0).min(25.0); // Limiting the distance to eliminate "extreme" results for very cose or very far object
        force = force.normalize(); // Normalize vector (distance doesn't matter, we just want this vector for direction)
        let strength = (Attractor::G * self.mass * m.mass) / (d * d); // Calculate gravitational force magnitude
        force * strength // Get force vector --> magnitude * direction
    }

    // Method to display
    fn display(&self, draw: &app::Draw) {
        let gray = if self.dragging {
            0.2
        } else if self.roll_over {
            0.4
        } else {
            0.75
        };
        draw.rect()
            .xy(self.position)
            .w_h(self.mass * 2.0, self.mass * 2.0)
            .rgba(gray, gray, gray, 0.8);
    }

    // The methods below are for mouse interaction
    fn clicked(&mut self, mx: f32, my: f32) {
        let d = self.position.distance(pt2(mx, my));
        if d < self.mass {
            self.dragging = true;
            self.drag_offset.x = self.position.x - mx;
            self.drag_offset.y = self.position.y - my;
        }
    }

    fn hover(&mut self, mx: f32, my: f32) {
        let d = self.position.distance(pt2(mx, my));
        if d < self.mass {
            self.roll_over = true;
        } else {
            self.roll_over = false;
        }
    }

    fn stop_dragging(&mut self) {
        self.dragging = false;
    }

    fn drag(&mut self, mx: f32, my: f32) {
        if self.dragging {
            self.position.x = mx + self.drag_offset.x;
            self.position.y = my + self.drag_offset.y;
        }
    }
}

impl Mover {
    fn new() -> Self {
        let position = pt2(80.0, 130.0);
        let velocity = vec2(1.0, 0.0);
        let acceleration = vec2(0.0, 0.0);
        let mass = 1.0;
        Mover {
            position,
            velocity,
            acceleration,
            mass,
        }
    }

    fn apply_force(&mut self, force: Vector2<f32>) {
        let f = force / self.mass;
        self.acceleration += f;
    }

    fn update(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
        self.acceleration *= 0.0;
    }

    fn display(&self, draw: &app::Draw) {
        draw.ellipse()
            .xy(self.position)
            .w_h(16.0, 16.0)
            .rgb(0.3, 0.3, 0.3);
    }

    fn _check_edges(&mut self, rect: Rect<f32>) {
        if self.position.x > rect.right() {
            self.position.x = rect.left();
        } else if self.position.x < rect.left() {
            self.position.x = rect.right();
        }
        if self.position.y < rect.bottom() {
            self.velocity.y *= -1.0;
            self.position.y = rect.bottom();
        }
    }
}

fn model(app: &App) -> Model {
    let rect = Rect::from_w_h(640.0, 360.0);
    let _window = app.new_window()
        .with_dimensions(rect.w() as u32, rect.h() as u32)
        .build()
        .unwrap();

    let mover = Mover::new();
    let attractor = Attractor::new(rect);

    Model { mover, attractor }
}

fn event(app: &App, mut m: Model, event: Event) -> Model {
    match event {
        Event::WindowEvent {
            simple: Some(event),
            ..
        } => {
            match event {
                // MOUSE EVENTS
                MousePressed(_button) => {
                    m.attractor.clicked(app.mouse.x, app.mouse.y);
                }

                MouseReleased(_buttom) => {
                    m.attractor.stop_dragging();
                }
                _other => (),
            }
        }
        // update gets called just before view every frame
        Event::Update(_dt) => {
            let force = m.attractor.attract(&m.mover);
            m.mover.apply_force(force);
            m.mover.update();
            m.attractor.drag(app.mouse.x, app.mouse.y);
            m.attractor.hover(app.mouse.x, app.mouse.y);
        }
        _ => (),
    }
    m
}

fn view(app: &App, m: &Model, frame: Frame) -> Frame {
    // Begin drawing
    let draw = app.draw();
    draw.background().color(WHITE);

    m.attractor.display(&draw);
    m.mover.display(&draw);

    // Write the result of our drawing to the window's OpenGL frame.
    draw.to_frame(app, &frame).unwrap();

    // Return the drawn frame.
    frame
}
