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
    pub fill: Option<String>,
    pub fill_ruler: Option<FillRule>,
    pub fill_opacity: Option<f64>,
    pub stroke: Option<String>,
    pub stroke_width: Option<f64>,
    pub stroke_linecap: Option<LineCap>,
    pub stroke_linejoin: Option<LineJoin>,
    pub stroke_miterlimit: Option<f64>,
    pub stroke_dasharray: Option<String>,
    pub stroke_dashoffset: Option<f64>,
    pub translate: Option<(f64, f64)>,
    pub rotate: Option<f64>,
    pub scale: Option<(f64, f64)>,
    pub skewX: Option<f64>,
    pub skewY: Option<f64>,
    pub fill_gradient: Option<String>,
}

impl Sstye {
    pub new() -> Self {
        Sstyle {
            None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None
        }
    }
    
    pub format() -> String{

(define (sstyle-format _sstyle)
  (with-output-to-string
    (lambda ()
      (if (SSTYLE-fill-gradient _sstyle)
          (printf " fill=\"url(#~a)\"" (SSTYLE-fill-gradient _sstyle))
          (if (SSTYLE-fill _sstyle)
              (printf " fill=\"~a\"" (SSTYLE-fill _sstyle))
              (printf " fill=\"none\"")))

      (when (SSTYLE-fill-rule _sstyle)
        (printf " fill-rule=\"~a\"" (SSTYLE-fill-rule _sstyle)))

      (when (SSTYLE-fill-opacity _sstyle)
        (printf " fill-opacity=\"~a\"" (~r (SSTYLE-fill-opacity _sstyle))))

      (when (SSTYLE-stroke-width _sstyle)
        (printf " stroke-width=\"~a\"" (~r (SSTYLE-stroke-width _sstyle))))

      (when (SSTYLE-stroke _sstyle)
        (printf " stroke=\"~a\"" (SSTYLE-stroke _sstyle)))

      (when (SSTYLE-stroke-linejoin _sstyle)
        (printf " stroke-linejoin=\"~a\"" (SSTYLE-stroke-linejoin _sstyle)))

      (when (SSTYLE-stroke-linecap _sstyle)
        (printf " stroke-linecap=\"~a\"" (SSTYLE-stroke-linecap _sstyle)))

      (when (SSTYLE-stroke-miterlimit _sstyle)
        (printf " stroke-miterlimit=\"~a\"" (SSTYLE-stroke-miterlimit _sstyle)))

      (when (SSTYLE-stroke-dasharray _sstyle)
        (printf " stroke-dasharray=\"~a\"" (SSTYLE-stroke-dasharray _sstyle)))

      (when (SSTYLE-stroke-dashoffset _sstyle)
        (printf " stroke-dashoffset=\"~a\"" (~r (SSTYLE-stroke-dashoffset _sstyle))))
      
      (when (or
             (SSTYLE-translate _sstyle)
             (SSTYLE-rotate _sstyle)
             (SSTYLE-scale _sstyle)
             (SSTYLE-skewX _sstyle)
             (SSTYLE-skewY _sstyle)
             )
        (printf " transform=\"~a\""
                (string-join
                 (filter
                  (lambda (a) (not (string=? a "")))
                  (list
                   (if (SSTYLE-translate _sstyle)
                       (format "translate(~a ~a)"
                               (~r (car (SSTYLE-translate _sstyle)))
                               (~r (cdr (SSTYLE-translate _sstyle))))
                       "")
                   (if (SSTYLE-rotate _sstyle)
                       (format "rotate(~a)" (~r (SSTYLE-rotate _sstyle)))
                       "")
                   (if (SSTYLE-scale _sstyle)
                       (if (pair? (SSTYLE-scale _sstyle))
                           (format "scale(~a ~a)"
                                   (~r (car (SSTYLE-scale _sstyle)))
                                   (~r (cdr (SSTYLE-scale _sstyle))))
                           (format "scale(~a)" (~r (SSTYLE-scale _sstyle))))
                       "")
                   (if (SSTYLE-skewX _sstyle)
                       (format "skewX(~a)" (~r (SSTYLE-skewX _sstyle)))
                       "")
                   (if (SSTYLE-skewY _sstyle)
                       (format "skewY(~a)" (~r (SSTYLE-skewY _sstyle)))
                       ""))))))
      )))

