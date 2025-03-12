
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Event {
    Pressed,
    Released,
}

impl Event {
    pub fn new(pressed: bool) -> Self {
        if pressed {
            Self::Pressed
        } else {
            Self::Released
        }
    }
}

pub fn events<const W: usize, const H: usize>(scan: &[[bool; W]; H], prev_scan: &[[bool; W]; H]) -> impl Iterator<Item = ((usize, usize), Event)> {
    (0..H).flat_map(move |y| {
        (0..W).filter_map(move |x| {
            (scan[y][x] != prev_scan[y][x])
                .then(|| ((x, y), Event::new(scan[y][x])))
        })
    })
}