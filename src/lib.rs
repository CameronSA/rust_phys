pub mod engine {
    use flo_draw::canvas::{Color, DrawingTarget, GraphicsContext, GraphicsPrimitives, SpriteId};

    type Units = f32;
    type UnitsPerFrame = f32;
    type FramesPerSecond = u64;

    pub trait PhysicsObject {
        fn render(&self, canvas: &DrawingTarget);
        fn update(&mut self, universe: &UniverseOptions);
        fn collide(&mut self, physics_object: &Circle);
        fn hit_box_size(&self) -> Area;
        fn center_coords(&self) -> Coords;
        fn velocity(&self) -> Velocity;
        fn elasticity(&self) -> f32;
        fn sprite_id(&self) -> SpriteId;
        fn change_velocity(&mut self, velocity: Velocity);
    }

    // pub struct PhysicsObjects {
    //     list: Vec<Box<dyn PhysicsObject>>,
    // }

    // impl PhysicsObjects {
    //     pub fn new(item: Vec<Box<dyn PhysicsObject>>) -> Self {
    //         Self { list: item }
    //     }

    //     pub fn push<S: PhysicsObject + 'static>(&mut self, item: S) -> &mut Self {
    //         self.list.push(Box::new(item));

    //         self
    //     }

    //     pub fn list_mut(&mut self) -> &mut Vec<Box<dyn PhysicsObject>> {
    //         &mut self.list
    //     }

    //     pub fn list(&self) -> &Vec<Box<dyn PhysicsObject>> {
    //         &self.list
    //     }
    // }

    pub struct UniverseOptions {
        gravity: UnitsPerFrame,
        fps: FramesPerSecond,
        canvas_height: Units,
        canvas_width: Units,
    }

    impl UniverseOptions {
        pub fn new(
            gravity: UnitsPerFrame,
            fps: FramesPerSecond,
            canvas_height: Units,
            canvas_width: Units,
        ) -> UniverseOptions {
            UniverseOptions {
                gravity,
                fps,
                canvas_height,
                canvas_width,
            }
        }

        pub fn gravity(&self) -> UnitsPerFrame {
            self.gravity
        }

        pub fn fps(&self) -> FramesPerSecond {
            self.fps
        }

        pub fn canvas_height(&self) -> UnitsPerFrame {
            self.canvas_height
        }

        pub fn canvas_width(&self) -> UnitsPerFrame {
            self.canvas_width
        }
    }

    pub struct Coords {
        pub x: f32,
        pub y: f32,
    }

    pub struct Area {
        pub height: f32,
        pub width: f32,
    }

    pub struct Velocity {
        pub dx: f32,
        pub dy: f32,
    }

    /// Container for circle sprites
    ///
    /// ```sprite_id```: The unique sprite ID
    ///
    /// ```radius```: Radius of the circle in canvas units
    ///
    /// ```x```: x coord of circle center in canvas units
    ///
    /// ```y```: y coord of circle center in canvas units
    ///
    /// ```dx```: x change in canvas units per frame
    ///
    /// ```dy```: y change in canvas units per frame
    ///
    /// ```colour```: The colour of the circle
    ///
    /// ```elasticity```: The elasticity of the circle. A value of 0 means no momentum is conserved on collision. A value of 1 means all momentum conserved on collision        
    pub struct Circle {
        sprite_id: SpriteId,
        radius: f32,
        x: f32,
        y: f32,
        dx: f32,
        dy: f32,
        colour: Color,
        elasticity: f32,
    }

    impl Circle {
        pub fn new(
            sprite_id: SpriteId,
            radius: f32,
            x: f32,
            y: f32,
            dx: f32,
            dy: f32,
            colour: Color,
            elasticity: f32,
        ) -> Circle {
            Circle {
                sprite_id,
                radius,
                x,
                y,
                dx,
                dy,
                colour,
                elasticity,
            }
        }

        pub fn copy(&self) -> Circle {
            Circle {
                sprite_id: self.sprite_id,
                radius: self.radius,
                x: self.x,
                y: self.y,
                dx: self.dx,
                dy: self.dy,
                colour: self.colour,
                elasticity: self.elasticity,
            }
        }
    }

    impl PhysicsObject for Circle {
        fn render(&self, canvas: &DrawingTarget) {
            // Declare the sprite
            canvas.draw(|gc| {
                gc.sprite(self.sprite_id);
                gc.clear_sprite();

                gc.new_path();
                gc.circle(0.0, 0.0, self.radius);
                gc.fill_color(self.colour);
                gc.fill();
            });
        }

        fn update(&mut self, universe: &UniverseOptions) {
            // Gravity
            if self.y >= self.radius {
                self.dy -= universe.gravity;
            }

            let circle_right_boundary = self.x + self.dx + self.radius;
            let circle_left_boundary = self.x + self.dx - self.radius;
            let circle_top_boundary = self.y + self.dy + self.radius;
            let circle_bottom_boundary = self.y + self.dy - self.radius;

            // Collision with sides of canvas
            if circle_right_boundary > universe.canvas_width && self.dx > 0.0 {
                self.dx = -self.elasticity * self.dx;
            }
            if circle_top_boundary > universe.canvas_height && self.dy > 0.0 {
                self.dy = -self.elasticity * self.dy;
            }
            if circle_left_boundary < 0.0 && self.dx < 0.0 {
                self.dx = -self.elasticity * self.dx;
            }
            if circle_bottom_boundary < 0.0 && self.dy < 0.0 {
                self.dy = -self.elasticity * self.dy;
            }

            // Move this circle in whatever direction it's going
            self.x += self.dx;
            self.y += self.dy;
        }

        fn collide(&mut self, physics_object: &Circle) {
            let right_boundary_1 = self.x + self.dx + self.radius;
            let left_boundary_1 = self.x + self.dx - self.radius;
            let top_boundary_1 = self.y + self.dy + self.radius;
            let bottom_boundary_1 = self.y + self.dy - self.radius;

            // Collision with other object
            if physics_object.sprite_id() == self.sprite_id {
                return;
            }

            let hit_box = physics_object.hit_box_size();
            let coords = physics_object.center_coords();
            let velocity = physics_object.velocity();
            //let elasticity = physics_object.elasticity();

            let right_boundary_2 = coords.x + velocity.dx + (hit_box.width / 2.0);
            let left_boundary_2 = coords.x + velocity.dx - (hit_box.width / 2.0);
            let top_boundary_2 = coords.y + velocity.dy + (hit_box.height / 2.0);
            let bottom_boundary_2 = coords.y + velocity.dy - (hit_box.height / 2.0);

            // circle on left side collision
            if left_boundary_1 < right_boundary_2
            && right_boundary_1 > left_boundary_2
            && top_boundary_1 > bottom_boundary_2
            && bottom_boundary_1 < top_boundary_2
            {
                self.dy = -self.elasticity * self.dy;
                self.dx = -self.elasticity * self.dx;
            }
        }

        fn hit_box_size(&self) -> Area {
            Area {
                height: self.radius * 2.0,
                width: self.radius * 2.0,
            }
        }

        fn center_coords(&self) -> Coords {
            Coords {
                x: self.x,
                y: self.y,
            }
        }

        fn velocity(&self) -> Velocity {
            Velocity {
                dx: self.dx,
                dy: self.dy,
            }
        }

        fn elasticity(&self) -> f32 {
            self.elasticity
        }

        fn sprite_id(&self) -> SpriteId {
            self.sprite_id
        }

        fn change_velocity(&mut self, velocity: Velocity) {
            self.dx = velocity.dx;
            self.dy = velocity.dy;
        }
    }
}
