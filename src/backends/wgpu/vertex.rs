use crate::geometry::Vertex as GeometryVertex;

#[repr(C)] // (representation c) этот атрибут указывает компилятору организовать структуру данных точно так же, как это сделал бы язык Си
// Это критически важно при работе с видиокартами через wgpu. Раст для оптимизации переставляет поля в структуре местами.
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            // array_stride определяет ширину вершины. В наше случае вероятно 24 байта
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            // step_mode указывает конвейеру, представляет ли кажждый элемент массива в этом буфере
            // данные для каждой вершины.
            step_mode: wgpu::VertexStepMode::Vertex,
            // Атрибуты вершины описывают отдельные части вершины.
            // Как правило это однозначное соответствие полям структуры.
            attributes: &[
                wgpu::VertexAttribute {
                    // Смещение для первого атрибута равно 0, для последующих сумму size_of данных предыдущих атрибутов
                    offset: 0,
                    // Указывает шейдеру, где хранить атрибут. Будет соответсовать полю позиции 0 структуры Vertex, т.е. position
                    shader_location: 0,
                    // Сообщает шейдеру форму атрибута
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                }
            ]
        }
    }
}

impl From<&GeometryVertex> for Vertex {
    fn from(vertex: &GeometryVertex) -> Self {
        Self {
            position: [vertex.x, vertex.y, 0.0],
            color: [1.0, 1.0, 1.0],
        }
    }
}