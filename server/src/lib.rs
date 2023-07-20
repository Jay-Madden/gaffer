use worker::*;

#[event(fetch)]
async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {

    let router = Router::new(); // if no data is needed, pass `()` or any other valid data

    router.get("/", |_, _| {
        Response::ok("Hello Kensey, I love you!")
    });

    router.get("/websocket", |_, _| {
            // Accept / handle a websocket connection
            let pair = WebSocketPair::new()?;
            let server = pair.server;
            server.accept()?;
            server.send_with_str("Hi")?;
            server.send_with_str("Other message")?;
            let inner_server = server.clone();
            server.on_message_async(move |event| {
                let server = inner_server.clone();
                async move {
                    server
                        .send_with_str(event.get_data().as_string().unwrap())
                        .unwrap();
                    console_log!("Message received");
                }
            })?;
            server.on_close(|close| {
                console_log!("{:?}", close);
            })?;
            server.on_error(|error| {
                console_log!("{:?}", error);
            })?;
            Ok(Response::empty()?
                .with_status(101)
                .with_websocket(Some(pair.client)))
        })
}
