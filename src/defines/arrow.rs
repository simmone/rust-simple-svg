#[derive(Debug, Clone)]
pub struct Arrow {
    pub start_x: f64,
    pub start_y: f64,
    pub end_x: f64,
    pub end_y: f64,
    pub handle_base: f64,
    pub head_base: f64,
    pub head_height: f64,
}

impl Arrow {
    pub fn new(start_x: f64, start_y: f64, end_x: f64, end_y: f64, handle_base: f64, head_base: f64, head_height: f64) -> Self {
        Arrow {
            start_x,
            start_y,
            end_x,
            end_y,
            handle_base: f64,
            head_base: f64,
            head_height: f64,
        }
    }

    pub fn format(&self, shape_id: String) -> String {
        let pre_end_x = self.end_x;
        let pre_end_y = self.end_y;
        let pre_toward_left? = if self.start_x > pre_end_x true false;
        let pre_toward_updown? = if self.start_x == pre_end_x true false;
        let pre_toward_up? = if (self.start_x == pre_end_x) && (self.start_y == pre_end_y) true false;
        let pre_x_offset = pre_end_x - start_x;
        let pre_y_offset = pre_end_y - start_y;
        let pre_theta = (if pre_x_offset == 0 0 (pre_y_offset / pre_x_offset)).atan();
        let pre_alpha = (PI / 2) - pre_theta;
        let pre_delta_r = (self.head_height * pre_theta.cos(), (self.head_height * pre_theta.sin()));
        let pre_R = 

         [pre_R (cons
             ((if pre_toward_left? + -) pre_end_x ((if pre_toward_updown? cdr car) pre_delta_r))
             ((cond
               [pre_toward_up? +]
               [pre_toward_left? +]
               [else -])
               pre_end_y ((if pre_toward_updown? car cdr) pre_delta_r)))]
         [end_x (car pre_R)]
         [end_y (cdr pre_R)]
         [toward_left? (if (> start_x end_x) #t #f)]
         [toward_updown? (if (= start_x end_x) #t #f)]
         [toward_up? (if (and (= start_x end_x) (> start_y end_y)) #t #f)]
         [x_offset (- end_x start_x)]
         [y_offset (- end_y start_y)]
         [theta (atan (if (= x_offset 0) 0 (/ y_offset x_offset)))]
         [alpha (- (/ pi 2) theta)]
         [delta_r
          (cons (* head_height (cos theta)) (* head_height (sin theta)))]
         [R (cons
             ((if toward_left? - +) end_x ((if toward_updown? cdr car) delta_r))
             ((cond
               [toward_up? -]
               [toward_left? -]
               [else +])
               end_y ((if toward_updown? car cdr) delta_r)))]
         [handle_delta_q
          (cons (* handle_base (cos alpha)) (* handle_base (sin alpha)))]
         [handle_bottom_left
          (cons
           ((if toward_left? + -) start_x ((if toward_updown? cdr car) handle_delta_q))
           ((if toward_left? - +) start_y ((if toward_updown? car cdr) handle_delta_q)))]
         [handle_bottom_right
          (cons
           ((if toward_left? + -) end_x ((if toward_updown? cdr car) handle_delta_q))
           ((if toward_left? - +) end_y ((if toward_updown? car cdr) handle_delta_q)))]
         [handle_top_left
          (cons
           ((if toward_left? - +) start_x ((if toward_updown? cdr car) handle_delta_q))
           ((if toward_left? + -) start_y ((if toward_updown? car cdr) handle_delta_q)))]
         [handle_top_right
            (cons
             ((if toward_left? - +) end_x ((if toward_updown? cdr car) handle_delta_q))
             ((if toward_left? + -) end_y ((if toward_updown? car cdr) handle_delta_q)))]
         [head_delta_q
          (cons (* total_base (cos alpha)) (* total_base (sin alpha)))]
         [Q (cons
             ((if toward_left? + -) end_x ((if toward_updown? cdr car) head_delta_q))
             ((if toward_left? - +) end_y ((if toward_updown? car cdr) head_delta_q)))]
         [S (cons
             ((if toward_left? - +) end_x ((if toward_updown? cdr car) head_delta_q))
             ((if toward_left? + -) end_y ((if toward_updown? car cdr) head_delta_q)))]
         )


        format!("    <arrow id=\"{}\" {} />\n", shape_id, {
            let mut shape_str = format!("width=\"{}\" height=\"{}\"", self.width, self.height);

            if self.radius_x.is_some() && self.radius_y.is_some() {
                shape_str.push_str(&format!(
                    " rx=\"{}\" ry=\"{}\"",
                    self.radius_x.unwrap(),
                    self.radius_y.unwrap(),
                ));
            }

            shape_str
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_new_arrow() {
        let arrow = Arrow {
            width: 30.0,
            height: 20f64,
            radius_x: Some(10.0),
            radius_y: Some(5f64),
        };

        assert_eq!(arrow.width, 30.0);
        assert_eq!(arrow.height, 20.0);
        assert_eq!(arrow.radius_x.unwrap(), 10.0);
        assert_eq!(arrow.radius_y.unwrap(), 5.0);
    }

    #[test]
    fn check_new() {
        let arrow = Arrow::new(30f64, 20f64);
        assert_eq!(arrow.width, 30.0);
        assert_eq!(arrow.height, 20.0);
        assert!(arrow.radius_x.is_none());
        assert!(arrow.radius_y.is_none());
    }

    #[test]
    fn check_format1() {
        let arrow = Arrow::new(30.0, 20.0);

        assert_eq!(
            Arrow::format(&arrow, "1".to_string()),
            "    <arrow id=\"1\" width=\"30\" height=\"20\" />\n"
        );
    }

    #[test]
    fn check_format2() {
        let arrow = Arrow {
            width: 30.0,
            height: 20.0,
            radius_x: Some(10.0),
            radius_y: Some(5.0),
        };

        assert_eq!(
            Arrow::format(&arrow, "1".to_string()),
            "    <arrow id=\"1\" width=\"30\" height=\"20\" rx=\"10\" ry=\"5\" />\n"
        );
    }
}
