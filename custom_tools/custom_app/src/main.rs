use std::fs::read_to_string;
use std::process::Command;
use web_view::*;
fn main() {
    let js = read_to_string("js/app.js").expect("Something went wrong reading the js/app.js");
    let css = read_to_string("css/app.css").expect("Something went wrong reading css/app.css");
    let html_content = format!(
        r#"
    <html>
        <head>
        <script src="https://ajax.googleapis.com/ajax/libs/jquery/3.5.1/jquery.min.js"></script>
        <script>
        {}
        </script>
        <style>
       {}
        </style> 
        </head>
        <body>
            <h1>A custom cpu stress test<h1>
            <input type="text" id="threads" name="threads" placeholder="enter number of theads">
            <button onclick="Hammer()">Click to start cpu stress test</button>
            <div id="result"></div>
            <button onclick="end_Hammer()">Click to end cpu stress test</button>
        </body>
    </html>"#,
        js, css
    );
    let _webserver = Command::new("bash")
        .arg("-c")
        .arg("./webserver/target/debug/webserver")
        .spawn()
        .expect("webserver was not able to start");
    web_view::builder()
        .title("Egee's cutsom app")
        .content(Content::Html(html_content))
        .size(320, 480)
        .resizable(false)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
    let _kill_webserver = Command::new("sh")
        .arg("-c")
        .arg("killall webserver")
        .output()
        .expect("failed to kill webserver");
}
