use GEMO::{
    App,
    Vertex,
    Polygon
};

fn main() {
    let mut app = App::new();
    
    let poly: Polygon = Polygon::new([
        Vertex { position: [-0.0868241, 0.49240386, 0.0], color: [-0.5, 0.0, 0.5] },  // A
        Vertex { position: [0.44147372, 0.2347359, 0.0], color: [0.5, 0.0, 0.5] },   
        Vertex { position: [0.35966998, -0.3473291, 0.0], color: [0.8, 0.3, -0.2] },  // D
        Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [1.0, 0.0, 0.5] }, // C
        Vertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.5, 1.0, 1.0] },  // B
    ]);

    app.add_polygon(poly);

    GEMO::run(app).unwrap();
}