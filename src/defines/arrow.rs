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
        let pre_toward_left = if self.start_x > pre_end_x true false;
        let pre_toward_updown = if self.start_x == pre_end_x true false;
        let pre_toward_up = if (self.start_x == pre_end_x) && (self.start_y == pre_end_y) true false;
        let pre_x_offset = pre_end_x - start_x;
        let pre_y_offset = pre_end_y - start_y;
        let pre_theta = (if pre_x_offset == 0 0 (pre_y_offset / pre_x_offset)).atan();
        let pre_alpha = (PI / 2) - pre_theta;
        let pre_delta_r = (self.head_height * pre_theta.cos(), (self.head_height * pre_theta.sin()));
        let pre_r_sub1 = if pre_toward_updown pre_delta_r.1 pre_delta_r.0;
        let pre_r_sub2 = if pre_toward_updown pre_delta_r.0 pre_delta_r.1;
        let pre_r = 
            (
                if pre_toward_left {
                    pre_end_x + pre_r_sub1
                } else {
                    pre_end_x - pre_r_sub1
                },
                if (pre_toward_up || pre_toward_left) {
                    pre_end_y + pre_r_sub2
                } else {
                    pre_end_y - pre_r_sub2
                }
            );
        let end_x = pre_r.0;
        let end_y = pre_r.1;
        let toward_left = if start_x > end_x true false;
        let toward_updown = if start_x == end_x true false;
        let toward_up = if (start_x == end_x) && (start_y > end_y) true false;
        let x_offset = end_x - start_x;
        let y_offset = end_y - start_y;
        let theta = (if x_offset == 0.0 0.0 (y_offset / x_offset)).atan();
        let alpha = (pi / 2.0) - theta;
        let delta_r = ((head_heat * theta.cos()), (head_height * theta.sin()));
        let r_sub1 = if toward_updown delta_r.1 delta_r.0;
        let r_sub2 = if toward_updown delta_r.0 delta_r.1;
        let r =
            (
                if toward_left (end_x - r_sub1) (end_x + r_sub1),
                if (toward_up || toward_left) {
                    end_y - r_sub2
                } else {
                    end_y + r_sub2
                }
            );
        let handle_delta_q = ((handle_base * alpha.cos()), (handle_base * alpha.sin()));
        let toward_updown_delta1 = if toward_updown handle_delta_q.1 handle_delta_q.0;
        let toward_updown_delta0 = if toward_updown handle_delta_q.0 handle_delta_q.1;
        let handle_bottom_left = 
            (
                if toward_left {
                    start_x + toward_updown_delta1
                } else {
                    start_x - toward_updown_delta1
                },
                if toward_left {
                    start_y - toward_updown_delta0
                } else {
                    start_y + toward_updown_delta0
                }
            );
        let handle_bottom_right =
            (
                if toward_left {
                    end_x + toward_updown_delta1
                } else {
                    end_x - toward_updown_delta1
                },
                if toward_left {
                    start_x - toward_updown_delta0
                } else {
                    start_y + toward_updown_delta0
                }
            );
        let handle_top_left =
            (
                if toward_left {
                    start_x - toward_updown_delta1
                } else {
                    start_x + toward_updown_delta1
                },
                if toward_left {
                    start_y + toward_updown_delta0
                } else {
                    start_y - toward_updown_delta0
                }
            );
        let handle_top_right =
            (
                if toward_left {
                    end_x - toward_updown_delta1
                } else {
                    end_x + toward_updown_delta1
                },
                if toward_left {
                    end_y + toward_updown_delta0
                } else {
                    end_y - toward_updown_delta0
                }
            );
        let head_delta_q =
            (
                total_base * alpha.cos(),
                total_base * alpha.sin()
            );
        let q =
            (
                if toward_left {
                    end_x + toward_updown_delta1
                } else {
                    end_x - toward_updown_delta1
                },
                if toward_left {
                    end_y - toward_updown_delta0
                } else {
                    end_y + toward_updown_delta1
                }
           );
        
        let s =
            (
                if toward_left {
                    end_x - toward_updown_delta1
                } else {
                    end_x + toward_updown_delta1
                },
                if toward_left {
                    end_y + toward_updown_delta0
                } else {
                    end_y - toward_updown_delta0
                }
            );

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
