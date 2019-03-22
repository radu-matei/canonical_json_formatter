extern crate itoa;

#[cfg_attr(macro_use, test)]
extern crate serde;
#[cfg_attr(macro_use, test)]
extern crate serde_derive;
#[cfg_attr(macro_use, test)]
extern crate serde_json;

extern crate spectral;
extern crate tempfile;
#[cfg(test)]
mod tests;

pub mod formatter;
