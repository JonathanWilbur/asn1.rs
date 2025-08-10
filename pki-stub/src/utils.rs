use chrono::{DateTime, FixedOffset, Local, TimeZone, Utc};
use wildboar_asn1::{GeneralizedTime, UTCTime};

// I thought about implementing this as a feature in wildboar_asn1, but chrono
// is not version 1.0.0 or higher.
pub(crate) fn gt_to_chrono(gt: &GeneralizedTime) -> Result<DateTime<Utc>, ()> {
    let (min, maybe_sec) = gt.min_and_sec.unwrap_or((0, Some(0)));
    let sec = maybe_sec.unwrap_or(0);
    if let Some(offset) = gt.utc_offset.as_ref() {
        let tz =
            FixedOffset::east_opt((offset.hour as i32 * 3600) + offset.minute as i32).ok_or(())?;
        let t = tz
            .with_ymd_and_hms(
                gt.date.year as i32,
                gt.date.month as u32,
                gt.date.day as u32,
                gt.hour as u32,
                min as u32,
                sec as u32,
            )
            .earliest()
            .ok_or(())?;
        Ok(t.to_utc())
    } else {
        let t = Local
            .with_ymd_and_hms(
                gt.date.year as i32,
                gt.date.month as u32,
                gt.date.day as u32,
                gt.hour as u32,
                min as u32,
                sec as u32,
            )
            .earliest()
            .ok_or(())?;
        Ok(t.to_utc())
    }
}

pub(crate) fn utctime_to_chrono(t: &UTCTime) -> Result<DateTime<Utc>, ()> {
    let maybe_tz =
        FixedOffset::east_opt((t.utc_offset.hour as i32 * 3600) + t.utc_offset.minute as i32);
    let tz = match maybe_tz {
        Some(x) => x,
        None => return Err(()),
    };
    let maybe_t = tz
        .with_ymd_and_hms(
            t.get_full_year_for_pki() as i32,
            t.month as u32,
            t.day as u32,
            t.hour as u32,
            t.minute as u32,
            t.second as u32,
        )
        .earliest();
    let t = match maybe_t {
        Some(x) => x,
        None => return Err(()),
    };
    Ok(t.to_utc())
}
