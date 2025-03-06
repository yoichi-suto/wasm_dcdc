use crate::traits::Filter;

pub struct CCurrent{
    pub dil_on:f64,
    pub dil_off:f64,
    pub intercept_on:f64,
    pub intercept_off:f64,
    pub ontime_cha:f64
}

fn heaviside(x: f64) -> f64 {
    if x >= 0.0 { 1.0 } else { 0.0 }
}

impl Filter for CCurrent{
    fn calc_time_response(&self, _time:f64)->f64 {
            (self.dil_on * _time + self.intercept_on) * (1.0-heaviside(_time-self.ontime_cha)) + (self.dil_off * _time + self.intercept_off) * heaviside(_time-self.ontime_cha)
    }
}

     #[test]
    fn test_calc_time_response() {
        let c_current = CCurrent {
            dil_on: 1.03,
            dil_off: -0.735,
            intercept_on: 0.464,
            intercept_off: 2.3,
            ontime_cha: 1.04,
        };

        let mut time = 0.0;
        while time <= 2.5 {
            let result = c_current.calc_time_response(time);
            println!("calc_time_response({:.1}) = {:.3}", time, result);
            time += 0.1;
        }
    }
    