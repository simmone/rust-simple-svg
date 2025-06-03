#![doc = include_str!("ARROW.md")]

use std::f64::consts::PI;
use crate::tools::precision::svg_round;

#[derive(Debug, Clone)]
pub struct Arrow {
    pub start_x: f64,
    pub start_y: f64,
    pub end_x: f64,
    pub end_y: f64,
    pub handle_base: f64,
    pub head_base: f64,
    pub head_height: f64,
    pub precision: usize,
}

impl Arrow {
    pub fn new(
        start_point: (f64, f64),
        end_point: (f64, f64),
        handle_base: f64,
        head_base: f64,
        head_height: f64,
    ) -> Self {
        Arrow {
            start_x: start_point.0,
            start_y: start_point.1,
            end_x: end_point.0,
            end_y: end_point.1,
            handle_base,
            head_base,
            head_height,
            precision: 0,
        }
    }

    pub fn format(&self, shape_id: String) -> String {
        let total_base = self.handle_base + self.head_base;
        let pre_end_x = self.end_x;
        let pre_end_y = self.end_y;
        let pre_toward_left = if self.start_x > pre_end_x {
            true
        } else {
            false
        };
        let pre_toward_updown = if self.start_x == pre_end_x {
            true
        } else {
            false
        };
        let pre_toward_up = if (self.start_x == pre_end_x) && (self.start_y > pre_end_y) {
            true
        } else {
            false
        };
        let pre_x_offset = pre_end_x - self.start_x;
        let pre_y_offset = pre_end_y - self.start_y;
        let pre_theta = (if pre_x_offset == 0.0 {
            0.0
        } else {
            pre_y_offset / pre_x_offset
        })
        .atan();
        let pre_delta_r = (
            self.head_height * pre_theta.cos(),
            (self.head_height * pre_theta.sin()),
        );
        let pre_r_sub1 = if pre_toward_updown {
            pre_delta_r.1
        } else {
            pre_delta_r.0
        };
        let pre_r_sub0 = if pre_toward_updown {
            pre_delta_r.0
        } else {
            pre_delta_r.1
        };
        let pre_r = (
            if pre_toward_left {
                pre_end_x + pre_r_sub1
            } else {
                pre_end_x - pre_r_sub1
            },
            if pre_toward_up || pre_toward_left {
                pre_end_y + pre_r_sub0
            } else {
                pre_end_y - pre_r_sub0
            },
        );
        let new_end_x = pre_r.0;
        let new_end_y = pre_r.1;
        let toward_left = if self.start_x > new_end_x {
            true
        } else {
            false
        };
        let toward_updown = if self.start_x == new_end_x {
            true
        } else {
            false
        };
        let toward_up = if (self.start_x == new_end_x) && (self.start_y > new_end_y) {
            true
        } else {
            false
        };
        let x_offset = new_end_x - self.start_x;
        let y_offset = new_end_y - self.start_y;
        let theta = (if x_offset == 0.0 {
            0.0
        } else {
            y_offset / x_offset
        })
        .atan();
        let alpha = (PI / 2.0) - theta;
        let delta_r = (
            (self.head_height * theta.cos()),
            (self.head_height * theta.sin()),
        );
        let r_sub1 = if toward_updown { delta_r.1 } else { delta_r.0 };
        let r_sub2 = if toward_updown { delta_r.0 } else { delta_r.1 };
        let r = (
            if toward_left {
                new_end_x - r_sub1
            } else {
                new_end_x + r_sub1
            },
            if toward_up || toward_left {
                new_end_y - r_sub2
            } else {
                new_end_y + r_sub2
            },
        );
        let handle_delta_q = (
            (self.handle_base * alpha.cos()),
            (self.handle_base * alpha.sin()),
        );
        let toward_updown_handle_delta1 = if toward_updown {
            handle_delta_q.1
        } else {
            handle_delta_q.0
        };
        let toward_updown_handle_delta0 = if toward_updown {
            handle_delta_q.0
        } else {
            handle_delta_q.1
        };
        let handle_bottom_left = (
            if toward_left {
                self.start_x + toward_updown_handle_delta1
            } else {
                self.start_x - toward_updown_handle_delta1
            },
            if toward_left {
                self.start_y - toward_updown_handle_delta0
            } else {
                self.start_y + toward_updown_handle_delta0
            },
        );
        let handle_bottom_right = (
            if toward_left {
                new_end_x + toward_updown_handle_delta1
            } else {
                new_end_x - toward_updown_handle_delta1
            },
            if toward_left {
                new_end_y - toward_updown_handle_delta0
            } else {
                new_end_y + toward_updown_handle_delta0
            },
        );
        let handle_top_left = (
            if toward_left {
                self.start_x - toward_updown_handle_delta1
            } else {
                self.start_x + toward_updown_handle_delta1
            },
            if toward_left {
                self.start_y + toward_updown_handle_delta0
            } else {
                self.start_y - toward_updown_handle_delta0
            },
        );
        let handle_top_right = (
            if toward_left {
                new_end_x - toward_updown_handle_delta1
            } else {
                new_end_x + toward_updown_handle_delta1
            },
            if toward_left {
                new_end_y + toward_updown_handle_delta0
            } else {
                new_end_y - toward_updown_handle_delta0
            },
        );
        let head_delta_q = (total_base * alpha.cos(), total_base * alpha.sin());
        let toward_updown_head_delta1 = if toward_updown {
            head_delta_q.1
        } else {
            head_delta_q.0
        };
        let toward_updown_head_delta0 = if toward_updown {
            head_delta_q.0
        } else {
            head_delta_q.1
        };
        let q = (
            if toward_left {
                new_end_x + toward_updown_head_delta1
            } else {
                new_end_x - toward_updown_head_delta1
            },
            if toward_left {
                new_end_y - toward_updown_head_delta0
            } else {
                new_end_y + toward_updown_head_delta0
            },
        );

        let s = (
            if toward_left {
                new_end_x - toward_updown_head_delta1
            } else {
                new_end_x + toward_updown_head_delta1
            },
            if toward_left {
                new_end_y + toward_updown_head_delta0
            } else {
                new_end_y - toward_updown_head_delta0
            },
        );
        
        format!("    <polygon id=\"{}\"\n{}", shape_id, {
            let mut shape_str = "          points=\"\n".to_string();
            shape_str.push_str(&format!("            {},{}\n", svg_round(handle_bottom_left.0, self.precision), svg_round(handle_bottom_left.1, self.precision)));
            shape_str.push_str(&format!("            {},{}\n", svg_round(handle_bottom_right.0, self.precision), svg_round(handle_bottom_right.1, self.precision)));
            shape_str.push_str(&format!("            {},{}\n", svg_round(q.0, self.precision), svg_round(q.1, self.precision)));
            shape_str.push_str(&format!("            {},{}\n", svg_round(r.0, self.precision), svg_round(r.1, self.precision)));
            shape_str.push_str(&format!("            {},{}\n", svg_round(s.0, self.precision), svg_round(s.1, self.precision)));
            shape_str.push_str(&format!("            {},{}\n", svg_round(handle_top_right.0, self.precision), svg_round(handle_top_right.1, self.precision)));
            shape_str.push_str(&format!("            {},{}\n", svg_round(handle_top_left.0, self.precision), svg_round(handle_top_left.1, self.precision)));
            shape_str.push_str(&format!("            \"/>\n"));
            shape_str
        })
    }

    pub fn unique(&self) -> String {
        format!("Arrow/start_x/{}/start_y/{}/end_x/{}/end_y/{}/handle_base/{}/head_base/{}/head_height/{}",
                self.start_x,
                self.start_y,
                self.end_x,
                self.end_y,
                self.handle_base,
                self.head_base,
                self.head_height)
    }
}
