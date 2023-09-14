use std::{path::PathBuf, time::Duration};

fn main() {
    let mut conf = MyConfiguration::default();
    conf.check = true;
    println!("conf: {:?}", conf);

    let conf1 = MyConfiguration {
        check: true,
        .. Default::default()
    };
    assert_eq!(conf, conf1);
}

#[derive(Default, Debug, PartialEq)]
struct MyConfiguration {
    output: Option<PathBuf>,
    search_path: Vec<PathBuf>,
    timeout: Duration,
    check: bool,
}

impl MyConfiguration {}
