// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;

use usvg_tree::{ImageRendering, ShapeRendering, Size, TextRendering};

use crate::ImageHrefResolver;

/// Processing options.
#[derive(Debug)]
pub struct Options {
    /// Target DPI.
    ///
    /// Impacts units conversion.
    ///
    /// Default: 96.0
    pub dpi: f32,

    /// A default font family.
    ///
    /// Will be used when no `font-family` attribute is set in the SVG.
    ///
    /// Default: Times New Roman
    pub font_family: String,

    /// A default font size.
    ///
    /// Will be used when no `font-size` attribute is set in the SVG.
    ///
    /// Default: 12
    pub font_size: f32,

    /// A list of languages.
    ///
    /// Will be used to resolve a `systemLanguage` conditional attribute.
    ///
    /// Format: en, en-US.
    ///
    /// Default: `[en]`
    pub languages: Vec<String>,

    /// Specifies the default shape rendering method.
    ///
    /// Will be used when an SVG element's `shape-rendering` property is set to `auto`.
    ///
    /// Default: GeometricPrecision
    pub shape_rendering: ShapeRendering,

    /// Specifies the default text rendering method.
    ///
    /// Will be used when an SVG element's `text-rendering` property is set to `auto`.
    ///
    /// Default: OptimizeLegibility
    pub text_rendering: TextRendering,

    /// Specifies the default image rendering method.
    ///
    /// Will be used when an SVG element's `image-rendering` property is set to `auto`.
    ///
    /// Default: OptimizeQuality
    pub image_rendering: ImageRendering,

    /// Default viewport size to assume if there is no `viewBox` attribute and
    /// the `width` or `height` attributes are relative.
    ///
    /// Default: `(100, 100)`
    pub default_size: Size,

    /// Specifies the way `xlink:href` in `<image>` elements should be handled.
    ///
    /// Default: see type's documentation for details
    pub image_href_resolver: ImageHrefResolver,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            dpi: 96.0,
            // Default font is user-agent dependent so we can use whichever we like.
            font_family: "Times New Roman".to_owned(),
            font_size: 12.0,
            languages: vec!["en".to_string()],
            shape_rendering: ShapeRendering::default(),
            text_rendering: TextRendering::default(),
            image_rendering: ImageRendering::default(),
            default_size: Size::from_wh(100.0, 100.0).unwrap(),
            image_href_resolver: ImageHrefResolver::default(),
        }
    }
}
