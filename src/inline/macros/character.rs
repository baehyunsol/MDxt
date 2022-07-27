use crate::utils::into_v16;
use std::collections::{HashSet, HashMap};
use lazy_static::lazy_static;

lazy_static! {

    static ref INDIRECT: Vec<(Vec<u16>, Vec<u16>)> = vec![
        (into_v16("won"), into_v16("8361")),
        (into_v16("intint"), into_v16("8748")),
        (into_v16("oint"), into_v16("8750")),
        (into_v16("therefore"), into_v16("there4")),
        (into_v16("because"), into_v16("8757")),
        (into_v16("nsup"), into_v16("8837")),
        (into_v16("earth"), into_v16("127759")),
        (into_v16("moon"), into_v16("127769")),
        (into_v16("sun"), into_v16("127774")),
        (into_v16("smile"), into_v16("128516")),
        (into_v16("love"), into_v16("128525")),
        (into_v16("cry"), into_v16("128546")),
    ];

    pub static ref DIRECT: HashSet<Vec<u16>> = {
        let vec = vec![
            into_v16("bull"),
            into_v16("euro"),
            into_v16("real"),
            into_v16("trade"),
            into_v16("copy"),
            into_v16("ohm"),
            into_v16("larr"),
            into_v16("uarr"),
            into_v16("rarr"),
            into_v16("darr"),
            into_v16("forall"),
            into_v16("part"),
            into_v16("exist"),
            into_v16("empty"),
            into_v16("nabla"),
            into_v16("isin"),
            into_v16("notin"),
            into_v16("ni"),
            into_v16("prod"),
            into_v16("sum"),
            into_v16("prop"),
            into_v16("infin"),
            into_v16("and"),
            into_v16("or"),
            into_v16("cap"),
            into_v16("cup"),
            into_v16("int"),
            into_v16("there4"),
            into_v16("cong"),
            into_v16("asymp"),
            into_v16("ne"),
            into_v16("equiv"),
            into_v16("le"),
            into_v16("ge"),
            into_v16("sub"),
            into_v16("sup"),
            into_v16("nsub"),
            into_v16("sube"),
            into_v16("supe"),
        ];
        let mut result = HashSet::with_capacity(vec.len());

        for d in vec.iter() {
            result.insert(d.clone());
        }

        result
    };

    static ref INDIRECT_NAMES: HashSet<Vec<u16>> = {
        let mut result = HashSet::with_capacity(INDIRECT.len());

        for (i, _) in INDIRECT.iter() {
            result.insert(i.clone());
        }

        result
    };

    pub static ref INDIRECT_MAPS: HashMap<Vec<u16>, Vec<u16>> = {
        let mut result = HashMap::with_capacity(INDIRECT.len());

        for (key, value) in INDIRECT.iter() {
            result.insert(key.clone(), value.clone());
        }

        result
    };

    pub static ref CHAR_NAMES: HashSet<Vec<u16>> = DIRECT.union(&INDIRECT_NAMES).map(|name| name.clone()).collect();

}
