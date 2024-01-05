// Linear time median search
use alloc::vec::Vec;
use core::cmp::PartialOrd;

pub fn mutable_median(input: &mut Vec<f64>) -> Option<f64> {
    let cmp = |a: &f64, b: &f64| a.partial_cmp(b).unwrap();

    let n = input.len();
    if n == 0 {
        return None;
    }
    if n % 2 == 1 {
        input.select_nth_unstable_by(n / 2, cmp);
        return Some(input[n / 2]);
    }

    input.select_nth_unstable_by(n / 2 - 1, cmp);
    input.select_nth_unstable_by(n / 2 - 0, cmp);
    let b: f64 = input[n / 2 - 1];
    let c: f64 = input[n / 2 - 0];
    Some((b + c) / 2.)
}

fn mutable_mad(input: &mut Vec<f64>) -> Option<f64> {
    let m = match mutable_median(input) {
        None => return None,
        Some(m) => m,
    };
    let mut a = input.iter().map(|e| (e - m).abs()).collect::<Vec<f64>>();
    return mutable_median(&mut a);
}

pub fn mutable_standard_deviation(input: &mut Vec<f64>) -> Option<f64> {
    // 1.0 / PPF(0.75)
    // PPF is normal distribution's percent point function
    let ppf34 = 1.482602218505602;
    match mutable_mad(input) {
        None => return None,
        Some(m) => return Some(ppf34 * m),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mutable_median() {
        let mut input = vec![-9., -6., -4., -1., -6., 5., 8., 5., 5., 4.];
        assert_eq!(mutable_median(&mut input), Some(1.5));

        let mut input = vec![
            15., 34., 26., -76., -19., 25., 93., -99., -52., 12., 6., -70., 59., 78., 69., -6.,
            -33., 2., -27.,
        ];
        assert_eq!(mutable_median(&mut input), Some(6.0));

        let mut input = vec![-19., 38., -45., 35., 36., 68., 26., -27., 52., 41.];
        assert_eq!(mutable_median(&mut input), Some(35.5));

        let mut input: Vec<f64> = vec![];
        assert_eq!(mutable_median(&mut input), None);

        let mut input = vec![50.];
        assert_eq!(mutable_median(&mut input), Some(50.));

        let mut input = vec![10., 11.];
        assert_eq!(mutable_median(&mut input), Some(10.5));
    }

    #[test]
    fn test_mutable_mad() {
        let mut a = vec![16., -16., -1., 8., -9., 4., -3., 17., 3., -7., 11., -1.];
        assert_eq!(mutable_mad(&mut a), Some(7.5));

        let mut a = vec![22., 1., -9., -35., -29., -40., -50., -45., 4.];
        assert_eq!(mutable_mad(&mut a), Some(20.0));

        let mut a = vec![-53., -36.];
        assert_eq!(mutable_mad(&mut a), Some(8.5));
    }

    #[test]
    fn test_mutable_standard_deviation() {
        // >>> np.random.normal(50., 10., 100)
        #[rustfmt::skip]
        let mut normal = vec![
            53.0832203 , 60.78675339, 49.15066951, 60.1084452 , 72.01118924,
            50.04284213, 52.83008308, 23.96785563, 35.51235652, 43.34002764,
            46.38651612, 44.12070351, 44.17867909, 50.98783254, 44.21536288,
            70.17936403, 48.84330478, 51.58408135, 49.24294933, 56.12224494,
            54.15417157, 58.76714865, 52.41643234, 48.81350439, 42.27442158,
            59.08548828, 40.58795014, 46.05835979, 61.0659236 , 42.13175052,
            52.97283003, 39.46370987, 52.007813  , 39.87764594, 47.84026502,
            54.53531844, 39.01183939, 43.53705067, 49.98653523, 60.4271226 ,
            28.35086716, 44.39726399, 43.61557885, 63.29068847, 41.32778574,
            51.68182699, 50.74441992, 47.43624869, 47.06234944, 55.33085634,
            60.1742633 , 53.26886399, 35.19542111, 56.83354548, 31.65618383,
            40.08374876, 50.15219264, 44.44536522, 48.30516233, 65.41939507,
            45.55690819, 55.68155501, 59.05170952, 45.17456062, 57.80619559,
            66.05259975, 46.00590789, 32.2621706 , 55.38730483, 45.73005193,
            45.71435278, 55.95660079, 55.62156553, 48.26003878, 31.2842824 ,
            55.10124146, 59.18713651, 49.60689857, 61.96388754, 30.00022221,
            60.35928071, 62.12555809, 46.91947312, 54.29469848, 37.60662842,
            47.93826864, 57.90926871, 44.36232644, 41.34588408, 42.27201939,
            51.36323355, 39.08440872, 53.04656841, 54.82787657, 46.40165516,
            25.48827449, 56.49926944, 42.0958349 , 33.46258109, 43.5237575 ];

        let expected = 9.427146244705945; // calculated by numpy.std

        match mutable_standard_deviation(&mut normal) {
            Some(stddev) => {
                assert!((stddev - expected).abs() < 0.5);
            }
            None => panic!(),
        }
    }
}
