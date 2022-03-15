#[derive(Debug)]
pub enum Musician {
    SoloAct(String), 
    Band(String, u8),
}


pub fn search_artist(musicians: &Vec<Musician>, artist_name: String) -> Option<&Musician> {
    for musician in musicians.iter() {
        match musician {
            Musician::SoloAct(solo_name) => {
                if artist_name == solo_name.to_string() {
                    return Some(musician)
                }
            }
            Musician::Band(band_name, _members) => {
                if artist_name == band_name.to_string() {
                    return Some(musician)
                }
            }
        }
    }

    return None
}
