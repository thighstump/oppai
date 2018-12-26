extern crate oppai;
extern crate web_view;

use oppai::server;
use std::thread;
use web_view::*;

fn main() {
    thread::spawn(move || {
        server::start_server();
    });

    let webview = WebViewBuilder::new()
        .title("Oppai")
        .content(Content::Url("http://127.0.0.1:10010"))
        .size(1280, 720)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .build()
        .unwrap();
    webview.run().unwrap();
}
