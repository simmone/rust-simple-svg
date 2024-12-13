use crate::defines::widget::Widget;

pub struct Group {
    pub widget_list: Vec<Widget>,
}

impl Group {
    pub fn new() -> Self {
        Group {
            widget_list: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn check_new() {
        let group = Group::new();
        
        assert_eq!(group.widget_list.len(), 0);
    }
}
