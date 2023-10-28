use crate::FREQUENCY_MAX;

pub struct Model {
    cfreq: [usize; 258],
    frozen: bool
}

impl Model {
    pub fn new() -> Model {
        let mut cfreq = [0; 258];
        for i in 0..258 {
            cfreq[i] = i;
        }
        Model{cfreq, frozen: false}
    }
    fn update(&mut self, c: usize){
        for i in (c as usize + 1)..258 {
            self.cfreq[i] += 1;
        }
        if self.cfreq[257] >= FREQUENCY_MAX {
            self.frozen = true;
        }
    }
    pub fn prob(&mut self, c: usize) -> (usize, usize, usize) {
        let p = (self.cfreq[c as usize], self.cfreq[c as usize], self.cfreq[257]);
        if !self.frozen {
            self.update(c as usize);
        }
        p
    }
    pub fn count(&self) -> usize {
        self.cfreq[257]
    }
    pub fn get_char(&mut self, scaled_value: usize) -> Option<((usize, usize, usize), usize)> {
        for i in 0..257 {
            if scaled_value < self.cfreq[i+1] {
                let c = i;
                let p = (self.cfreq[c as usize], self.cfreq[c as usize], self.cfreq[257]);
                if !self.frozen {
                    self.update(c as usize);
                }
                return Some((p, c));
            }
        }
        None
    }
}