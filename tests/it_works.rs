#[test]
fn it_works() {
    rogu::set_level(rogu::Level::DEBUG);
    rogu::error!("error {}, {}", "sad", "2");
    rogu::warn!("warn!");
    rogu::info!("info!");
    rogu::debug!("debug!");
    rogu::trace!("trace!");

    #[cfg(feature = "log")]
    {
        log::info!("LOG INFO!");
        log::trace!("LOG INFO!");
    }
}
