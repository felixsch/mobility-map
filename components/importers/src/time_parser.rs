pub mod messy_time {
    use serde::de::{self, Visitor};
    use serde::{self, Deserializer};
    use sqlx::types::chrono::NaiveTime;
    use std::fmt;

    struct NaiveTimeVisitor;

    impl<'de> Visitor<'de> for NaiveTimeVisitor {
        type Value = NaiveTime;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a time string in the format hh:mm:ss")
        }

        fn visit_str<E>(self, value: &str) -> Result<NaiveTime, E>
        where
            E: de::Error,
        {
            let parts: Vec<&str> = value.split(':').collect();
            if parts.len() != 3 {
                return Err(E::custom(format!(
                    "Invalid time `{}` found. Expected hh:mm:ss format",
                    value
                )));
            }

            let parse_u32 = |p: &str| p.parse::<u32>().map_err(|_| E::custom("Cannot parse u32"));

            // There is a possibility that a timetable entry has hours
            // like 25:00:01 or 24:00:03. Instead of formatting times in 00 or 03,
            // it's 24 or 27.
            let h = parse_u32(parts[0])? % 24;
            let m = parse_u32(parts[1])?;
            let s = parse_u32(parts[2])?;

            NaiveTime::from_hms_opt(h, m, s)
                .ok_or_else(|| E::custom(format!("Could not assemble time from {}:{}:{}", h, m, s)))
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<NaiveTime, D::Error> {
        deserializer.deserialize_str(NaiveTimeVisitor)
    }
}
