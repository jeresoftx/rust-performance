use rust_performance::profile::{Profile, ProfileSample};

fn main() {
    let profile = Profile::new(vec![
        ProfileSample::new("main", 20, 5),
        ProfileSample::new("main::parse", 15, 15),
    ])
    .expect("las atribuciones son válidas");

    println!("hot path: {}", profile.hottest_path());
}
