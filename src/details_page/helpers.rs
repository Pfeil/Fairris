pub struct DOM;

impl DOM {
    pub fn get_element<T, I>(id: I) -> T
    where
        I: AsRef<str>,
        T: wasm_bindgen::JsCast,
    {
        {
            use wasm_bindgen::JsCast;
            let window = web_sys::window().expect("no global `window` exists");
            let document = window.document().expect("should have a document on window");
            let x = document.get_element_by_id(id.as_ref()).unwrap();
            x.unchecked_into::<T>()
        }
    }
}
