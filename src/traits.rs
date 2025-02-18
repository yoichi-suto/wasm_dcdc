pub trait Filter{
    fn calc_time_response(&self,time:f64)->f64;
}