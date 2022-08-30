use crate::utils::into_v16;
use lazy_static::lazy_static;
use std::collections::{HashSet, HashMap};

lazy_static! {

    static ref INDIRECT_MAPPINGS_VEC: Vec<(Vec<u16>, Vec<u16>)> = vec![
        (into_v16("won"), into_v16("8361")),
        (into_v16("intint"), into_v16("8748")),
        (into_v16("oint"), into_v16("8750")),
        (into_v16("therefore"), into_v16("there4")),
        (into_v16("because"), into_v16("8757")),
        (into_v16("nsup"), into_v16("8837")),
        (into_v16("star"), into_v16("8902")),
        (into_v16("bigalpha"), into_v16("Alpha")),
        (into_v16("bigbeta"), into_v16("Beta")),
        (into_v16("biggamma"), into_v16("Gamma")),
        (into_v16("bigdelta"), into_v16("Delta")),
        (into_v16("bigepsilon"), into_v16("Epsilon")),
        (into_v16("bigzeta"), into_v16("Zeta")),
        (into_v16("bigeta"), into_v16("Eta")),
        (into_v16("bigtheta"), into_v16("Theta")),
        (into_v16("bigiota"), into_v16("Iota")),
        (into_v16("bigkappa"), into_v16("Kappa")),
        (into_v16("biglambda"), into_v16("Lambda")),
        (into_v16("bigmu"), into_v16("Mu")),
        (into_v16("bignu"), into_v16("Nu")),
        (into_v16("bigxi"), into_v16("Xi")),
        (into_v16("bigomicron"), into_v16("Omicron")),
        (into_v16("bigpi"), into_v16("Pi")),
        (into_v16("bigrho"), into_v16("Rho")),
        (into_v16("bigsigma"), into_v16("Sigma")),
        (into_v16("bigtau"), into_v16("Tau")),
        (into_v16("bigupsilon"), into_v16("Upsilon")),
        (into_v16("bigphi"), into_v16("Phi")),
        (into_v16("bigchi"), into_v16("Chi")),
        (into_v16("bigpsi"), into_v16("Psi")),
        (into_v16("bigomega"), into_v16("Omega")),
    ];

    pub static ref DIRECT_MAPPINGS: HashSet<Vec<u16>> = {
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
            into_v16("alpha"),
            into_v16("beta"),
            into_v16("gamma"),
            into_v16("delta"),
            into_v16("epsilon"),
            into_v16("zeta"),
            into_v16("eta"),
            into_v16("theta"),
            into_v16("iota"),
            into_v16("kappa"),
            into_v16("lambda"),
            into_v16("mu"),
            into_v16("nu"),
            into_v16("xi"),
            into_v16("omicron"),
            into_v16("pi"),
            into_v16("rho"),
            into_v16("sigma"),
            into_v16("tau"),
            into_v16("upsilon"),
            into_v16("phi"),
            into_v16("chi"),
            into_v16("psi"),
            into_v16("omega"),
        ];
        let mut result = HashSet::with_capacity(vec.len());

        for d in vec.iter() {
            result.insert(d.clone());
        }

        result
    };

    static ref INDIRECT_MAPPING_NAMES: HashSet<Vec<u16>> = {
        let mut result = HashSet::with_capacity(INDIRECT_MAPPINGS_VEC.len());

        for (i, _) in INDIRECT_MAPPINGS_VEC.iter() {
            result.insert(i.clone());
        }

        result
    };

    pub static ref INDIRECT_MAPPINGS: HashMap<Vec<u16>, Vec<u16>> = {
        let mut result = HashMap::with_capacity(INDIRECT_MAPPINGS_VEC.len());

        for (key, value) in INDIRECT_MAPPINGS_VEC.iter() {
            result.insert(key.clone(), value.clone());
        }

        result
    };

    pub static ref CHAR_NAMES: HashSet<Vec<u16>> = DIRECT_MAPPINGS.union(&INDIRECT_MAPPING_NAMES).map(|name| name.clone()).collect();

}
