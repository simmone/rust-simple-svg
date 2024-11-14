pub enum FillRule {
    nonzero,
    evenodd,
    inerit,
}

pub enum LineCap {
    butt,
    round,
    square,
    inherit,
}

pub enum LineJoin {
    miter,
    round,
    bevel,
}

pub struct Sstyle {
    pub fill: String,
    pub fill-ruler: FillRule,
    pub fill-opacity: f32,
    pub stroke: String,
    pub stroke-width: f32,
    pub stroke-linecap: LineCap,
    pub stroke-linejoin: LineJoin,
    pub stroke-miterlimit: f32,
    pub stroke-dasharray: String,
    pub stroke-dashoffset: f32,
    pub translate: (f32, f32),
    pub rotate: f32,
    pub scale: (f32, f32),
    pub skewX: f32,
    pub skewY: f32,
    pub fill-gradient: String,
}
