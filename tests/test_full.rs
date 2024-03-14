#![cfg(feature = "img")]

use std::collections::HashSet;

#[test]
fn test_full_time() {
    let jpg = image::open("tests/data/full/gogh.jpg").unwrap().to_luma8();

    let mut search_img = rqrr::PreparedImage::prepare(jpg);
    let grids = search_img.detect_grids();
    assert_eq!(grids.len(), 3);

    let mut codes = HashSet::new();
    for grid in grids {
        let (_meta, content) = grid.decode().unwrap();
        codes.insert(content);
    }
    let mut ref_set = HashSet::new();
    ref_set.insert("https://github.com/WanzenBug/rqrr".to_string());
    ref_set.insert("rqrr".to_string());
    ref_set.insert("1234567891011121314151617181920".to_string());
    assert_eq!(codes, ref_set);
}

#[test]
fn test_full_large_version() {
    let gif = image::open("tests/data/full/superlong.gif")
        .unwrap()
        .to_luma8();

    let mut search_img = rqrr::PreparedImage::prepare(gif);
    let grids = search_img.detect_grids();
    assert_eq!(grids.len(), 1);

    let (meta, content) = grids[0].decode().unwrap();
    assert_eq!(meta.version, rqrr::Version(14));
    assert_eq!(content, "superlongdatasuperlongdatasuperlongdatasuperlongdatasuperlongdatasuperlongdatasuperlongdatasuperlongdatasuperlongdatasuperlongdatasuperlongdatasuperlongdatasuperlongdatasuperlongdatasuperlongdatasuperlongdatasuperlongdatasuperlongdatasuperlongdatasuperlongdatasuperlongdatasuperlongdatasuperlongdatasuperlongdatasuperlongdatasuperlongdata");
}
