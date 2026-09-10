use nu_plugin::{MsgPackSerializer, serve_plugin};
use nu_plugin_ipscan::IpScanPlugin;

fn main() {
    serve_plugin(&IpScanPlugin, MsgPackSerializer);
}
