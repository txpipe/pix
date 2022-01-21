use image::DynamicImage;

pub type Traits = Vec<SimpleTrait>;

pub struct SimpleTrait {
    pub common: Vec<DynamicImage>,
    pub uncommon: Vec<DynamicImage>,
    pub rare: Vec<DynamicImage>,
    pub mythical: Vec<DynamicImage>,
    pub legendary: Vec<DynamicImage>,
}
