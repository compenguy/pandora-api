/*!
Low-level bindings to the (unofficial) Pandora web api.

The implementation is based on (and this documentation derives heavily from)
the [Unofficial Pandora API documentation](https://6xq.net/pandora-apidoc/),
used with permission under the [WTFPL](https://github.com/PromyLOPh/pandora-apidoc/blob/master/LICENSE).

Not all possible messages are supported.  This crate does export stub functions
for unsupported message types that are known, but they call `unimplemented!()`,
and are clearly indicated in the API documentation.

*/
// SPDX-License-Identifier: MIT AND WTFPL

#![deny(missing_docs)]
pub mod errors;
pub mod json;
// TODO: add REST support
// https://6xq.net/pandora-apidoc/rest/
//pub mod rest;
