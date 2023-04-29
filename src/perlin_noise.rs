use noise::{Fbm, Perlin};
use noise::utils::{NoiseMapBuilder, PlaneMapBuilder};
use std::fs;
use std::path::Path;
use rand::{thread_rng, Rng};


pub fn make_floor() -> std::io::Result<()> {

    let mut rng = thread_rng();

    let fbm = Fbm::<Perlin>::new(0);

    PlaneMapBuilder::<_, 2>::new(&fbm)
        .set_size(1200, 800)
        .set_x_bounds(rng.gen_range(-10..0) as f64, rng.gen_range(0..10) as f64)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("testing.png");

    let src = Path::new("example_images");
    let dst = Path::new("assets");
    move_dir_recursive(src, dst);

    Ok(())

}

fn move_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    if src.is_dir() {
        fs::create_dir_all(dst)?;

        for entry in src.read_dir()? {
            let entry = entry?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());
            if src_path.is_dir() {
                move_dir_recursive(&src_path, &dst_path)?;
            } else {
                fs::rename(&src_path, &dst_path)?;
            }
        }
    } else {
        fs::rename(src, dst)?;
    }

    fs::remove_dir_all(src)?;
    Ok(())
}
