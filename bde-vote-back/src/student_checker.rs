pub const LOGINS: &'static str = include_str!("../../logins.txt");

pub fn check_login(login: &str) -> bool {
    if login.len() > 8 {
        false
    } else {
        LOGINS.split("\n").find(|&l| l == login).is_some()
    }
}
