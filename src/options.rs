use strum::EnumIter;

// 車牌樣式
#[derive(Debug, Copy, Clone, EnumIter, PartialEq, Eq, Hash)]
pub enum PlateVer {
    Old = 1, // 原型式車牌
    New = 2, // 新式車牌
}

impl PlateVer {
    pub fn as_str(&self) -> &str {
        match self {
            PlateVer::Old => "1",
            PlateVer::New => "2",
        }
    }
    pub fn as_name(&self) -> &str {
        match self {
            PlateVer::Old => "原型式車牌",
            PlateVer::New => "新式車牌",
        }
    }
}

// 車種別
#[derive(Debug, Copy, Clone, EnumIter, PartialEq, Eq, Hash)]
pub enum VehicleType {
    Car,        // 汽車
    Motorcycle, // 機車
    Trailer,    // 拖車
}

impl VehicleType {
    pub fn as_str(&self) -> &str {
        match self {
            VehicleType::Car => "C",
            VehicleType::Motorcycle => "M",
            VehicleType::Trailer => "T",
        }
    }

    pub fn as_name(&self) -> &str {
        match self {
            VehicleType::Car => "汽車",
            VehicleType::Motorcycle => "機車",
            VehicleType::Trailer => "拖車",
        }
    }
}

// 管轄監理單位
#[derive(Debug, Copy, Clone, EnumIter, PartialEq, Eq, Hash)]
pub enum Region {
    Taipei = 2,            // 臺北市
    Kaohsiung = 3,         // 高雄市
    TaipeiDistrict = 4,    // 臺北區
    HsinchuDistrict = 5,   // 新竹區
    TaichungDistrict = 6,  // 臺中區
    ChiayiDistrict = 7,    // 嘉義區
    KaohsiungDistrict = 8, // 高雄區
}

impl Region {
    pub fn as_str(&self) -> &str {
        match self {
            Region::Taipei => "2",
            Region::Kaohsiung => "3",
            Region::TaipeiDistrict => "4",
            Region::HsinchuDistrict => "5",
            Region::TaichungDistrict => "6",
            Region::ChiayiDistrict => "7",
            Region::KaohsiungDistrict => "8",
        }
    }
    pub fn as_name(&self) -> &str {
        match self {
            Region::Taipei => "臺北市",
            Region::Kaohsiung => "高雄市",
            Region::TaipeiDistrict => "臺北區",
            Region::HsinchuDistrict => "新竹區",
            Region::TaichungDistrict => "臺中區",
            Region::ChiayiDistrict => "嘉義區",
            Region::KaohsiungDistrict => "高雄區",
        }
    }
}

// 能源別
#[derive(Debug, Copy, Clone, EnumIter, PartialEq, Eq, Hash)]
pub enum EnegryType {
    NonElectric, // 非電能
    Electric,    // 電能
}

impl EnegryType {
    pub fn as_str(&self) -> &str {
        match self {
            EnegryType::NonElectric => "C",
            EnegryType::Electric => "E",
        }
    }

    pub fn as_name(&self) -> &str {
        match self {
            EnegryType::NonElectric => "非電能",
            EnegryType::Electric => "電能",
        }
    }
}

// 車牌別
#[derive(Debug, Copy, Clone, EnumIter, PartialEq, Eq, Hash)]
pub enum PlateType {
    // 汽車(非電能)
    CarOwn,                  // 自用小客貨車
    CarRent,                 // 租賃小客貨車
    CarBusiness,             // 營業小客車
    CarBusinessT,            // 營業小貨車
    CarBigOwn,               // 自用大客車
    CarBigRent,              // 自用大貨車
    CarBigBusiness,          // 營業大客車
    CarBigBusinessT,         // 營業大貨車
    CarBigBusinessContainer, // 營業貨櫃曳引
    CarTour,                 // 遊覽大客車
    // 汽車(純電能)
    CarElectricOwn,         // 電動自小客
    CarElectricRent,        // 電動租賃車
    CarElectricBusiness,    // 電動小營客車
    CarElectricBigBusiness, // 電動大營客車
    // 機車(非電能)
    Motorcycle550ccBelow,  // 550cc以下重機
    Motorcycle550ccAbove,  // 550cc以上重機
    MotorcycleNormalHeavy, // 普通重型機車
    MotorcycleNormalLight, // 普通輕型機車
    // 機車(純電能)
    MotorcycleElectric550ccBelow,  // 電動550cc以下重機
    MotorcycleElectric550ccAbove,  // 電動550cc以上重機
    MotorcycleElectricNormalHeavy, // 電動普通重型機車
    MotorcycleElectricNormalLight, // 電動普通輕型機車
    // 拖車(非電能)
    TrailerOwn,      // 自用拖車
    TrailerBusiness, // 營業拖車
}

// Get plate type by vehicle type and energy type
pub fn get_plate_type(vehicle_type: VehicleType, energy_type: EnegryType) -> Vec<PlateType> {
    match vehicle_type {
        VehicleType::Car => match energy_type {
            EnegryType::NonElectric => vec![
                PlateType::CarOwn,
                PlateType::CarRent,
                PlateType::CarBusiness,
                PlateType::CarBusinessT,
                PlateType::CarBigOwn,
                PlateType::CarBigRent,
                PlateType::CarBigBusiness,
                PlateType::CarBigBusinessT,
                PlateType::CarBigBusinessContainer,
                PlateType::CarTour,
            ],
            EnegryType::Electric => vec![
                PlateType::CarElectricOwn,
                PlateType::CarElectricRent,
                PlateType::CarElectricBusiness,
                PlateType::CarElectricBigBusiness,
            ],
        },
        VehicleType::Motorcycle => match energy_type {
            EnegryType::NonElectric => vec![
                PlateType::Motorcycle550ccBelow,
                PlateType::Motorcycle550ccAbove,
                PlateType::MotorcycleNormalHeavy,
                PlateType::MotorcycleNormalLight,
            ],
            EnegryType::Electric => vec![
                PlateType::MotorcycleElectric550ccBelow,
                PlateType::MotorcycleElectric550ccAbove,
                PlateType::MotorcycleElectricNormalHeavy,
                PlateType::MotorcycleElectricNormalLight,
            ],
        },
        VehicleType::Trailer => match energy_type {
            EnegryType::NonElectric => vec![PlateType::TrailerOwn, PlateType::TrailerBusiness],
            EnegryType::Electric => vec![],
        },
    }
}

impl PlateType {
    pub fn as_str(&self) -> &str {
        match self {
            PlateType::CarOwn => "1",
            PlateType::CarRent => "2",
            PlateType::CarBusiness => "3",
            PlateType::CarBusinessT => "4",
            PlateType::CarBigOwn => "5",
            PlateType::CarBigRent => "6",
            PlateType::CarBigBusiness => "7",
            PlateType::CarBigBusinessT => "8",
            PlateType::CarBigBusinessContainer => "9",
            PlateType::CarTour => "a",
            PlateType::CarElectricOwn => "g",
            PlateType::CarElectricRent => "h",
            PlateType::CarElectricBusiness => "i",
            PlateType::CarElectricBigBusiness => "j",
            PlateType::Motorcycle550ccBelow => "F",
            PlateType::Motorcycle550ccAbove => "G",
            PlateType::MotorcycleNormalHeavy => "H",
            PlateType::MotorcycleNormalLight => "L",
            PlateType::MotorcycleElectric550ccBelow => "N",
            PlateType::MotorcycleElectric550ccAbove => "O",
            PlateType::MotorcycleElectricNormalHeavy => "P",
            PlateType::MotorcycleElectricNormalLight => "Q",
            PlateType::TrailerOwn => "t",
            PlateType::TrailerBusiness => "u",
        }
    }

    pub fn as_name(&self) -> &str {
        match self {
            PlateType::CarOwn => "自用小客貨車",
            PlateType::CarRent => "租賃小客貨車",
            PlateType::CarBusiness => "營業小客車",
            PlateType::CarBusinessT => "營業小貨車",
            PlateType::CarBigOwn => "自用大客車",
            PlateType::CarBigRent => "自用大貨車",
            PlateType::CarBigBusiness => "營業大客車",
            PlateType::CarBigBusinessT => "營業大貨車",
            PlateType::CarBigBusinessContainer => "營業貨櫃曳引",
            PlateType::CarTour => "遊覽大客車",
            PlateType::CarElectricOwn => "電動自小客",
            PlateType::CarElectricRent => "電動租賃車",
            PlateType::CarElectricBusiness => "電動小營客車",
            PlateType::CarElectricBigBusiness => "電動大營客車",
            PlateType::Motorcycle550ccBelow => "550cc以下重機",
            PlateType::Motorcycle550ccAbove => "550cc以上重機",
            PlateType::MotorcycleNormalHeavy => "普通重型機車",
            PlateType::MotorcycleNormalLight => "普通輕型機車",
            PlateType::MotorcycleElectric550ccBelow => "電動550cc以下重機",
            PlateType::MotorcycleElectric550ccAbove => "電動550cc以上重機",
            PlateType::MotorcycleElectricNormalHeavy => "電動普通重型機車",
            PlateType::MotorcycleElectricNormalLight => "電動普通輕型機車",
            PlateType::TrailerOwn => "自用拖車",
            PlateType::TrailerBusiness => "營業拖車",
        }
    }

    pub fn energy_type(&self) -> EnegryType {
        match self {
            PlateType::CarOwn
            | PlateType::CarRent
            | PlateType::CarBusiness
            | PlateType::CarBusinessT
            | PlateType::CarBigOwn
            | PlateType::CarBigRent
            | PlateType::CarBigBusiness
            | PlateType::CarBigBusinessT
            | PlateType::CarBigBusinessContainer
            | PlateType::CarTour
            | PlateType::Motorcycle550ccBelow
            | PlateType::Motorcycle550ccAbove
            | PlateType::MotorcycleNormalHeavy
            | PlateType::MotorcycleNormalLight
            | PlateType::TrailerOwn
            | PlateType::TrailerBusiness => EnegryType::NonElectric,
            PlateType::CarElectricOwn
            | PlateType::CarElectricRent
            | PlateType::CarElectricBusiness
            | PlateType::CarElectricBigBusiness
            | PlateType::MotorcycleElectric550ccBelow
            | PlateType::MotorcycleElectric550ccAbove
            | PlateType::MotorcycleElectricNormalHeavy
            | PlateType::MotorcycleElectricNormalLight => EnegryType::Electric,
        }
    }

    pub fn vehicle_type(&self) -> VehicleType {
        match self {
            PlateType::CarOwn
            | PlateType::CarRent
            | PlateType::CarBusiness
            | PlateType::CarBusinessT
            | PlateType::CarBigOwn
            | PlateType::CarBigRent
            | PlateType::CarBigBusiness
            | PlateType::CarBigBusinessT
            | PlateType::CarBigBusinessContainer
            | PlateType::CarTour
            | PlateType::CarElectricOwn
            | PlateType::CarElectricRent
            | PlateType::CarElectricBusiness
            | PlateType::CarElectricBigBusiness => VehicleType::Car,
            PlateType::Motorcycle550ccBelow
            | PlateType::Motorcycle550ccAbove
            | PlateType::MotorcycleNormalHeavy
            | PlateType::MotorcycleNormalLight
            | PlateType::MotorcycleElectric550ccBelow
            | PlateType::MotorcycleElectric550ccAbove
            | PlateType::MotorcycleElectricNormalHeavy
            | PlateType::MotorcycleElectricNormalLight => VehicleType::Motorcycle,
            PlateType::TrailerOwn | PlateType::TrailerBusiness => VehicleType::Trailer,
        }
    }
}

// 監理站
#[derive(Debug, Copy, Clone, EnumIter, PartialEq, Eq, Hash)]
pub enum Station {
    // 臺北市
    TaipeiCity = 20, // 臺北市區監理所
    Shilin = 21,     // 士林監理站
    Keelung = 25,    // 基隆監理站
    Kinmen = 26,     // 金門監理站
    Lienchiang = 28, // 連江監理站
    // 高雄市
    KaohsiungCity = 30, // 高雄市區監理所
    Lingya = 31,        // 苓雅監理站
    Qishan = 33,        // 旗山監理站
    // 臺北區
    TaipeiDistrict = 40, // 臺北區監理所
    Banqiao = 41,        // 板橋監理站
    Yilan = 43,          // 宜蘭監理站
    Hualien = 44,        // 花蓮監理站
    Yuli = 45,           // 玉里監理分站
    Luzhou = 46,         // 蘆洲監理站
    // 新竹區
    HsinchuDistrict = 50, // 新竹區監理所
    HsinchuCity = 51,     // 新竹市監理站
    Taoyuan = 52,         // 桃園監理站
    Zhongli = 53,         // 中壢監理站
    Miaoli = 54,          // 苗栗監理站
    // 臺中區
    TaichungDistrict = 60, // 臺中區監理所
    TaichungCity = 61,     // 臺中市監理站
    Puli = 62,             // 埔里監理分站
    Fengyuan = 63,         // 豐原監理站
    Changhua = 64,         // 彰化監理站
    Nantou = 65,           // 南投監理站
    // 嘉義區
    ChiayiDistrict = 70, // 嘉義區監理所
    Dongshi = 71,        // 東勢監理分站
    Yunlin = 72,         // 雲林監理站
    Xinying = 73,        // 新營監理站
    Tainan = 74,         // 臺南監理站
    Madou = 75,          // 麻豆監理站
    ChiayiCity = 76,     // 嘉義市監理站
    // 高雄區
    KaohsiungDistrict = 80, // 高雄區監理所
    Taitung = 81,           // 臺東監理站
    Pingtung = 82,          // 屏東監理站
    Hengchun = 83,          // 恆春監理分站
    Penghu = 84,            // 澎湖監理站
}

// Get station by region
pub fn get_station(region: Region) -> Vec<Station> {
    match region {
        Region::Taipei => vec![
            Station::TaipeiCity,
            Station::Shilin,
            Station::Keelung,
            Station::Kinmen,
            Station::Lienchiang,
        ],
        Region::Kaohsiung => vec![Station::KaohsiungCity, Station::Lingya, Station::Qishan],
        Region::TaipeiDistrict => vec![
            Station::TaipeiDistrict,
            Station::Banqiao,
            Station::Yilan,
            Station::Hualien,
            Station::Yuli,
            Station::Luzhou,
        ],
        Region::HsinchuDistrict => vec![
            Station::HsinchuDistrict,
            Station::HsinchuCity,
            Station::Taoyuan,
            Station::Zhongli,
            Station::Miaoli,
        ],
        Region::TaichungDistrict => vec![
            Station::TaichungDistrict,
            Station::TaichungCity,
            Station::Puli,
            Station::Fengyuan,
            Station::Changhua,
            Station::Nantou,
        ],
        Region::ChiayiDistrict => vec![
            Station::ChiayiDistrict,
            Station::Dongshi,
            Station::Yunlin,
            Station::Xinying,
            Station::Tainan,
            Station::Madou,
            Station::ChiayiCity,
        ],
        Region::KaohsiungDistrict => vec![
            Station::KaohsiungDistrict,
            Station::Taitung,
            Station::Pingtung,
            Station::Hengchun,
            Station::Penghu,
        ],
    }
}

impl Station {
    pub fn as_str(&self) -> &str {
        match self {
            Station::TaipeiCity => "20",
            Station::Shilin => "21",
            Station::Keelung => "25",
            Station::Kinmen => "26",
            Station::Lienchiang => "28",
            Station::KaohsiungCity => "30",
            Station::Lingya => "31",
            Station::Qishan => "33",
            Station::TaipeiDistrict => "40",
            Station::Banqiao => "41",
            Station::Yilan => "43",
            Station::Hualien => "44",
            Station::Yuli => "45",
            Station::Luzhou => "46",
            Station::HsinchuDistrict => "50",
            Station::HsinchuCity => "51",
            Station::Taoyuan => "52",
            Station::Zhongli => "53",
            Station::Miaoli => "54",
            Station::TaichungDistrict => "60",
            Station::TaichungCity => "61",
            Station::Puli => "62",
            Station::Fengyuan => "63",
            Station::Changhua => "64",
            Station::Nantou => "65",
            Station::ChiayiDistrict => "70",
            Station::Dongshi => "71",
            Station::Yunlin => "72",
            Station::Xinying => "73",
            Station::Tainan => "74",
            Station::Madou => "75",
            Station::ChiayiCity => "76",
            Station::KaohsiungDistrict => "80",
            Station::Taitung => "81",
            Station::Pingtung => "82",
            Station::Hengchun => "83",
            Station::Penghu => "84",
        }
    }
    pub fn as_name(&self) -> &str {
        match self {
            Station::TaipeiCity => "臺北市區監理所",
            Station::Shilin => "士林監理站",
            Station::Keelung => "基隆監理站",
            Station::Kinmen => "金門監理站",
            Station::Lienchiang => "連江監理站",
            Station::KaohsiungCity => "高雄市區監理所",
            Station::Lingya => "苓雅監理站",
            Station::Qishan => "旗山監理站",
            Station::TaipeiDistrict => "臺北區監理所",
            Station::Banqiao => "板橋監理站",
            Station::Yilan => "宜蘭監理站",
            Station::Hualien => "花蓮監理站",
            Station::Yuli => "玉里監理分站",
            Station::Luzhou => "蘆洲監理站",
            Station::HsinchuDistrict => "新竹區監理所",
            Station::HsinchuCity => "新竹市監理站",
            Station::Taoyuan => "桃園監理站",
            Station::Zhongli => "中壢監理站",
            Station::Miaoli => "苗栗監理站",
            Station::TaichungDistrict => "臺中區監理所",
            Station::TaichungCity => "臺中市監理站",
            Station::Puli => "埔里監理分站",
            Station::Fengyuan => "豐原監理站",
            Station::Changhua => "彰化監理站",
            Station::Nantou => "南投監理站",
            Station::ChiayiDistrict => "嘉義區監理所",
            Station::Dongshi => "東勢監理分站",
            Station::Yunlin => "雲林監理站",
            Station::Xinying => "新營監理站",
            Station::Tainan => "臺南監理站",
            Station::Madou => "麻豆監理站",
            Station::ChiayiCity => "嘉義市監理站",
            Station::KaohsiungDistrict => "高雄區監理所",
            Station::Taitung => "臺東監理站",
            Station::Pingtung => "屏東監理站",
            Station::Hengchun => "恆春監理分站",
            Station::Penghu => "澎湖監理站",
        }
    }
}

// 領牌地點(窗口地點)
#[derive(Debug, Copy, Clone, EnumIter, PartialEq, Eq, Hash)]
pub enum WindowNo {
    One = 1,
}

impl WindowNo {
    pub fn as_str(&self) -> &str {
        match self {
            WindowNo::One => "01",
        }
    }
}
