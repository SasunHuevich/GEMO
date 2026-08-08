use GEMO::{
    App,
    geometry::Polygon,
    geometry::Vertex,
};

fn main() {
    let mut app = App::new();
    
    let poly: Polygon = Polygon::new([
        Vertex { x: -0.0868241, y: 0.49240386 },  // A
        Vertex { x: 0.44147372, y: 0.2347359 },   
        Vertex { x: 0.35966998, y: -0.3473291 },  // D
        Vertex { x: -0.21918549, y: -0.44939706 }, // C
        Vertex { x: -0.49513406, y: 0.06958647 },  // B
    ]);

    app.add_polygon(poly);

    GEMO::run(app).unwrap();
}