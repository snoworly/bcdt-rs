
use time::PrimitiveDateTime;
use std::mem::transmute;

pub fn encode_datetime(epoch: i64) -> [u8; 4] {
    let dt = time::OffsetDateTime::from_unix_timestamp(epoch).unwrap();

    let mut ym : u8 = (dt.year() - 2022) as u8;
    ym <<= 4;
    ym |= dt.month() as u8;

    let d = dt.day() as u8;
    let h = dt.hour() as u8;
    let m = dt.minute() as u8;

    return [ym, d, h, m];
}

pub fn decode_datetime(datetime: [u8; 4]) -> i64 {
    let y8 = (datetime[0] & (15 << 4)) >> 4;
    let m8 = datetime[0] & 15;

    let y = y8 as i32 + 2022;
    let mm: time::Month = unsafe { transmute(m8 as u8) };
    let d = datetime[1];
    let h = datetime[2];
    let m = datetime[3];

    let date = time::Date::from_calendar_date(y, mm, d).unwrap();
    let time = time::Time::from_hms(h, m, 0).unwrap();

    return PrimitiveDateTime::new(date, time).assume_utc().unix_timestamp();
}
