use crate::traits::Filter;

pub struct tran_coil{
    pub vin:f64,
    pub vout:f64,
    pub iout:f64,
    pub fsw:f64,
    pub ind:f64
}


impl tran for tran_coil{
    fn calc_tran_coil(&self,time:f64)->f64 {
        1.0-(-1.0*time/(self.res*self.cap)).exp()
    }
}