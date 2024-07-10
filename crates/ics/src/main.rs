use ics::{CalProps, Eventc};

fn main() {
    let evt = Eventc::default()
        .with_start(chrono::Utc::now())
        .with_location("OHP 17\nRaum 16");

    println!("{}\n\n", evt);

    let mut props = CalProps::new();

    props.x_prop = vec!["Foo".into(), "bar".into()];

    println!("{}", props);
}
