# Webview cpu stress test

Look egee no electron!

This is a rust based gui application meant to front end stress_cpu_x64 a thrown togather cpu stresser. The idea behind this came to me when Egee asked if he could front end the application in electron. His main reasoning was the electon guis are easir to make since they use a html+css+js fronted.

I found electron to be overly complicated and wanted to create a solution that was as simple if not better then. Thus this project was born. This use less ram then a  regular electron application while being a easy to create gui as it uses html+css+js on the frontend. This project use webview to create a browser window to load html too.

## Future plans
 
I plan to package this etheir in a flatpak or app image depending on which is possible. In the mean time you can run this application by running first compliing the webserver with `cargo build` in the webserver directory. Then you can run/complie the gui with `cargo run` 
