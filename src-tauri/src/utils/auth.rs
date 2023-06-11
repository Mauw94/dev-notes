use crate::config::Config;

pub fn login(name: String, pass: String) -> Result<bool, ()> {
    let cfg = Config::default();
    Ok(name.eq(&cfg.login_name) && pass.eq(&cfg.login_pass))
}
