
use super::utils::window::Handle;
use super::utils::window::Window;
mod mock_display_server;
mod xlib_display_server;
mod event_handler;

pub use self::mock_display_server::MockDisplayServer;
pub use self::xlib_display_server::XlibDisplayServer;
pub use self::event_handler::Events;


pub trait DisplayServer<'a> {
    fn new() -> Self;
    fn find_all_windows(&self) -> Vec<Window>;

    fn event_handler(&mut self, handler: &'a event_handler::Events);

}




#[test]
fn it_should_alert_for_new_windows(){
    struct H {}
    impl Events for H{
        fn on_new_window(&self, w:Window){
            assert!(true);
        }
    }
    let mut ds:MockDisplayServer = DisplayServer::new();
    let handler = H{};
    ds.event_handler( &handler );
    ds.create_mock_window();
}

