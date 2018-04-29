// Code for displaying the rendered product on the screen go in this
// module.

use ndarray::prelude::*;

use ggez::conf;
use ggez::event;
use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::graphics::{DrawMode, Point2};
use ggez::timer;


struct MainState {
    // nodes: Vec<Node>,
    // edges: Vec<Edge>,
    zoomlevel: f32,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {   
        let s = MainState {
            zoomlevel: 1.0,
        };

        Ok(s)
    }
}

fn build_mesh(ctx: &mut Context) -> GameResult<graphics::Mesh> {
    let mb = &mut graphics::MeshBuilder::new();

    mb.line(
        &[
            Point2::new(200.0, 200.0),
            Point2::new(400.0, 200.0),
            Point2::new(400.0, 400.0),
            Point2::new(200.0, 400.0),
            Point2::new(200.0, 300.0),
        ],
        4.0,
    );

    mb.ellipse(DrawMode::Fill, Point2::new(600.0, 200.0), 50.0, 120.0, 1.0);

    mb.circle(DrawMode::Fill, Point2::new(600.0, 380.0), 40.0, 1.0);
    mb.build(ctx)
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.zoomlevel += 0.01;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::set_color(ctx, graphics::WHITE)?;
        // let src = graphics::Rect::new(0.25, 0.25, 0.5, 0.5);
        // let src = graphics::Rect::one();
        let dst = graphics::Point2::new(20.0, 20.0);
        // graphics::draw(ctx, &self.image1, dst, 0.0)?;
        let dst = graphics::Point2::new(200.0, 100.0);
        let dst2 = graphics::Point2::new(400.0, 400.0);
        let scale = graphics::Point2::new(10.0, 10.0);
        let shear = graphics::Point2::new(self.zoomlevel, self.zoomlevel);
        graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0));
        
        let rect = graphics::Rect::new(450.0, 450.0, 50.0, 50.0);
        graphics::rectangle(ctx, graphics::DrawMode::Fill, rect)?;

        graphics::set_color(ctx, graphics::Color::new(1.0, 0.0, 0.0, 1.0))?;
        let rect = graphics::Rect::new(450.0, 450.0, 50.0, 50.0);
        graphics::rectangle(ctx, graphics::DrawMode::Line(1.0), rect)?;

        let mesh = build_mesh(ctx)?;
        graphics::set_color(ctx, (0, 0, 255).into())?;
        graphics::draw_ex(ctx, &mesh, Default::default())?;

        graphics::present(ctx);
        Ok(())
    }
}

pub fn render(projection: &Vec<Array1<f64>>) {
    // Attempting to render lines using ggez
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("drawing", "ggez", c).unwrap();
    
    // We add the CARGO_MANIFEST_DIR/resources do the filesystems paths so
    // we we look in the cargo project for files.
    // if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
    //     let mut path = path::PathBuf::from(manifest_dir);
    //     path.push("resources");
    //     ctx.filesystem.mount(&path, true);
    // }

    // let points = Point2
    // graphics::line(ctx, points, 200)

    for line in projection.iter() {

    }

    println!("{}", graphics::get_renderer_info(ctx).unwrap());
    let state = &mut MainState::new(ctx).unwrap();
    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}