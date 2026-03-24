use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

fn main() {
    // Создаем цикл обработки событий?
    let event_loop = EventLoop::new().unwrap();

    // Что за метод into
    let window_size: PhysicalSize<u32> = (800, 600).into();

    // Создаем окно задав его параметры
    let window = WindowBuilder::new()
        .with_fullscreen(None)
        .with_inner_size(window_size)
        .with_title("wgpu first steps")
        .build(&event_loop)
        .unwrap();

    // Запустим цикл обработки событий, передав в него замыкание,
    //которое будет выполняться на каждой итерации цикла
    event_loop.run(move |event, elwt| {
        // будем попадать в тело цикла только при появлении события ос
        match event {
            // если было запрошено событие окна, завершаем цикл
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                elwt.exit();
            } 

            // Остальные события нам не интересны
            _ => {}
        }
    });;
}
