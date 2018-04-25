extern crate poppler;
extern crate glib;
extern crate glib_sys;


fn run() -> Result<(), glib::error::Error> {
    let filename = "test.pdf";
    let doc = poppler::PopplerDocument::new_from_file(filename, "")?;
    let num_pages = doc.get_n_pages();

    println!("Document has {} page(s)", num_pages);

    // FIXME: move iterator to poppler
    for page_num in 0..num_pages {
        let page = doc.get_page(page_num).unwrap();
        let (w, h) = page.get_size();
        println!("page {} has size {}, {}", page_num, w, h);
        // surface.set_size(w as i32, h as i32);  // ??

        let text = page.get_text();
        let text_lossy = page.get_text_lossy();
        let layout = page.get_text_layout().unwrap();
        let bb = text.chars().fold(0, |n, _| n + 1);
        assert!(bb == layout.len());
        println!("exact={}, lossy={}, bb={}", text.len(), text_lossy.len(), bb);

        println!("res={}", layout.len());


        println!("vec={:?}", page.get_text_attributes());

//        for (c,r) in text.chars().zip(layout.iter()) {
//            println!("{} {:?}", c, r);
//        }
    }
    //         g_object_unref (page);

    Ok(())
}


fn main() {
    match run() {
        Ok(()) => (),
        Err(e) => {
            println!("ERROR: {}", e);
        }
    };
}
