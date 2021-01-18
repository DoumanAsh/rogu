#[test]
fn it_works() {
    rogu::set_level(rogu::Level::DEBUG);
    rogu::error!("error {}, {}", "sad", "2");
    rogu::error!("asdjahsdkjsakjdhsakdhas;kfdahf;kjdshgf;kjdsgfhea;kjghfda;kjgrdal;kjghfd;kghrdskjghfdskjghfdskjgdf12323");
    rogu::error!("asdjahsdkjsakjdhsakdhas;kfdahf;kjdshgf;kjdsgfhea;kj");
    rogu::error!("asdjahsdkjsakjdhsakdhas;kfdahf;kjdshgf;kjdsgfhea;kjg");
    rogu::error!("asdjahsdkjsakjdhsakdhas;kfdahf;kjdshgf;kjdsgfhea;kjg1");
    rogu::warn!("");
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
