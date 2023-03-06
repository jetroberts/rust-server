use sim::new_boundary;

mod sim;
fn main() {
    let b = new_boundary(500, 500);

    let mut simulation = sim::setup(b, 100);
    simulation.run()

    // let app = Router::new()
    //     .route("/", get(root))
    //     .route("/ws", get(server::ws_handler));

    // let addr = SocketAddr::from(([127, 0, 0, 1], 3030));

    // axum::Server::bind(&addr)
    //     .serve(app.into_make_service_with_connect_info::<SocketAddr>())
    //     .await
    //     .unwrap();
}