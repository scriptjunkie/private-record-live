# private-record-live
Record video live to a server you control, just for fun or in case you need to collect evidence and don't want it just on your device or stored publicly. Can be built as a single Rust executable that acts as an HTTPS server or deployed on Apache/PHP server. (Note, you need to serve over HTTPS since modern browsers won't enable the camera/microphone on non-HTTPS pages)
Based on https://raw.githubusercontent.com/muaz-khan/RecordRTC/

# Deployment - standalone
## Get the binary
Download a release file from the [release pages](https://github.com/scriptjunkie/private-record-live/releases)

OR

check out this repository somewhere with an updated copy of Rust and `cargo build --release` it

## Change SSL keys
Ideally you use valid SSL keys issued by a real certificate authority. Failing that, just
generate self-signed keys by running `openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365`

## Run it
Be sure to execute from a directory the program can create and write a subfolder of. Once it's running, go to https://[your_ip]:8443/ and your page should be visible.

# Deployment - PHP
- Drop the .htaccess, PHP, HTML, CSS, and JS files on an Apache PHP host with mod_rewrite and htaccess files enabled
- Create a subfolder named "ups"
- On your device, go to the rcrdr.html page and you should be good to go

# Usage
- Open your device's browser and go to the appropriate recording home page of the server.
- Hit the button to start recording.
- When prompted to enable video/microphone access, be sure to check the box to remember this permission grant.
- Stop recording.
- Bookmark the page. Better yet, create a home page shortcut.
- When you see something you might want to record, open the shortcut and hit record.
- If you want to delete a video, by design you cannot do that from the page. You'll have to log into the server and delete the files manually.
- Keep in mind this code does not implement access control. You can place this behind .htpasswd restrictions, client certificate restrictions, VPN/IP restrictions, or just hide on an obscure randomized not-enumerable path, but pick a security measure you're comfortable with.
