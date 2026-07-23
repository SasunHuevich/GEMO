// Vertex shader

// Структура для хранения выходных данных вершинного шейдера
struct VertexOutput {
    // Координаты вершины
    // bit builtin(position) сообщает wgpu, что это значение,
    // которое мы хотим использовать в качестве вершины отсечения
    @builtin(position) clip_position: vec4<f32>,
}

// Точка входа вершинного шейдера
@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    return out;
}


// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Это устанавливает цвет фрагмента на коричневый
    return vec4<f32>(0.0, 0.0, 0.0, 0.0);
}