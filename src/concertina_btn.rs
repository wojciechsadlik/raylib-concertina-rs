pub struct ConcertinaBtn {
    btn_nr: String,
    is_pushing: bool
}

impl ConcertinaBtn {
    pub fn new(btn_nr: String, is_pushing: bool) -> ConcertinaBtn {
        ConcertinaBtn {
            btn_nr: btn_nr,
            is_pushing: is_pushing
        }
    }

    pub fn to_string(&self) -> String {
        self.btn_nr.clone() + if self.is_pushing { "ps" } else { "pl" }
    }
}
