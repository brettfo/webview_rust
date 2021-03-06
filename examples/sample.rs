use webview_rust_sys::{SizeHint, WebviewBuilder};

fn main() {
    let mut webview = WebviewBuilder::new()
        .debug(true)
        .title("TEST")
        .width(1024)
        .height(768)
        .resize(SizeHint::NONE)
        .init("window.x = 42")
        .dispatch(|w| {
            w.set_size(800, 600, SizeHint::MIN);
            println!("Hello World");
        })
        .url("https://google.com")
        .build();

    let mut w = webview.clone();
    webview.bind("xxx", move |seq, _req| {
        w.eval("console.log('The anwser is ' + window.x);");
        w.r#return(seq, 0, "{ result: 'We always knew it!' }");
    });
    webview.run();
}
