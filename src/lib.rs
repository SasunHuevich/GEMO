use winit::{
    event_loop::EventLoop,
};

pub mod app;
pub use app::App;

pub mod state;
pub mod vertex;
pub use vertex::Vertex;

pub mod polygon;
pub use polygon::Polygon;

pub fn run(mut app: App) -> anyhow::Result<()> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
    }
    #[cfg(target_arch = "wasm32")]
    {
        console_log::init_with_level(log::Level::Info).unwrap_throw();
    }

    let event_loop = EventLoop::with_user_event().build()?;
    #[cfg(not(target_arch = "wasm32"))]
    {
        event_loop.run_app(&mut app)?;
    }
    #[cfg(target_arch = "wasm32")]
    {
        let app = App::new(&event_loop);
        event_loop.spawn_app(app);
    }

    Ok(())
}

#[cfg(target_arch = "wasm32")]
#[wasm_bongen(start)]
pub fn run_web() -> Result<(), wasm_bingen::JsValue> {
    colsole_error_panic_hook::set_once();
    run().unwrap_throw();

    Ok(())
}

