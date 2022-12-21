use rustler::Env;
use rustler::Binary;
use rustler::NewBinary;

use resvg::usvg;
use resvg::tiny_skia;
use resvg::usvg_text_layout::{fontdb, TreeTextToPath};

#[rustler::nif]
fn render_svg_to_png<'a>(env: Env<'a>, data: Binary) -> Binary<'a> {
    let svg_data = data.as_slice();

    let mut fontdb = fontdb::Database::new();
    fontdb.load_system_fonts();

    let opt = usvg::Options::default();
    let mut tree = usvg::Tree::from_data(&svg_data, &opt).unwrap();
    tree.convert_text(&fontdb, opt.keep_named_groups);

    let pixmap_size = tree.size.to_screen_size();

    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    resvg::render(
        &tree,
        usvg::FitTo::Original,
        tiny_skia::Transform::default(),
        pixmap.as_mut(),
    )
    .unwrap();

    let png_data = pixmap.encode_png().unwrap();

    let mut erl_bin: NewBinary = NewBinary::new(env, png_data.len());
    erl_bin.as_mut_slice().copy_from_slice(&png_data);
    erl_bin.into()
}

rustler::init!("Elixir.Resvg", [render_svg_to_png]);
