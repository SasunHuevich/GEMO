use super::Vertex;

#[derive(Clone)]
pub struct Polygon {
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
}

impl Polygon {
    pub fn new<I>(vertices: I) -> Self where I: IntoIterator<Item = Vertex> {
        // Self - алиас для имени структуры, self - кокнретный объект
        let mut indices = Vec::new();

        let vertices: Vec<Vertex> = vertices.into_iter().collect();
        if vertices.len() < 3 {
            panic!("Ошибка создания многоугольника: количество вершин ({}) меньше трёх! Минимально нужно 3 вершины.", vertices.len());
        }

        // Количество трегольников в выпуклом многоугольне равно количеству вершин - 2
        // Мы не учитываем две крайние точки, а кол-во треугольников становиться равным количеству проведенных разделителей
        for i in 0..(vertices.len() - 2) {
            indices.push(0);
            indices.push((i + 1) as u16);
            indices.push((i + 2) as u16);
        }

        Self {
            vertices: vertices,
            indices: indices,
        }
    }

    pub fn get_vertices(&self) -> Vec<Vertex> {
        self.vertices.clone()
    }

    pub fn get_indices(&self) -> Vec<u16> {
        self.indices.clone()
    }
}