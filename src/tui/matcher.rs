#![allow(dead_code)]

use std::ops::{Deref, DerefMut};

use nucleo::{Injector, Nucleo};

use crate::links::Link;

pub struct Matcher {
    pub matcher: Nucleo<Link>,
    pub matcher_config: nucleo::Config,
    pub injector: Injector<Link>,
}

impl Deref for Matcher {
    type Target = Nucleo<Link>;

    fn deref(&self) -> &Self::Target {
        &self.matcher
    }
}

impl DerefMut for Matcher {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.matcher
    }
}
