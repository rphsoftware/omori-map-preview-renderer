use serde::Deserialize;
use serde;

#[derive(Deserialize, Debug)]
pub struct Object {

}

#[derive(Deserialize, Debug)]
pub struct TileLayer {
    pub x: i32,
    pub y: i32,
    pub height: i32,
    pub width: i32,

    pub visible: bool,
    pub opacity: f64,

    pub data: Vec<i32>,

    // pub properties: Option<HashMap<String, String>>,
    // pub propertytypes: Option<HashMap<String, String>>
}

#[derive(Deserialize, Debug)]
pub struct ObjectGroup {

    // pub properties: Option<HashMap<String, String>>,
    // pub propertytypes: Option<HashMap<String, String>>
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub struct ImageLayer {

}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Layer {
    #[serde(alias = "tilelayer")]
    TileLayer(TileLayer),
    #[serde(alias = "objectgroup")]
    ObjectGroup(ObjectGroup),
    #[serde(alias = "imagelayer")]
    ImageLayer(ImageLayer)
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum InMapTileset {
    Tileset(Tileset),
    TiledTileset(AdvancedTiledTileset)
}

#[derive(Deserialize, Debug)]
pub struct Tileset {
    pub firstgid: i32,
    pub source: String
}

#[derive(Deserialize, Debug)]
pub struct TiledMap {
    pub width: i32,
    pub height: i32,
    pub layers: Vec<Layer>,
    pub tileheight: i32,
    pub tilewidth: i32,
    // pub version: f64,
    pub tilesets: Vec<InMapTileset>
}

#[derive(Deserialize, Debug)]
pub struct TiledTileset {
    pub image: String,
    pub imagewidth: i32,
    pub margin: i32,
    pub spacing: i32,
    pub tilecount: i32,
    pub tileheight: i32,
    pub tilewidth: i32
}

#[derive(Deserialize, Debug)]
pub struct AdvancedTiledTileset {
    pub firstgid: i32,
    pub image: String,
    pub imagewidth: i32,
    pub margin: i32,
    pub spacing: i32,
    pub tilecount: i32,
    pub tileheight: i32,
    pub tilewidth: i32
}