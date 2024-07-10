use ics::{CalProps, Eventc};

fn main() {
    let evt = Eventc::default().with_date(chrono::Utc::now());

    println!("{}\n\n", evt);

    let mut props = CalProps::new();

    props.x_prop = vec!["Foo".into(), "bar".into()];

    println!("{}", props);
}
