//! # amethyst_assets
//!
//! Asset management crate.
//! Designed with the following goals in mind:
//!
//! * extensibility
//! * asynchronous & parallel using rayon
//! * allow different sources

#![warn(missing_docs)]

extern crate amethyst_assets;
extern crate amethyst_core;
#[macro_use]
extern crate log;
extern crate fluent;

#[macro_use]
#[cfg(feature = "profiler")]
extern crate thread_profiler;

#[cfg(feature = "profiler")]
use thread_profiler::{register_thread_with_profiler, write_profile};

use amethyst_assets::{Asset, Handle, Result, SimpleFormat};
use amethyst_core::specs::VecStorage;
use fluent::MessageContext;

/// Loads the strings from localisation files.
#[derive(Clone)]
pub struct LocaleFormat;

impl SimpleFormat<Locale> for LocaleFormat {
    const NAME: &'static str = "FTL";

    type Options = ();

    fn import(&self, bytes: Vec<u8>, _: ()) -> Result<Locale> {
        let s = String::from_utf8(bytes)?;
        let mut ctx = MessageContext::new(&[]);
        ctx.add_messages(&s);
        Ok(Locale { context: ctx })
    }
}

impl Into<Result<Locale>> for Locale {
    fn into(self) -> Result<Locale> {
        Ok(self)
    }
}

/// A handle to a locale.
pub type LocaleHandle = Handle<Locale>;

/// A loaded locale.
//#[derive(Clone)]
pub struct Locale {
    /// The message context.
    pub context: MessageContext<'static>,
}

impl Asset for Locale {
    const NAME: &'static str = "locale::Locale";
    type Data = Locale;
    type HandleStorage = VecStorage<LocaleHandle>;
}
