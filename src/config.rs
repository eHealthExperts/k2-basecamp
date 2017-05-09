extern crate envy;

use std::env::var;

#[derive(Deserialize)]
struct CtnPn {
    #[serde(rename="K2_CTN")]
    ctn: u16,

    #[serde(rename="K2_PN")]
    pn: u16,
}

pub fn ctn_or(ctn: u16) -> u16 {
    match envy::from_env::<CtnPn>() {
        Ok(config) => {
            debug!("From env: ctn {}", config.ctn);
            config.ctn
        }
        _ => ctn,
    }
}

pub fn pn_or(pn: u16) -> u16 {
    match envy::from_env::<CtnPn>() {
        Ok(config) => {
            debug!("From env: pn {}", config.pn);
            config.pn
        }
        _ => pn,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn ctn_or_return_env_value_if_pn_is_set_too() {
        env::set_var("K2_CTN", "2");
        env::set_var("K2_PN", "4");
        let ctn = ctn_or(1);

        assert_eq!(ctn, 2);
    }

    #[test]
    fn ctn_or_return_given_value_if_pn_not_set() {
        env::remove_var("K2_CTN");
        env::remove_var("K2_PN");
        let ctn = ctn_or(1);

        assert_eq!(ctn, 1);

        env::set_var("K2_CTN", "2");
        let ctn = ctn_or(1);

        assert_eq!(ctn, 1);
    }

    #[test]
    fn pn_or_return_env_value_if_ctn_is_set_too() {
        env::set_var("K2_CTN", "2");
        env::set_var("K2_PN", "4");
        let pn = pn_or(1);

        assert_eq!(pn, 4);
    }

    #[test]
    fn pn_or_return_given_value_if_ctn_not_set() {
        env::remove_var("K2_CTN");
        env::remove_var("K2_PN");
        let pn = pn_or(1);

        assert_eq!(pn, 1);

        env::set_var("K2_PN", "2");
        let pn = pn_or(1);

        assert_eq!(pn, 1);
    }
}
