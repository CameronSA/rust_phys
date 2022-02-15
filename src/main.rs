use std::{thread, time::Duration};
use rand::{Rng, random};

use flo_draw::{
    canvas::{Color, GraphicsContext, LayerId, SpriteId, SpriteTransform},
    create_drawing_window, with_2d_graphics,
};

use rust_phys::engine::{Circle, PhysicsObject, UniverseOptions};

pub fn main() {
    // 'with_2d_graphics' is used to support operating systems that can't run event loops anywhere other than the main thread
    with_2d_graphics(|| {
        // Create a window with a canvas to draw on
        let canvas = create_drawing_window("Physics Engine");

        // Clear the canvas to set a background colour
        canvas.draw(|gc| {
            gc.clear_canvas(Color::Rgba(0.6, 0.7, 0.8, 1.0));
        });

        let mut circles = Vec::new();
        for i in 0..10{
            circles.push(Circle::new(
                SpriteId(i),
                32.0,
                rand::thread_rng().gen_range(0..1001) as f32,
                rand::thread_rng().gen_range(0..1001) as f32,
                rand::thread_rng().gen_range(-5..6) as f32,
                rand::thread_rng().gen_range(-5..6) as f32,
                Color::Hsluv(random::<f32>()*360.0, random::<f32>()*100.0, random::<f32>()*75.0 + 25.0, 1.0),
                0.9,
            ));
        }

        // Set up the universe
        let universe = UniverseOptions::new(0.2, 60, 1000.0, 1000.0);

        for circle in circles.iter() {
            circle.render(&canvas);
        }

        // Animate
        loop {
            let circles_len = circles.len();
            for i in 0..circles_len {
                for j in 0..circles_len {
                    if i == j {
                        continue;
                    }
                    
                    let copy = circles[j].copy();
                    circles[i].collide(&copy);
                }

                circles[i].update(&universe)
            }

            // Render the frame on layer 0
            canvas.draw(|gc| {
                gc.layer(LayerId(0));
                gc.clear_layer();
                gc.canvas_height(universe.canvas_height());
                gc.center_region(0.0, 0.0, universe.canvas_width(), universe.canvas_height());

                for circle in circles.iter() {
                    let circle_coords = circle.center_coords();
                    let circle_id = circle.sprite_id();

                    // Render the circle's sprite at its location
                    gc.sprite_transform(SpriteTransform::Identity);
                    gc.sprite_transform(SpriteTransform::Translate(
                        circle_coords.x,
                        circle_coords.y,
                    ));
                    gc.draw_sprite(circle_id);
                }
            });

            // Wait for the next frame
            thread::sleep(Duration::from_nanos(1_000_000_000 / &universe.fps()));
        }
    });
}
