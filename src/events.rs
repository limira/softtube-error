// You don't need these two lines here:
// #[path="softtube.rs"]
// mod softtube;

// And other files need to remove those lines too.

// If you want to bring the module `softtube` into this scope,
// add the next line:

use crate::softtube;
// Then, later you just use: `softtube::Search` like this:
pub fn use_soft_tube() {
    let _s = softtube::Search::Limit(-1);
}

// Or 
use crate::softtube::Search;
// Then:
pub fn use_soft_tube_search() {
    let _s = Search::Limit(-1);
}

pub fn search_videos(text : String) {
    let search : crate::softtube::Search = match text.len() {
        0 => crate::softtube::Search::Limit(-1),
        _ => crate::softtube::Search::Text(text)
    };
    
    crate::softtube::Videos::search(Some(search));
}
