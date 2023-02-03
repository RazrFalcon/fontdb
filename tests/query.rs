use std::path::PathBuf;
use fontdb::{Database, FaceInfo, Family, ID, Language, Query, Source, Style, Weight};
use test_case::test_case;
use ttf_parser::Width;

// Tests for exact matches
#[test_case(Query{families: &[Family::Name("Foo Sans")], weight: Weight::NORMAL, stretch: Width::Normal, style: Style::Normal}, Some("FooSans-Regular") ; "exact_match_10")]
#[test_case(Query{families: &[Family::Name("Foo Sans")], weight: Weight::LIGHT, stretch: Width::Normal, style: Style::Normal}, Some("FooSans-Light") ; "exact_match_11")]
#[test_case(Query{families: &[Family::Name("Foo Sans")], weight: Weight::BOLD, stretch: Width::Normal, style: Style::Normal}, Some("FooSans-Bold") ; "exact_match_12")]

#[test_case(Query{families: &[Family::Name("Foo Sans")], weight: Weight::NORMAL, stretch: Width::Condensed, style: Style::Normal}, Some("FooSansCondensed-Regular") ; "exact_match_20")]
#[test_case(Query{families: &[Family::Name("Foo Sans")], weight: Weight::LIGHT, stretch: Width::Condensed, style: Style::Normal}, Some("FooSansCondensed-Light") ; "exact_match_21")]
#[test_case(Query{families: &[Family::Name("Foo Sans")], weight: Weight::BOLD, stretch: Width::Condensed, style: Style::Normal}, Some("FooSansCondensed-Bold") ; "exact_match_22")]

#[test_case(Query{families: &[Family::Name("Foo Sans")], weight: Weight::NORMAL, stretch: Width::Normal, style: Style::Italic}, Some("FooSans-RegularItalic") ; "exact_match_30")]
#[test_case(Query{families: &[Family::Name("Foo Sans")], weight: Weight::LIGHT, stretch: Width::Normal, style: Style::Italic}, Some("FooSans-LightItalic") ; "exact_match_31")]
#[test_case(Query{families: &[Family::Name("Foo Sans")], weight: Weight::BOLD, stretch: Width::Normal, style: Style::Italic}, Some("FooSans-BoldItalic") ; "exact_match_32")]

#[test_case(Query{families: &[Family::Name("Foo Sans")], weight: Weight::NORMAL, stretch: Width::Condensed, style: Style::Italic}, Some("FooSansCondensed-RegularItalic") ; "exact_match_40")]
#[test_case(Query{families: &[Family::Name("Foo Sans")], weight: Weight::LIGHT, stretch: Width::Condensed, style: Style::Italic}, Some("FooSansCondensed-LightItalic") ; "exact_match_41")]
#[test_case(Query{families: &[Family::Name("Foo Sans")], weight: Weight::BOLD, stretch: Width::Condensed, style: Style::Italic}, Some("FooSansCondensed-BoldItalic") ; "exact_match_42")]

// Tests for case-insensitive and/or localized matching of font family names
#[test_case(Query{families: &[Family::Name("foo SANS")], weight: Weight::BOLD, stretch: Width::Normal, style: Style::Normal}, Some("FooSans-Bold") ; "ci_l_family_name_10")]
#[test_case(Query{families: &[Family::Name("FOO sans")], weight: Weight::BOLD, stretch: Width::Normal, style: Style::Normal}, Some("FooSans-Bold") ; "ci_l_family_name_11")]

#[test_case(Query{families: &[Family::Name("Föö Sans")], weight: Weight::BOLD, stretch: Width::Normal, style: Style::Normal}, Some("FooSans-Bold") ; "ci_l_family_name_20")]
#[test_case(Query{families: &[Family::Name("FÖÖ Sans")], weight: Weight::BOLD, stretch: Width::Normal, style: Style::Normal}, Some("FooSans-Bold") ; "ci_l_family_name_21")]

#[test_case(Query{families: &[Family::Name("Maßanzug Serif")], weight: Weight::BOLD, stretch: Width::Normal, style: Style::Normal}, Some("MassanzugSerif-Bold") ; "ci_l_family_name_30")]
#[test_case(Query{families: &[Family::Name("massanzug serif")], weight: Weight::BOLD, stretch: Width::Normal, style: Style::Normal}, Some("MassanzugSerif-Bold") ; "ci_l_family_name_31")]

// Tests for substitution of generic font families
#[test_case(Query{families: &[Family::Serif], weight: Weight::BOLD, stretch: Width::Normal, style: Style::Normal}, Some("GenericSerif-Bold") ; "generic_10")]
#[test_case(Query{families: &[Family::SansSerif], weight: Weight::BOLD, stretch: Width::Normal, style: Style::Normal}, Some("GenericSans-Bold") ; "generic_11")]
#[test_case(Query{families: &[Family::Cursive], weight: Weight::BOLD, stretch: Width::Normal, style: Style::Normal}, Some("GenericCursive-Bold") ; "generic_12")]
#[test_case(Query{families: &[Family::Fantasy], weight: Weight::BOLD, stretch: Width::Normal, style: Style::Normal}, Some("GenericFantasy-Bold") ; "generic_13")]
#[test_case(Query{families: &[Family::Monospace], weight: Weight::BOLD, stretch: Width::Normal, style: Style::Normal}, Some("GenericMonospace-Bold") ; "generic_14")]

// Tests for matching priority
#[test_case(Query{families: &[Family::Name("Primus Serif")], weight: Weight::SEMIBOLD, stretch: Width::Condensed, style: Style::Italic}, Some("PrimusSerifCondensed-SemiBold") ; "priority_10")] // stretch has priority over style
#[test_case(Query{families: &[Family::Name("Primus Serif")], weight: Weight::BOLD, stretch: Width::Normal, style: Style::Italic}, Some("PrimusSerif-SemiBoldItalic") ; "priority_11")] // style has priority over weight
#[test_case(Query{families: &[Family::Name("Primus Serif")], weight: Weight::SEMIBOLD, stretch: Width::Expanded, style: Style::Italic}, Some("PrimusSerifExpanded-Bold") ; "priority_12")] // stretch has priority over style and weight

// Tests for unsuccessful matching
#[test_case(Query{families: &[Family::Name("Doesntexist Serif")], weight: Weight::BOLD, stretch: Width::Normal, style: Style::Normal}, None ; "unsuccessful_10")]
#[test_case(Query{families: &[], weight: Weight::BOLD, stretch: Width::Normal, style: Style::Normal}, None ; "unsuccessful_11")]

// Tests for font family name fallback
#[test_case(Query{families: &[Family::Name("Doesntexist Sans"), Family::Name("Foo Sans"), Family::Name("Foobar Sans")], weight: Weight::BOLD, stretch: Width::Normal, style: Style::Normal}, Some("FooSans-Bold") ; "fallback_family_name_10")]
#[test_case(Query{families: &[Family::Name("Doesntexist Sans"), Family::SansSerif, Family::Name("Foobar Sans")], weight: Weight::BOLD, stretch: Width::Normal, style: Style::Normal}, Some("GenericSans-Bold") ; "fallback_family_name_11")] // fallback to generic font

// Tests for font stretch fallback
#[test_case(Query{families: &[Family::Name("FBStretchOne")], weight: Weight::BOLD, stretch: Width::Normal, style: Style::Normal}, Some("FBStretchOneSemiCondensed-Bold") ; "fallback_stretch_10")] // fallback to nearestmost condensed face, if normal stretch is requested but no normal face available
#[test_case(Query{families: &[Family::Name("FBStretchTwo")], weight: Weight::BOLD, stretch: Width::Normal, style: Style::Normal}, Some("FBStretchTwoSemiExpanded-Bold") ; "fallback_stretch_11")] // fallback to nearestmost expanded face, if normal stretch is requested but no normal or condensed faces available

#[test_case(Query{families: &[Family::Name("FBStretchThree")], weight: Weight::BOLD, stretch: Width::Condensed, style: Style::Normal}, Some("FBStretchThreeUltraCondensed-Bold") ; "fallback_stretch_21")] // fallback to next narrowest face, if condensed stretch is requested but not available
#[test_case(Query{families: &[Family::Name("FBStretchOne")], weight: Weight::BOLD, stretch: Width::ExtraCondensed, style: Style::Normal}, Some("FBStretchOneCondensed-Bold") ; "fallback_stretch_22")] // fallback to nearestmost face, if condensed stretch is requested but unavailable and no narrower faces are available
#[test_case(Query{families: &[Family::Name("FBStretchFive")], weight: Weight::BOLD, stretch: Width::Condensed, style: Style::Normal}, Some("FBStretchFive-Bold") ; "fallback_stretch_23")] // fallback to normal face if condensed stretch is requested but no condensed faces are available
#[test_case(Query{families: &[Family::Name("FBStretchTwo")], weight: Weight::BOLD, stretch: Width::Condensed, style: Style::Normal}, Some("FBStretchTwoSemiExpanded-Bold") ; "fallback_stretch_24")] // fallback to expanded face if condensed stretch is requested but no condensed or normal faces are available

#[test_case(Query{families: &[Family::Name("FBStretchFour")], weight: Weight::BOLD, stretch: Width::Expanded, style: Style::Normal}, Some("FBStretchFourUltraExpanded-Bold") ; "fallback_stretch_31")] // fallback to next widest face, if expanded stretch is requested but not available
#[test_case(Query{families: &[Family::Name("FBStretchOne")], weight: Weight::BOLD, stretch: Width::ExtraExpanded, style: Style::Normal}, Some("FBStretchOneExpanded-Bold") ; "fallback_stretch_32")] // fallback to nearestmost face, if expanded stretch is requested but unavailable and no wider faces are available
#[test_case(Query{families: &[Family::Name("FBStretchSix")], weight: Weight::BOLD, stretch: Width::Expanded, style: Style::Normal}, Some("FBStretchSix-Bold") ; "fallback_stretch_33")] // fallback to normal face if expanded stretch is requested but no expanded faces are available
#[test_case(Query{families: &[Family::Name("FBStretchSeven")], weight: Weight::BOLD, stretch: Width::Expanded, style: Style::Normal}, Some("FBStretchSevenSemiCondensed-Bold") ; "fallback_stretch_34")] // fallback to condensed face if expanded stretch is requested but no expanded or normal faces are available

// Tests for font style fallback
#[test_case(Query{families: &[Family::Name("FBStyleOne")], weight: Weight::BOLD, stretch: Width::Normal, style: Style::Italic}, Some("FBStyleOne-BoldOblique") ; "fallback_style_10")] // fallback to oblique face, if no italic face can be found
#[test_case(Query{families: &[Family::Name("FBStyleTwo")], weight: Weight::BOLD, stretch: Width::Normal, style: Style::Italic}, Some("FBStyleTwo-Bold") ; "fallback_style_11")] // fallback to normal face, if no italic or oblique face can be found

#[test_case(Query{families: &[Family::Name("FBStyleThree")], weight: Weight::BOLD, stretch: Width::Normal, style: Style::Oblique}, Some("FBStyleThree-BoldItalic") ; "fallback_style_20")] // fallback to italic face, if no oblique face can be found
#[test_case(Query{families: &[Family::Name("FBStyleTwo")], weight: Weight::BOLD, stretch: Width::Normal, style: Style::Oblique}, Some("FBStyleTwo-Bold") ; "fallback_style_21")] // fallback to normal face, if no oblique or italic face can be found

#[test_case(Query{families: &[Family::Name("FBStyleFour")], weight: Weight::BOLD, stretch: Width::Normal, style: Style::Normal}, Some("FBStyleFour-BoldOblique") ; "fallback_style_30")] // fallback to oblique face, if no normal face can be found
#[test_case(Query{families: &[Family::Name("FBStyleFive")], weight: Weight::BOLD, stretch: Width::Normal, style: Style::Normal}, Some("FBStyleFive-BoldItalic") ; "fallback_style_31")] // fallback to italic face, if no normal or oblique face can be found

// Tests for font weight fallback
#[test_case(Query{families: &[Family::Name("FBWeightOne")], weight: Weight::NORMAL, stretch: Width::Normal, style: Style::Normal}, Some("FBWeightOne-Medium") ; "fallback_weight_10")] // fallback to weight 500, if weight 400 was requested but not found
#[test_case(Query{families: &[Family::Name("FBWeightTwo")], weight: Weight::NORMAL, stretch: Width::Normal, style: Style::Normal}, Some("FBWeightTwo-ExtraLight") ; "fallback_weight_11")] // fallback to next thinnest weight, if weight 400 was requested but neither it nor 500 found
#[test_case(Query{families: &[Family::Name("FBWeightThree")], weight: Weight::NORMAL, stretch: Width::Normal, style: Style::Normal}, Some("FBWeightThree-Bold") ; "fallback_weight_12")] // fallback to next thickest weight, if weight 400 was requested but neither it nor 500 nor thinner weights found

#[test_case(Query{families: &[Family::Name("FBWeightFour")], weight: Weight::MEDIUM, stretch: Width::Normal, style: Style::Normal}, Some("FBWeightFour-Regular") ; "fallback_weight_20")] // fallback to weight 400, if weight 500 was requested but not found
#[test_case(Query{families: &[Family::Name("FBWeightTwo")], weight: Weight::MEDIUM, stretch: Width::Normal, style: Style::Normal}, Some("FBWeightTwo-ExtraLight") ; "fallback_weight_21")] // fallback to next thinnest weight, if weight 500 was requested but neither it nor 400 found
#[test_case(Query{families: &[Family::Name("FBWeightThree")], weight: Weight::MEDIUM, stretch: Width::Normal, style: Style::Normal}, Some("FBWeightThree-Bold") ; "fallback_weight_22")] // fallback to next thickest weight, if weight 500 was requested but neither it nor 400 nor thinner weights found

#[test_case(Query{families: &[Family::Name("Foobar Sans")], weight: Weight(399), stretch: Width::Normal, style: Style::Normal}, Some("FoobarSans-Light") ; "fallback_weight_30")] // fallback to next thinnest weight, if weight less than 400 was requested but not found
#[test_case(Query{families: &[Family::Name("Foobar Sans")], weight: Weight(199), stretch: Width::Normal, style: Style::Normal}, Some("FoobarSans-ExtraLight") ; "fallback_weight_31")] // fallback to next thickest weight, if weight less than 400 was requested but neither it nor thinner weights found

#[test_case(Query{families: &[Family::Name("Foobar Sans")], weight: Weight(501), stretch: Width::Normal, style: Style::Normal}, Some("FoobarSans-Bold") ; "fallback_weight_40")] // fallback to next thickest weight, if weight greater than 500 was requested but not found
#[test_case(Query{families: &[Family::Name("Foobar Sans")], weight: Weight(801), stretch: Width::Normal, style: Style::Normal}, Some("FoobarSans-ExtraBold") ; "fallback_weight_41")] // fallback to next thinnest weight, if weight greater than 500 was requested but neither it nor thicker weights found

#[test_case(Query{families: &[Family::Name("FBWeightFive")], weight: Weight(401), stretch: Width::Normal, style: Style::Normal}, Some("FBWeightFive-Medular") ; "fallback_weight_50")] // fallback to next thickest weight between 400 and 500 inclusive if weight between 400 and 500 exclusive was requested but not found
#[test_case(Query{families: &[Family::Name("FBWeightTwo")], weight: Weight(401), stretch: Width::Normal, style: Style::Normal}, Some("FBWeightTwo-ExtraLight") ; "fallback_weight_51")] // fallback to next thinnest weight below 400 if weight between 400 and 500 exclusive was requested but neither it nor weights between 400 and 500 inclusive were found
#[test_case(Query{families: &[Family::Name("FBWeightThree")], weight: Weight(401), stretch: Width::Normal, style: Style::Normal}, Some("FBWeightThree-Bold") ; "fallback_weight_52")] // fallback to next thickest weight above 500 if weight between 400 and 500 exclusive was requested but neither it nor weights below 500 were found
fn test_query(query: Query, expected_postscript_name: Option<&str>) {
    let database = build_test_database();

    let actual_postscript_name = database
        .query(&query)
        .map(|id| database.face(id).unwrap())
        .map(|face_info| face_info.post_script_name.as_str());

    assert_eq!(expected_postscript_name, actual_postscript_name);
}

fn build_test_database() -> Database {
    let dummy_source = Source::File(PathBuf::new());

    let build_face_info = |families: &[(String,Language)], style, stretch, post_script_name: &str, weight| -> FaceInfo {
        FaceInfo{
            id: ID::dummy(),
            source: dummy_source.clone(),
            index: 123,
            monospaced: false,
            families: Vec::from(families),
            post_script_name: post_script_name.to_string(),
            style,
            stretch,
            weight,
        }
    };

    let mut database = Database::new();


    // "Foo Sans", a pretty complete font without surprises.
    let families = [("Foo Sans".to_string(), Language::English_UnitedStates), ("Föö Sans".to_string(), Language::German_Germany)];

    database.push_face_info(build_face_info(&families, Style::Normal, Width::Normal, "FooSans-Thin", Weight::THIN));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Normal, "FooSans-ExtraLight", Weight::EXTRA_LIGHT));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Normal, "FooSans-Light", Weight::LIGHT));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Normal, "FooSans-Regular", Weight::NORMAL));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Normal, "FooSans-Medium", Weight::MEDIUM));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Normal, "FooSans-SemiBold", Weight::SEMIBOLD));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Normal, "FooSans-Bold", Weight::BOLD));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Normal, "FooSans-ExtraBold", Weight::EXTRA_BOLD));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Normal, "FooSans-Black", Weight::BLACK));

    database.push_face_info(build_face_info(&families, Style::Normal, Width::Condensed, "FooSansCondensed-Thin", Weight::THIN));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Condensed, "FooSansCondensed-ExtraLight", Weight::EXTRA_LIGHT));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Condensed, "FooSansCondensed-Light", Weight::LIGHT));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Condensed, "FooSansCondensed-Regular", Weight::NORMAL));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Condensed, "FooSansCondensed-Medium", Weight::MEDIUM));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Condensed, "FooSansCondensed-SemiBold", Weight::SEMIBOLD));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Condensed, "FooSansCondensed-Bold", Weight::BOLD));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Condensed, "FooSansCondensed-ExtraBold", Weight::EXTRA_BOLD));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Condensed, "FooSansCondensed-Black", Weight::BLACK));

    database.push_face_info(build_face_info(&families, Style::Normal, Width::Expanded, "FooSansExpanded-Thin", Weight::THIN));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Expanded, "FooSansExpanded-ExtraLight", Weight::EXTRA_LIGHT));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Expanded, "FooSansExpanded-Light", Weight::LIGHT));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Expanded, "FooSansExpanded-Regular", Weight::NORMAL));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Expanded, "FooSansExpanded-Medium", Weight::MEDIUM));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Expanded, "FooSansExpanded-SemiBold", Weight::SEMIBOLD));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Expanded, "FooSansExpanded-Bold", Weight::BOLD));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Expanded, "FooSansExpanded-ExtraBold", Weight::EXTRA_BOLD));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Expanded, "FooSansExpanded-Black", Weight::BLACK));

    database.push_face_info(build_face_info(&families, Style::Italic, Width::Normal, "FooSans-ThinItalic", Weight::THIN));
    database.push_face_info(build_face_info(&families, Style::Italic, Width::Normal, "FooSans-ExtraLightItalic", Weight::EXTRA_LIGHT));
    database.push_face_info(build_face_info(&families, Style::Italic, Width::Normal, "FooSans-LightItalic", Weight::LIGHT));
    database.push_face_info(build_face_info(&families, Style::Italic, Width::Normal, "FooSans-RegularItalic", Weight::NORMAL));
    database.push_face_info(build_face_info(&families, Style::Italic, Width::Normal, "FooSans-MediumItalic", Weight::MEDIUM));
    database.push_face_info(build_face_info(&families, Style::Italic, Width::Normal, "FooSans-SemiBoldItalic", Weight::SEMIBOLD));
    database.push_face_info(build_face_info(&families, Style::Italic, Width::Normal, "FooSans-BoldItalic", Weight::BOLD));
    database.push_face_info(build_face_info(&families, Style::Italic, Width::Normal, "FooSans-ExtraBoldItalic", Weight::EXTRA_BOLD));
    database.push_face_info(build_face_info(&families, Style::Italic, Width::Normal, "FooSans-BlackItalic", Weight::BLACK));

    database.push_face_info(build_face_info(&families, Style::Italic, Width::Condensed, "FooSansCondensed-ThinItalic", Weight::THIN));
    database.push_face_info(build_face_info(&families, Style::Italic, Width::Condensed, "FooSansCondensed-ExtraLightItalic", Weight::EXTRA_LIGHT));
    database.push_face_info(build_face_info(&families, Style::Italic, Width::Condensed, "FooSansCondensed-LightItalic", Weight::LIGHT));
    database.push_face_info(build_face_info(&families, Style::Italic, Width::Condensed, "FooSansCondensed-RegularItalic", Weight::NORMAL));
    database.push_face_info(build_face_info(&families, Style::Italic, Width::Condensed, "FooSansCondensed-MediumItalic", Weight::MEDIUM));
    database.push_face_info(build_face_info(&families, Style::Italic, Width::Condensed, "FooSansCondensed-SemiBoldItalic", Weight::SEMIBOLD));
    database.push_face_info(build_face_info(&families, Style::Italic, Width::Condensed, "FooSansCondensed-BoldItalic", Weight::BOLD));
    database.push_face_info(build_face_info(&families, Style::Italic, Width::Condensed, "FooSansCondensed-ExtraBoldItalic", Weight::EXTRA_BOLD));
    database.push_face_info(build_face_info(&families, Style::Italic, Width::Condensed, "FooSansCondensed-BlackItalic", Weight::BLACK));

    database.push_face_info(build_face_info(&families, Style::Italic, Width::Expanded, "FooSansExpanded-ThinItalic", Weight::THIN));
    database.push_face_info(build_face_info(&families, Style::Italic, Width::Expanded, "FooSansExpanded-ExtraLightItalic", Weight::EXTRA_LIGHT));
    database.push_face_info(build_face_info(&families, Style::Italic, Width::Expanded, "FooSansExpanded-LightItalic", Weight::LIGHT));
    database.push_face_info(build_face_info(&families, Style::Italic, Width::Expanded, "FooSansExpanded-RegularItalic", Weight::NORMAL));
    database.push_face_info(build_face_info(&families, Style::Italic, Width::Expanded, "FooSansExpanded-MediumItalic", Weight::MEDIUM));
    database.push_face_info(build_face_info(&families, Style::Italic, Width::Expanded, "FooSansExpanded-SemiBoldItalic", Weight::SEMIBOLD));
    database.push_face_info(build_face_info(&families, Style::Italic, Width::Expanded, "FooSansExpanded-BoldItalic", Weight::BOLD));
    database.push_face_info(build_face_info(&families, Style::Italic, Width::Expanded, "FooSansExpanded-ExtraBoldItalic", Weight::EXTRA_BOLD));
    database.push_face_info(build_face_info(&families, Style::Italic, Width::Expanded, "FooSansExpanded-BlackItalic", Weight::BLACK));


    // "Foobar Sans", a font with a less fine-grained set of weights.
    let families = [("Foobar Sans".to_string(), Language::English_UnitedStates), ("Fööbär Sans".to_string(), Language::German_Germany)];

    database.push_face_info(build_face_info(&families, Style::Normal, Width::Normal, "FoobarSans-ExtraLight", Weight::EXTRA_LIGHT));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Normal, "FoobarSans-Light", Weight::LIGHT));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Normal, "FoobarSans-Regular", Weight::NORMAL));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Normal, "FoobarSans-Medium", Weight::MEDIUM));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Normal, "FoobarSans-Bold", Weight::BOLD));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Normal, "FoobarSans-ExtraBold", Weight::EXTRA_BOLD));


    // Generic fonts, for substitution tests.
    let generic_serif_family = "Generic Serif";
    let generic_sans_serif_family = "Generic Sans";
    let generic_cursive_family = "Generic Cursive";
    let generic_fantasy_family = "Generic Fantasy";
    let generic_monospace_family = "Generic Monospace";
    database.push_face_info(build_face_info(&[(generic_serif_family.to_string(), Language::English_UnitedStates)], Style::Normal, Width::Normal, "GenericSerif-Bold", Weight::BOLD));
    database.push_face_info(build_face_info(&[(generic_sans_serif_family.to_string(), Language::English_UnitedStates)], Style::Normal, Width::Normal, "GenericSans-Bold", Weight::BOLD));
    database.push_face_info(build_face_info(&[(generic_cursive_family.to_string(), Language::English_UnitedStates)], Style::Normal, Width::Normal, "GenericCursive-Bold", Weight::BOLD));
    database.push_face_info(build_face_info(&[(generic_fantasy_family.to_string(), Language::English_UnitedStates)], Style::Normal, Width::Normal, "GenericFantasy-Bold", Weight::BOLD));

    let mut generic_monospace = build_face_info(&[(generic_monospace_family.to_string(), Language::English_UnitedStates)], Style::Normal, Width::Normal, "GenericMonospace-Bold", Weight::BOLD);
    generic_monospace.monospaced = true;
    database.push_face_info(generic_monospace);

    database.set_serif_family(generic_serif_family);
    database.set_sans_serif_family(generic_sans_serif_family);
    database.set_cursive_family(generic_cursive_family);
    database.set_fantasy_family(generic_fantasy_family);
    database.set_monospace_family(generic_monospace_family);


    // "Maßanzug Serif", a font to test the localized, folding, case-insensitive matching of font family names.
    database.push_face_info(build_face_info(&[("Maßanzug Serif".to_string(), Language::German_Germany)], Style::Normal, Width::Normal, "MassanzugSerif-Bold", Weight::BOLD));


    // "Primus Serif", a font for testing the priority of different matching criteria.
    let families = [("Primus Serif".to_string(), Language::English_UnitedStates)];

    database.push_face_info(build_face_info(&families, Style::Normal, Width::Normal, "PrimusSerif-Bold", Weight::BOLD));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Condensed, "PrimusSerifCondensed-SemiBold", Weight::SEMIBOLD));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Expanded, "PrimusSerifExpanded-Bold", Weight::BOLD));
    database.push_face_info(build_face_info(&families, Style::Italic, Width::Normal, "PrimusSerif-SemiBoldItalic", Weight::SEMIBOLD));


    // "FBStretchOne", a font for font stretch fallback testing
    let families = [("FBStretchOne".to_string(), Language::English_UnitedStates)];

    database.push_face_info(build_face_info(&families, Style::Normal, Width::Condensed, "FBStretchOneCondensed-Bold", Weight::BOLD));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::SemiCondensed, "FBStretchOneSemiCondensed-Bold", Weight::BOLD));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::SemiExpanded, "FBStretchOneSemiExpanded-Bold", Weight::BOLD));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Expanded, "FBStretchOneExpanded-Bold", Weight::BOLD));


    // "FBStretchTwo", a font for font stretch fallback testing
    let families = [("FBStretchTwo".to_string(), Language::English_UnitedStates)];

    database.push_face_info(build_face_info(&families, Style::Normal, Width::SemiExpanded, "FBStretchTwoSemiExpanded-Bold", Weight::BOLD));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Expanded, "FBStretchTwoExpanded-Bold", Weight::BOLD));


    // "FBStretchThree", a font for font stretch fallback testing
    let families = [("FBStretchThree".to_string(), Language::English_UnitedStates)];

    database.push_face_info(build_face_info(&families, Style::Normal, Width::UltraCondensed, "FBStretchThreeUltraCondensed-Bold", Weight::BOLD));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::SemiCondensed, "FBStretchThreeSemiCondensed-Bold", Weight::BOLD));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::SemiExpanded, "FBStretchThreeSemiExpanded-Bold", Weight::BOLD));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Expanded, "FBStretchFourExpanded-Bold", Weight::BOLD));


    // "FBStretchFour", a font for font stretch fallback testing
    let families = [("FBStretchFour".to_string(), Language::English_UnitedStates)];

    database.push_face_info(build_face_info(&families, Style::Normal, Width::UltraCondensed, "FBStretchFourUltraCondensed-Bold", Weight::BOLD));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::SemiCondensed, "FBStretchFourSemiCondensed-Bold", Weight::BOLD));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::SemiExpanded, "FBStretchFourSemiExpanded-Bold", Weight::BOLD));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::UltraExpanded, "FBStretchFourUltraExpanded-Bold", Weight::BOLD));


    // "FBStretchFive", a font for font stretch fallback testing
    let families = [("FBStretchFive".to_string(), Language::English_UnitedStates)];

    database.push_face_info(build_face_info(&families, Style::Normal, Width::Normal, "FBStretchFive-Bold", Weight::BOLD));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::SemiExpanded, "FBStretchFiveSemiExpanded-Bold", Weight::BOLD));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::UltraExpanded, "FBStretchFiveUltraExpanded-Bold", Weight::BOLD));


    // "FBStretchSix", a font for font stretch fallback testing
    let families = [("FBStretchSix".to_string(), Language::English_UnitedStates)];

    database.push_face_info(build_face_info(&families, Style::Normal, Width::Normal, "FBStretchSix-Bold", Weight::BOLD));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::SemiCondensed, "FBStretchSixSemiCondensed-Bold", Weight::BOLD));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::UltraCondensed, "FBStretchSixUltraCondensed-Bold", Weight::BOLD));


    // "FBStretchSeven", a font for font stretch fallback testing
    let families = [("FBStretchSeven".to_string(), Language::English_UnitedStates)];

    database.push_face_info(build_face_info(&families, Style::Normal, Width::SemiCondensed, "FBStretchSevenSemiCondensed-Bold", Weight::BOLD));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Condensed, "FBStretchSevenCondensed-Bold", Weight::BOLD));


    // "FBStyleOne", a font for font style fallback testing
    let families = [("FBStyleOne".to_string(), Language::English_UnitedStates)];

    database.push_face_info(build_face_info(&families, Style::Normal, Width::Normal, "FBStyleOne-Bold", Weight::BOLD));
    database.push_face_info(build_face_info(&families, Style::Oblique, Width::Normal, "FBStyleOne-BoldOblique", Weight::BOLD));


    // "FBStyleTwo", a font for font style fallback testing
    let families = [("FBStyleTwo".to_string(), Language::English_UnitedStates)];

    database.push_face_info(build_face_info(&families, Style::Normal, Width::Normal, "FBStyleTwo-Bold", Weight::BOLD));


    // "FBStyleThree", a font for font style fallback testing
    let families = [("FBStyleThree".to_string(), Language::English_UnitedStates)];

    database.push_face_info(build_face_info(&families, Style::Normal, Width::Normal, "FBStyleThree-Bold", Weight::BOLD));
    database.push_face_info(build_face_info(&families, Style::Italic, Width::Normal, "FBStyleThree-BoldItalic", Weight::BOLD));


    // "FBStyleFour", a font for font style fallback testing
    let families = [("FBStyleFour".to_string(), Language::English_UnitedStates)];

    database.push_face_info(build_face_info(&families, Style::Oblique, Width::Normal, "FBStyleFour-BoldOblique", Weight::BOLD));
    database.push_face_info(build_face_info(&families, Style::Italic, Width::Normal, "FBStyleFour-BoldItalic", Weight::BOLD));


    // "FBStyleFive", a font for font style fallback testing
    let families = [("FBStyleFive".to_string(), Language::English_UnitedStates)];

    database.push_face_info(build_face_info(&families, Style::Italic, Width::Normal, "FBStyleFive-BoldItalic", Weight::BOLD));


    // "FBWeightOne", a font for font weight fallback testing.
    let families = [("FBWeightOne".to_string(), Language::English_UnitedStates)];

    database.push_face_info(build_face_info(&families, Style::Normal, Width::Normal, "FBWeightOne-ExtraLight", Weight::EXTRA_LIGHT));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Normal, "FBWeightOne-Medium", Weight::MEDIUM));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Normal, "FBWeightOne-Bold", Weight::BOLD));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Normal, "FBWeightOne-Black", Weight::BLACK));


    // "FBWeightTwo", a font for font weight fallback testing.
    let families = [("FBWeightTwo".to_string(), Language::English_UnitedStates)];

    database.push_face_info(build_face_info(&families, Style::Normal, Width::Normal, "FBWeightTwo-ExtraLight", Weight::EXTRA_LIGHT));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Normal, "FBWeightTwo-Bold", Weight::BOLD));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Normal, "FBWeightTwo-Black", Weight::BLACK));


    // "FBWeightThree", a font for font weight fallback testing.
    let families = [("FBWeightThree".to_string(), Language::English_UnitedStates)];

    database.push_face_info(build_face_info(&families, Style::Normal, Width::Normal, "FBWeightThree-Bold", Weight::BOLD));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Normal, "FBWeightThree-Black", Weight::BLACK));


    // "FBWeightFour", a font for font weight fallback testing.
    let families = [("FBWeightFour".to_string(), Language::English_UnitedStates)];

    database.push_face_info(build_face_info(&families, Style::Normal, Width::Normal, "FBWeightFour-ExtraLight", Weight::EXTRA_LIGHT));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Normal, "FBWeightFour-Regular", Weight::NORMAL));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Normal, "FBWeightFour-Bold", Weight::BOLD));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Normal, "FBWeightFour-Black", Weight::BLACK));


    // "FBWeightFive", a font for font weight fallback testing.
    let families = [("FBWeightFive".to_string(), Language::English_UnitedStates)];

    database.push_face_info(build_face_info(&families, Style::Normal, Width::Normal, "FBWeightFive-ExtraLight", Weight::EXTRA_LIGHT));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Normal, "FBWeightFive-Regular", Weight::NORMAL));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Normal, "FBWeightFive-Medular", Weight(470)));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Normal, "FBWeightFive-Medium", Weight::MEDIUM));
    database.push_face_info(build_face_info(&families, Style::Normal, Width::Normal, "FBWeightFive-Bold", Weight::BOLD));


    database
}
