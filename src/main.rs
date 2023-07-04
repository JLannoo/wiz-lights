mod Wiz;
mod Light;
mod Messenger;

fn main() {
    let mut wiz = Wiz::Wiz::new();
    wiz.find_lights();

    println!("{:#?}",wiz.get_pilot(&wiz.lights[0]));
}
