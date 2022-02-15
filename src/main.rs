use std::{thread, time::Duration};

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

        // Set up some objects
        let circle1 = Circle::new(
            SpriteId(1),
            32.0,
            250.0,
            500.0,
            5.0,
            0.0,
            Color::Hsluv(270.0, 95.0, 50.0, 1.0),
            0.8,
        );

        let circle2 = Circle::new(
            SpriteId(1),
            32.0,
            750.0,
            500.0,
            -5.0,
            0.0,
            Color::Hsluv(270.0, 95.0, 50.0, 1.0),
            0.8,
        );

        // Set up the universe
        let universe = UniverseOptions::new(0.2, 60, 1000.0, 1000.0);

        let mut circles = vec![circle1, circle2];


        for circle in circles.iter() {
            circle.render(&canvas);
        }

        // Animate
        loop {            
            for circle in circles.iter_mut(){
                circle.update(&universe, &mut Vec::new());
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
