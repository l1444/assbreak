///
/// It is a structure to be able to filter strings.
/// 
pub struct Filter {
    str: String,
    space: bool,
    enter: bool
}

///
/// Examples : 
/// ```
/// Filter::new(String::from("\n\rlolxptdrrr\r"), true, true).filter_now();
/// 
/// 
impl Filter {

    ///
    /// Create a new instance for Filter.
    /// 
    /// 
    pub fn new(str: String, space: bool, enter: bool) -> Self {
        Filter {
            str,
            space,
            enter
        }
    }

    /// 
    /// Filter :)
    /// 
    pub fn filter_now(&self) -> String {
        return if &self.str != "" {
            if &self.space == &true && &self.enter == &true {
                self.str.replace("\r", "").replace("\n", "")
            } else if &self.space == &false && &self.enter == &true {
                self.str.replace("\r", "")
            } else if &self.space == &true && &self.enter == &false {
                self.str.replace("\n", "")
            } else if &self.space == &false && &self.enter == &false {
                self.str.to_string()
            } else {
                "".to_string()
            }
        } else {
            "".to_string()
        }
    }

}