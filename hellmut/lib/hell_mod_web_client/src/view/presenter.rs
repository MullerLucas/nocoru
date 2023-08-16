pub trait ElementTree {
    fn root(&self) -> &web_sys::Element ;
}

pub trait Presenter: ElementTree { }
