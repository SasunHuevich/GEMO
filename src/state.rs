use std::sync::Arc;

use winit::{
    event_loop::ActiveEventLoop, window::Window
};
use winit::keyboard::{KeyCode};

// Атрибуты условной компиляции 
// configuration
// "Включай следующую строчку только тогда, когда выполняется условие внутри скобок"
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use winit::platform::web::EventLoopExtWebSys;

pub struct State {
    // 'static - этот тип являяется независимым владельцем своей памяти
    // и не содержит внутри себя никаких временных ссылок
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
    window: Arc<Window>,
}

impl State {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let size = window.inner_size();

        // instance - первое, что мы создаём при использовании wgpu
        // его основная цель создание adapter и surface
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
            flags:Default::default(),
            memory_budget_thresholds: Default::default(),
            backend_options: Default::default(),
            display: None,
        });

        // surface - та часть окна к которой мы рисуем
        // необходимо реализовать трейт raw-window-handle для создания поверхности
        let surface = instance.create_surface(window.clone()).unwrap();

        // adapter - идентификатор нашей видеокарты
        // используем его для получения информации о видеокарте 
        // будем использовать для последующего создания Device и Queue
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                // power_perfomance - имеет три варианта: None, LowPower, HighPerformance.
                // по умолчанию None, где энергопотреблениене не учитывается при выборе адаптера
                power_preference: wgpu::PowerPreference::default(),
                // compatible_surface - указывает wgpu найти адаптер,
                // который может взаимодействовать с предоставленной поверхностью
                compatible_surface: Some(&surface),
                // force_fallback_adapter - заставляет wgpu выбрать адаптер,
                // который будет работать на любом оборудовании
                // обычно это означает, что в качестве бэкэнда рендеринга
                // будет использоваться "Программная" система, а не аппаратная,
                // например, гравияечкий процессор
                force_fallback_adapter: false,
            }).await?;


        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                // required_features - дополнительные функции
                required_features: wgpu::Features::empty(),
                // experimental_features -  собираемся ли мы использовать не стаблиьные функции
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                // Некоторые фунуции не работают в вебе
                // Ограничение на кол-во создаваемые ресурсов определенных типов
                required_limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                // memory_hints - предпочтительная стратегия распределения памяти для адаптера,
                // если она поддерживается
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            }).await?;

        let surface_caps = surface.get_capabilities(&adapter);
        // код шейдера рассчитан на использования текстуры поверхности в формате sRGB
        // при использовании структуры в другом формате все цвета будут выглядеть темнее
        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            // usage - как SurfaceTexture будут использоваться тестуры
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            // format - определяет как SurfaceTexture данные будут храниться на графическом процессоре
            format: surface_format,
            // width и height  - это ширина и высота в пикселях элемента SurfaceTexture
            width: size.width,
            height: size.height,
            // используется wgpu::PresentMode перечисление,
            // определяющее способ синхронизации поверхности с дисплеем
            // выбираем первый доступный для простоты
            // PresentMode::Fifoчастота обновления экрана будет ограничена частотой кадров дисплея
            // По сути, это VSync. Гарантируется поддержка этого режима на всех платформах
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            // view_formats - Это список TextureFormatпараметров, 
            // которые можно использовать при создании TextureViewтекстур
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        Ok(Self {
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
            window
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            // Обратите внимание, что максимально поддерживаемые размеры в WebGL составляют 2048 пикселей.
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;
        }
    }

    pub fn handle_key(&self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        match (code, is_pressed) {
            (KeyCode::Escape, true) => event_loop.exit(),
            _ => {}
        }
    }

    pub fn update(&mut self) {
        // TODO
    }

    pub fn render(&mut self) -> anyhow::Result<()> {
        // сообщает ос, что окно надо постоянно перерисовывать
        self.window.request_redraw();

        // мы не можем рендерить пока поверхность не сконфигурирована
        // например, если окно только создается
        if !self.is_surface_configured {
            return Ok(())
        }

        // Чтобы отрисовать текстуру мы должны её получить
        // У неё могут бытть разные статусы
        let output = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(surface_texture) => surface_texture,
            wgpu::CurrentSurfaceTexture::Suboptimal(surface_texture) => surface_texture,
            wgpu::CurrentSurfaceTexture::Timeout
            | wgpu::CurrentSurfaceTexture::Occluded
            | wgpu::CurrentSurfaceTexture::Validation => {
                // Пропускаеем
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Outdated => {
                self.surface.configure(&self.device, &self.config);
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Lost => {
                anyhow::bail!("Lost device");
            }
        };

        // На текстуре напрямую мы не можем рисовать
        // нам нужен "вид на текстуру"
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Буфер команд, которые затем можно отправить на графический процессор
        // В реадльном времени видеокарта не понимает команды
        // Мы должны сначала ей записать список дел, а потом отправить его целиком
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        // Теперь перейдем к очистке экрана 
        // Нам нужно исопльзовать encoder для создания RenderPass
        // RenderPass содержит все методы для фактического рисования

        // {}
        // begin_render_pass() заимствует переменные изменяемым образом
        // Мы не можем вызвать encoder.finish() пока не освободим изменяемое заимствование
        // Блок указывает rust, что можно удалить все переменные внутри него,
        // когда блока покинет эту область видимости
        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });
        }

        // Эти строки указывают wgpu на необходимость завершения буфера команд
        // и отправки его в очередь процессора
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())

    }


}