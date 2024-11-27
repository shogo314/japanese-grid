#[derive(Debug, PartialEq)]
pub struct GeoCoordinate {
    lat: i32, // 10^-7 度単位
    lon: i32, // 10^-7 度単位
}

impl GeoCoordinate {
    /// decimicro (10^-7 度単位) で新しいGeoCoordinateを生成する関数
    pub fn new_from_decimicro(lat: i32, lon: i32) -> Result<Self, String> {
        if lat < -900_000_000 || lat > 900_000_000 {
            return Err(
                "Invalid latitude: must be between -90 and 90 degrees in decimicro".to_string(),
            );
        }
        if lon < -1_800_000_000 || lon > 1_800_000_000 {
            return Err(
                "Invalid longitude: must be between -180 and 180 degrees in decimicro".to_string(),
            );
        }
        Ok(Self { lat, lon })
    }

    /// decimicro (10^-7 度単位) で新しいGeoCoordinateを生成する関数
    pub const fn new_from_decimicro_const(lat: i32, lon: i32) -> Self {
        if lat < -900_000_000 || lat > 900_000_000 {
            panic!();
        }
        if lon < -1_800_000_000 || lon > 1_800_000_000 {
            panic!();
        }
        Self { lat, lon }
    }

    /// f64 (度単位) で新しいGeoCoordinateを生成する関数
    pub fn new_from_f64(lat: f64, lon: f64) -> Result<Self, String> {
        let lat_decimicro = (lat * 10_000_000.0).round() as i32;
        let lon_decimicro = (lon * 10_000_000.0).round() as i32;
        Self::new_from_decimicro(lat_decimicro, lon_decimicro)
    }

    /// 緯度を取得 (f64 度単位)
    pub fn lat_f64(&self) -> f64 {
        self.lat as f64 / 10_000_000.0
    }

    /// 経度を取得 (f64 度単位)
    pub fn lon_f64(&self) -> f64 {
        self.lon as f64 / 10_000_000.0
    }

    /// 緯度を取得 (decimicro単位)
    pub const fn lat_decimicro(&self) -> i32 {
        self.lat
    }

    /// 経度を取得 (decimicro単位)
    pub const fn lon_decimicro(&self) -> i32 {
        self.lon
    }

    /// 緯度と経度を文字列で表示 (度単位)
    pub fn to_string(&self) -> String {
        format!("Lat: {:.7}, Lon: {:.7}", self.lat_f64(), self.lon_f64())
    }

    pub fn to_second_mesh(&self) -> String {
        let lat = self.lat_f64();
        let lon = self.lon_f64();
        // 第1次メッシュ
        let lat_1st = (lat * 1.5).floor() as i32;
        let lon_1st = (lon - 100.0).floor() as i32;
        // 第2次メッシュ
        let lat_2nd = (lat * 1.5 % 1.0 * 8.0).floor() as i32;
        let lon_2nd = ((lon - 100.0) % 1.0 * 8.0).floor() as i32;
        // メッシュコード生成
        format!("{:02}{:02}{:01}{:01}", lat_1st, lon_1st, lat_2nd, lon_2nd)
    }
}

pub fn from_second_mesh_code(mesh_code: &str) -> Result<(GeoCoordinate, GeoCoordinate), String> {
    if mesh_code.len() != 6 {
        return Err("Invalid mesh code: must be 6 digits long".to_string());
    }

    // メッシュコードの解析
    let lat_1st: i32 = mesh_code[0..2]
        .parse()
        .map_err(|_| "Invalid latitude part")?;
    let lon_1st: i32 = mesh_code[2..4]
        .parse()
        .map_err(|_| "Invalid longitude part")?;
    let lat_2nd: i32 = mesh_code[4..5]
        .parse()
        .map_err(|_| "Invalid latitude fraction")?;
    let lon_2nd: i32 = mesh_code[5..6]
        .parse()
        .map_err(|_| "Invalid longitude fraction")?;

    // 緯度と経度の計算（度単位）
    let base_lat = lat_1st as f64 / 1.5; // 第1次メッシュ基準緯度
    let base_lon = lon_1st as f64 + 100.0; // 第1次メッシュ基準経度

    let lat_lower = base_lat + (lat_2nd as f64 / 12.0);
    let lat_upper = lat_lower + 1.0 / 12.0; // 5分 = 1/12度
    let lon_lower = base_lon + (lon_2nd as f64 / 8.0);
    let lon_upper = lon_lower + 1.0 / 8.0; // 7.5分 = 1/8度

    // GeoCoordinateインスタンスを生成
    let lower_left = GeoCoordinate::new_from_f64(lat_lower, lon_lower)?;
    let upper_right = GeoCoordinate::new_from_f64(lat_upper, lon_upper)?;

    Ok((lower_left, upper_right))
}
