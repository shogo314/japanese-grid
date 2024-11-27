pub mod base;
pub use base::*;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        // decimicroで初期化
        let coord_from_decimicro =
            GeoCoordinate::new_from_decimicro(327_903_862, 1_306_883_252).unwrap(); // 熊本駅
        assert_eq!(
            coord_from_decimicro.to_string(),
            "Lat: 32.7903862, Lon: 130.6883252"
        );

        // f32で初期化
        let coord_from_f64 = GeoCoordinate::new_from_f64(35.6895, 139.6917).unwrap(); // 東京の座標
        assert_eq!(
            coord_from_f64.to_string(),
            "Lat: 35.6895000, Lon: 139.6917000"
        );

        // 範囲外の値で初期化
        let invalid_coord = GeoCoordinate::new_from_decimicro(1_000_000_000, 2_000_000_000);
        assert_eq!(
            invalid_coord.err().unwrap(),
            "Invalid latitude: must be between -90 and 90 degrees in decimicro"
        );

        let mesh_code = coord_from_decimicro.to_second_mesh();
        assert_eq!(mesh_code, "493015");

        let result = from_second_mesh_code("493015").unwrap();
        assert_eq!(result.0.to_string(), "Lat: 32.7500000, Lon: 130.6250000");
        assert_eq!(result.1.to_string(), "Lat: 32.8333333, Lon: 130.7500000");

        const CO: GeoCoordinate = GeoCoordinate::new_from_decimicro_const(327_903_862, 1_306_883_252);
        assert_eq!(CO.lat_decimicro(), 327_903_862);
    }
}
