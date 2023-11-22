#[derive(Debug)]
pub struct Track {
   pub name: String,
}

impl Track {
    pub fn new(name: String) -> Self {
        Track { name }
    }
}

#[derive(Debug)]
pub struct Album {
    pub name: String,
    pub url: String,
    pub tracks: Vec<Track>,
}

impl Album {
    pub fn new(name: String, url: String, tracks: Vec<Track>) -> Self {
        Album { name, url, tracks }
    }
}
