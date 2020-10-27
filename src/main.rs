use image::ImageBuffer;
use obj;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn obj_to_png(obj_path: &str) {
    let file_dir = Path::new(obj_path).parent().unwrap();
    let file_name = Path::new(obj_path)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let obj_verts = obj::Obj::load(obj_path).unwrap().data.position;
    let scale = get_scale(&obj_verts);
    let img_res = (obj_verts.len() as f32).sqrt().floor() as u32;
    let depth = (2 as usize).pow(16) as f32 - 1.0;
    let mut img = ImageBuffer::new(img_res, img_res);
    for (i, [x, y, z]) in obj_verts.iter().enumerate() {
        let pix_x = i as u32 % img_res;
        let pix_y = i as u32 / img_res;
        if pix_x < img_res && pix_y < img_res {
            let r = (depth * (x / scale + 0.5)) as u16;
            let g = (depth * (y / scale + 0.5)) as u16;
            let b = (depth * (z / scale + 0.5)) as u16;
            img.put_pixel(pix_x, pix_y, image::Rgb([r, g, b]));
        }
    }
    img.save(file_dir.join(file_name + ".png")).unwrap();
    println!("Converted to PNG");
}

fn get_scale(verts: &Vec<[f32; 3]>) -> f32 {
    let mut min_x = 0.0;
    let mut max_x = 0.0;
    let mut min_y = 0.0;
    let mut max_y = 0.0;
    let mut min_z = 0.0;
    let mut max_z = 0.0;

    for [x, y, z] in verts {
        if *x < min_x {
            min_x = *x;
        } else if *x > max_x {
            max_x = *x;
        }
        if *y < min_y {
            min_y = *y;
        } else if *y > max_y {
            max_y = *y;
        }
        if *z < min_z {
            min_z = *z;
        } else if *z > max_z {
            max_z = *z;
        }
    }

    *[max_x - min_x, max_y - min_y, max_z - min_z]
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()
}

fn png_to_obj(png_path: &str) {
    let file_dir = Path::new(png_path).parent().unwrap();
    let file_name = Path::new(png_path)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let file_path = file_dir.join(file_name.to_owned() + ".obj");

    let mut file = File::create(file_path).unwrap();

    writeln!(&mut file, "# Point cloud converted from 16 bit image.").unwrap();
    writeln!(&mut file, "# By Roman Chumak @ PHYGITALISM.\n").unwrap();
    writeln!(&mut file, "o {}\n", &file_name).unwrap();

    let img = image::open(png_path).unwrap();
    let scale = 1.0;
    let depth = (2 as usize).pow(16) as f32 - 1.0;

    for pix in img.as_rgb16().unwrap().pixels() {
        let x = (pix[0] as f32 - depth / 2.0) / depth * 2.0 * scale;
        let y = (pix[1] as f32 - depth / 2.0) / depth * 2.0 * scale;
        let z = (pix[2] as f32 - depth / 2.0) / depth * 2.0 * scale;
        writeln!(&mut file, "v {} {} {}", x, y, z).unwrap();
    }
    println!("Converted to OBJ");
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let file_path = &args[1];
    let extension = Path::new(&file_path)
        .extension()
        .unwrap()
        .to_str()
        .unwrap()
        .to_lowercase();

    match extension.as_str() {
        "obj" => obj_to_png(file_path),
        "png" => png_to_obj(file_path),
        _ => println!("OBJ or PNG file expected."),
    };
}
