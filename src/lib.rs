/*!
`fontdb` is a simple, in-memory font database with CSS-like queries.

# Features

- The database can load fonts from files, directories and raw data (`Vec<u8>`).
- The database can match a font using CSS-like queries. See `Database::query`.

# Non-goals

- System fonts loading.<br>
  This library is intentionally doesn't load system fonts.
  This is a very complex feature and should be handled by the caller or other libraries.

- Font properties querying.<br>
  The database provides only storage and matching capabilities.
  For font properties querying you can use [ttf-parser].

- A font fallback mechanism.<br>
  This library can be used to implement a font fallback mechanism, but it doesn't implement it.

- Application's global database.<br>
  The database doesn't use `static`, therefore it's up to the caller where it should be stored.

- Font types support other than TrueType.

# Font vs Face

A font is a collection of font faces. Therefore, a font face is a subset of a font.
A simple font (\*.ttf/\*.otf) usually contains a single font face,
but a font collection (\*.ttc) can contain multiple font faces.

`fontdb` stores and matches font faces, not fonts.
Therefore, after loading a font collection with 5 faces (for example), the database will be populated
with 5 `FaceInfo` objects, all of which will be pointing to the same file or binary data.

# Performance

The database performance is largely limited by the storage itself.
We are using [ttf-parser], so the parsing should not be a bottleneck.

On my machine with Samsung SSD 860 and Gentoo Linux, it takes ~20ms
to load 1906 font faces (most of them are from Google Noto collection)
with a hot disk cache and ~860ms with a cold one.

# Safety

The library relies on memory-mapped files, which is inherently unsafe.
But we do not keep such files open forever. Instead, we are memory-mapping files only when needed.

[ttf-parser]: https://github.com/RazrFalcon/ttf-parser
*/

#![doc(html_root_url = "https://docs.rs/fontdb/0.1.0")]

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]

use std::path::{Path, PathBuf};
use std::rc::Rc;

use uuid::Uuid;
use log::warn;
pub use ttf_parser::Width as Stretch;


/// An unique font ID.
///
/// Stored as UUIDv4 internally.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd, Debug)]
pub struct ID(Uuid);


/// A list of possible font loading errors.
#[derive(Debug)]
pub enum LoadError {
    /// A malformed font.
    ///
    /// Typically means that [ttf-parser](https://github.com/RazrFalcon/ttf-parser)
    /// wasn't able to parse it.
    MalformedFont,
    /// A valid TrueType font without a valid *Family Name*.
    UnnamedFont,
    /// A file IO related error.
    IoError(std::io::Error),
}

impl From<std::io::Error> for LoadError {
    #[inline]
    fn from(e: std::io::Error) -> Self {
        LoadError::IoError(e)
    }
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadError::MalformedFont => write!(f, "malformed font"),
            LoadError::UnnamedFont => write!(f, "font doesn't have a family name"),
            LoadError::IoError(ref e) => write!(f, "{}", e),
        }
    }
}


/// A font database.
#[allow(missing_debug_implementations)]
pub struct Database {
    faces: Vec<FaceInfo>,
    family_serif: Option<String>,
    family_sans_serif: Option<String>,
    family_cursive: Option<String>,
    family_fantasy: Option<String>,
    family_monospace: Option<String>,
}

impl Database {
    /// Create a new, empty `Database`.
    #[inline]
    pub fn new() -> Self {
        Database {
            faces: Vec::new(),
            family_serif: None,
            family_sans_serif: None,
            family_cursive: None,
            family_fantasy: None,
            family_monospace: None,
        }
    }

    /// Loads a font data into the `Database`.
    ///
    /// Will load all font faces in case of a font collection.
    pub fn load_font_data(&mut self, data: Vec<u8>) -> Result<(), LoadError> {
        let source = Rc::new(Source::Binary(data));

        // Borrow `source` data.
        let data = match &*source {
            Source::Binary(ref data) => data,
            Source::File(_) => unreachable!(),
        };

        let n = ttf_parser::fonts_in_collection(&data).unwrap_or(1);
        for index in 0..n {
            let info = parse_face_info(source.clone(), &data, index)?;
            self.faces.push(info);
        }

        Ok(())
    }

    /// Loads a font file into the `Database`.
    ///
    /// Will load all font faces in case of a font collection.
    pub fn load_font_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(), LoadError> {
        let source = Rc::new(Source::File(path.as_ref().into()));

        let file = std::fs::File::open(path)?;
        let data = unsafe { memmap2::MmapOptions::new().map(&file)? };

        let n = ttf_parser::fonts_in_collection(&data).unwrap_or(1);
        for index in 0..n {
            let info = parse_face_info(source.clone(), &data, index)?;
            self.faces.push(info);
        }

        Ok(())
    }

    /// Loads font files from the selected directory into the `Database`.
    ///
    /// This method will scan directories recursively.
    ///
    /// Will load `ttf`, `otf`, `ttc` and `otc` fonts.
    ///
    /// Unlike other `load_*` methods, this one doesn't return an error.
    /// It will simply skip malformed fonts and will print a warning into the log for each of them.
    pub fn load_fonts_dir<P: AsRef<Path>>(&mut self, dir: P) {
        let fonts_dir = match std::fs::read_dir(dir.as_ref()) {
            Ok(dir) => dir,
            Err(_) => return,
        };

        for entry in fonts_dir {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    match path.extension().and_then(|e| e.to_str()) {
                        Some("ttf") | Some("ttc") | Some("TTF") | Some("TTC") |
                        Some("otf") | Some("otc") | Some("OTF") | Some("OTC") => {
                            if let Err(e) = self.load_font_file(&path) {
                                warn!("Failed to load '{}' cause {}.", path.display(), e);
                            }
                        }
                        _ => {}
                    }
                } else if path.is_dir() {
                    // TODO: ignore symlinks?
                    self.load_fonts_dir(path);
                }
            }
        }
    }

    /// Removes a font face by `id` from the database.
    ///
    /// Returns `false` while attempting to remove a non-existing font face.
    ///
    /// Useful when you want to ignore some specific font face(s)
    /// after loading a large directory with fonts.
    /// Or a specific face from a font.
    pub fn remove_face(&mut self, id: ID) -> bool {
        match self.faces.iter().position(|item| item.id == id) {
            Some(idx) => {
                self.faces.remove(idx);
                true
            }
            None => false,
        }
    }

    /// Returns `true` if the `Database` contains no font faces.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.faces.is_empty()
    }

    /// Returns the number of font faces in the `Database`.
    ///
    /// Note that `Database` stores font faces, not fonts.
    /// For example, if a caller will try to load a font collection (`*.ttc`) that contains 5 faces,
    /// then the `Database` will load 5 font faces and this method will return 5, not 1.
    #[inline]
    pub fn len(&self) -> usize {
        self.faces.len()
    }

    /// Sets the family that will be used by `Family::Serif`.
    pub fn set_serif_family<S: Into<String>>(&mut self, family: S) {
        self.family_serif = Some(family.into());
    }

    /// Sets the family that will be used by `Family::SansSerif`.
    pub fn set_sans_serif_family<S: Into<String>>(&mut self, family: S) {
        self.family_sans_serif = Some(family.into());
    }

    /// Sets the family that will be used by `Family::Cursive`.
    pub fn set_cursive_family<S: Into<String>>(&mut self, family: S) {
        self.family_cursive = Some(family.into());
    }

    /// Sets the family that will be used by `Family::Fantasy`.
    pub fn set_fantasy_family<S: Into<String>>(&mut self, family: S) {
        self.family_fantasy = Some(family.into());
    }

    /// Sets the family that will be used by `Family::Monospace`.
    pub fn set_monospace_family<S: Into<String>>(&mut self, family: S) {
        self.family_monospace = Some(family.into());
    }

    /// Performs a CSS-like query and returns the best matched font face.
    pub fn query(&self, query: &Query) -> Option<ID> {
        for family in query.families {
            let name = match self.family_name(family) {
                Some(name) => name,
                None => continue, // skip unset generic font families
            };

            let mut ids = Vec::new();
            let mut candidates = Vec::new();
            for item in self.faces.iter().filter(|font| &font.family == name) {
                ids.push(item.id);
                candidates.push(item.properties);
            }

            if !candidates.is_empty() {
                if let Some(index) = find_best_match(&candidates, query) {
                    return Some(ids[index]);
                }
            }
        }

        None
    }

    fn family_name<'a>(&'a self, family: &'a Family) -> Option<&'a str> {
        match family {
            Family::Name(ref name) => Some(name),
            Family::Serif => self.family_serif.as_deref(),
            Family::SansSerif => self.family_sans_serif.as_deref(),
            Family::Cursive => self.family_cursive.as_deref(),
            Family::Fantasy => self.family_fantasy.as_deref(),
            Family::Monospace => self.family_monospace.as_deref(),
        }
    }

    /// Returns a reference to an internal storage.
    ///
    /// This can be used for manual font matching.
    #[inline]
    pub fn faces(&self) -> &[FaceInfo] {
        &self.faces
    }

    /// Selects a `FaceInfo` by `id`.
    ///
    /// Returns `None` if a face with such ID was already removed,
    /// or this ID belong to the other `Database`.
    pub fn face(&self, id: ID) -> Option<&FaceInfo> {
        self.faces.iter().find(|item| item.id == id)
    }

    /// Returns font face storage and the face index by `ID`.
    pub fn face_source(&self, id: ID) -> Option<(Rc<Source>, u32)> {
        self.face(id).map(|info| (info.source.clone(), info.index))
    }

    /// Executes a closure with a font's data.
    ///
    /// We can't return a reference to a font binary data because of lifetimes.
    /// So instead, you can use this method to process font's data.
    ///
    /// The closure accepts raw font data and font face index.
    ///
    /// In case of `Source::File`, the font file will be memory mapped.
    ///
    /// # Example
    ///
    /// ```no_run
    /// let is_variable = db.with_font_data(id, |font_data, face_index| {
    ///     let font = ttf_parser::Font::from_data(font_data, face_index).unwrap();
    ///     font.is_variable()
    /// })?;
    /// ```
    pub fn with_face_data<P, T>(&self, id: ID, p: P) -> Option<T>
        where P: FnOnce(&[u8], u32) -> T
    {
        let (src, face_index) = self.face_source(id)?;
        match &*src {
            Source::File(ref path) => {
                let file = std::fs::File::open(path).ok()?;
                let mmap = unsafe { memmap2::MmapOptions::new().map(&file).ok()? };
                Some(p(&mmap, face_index))
            }
            Source::Binary(ref data) => {
                Some(p(data, face_index))
            }
        }
    }
}


/// A single font face info.
///
/// A font can have multiple faces.
///
/// A single item of the `Database`.
#[derive(Clone, Debug)]
pub struct FaceInfo {
    /// An unique ID.
    pub id: ID,

    /// A font source.
    ///
    /// We have to use `Rc`, because multiple `FaceInfo` objects can reference
    /// the same data in case of font collections.
    pub source: Rc<Source>,

    /// A face index in the `source`.
    pub index: u32,

    /// A family name.
    ///
    /// Corresponds to a *Typographic Family* or a *Family* in the TrueType font.
    pub family: String,

    /// Face properties.
    pub properties: FaceProperties,
}


/// Common face properties.
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct FaceProperties {
    /// A font style.
    pub style: Style,
    /// A font weight.
    pub weight: Weight,
    /// A font stretch.
    pub stretch: Stretch,
}


/// A font source.
///
/// Either a raw binary data or a file path.
///
/// Stores the whole font and not just a single face.
#[derive(Clone, Debug)]
pub enum Source {
    /// A font's raw data. Owned by the database.
    Binary(Vec<u8>),
    /// A font's path.
    File(PathBuf),
}


/// A database query.
///
/// Mainly used by `Database::query()`.
#[derive(Clone, Copy, Default, Debug)]
pub struct Query<'a> {
    /// A prioritized list of font family names or generic family names.
    ///
    /// [font-family](https://www.w3.org/TR/2018/REC-css-fonts-3-20180920/#propdef-font-family) in CSS.
    pub families: &'a [Family<'a>],

    /// Specifies the weight of glyphs in the font, their degree of blackness or stroke thickness.
    ///
    /// [font-weight](https://www.w3.org/TR/2018/REC-css-fonts-3-20180920/#font-weight-prop) in CSS.
    pub weight: Weight,

    /// Selects a normal, condensed, or expanded face from a font family.
    ///
    /// [font-stretch](https://www.w3.org/TR/2018/REC-css-fonts-3-20180920/#font-stretch-prop) in CSS.
    pub stretch: Stretch,

    /// Allows italic or oblique faces to be selected.
    ///
    /// [font-style](https://www.w3.org/TR/2018/REC-css-fonts-3-20180920/#font-style-prop) in CSS.
    pub style: Style,
}


// Enum value descriptions are from the CSS spec.
/// A [font family](https://www.w3.org/TR/2018/REC-css-fonts-3-20180920/#propdef-font-family).
#[derive(Clone, Copy, Debug)]
pub enum Family<'a> {
    /// The name of a font family of choice.
    Name(&'a str),

    /// Serif fonts represent the formal text style for a script.
    Serif,

    /// Glyphs in sans-serif fonts, as the term is used in CSS, are generally low contrast
    /// and have stroke endings that are plain — without any flaring, cross stroke,
    /// or other ornamentation.
    SansSerif,

    /// Glyphs in cursive fonts generally use a more informal script style,
    /// and the result looks more like handwritten pen or brush writing than printed letterwork.
    Cursive,

    /// Fantasy fonts are primarily decorative or expressive fonts that
    /// contain decorative or expressive representations of characters.
    Fantasy,

    /// The sole criterion of a monospace font is that all glyphs have the same fixed width.
    Monospace,
}


/// Specifies the weight of glyphs in the font, their degree of blackness or stroke thickness.
#[derive(Clone, Copy, PartialOrd, PartialEq, Debug)]
pub struct Weight(pub u16);

impl Default for Weight {
    #[inline]
    fn default() -> Weight {
        Weight::NORMAL
    }
}

impl Weight {
    /// Thin weight (100), the thinnest value.
    pub const THIN: Weight = Weight(100);
    /// Extra light weight (200).
    pub const EXTRA_LIGHT: Weight = Weight(200);
    /// Light weight (300).
    pub const LIGHT: Weight = Weight(300);
    /// Normal (400).
    pub const NORMAL: Weight = Weight(400);
    /// Medium weight (500, higher than normal).
    pub const MEDIUM: Weight = Weight(500);
    /// Semibold weight (600).
    pub const SEMIBOLD: Weight = Weight(600);
    /// Bold weight (700).
    pub const BOLD: Weight = Weight(700);
    /// Extra-bold weight (800).
    pub const EXTRA_BOLD: Weight = Weight(800);
    /// Black weight (900), the thickest value.
    pub const BLACK: Weight = Weight(900);
}


/// Allows italic or oblique faces to be selected.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Style {
    /// A face that is neither italic not obliqued.
    Normal,
    /// A form that is generally cursive in nature.
    Italic,
    /// A typically-sloped version of the regular face.
    Oblique,
}

impl Default for Style {
    fn default() -> Style {
        Style::Normal
    }
}


fn parse_face_info(
    source: Rc<Source>,
    data: &[u8],
    index: u32,
) -> Result<FaceInfo, LoadError> {
    let font = ttf_parser::Font::from_data(data, index).ok_or(LoadError::MalformedFont)?;

    let family = font.family_name().ok_or(LoadError::UnnamedFont)?;

    let style = if font.is_italic() {
        Style::Italic
    } else if font.is_oblique() {
        Style::Oblique
    } else {
        Style::Normal
    };

    let weight = Weight(font.weight().to_number());
    let stretch = font.width();

    let properties = FaceProperties { style, weight, stretch };

    Ok(FaceInfo {
        id: ID(Uuid::new_v4().unwrap()),
        source,
        index,
        family,
        properties,
    })
}

// https://www.w3.org/TR/2018/REC-css-fonts-3-20180920/#font-style-matching
// Based on https://github.com/servo/font-kit
#[inline(never)]
fn find_best_match(candidates: &[FaceProperties], query: &Query) -> Option<usize> {
    debug_assert!(!candidates.is_empty());

    // Step 4.
    let mut matching_set: Vec<usize> = (0..candidates.len()).collect();

    // Step 4a (`font-stretch`).
    let matches = matching_set.iter().any(|&index| candidates[index].stretch == query.stretch);
    let matching_stretch = if matches {
        // Exact match.
        query.stretch
    } else if query.stretch <= Stretch::Normal {
        // Closest stretch, first checking narrower values and then wider values.
        let stretch = matching_set
            .iter()
            .filter(|&&index| candidates[index].stretch < query.stretch)
            .min_by_key(|&&index| {
                query.stretch.to_number() - candidates[index].stretch.to_number()
            });

        match stretch {
            Some(&matching_index) => candidates[matching_index].stretch,
            None => {
                let matching_index = *matching_set
                    .iter()
                    .min_by_key(|&&index| {
                        candidates[index].stretch.to_number() - query.stretch.to_number()
                    })?;

                candidates[matching_index].stretch
            }
        }
    } else {
        // Closest stretch, first checking wider values and then narrower values.
        let stretch = matching_set
            .iter()
            .filter(|&&index| candidates[index].stretch > query.stretch)
            .min_by_key(|&&index| {
                candidates[index].stretch.to_number() - query.stretch.to_number()
            });

        match stretch {
            Some(&matching_index) => candidates[matching_index].stretch,
            None => {
                let matching_index = *matching_set
                    .iter()
                    .min_by_key(|&&index| {
                        query.stretch.to_number() - candidates[index].stretch.to_number()
                    })?;

                candidates[matching_index].stretch
            }
        }
    };
    matching_set.retain(|&index| candidates[index].stretch == matching_stretch);

    // Step 4b (`font-style`).
    let style_preference = match query.style {
        Style::Italic => [Style::Italic, Style::Oblique, Style::Normal],
        Style::Oblique => [Style::Oblique, Style::Italic, Style::Normal],
        Style::Normal => [Style::Normal, Style::Oblique, Style::Italic],
    };
    let matching_style = *style_preference
        .iter()
        .filter(|&query_style| {
            matching_set
                .iter()
                .any(|&index| candidates[index].style == *query_style)
        })
        .next()?;

    matching_set.retain(|&index| candidates[index].style == matching_style);

    // Step 4c (`font-weight`).
    //
    // The spec doesn't say what to do if the weight is between 400 and 500 exclusive, so we
    // just use 450 as the cutoff.
    let weight = query.weight.0;
    let matches = weight >= 400
        && weight < 450
        && matching_set
        .iter()
        .any(|&index| candidates[index].weight.0 == 500);

    let matching_weight = if matches {
        // Check 500 first.
        Weight::MEDIUM
    } else if weight >= 450 && weight <= 500 &&
        matching_set.iter().any(|&index| candidates[index].weight.0 == 400)
    {
        // Check 400 first.
        Weight::NORMAL
    } else if weight <= 500 {
        // Closest weight, first checking thinner values and then fatter ones.
        let idx = matching_set
            .iter()
            .filter(|&&index| candidates[index].weight.0 <= weight)
            .min_by_key(|&&index| weight - candidates[index].weight.0);

        match idx {
            Some(&matching_index) => candidates[matching_index].weight,
            None => {
                let matching_index = *matching_set
                    .iter()
                    .min_by_key(|&&index| candidates[index].weight.0 - weight)?;
                candidates[matching_index].weight
            }
        }
    } else {
        // Closest weight, first checking fatter values and then thinner ones.
        let idx = matching_set
            .iter()
            .filter(|&&index| candidates[index].weight.0 >= weight)
            .min_by_key(|&&index| candidates[index].weight.0 - weight);

        match idx {
            Some(&matching_index) => candidates[matching_index].weight,
            None => {
                let matching_index = *matching_set
                    .iter()
                    .min_by_key(|&&index| weight - candidates[index].weight.0)?;
                candidates[matching_index].weight
            }
        }
    };
    matching_set.retain(|&index| candidates[index].weight == matching_weight);

    // Ignore step 4d (`font-size`).

    // Return the result.
    matching_set.into_iter().next()
}
