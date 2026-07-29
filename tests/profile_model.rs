use rust_performance::profile::{Profile, ProfileError, ProfileSample};

#[test]
fn reports_inclusive_and_exclusive_work_separately() {
    let profile = Profile::new(vec![
        ProfileSample::new("main", 20, 5),
        ProfileSample::new("main::parse", 15, 15),
    ])
    .expect("valid profile");

    assert_eq!(profile.hottest_path(), "main");
    assert_eq!(profile.exclusive_units("main"), Some(5));
    assert_eq!(profile.inclusive_units("main"), Some(20));
}

#[test]
fn rejects_an_empty_profile_path() {
    assert_eq!(
        Profile::new(vec![ProfileSample::new("", 1, 1)]),
        Err(ProfileError::MissingPath)
    );
}

#[test]
fn rejects_exclusive_work_greater_than_inclusive_work() {
    assert_eq!(
        Profile::new(vec![ProfileSample::new("main", 2, 3)]),
        Err(ProfileError::InvalidAttribution)
    );
}
