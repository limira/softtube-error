#[path="softtube.rs"]
mod softtube;

pub fn search_videos(text : String) {
    let search : crate::softtube::Search = match text.len() {
        0 => crate::softtube::Search::Limit(-1),
        _ => crate::softtube::Search::Text(text)
    };
    
    crate::softtube::Videos::search(Some(search));
}