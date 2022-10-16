pub struct Mouse {
    x: u16,
    y: u16,
    //left_click:
    //right-click:
    //middle-click:
}
impl Mouse {
    pub fn init() -> Self {
        let x: u16;
        let y: u16;
        (x, y) = Self::get_mouse_position();
        Self { x, y }
    }
    fn get_mouse_position() -> (u16, u16) { //Needs to be implemented
        (4, 4)
    }
}