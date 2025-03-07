use std::fs::File;
use std::io::prelude::*;
use crate::map_data::{TiledMap, TiledTileset};
use std::io::BufWriter;
use std::collections::HashMap;
use crate::map_data::Layer::TileLayer;
use crate::map_data::InMapTileset::Tileset;
use crate::map_data::InMapTileset::TiledTileset as OtherTiledTileset;
use rayon::prelude::*;
use resize::px::RGBA;
use rgb::ComponentBytes;
use rgb::FromSlice;



mod map_data;

#[derive(Debug)]
struct FunnyBitmap {
    width: i32,
    height: i32,
    data: Vec<(u8, u8, u8, u8)>
}

fn scale150(a: String, b: String) {
    let decoder = png::Decoder::new(File::open(a).unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();

    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf).unwrap();

    let buf = buf.as_rgba();
    let mut resizer = resize::new(
        info.width as usize,
        info.height as usize,
        (info.width as f64 * 1.5f64) as usize,
    (info.height as f64 * 1.5f64) as usize,
    resize::Pixel::RGBA8, resize::Type::Lanczos3).unwrap();

    let mut tgt = vec![RGBA::new(0u8, 0, 0, 0);
                       ((info.width as f64 * 1.5f64) as usize) *
                        ((info.height as f64 * 1.5f64) as usize)];

    resizer.resize(&buf, &mut tgt).unwrap();
    let tgt = tgt.as_bytes();

    let file = File::create(b).unwrap();
    let w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, (info.width as f64 * 1.5f64) as u32, (info.height as f64 * 1.5f64) as u32);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(tgt).unwrap();

}

impl FunnyBitmap {
    fn new(w: i32, h: i32) -> FunnyBitmap {
        let mut b : FunnyBitmap = FunnyBitmap {
            width: w,
            height: h,
            data: Vec::new()
        };

        for _ in 0..(w*h) {
            b.data.push((0, 0, 0, 0));
        }

        b
    }

    fn set(&mut self, x: i32, y: i32, data: (u8, u8, u8, u8)) {
        self.data[(x + (y * self.width)) as usize] = data;
    }

    fn get(&self, x: i32, y: i32) -> (u8, u8, u8, u8) {
        if ((x + (y * self.width)) as usize) >= self.data.len() {
            return (0, 0, 0, 0);
        }
        return self.data[(x + (y * self.width)) as usize];
    }

    fn save(&self, loc: String) -> std::io::Result<()> {
        let file = File::create(loc)?;
        let w = BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, self.width as u32, self.height as u32);
        encoder.set_color(png::ColorType::RGBA);
        encoder.set_depth(png::BitDepth::Eight);
        let mut data = Vec::with_capacity((self.width * self.height * 4) as usize);

        let mut writer = encoder.write_header().unwrap();
        for entry in &self.data {
            data.push(entry.0);
            data.push(entry.1);
            data.push(entry.2);
            data.push(entry.3);
        }

        writer.write_image_data(&*data)?;

        Ok(())
    }

    fn from(loc: String) -> FunnyBitmap {
        let decoder = png::Decoder::new(File::open(loc).unwrap());
        let (info, mut reader) = decoder.read_info().unwrap();

        let mut bm = FunnyBitmap::new(info.width as i32, info.height as i32);
        let mut buf = vec![0; info.buffer_size()];
        reader.next_frame(&mut buf).unwrap();

        for i in 0..(buf.len() / 4) {
            bm.data[i] = (buf[i * 4], buf[i * 4 + 1], buf[i * 4 + 2], buf[i * 4 + 3]);
        }

        bm
    }
}

fn render_image(path: &String) -> std::io::Result<()> {
    println!("Starting: {}", path);
    // Load map data
    let mut fd = File::open(format!("maps/{}", path.clone()))?;
    let mut contents = String::new();
    fd.read_to_string(&mut contents)?;

    let map : TiledMap = serde_json::from_str(&*contents)?;

    let mut maxgid = 0;

    let mut tiles : HashMap<i32, FunnyBitmap>= HashMap::new();
    tiles.insert(0, FunnyBitmap::new(map.tilewidth, map.tileheight));

    for tileset in &map.tilesets {
        if let Tileset(tileset) = tileset {
            let tspath = tileset.source.split("/").collect::<Vec<&str>>();
            let tspath = tspath[tspath.len() - 1];
            let mut result: String = String::from("it didn't work fuckass");
            let dirs = std::fs::read_dir("maps")?;
            for dir in dirs {
                let dir = dir.unwrap();
                let a = dir.file_name().into_string().unwrap();
                if a.to_lowercase() == tspath.to_lowercase() {
                    result = a;
                    break;
                }
            }

            let mut fd = File::open(format!("maps/{}", result))?;
            let mut contents = String::new();
            fd.read_to_string(&mut contents)?;

            let tset: TiledTileset = serde_json::from_str(&*contents)?;

            if tset.spacing != 0 || tset.margin != 0 {
                panic!("bad tileset");
            }

            let tspath = tset.image.split("/").collect::<Vec<&str>>();
            let tspath = tspath[tspath.len() - 1];
            let tspath2 = String::from(tspath);

            let mut result = String::from("it broken fuckass");

            let dirs = std::fs::read_dir("img/tilesets")?;
            for dir in dirs {
                let dir = dir.unwrap();
                let a = dir.file_name().into_string().unwrap();
                if a.to_lowercase() == tspath.to_lowercase() {
                    result = a;
                    break;
                }
            }

            let mut base = String::from("img/tilesets/");
            base.push_str(&*result);

            let mimg = FunnyBitmap::from(base);


            let mgid = tileset.firstgid + tset.tilecount;
            for i in 0..tset.tilecount {
                if tspath2.starts_with("Tile_") {
                    let mut tile = FunnyBitmap::new(tset.tilewidth, tset.tileheight);
                    for x in 0..tset.tilewidth {
                        for y in 0..tset.tileheight {
                            tile.set(x, y, (0, 0, 0, 0));
                        }
                    }

                    tiles.insert(tileset.firstgid + i, tile);
                } else {
                    let start_x = (i % (tset.imagewidth / tset.tilewidth)) * tset.tilewidth;
                    let start_y = (i / (tset.imagewidth / tset.tilewidth)) * tset.tileheight;

                    let mut tile = FunnyBitmap::new(tset.tilewidth, tset.tileheight);
                    for x in 0..tset.tilewidth {
                        for y in 0..tset.tileheight {
                            tile.set(x, y, mimg.get(start_x + x, start_y + y));
                        }
                    }

                    tiles.insert(tileset.firstgid + i, tile);
                }
            }

            if mgid > maxgid {
                maxgid = mgid;
            }
        }
        if let OtherTiledTileset(tset) = tileset {
            if tset.spacing != 0 || tset.margin != 0 {
                panic!("bad tileset");
            }

            let tspath = tset.image.split("/").collect::<Vec<&str>>();
            let tspath = tspath[tspath.len() - 1];
            let tspath2 = String::from(tspath);

            let mut result = String::from("it broken fuckass");

            let dirs = std::fs::read_dir("img/tilesets")?;
            for dir in dirs {
                let dir = dir.unwrap();
                let a = dir.file_name().into_string().unwrap();
                if a.to_lowercase() == tspath.to_lowercase() {
                    result = a;
                    break;
                }
            }

            let mut base = String::from("img/tilesets/");
            base.push_str(&*result);

            let mimg = FunnyBitmap::from(base);


            let mgid = tset.firstgid + tset.tilecount;
            for i in 0..tset.tilecount {
                if tspath2.starts_with("Tile_") {
                    let mut tile = FunnyBitmap::new(tset.tilewidth, tset.tileheight);
                    for x in 0..tset.tilewidth {
                        for y in 0..tset.tileheight {
                            tile.set(x, y, (0, 0, 0, 0));
                        }
                    }

                    tiles.insert(tset.firstgid + i, tile);
                } else {
                    let start_x = (i % (tset.imagewidth / tset.tilewidth)) * tset.tilewidth;
                    let start_y = (i / (tset.imagewidth / tset.tilewidth)) * tset.tileheight;

                    let mut tile = FunnyBitmap::new(tset.tilewidth, tset.tileheight);
                    for x in 0..tset.tilewidth {
                        for y in 0..tset.tileheight {
                            tile.set(x, y, mimg.get(start_x + x, start_y + y));
                        }
                    }

                    tiles.insert(tset.firstgid + i, tile);
                }
            }

            if mgid > maxgid {
                maxgid = mgid;
            }
        }
    }

    let mut target = FunnyBitmap::new(map.width * map.tilewidth, map.height * map.tileheight);
    for i in 0..map.layers.len() {
        let layer = &map.layers[i];
        if let TileLayer(layer) = layer {
            if !layer.visible { continue; }
            let x_offset = layer.x as usize;
            let y_offset = layer.y as usize;

            for x in 0..(layer.width as usize) {
                for y in 0..(layer.height as usize) {
                    let gid = layer.data[x + (y * (layer.width as usize))];
                    let bitmap = tiles.get(&gid).unwrap();

                    let plot_start_x = ((x + x_offset) * (map.tilewidth as usize)) as i32;
                    let plot_start_y = ((y + y_offset) * (map.tileheight as usize)) as i32;

                    for px in 0..bitmap.width {
                        for py in 0..bitmap.height {
                            let (r, g, b, mut a) = bitmap.get(px, py);
                            // TODO: Proper alpha blending
                            a = ((a as f64) * layer.opacity) as u8;
                            if a == 255 {
                                target.set(plot_start_x + px, plot_start_y + py, (r, g, b, a))
                            }
                            if a != 255 && a != 0 {
                                // Sample existing target r, g, b
                                let (t_r, t_g, t_b, _) = target.get(plot_start_x + px, plot_start_y + py);
                                let (t_r, t_g, t_b, r, g, b) = (t_r as f64, t_g as f64, t_b as f64, r as f64, g as f64, b as f64);
                                let mul_tgt = (a as f64) / 255f64;
                                let mul_ori = 1f64 - mul_tgt;

                                let r_r = (r * mul_tgt) + (t_r * mul_ori);
                                let r_g = (g * mul_tgt) + (t_g * mul_ori);
                                let r_b = (b * mul_tgt) + (t_b * mul_ori);

                                let r_r = r_r as u8;
                                let r_g = r_g as u8;
                                let r_b = r_b as u8;

                                target.set(plot_start_x + px, plot_start_y + py, (r_r, r_g, r_b, 255));
                            }
                        }
                    }
                }
            }
        }
    }

    let base = path.split(".").collect::<Vec<&str>>();
    target.save(String::from(format!("render/{}.png", base[0])))?;
    scale150(format!("render/{}.png", base[0]), format!("scaled/{}.png", base[0]));

    println!("Finished: {}", path);
    Ok(())
}

fn wrapper(z: &String) {
    if let Err(e) = render_image(z) {
        println!("Failed: {}! Reason: {:?}", z, e);
    }
}

fn main() -> std::io::Result<()> {
    let _ = std::fs::create_dir("scaled");
    let _ = std::fs::create_dir("render");
    let dirs = std::fs::read_dir("maps")?;
    let mut dir_names : Vec<String> = Vec::new();

    for a in dirs {
        let a = a.unwrap();
        let a = a.file_name().into_string().unwrap();
        if a.starts_with("map") {
            dir_names.push(a);
        }
    }
    dir_names.par_iter()
        .for_each(|i| wrapper(&i));

    Ok(())
}
