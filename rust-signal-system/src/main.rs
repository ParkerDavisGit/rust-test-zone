use rss::player::Player;
//use cargo::signal_bus;

fn main() {
    let test_player = Player::new(1f32, 1f32);

    println!("{}", test_player);
}
