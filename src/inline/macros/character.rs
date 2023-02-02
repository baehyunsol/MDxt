use crate::utils::into_v32;
use lazy_static::lazy_static;
use std::collections::{HashSet, HashMap};

lazy_static! {

    static ref INDIRECT_MAPPINGS_VEC: Vec<(Vec<u32>, Vec<u32>)> = vec![
        (into_v32("won"), into_v32("8361")),
        (into_v32("intint"), into_v32("8748")),
        (into_v32("oint"), into_v32("8750")),
        (into_v32("therefore"), into_v32("there4")),
        (into_v32("because"), into_v32("8757")),
        (into_v32("nsup"), into_v32("8837")),
        (into_v32("star"), into_v32("8902")),
        (into_v32("bigalpha"), into_v32("Alpha")),
        (into_v32("bigbeta"), into_v32("Beta")),
        (into_v32("biggamma"), into_v32("Gamma")),
        (into_v32("bigdelta"), into_v32("Delta")),
        (into_v32("bigepsilon"), into_v32("Epsilon")),
        (into_v32("bigzeta"), into_v32("Zeta")),
        (into_v32("bigeta"), into_v32("Eta")),
        (into_v32("bigtheta"), into_v32("Theta")),
        (into_v32("bigiota"), into_v32("Iota")),
        (into_v32("bigkappa"), into_v32("Kappa")),
        (into_v32("biglambda"), into_v32("Lambda")),
        (into_v32("bigmu"), into_v32("Mu")),
        (into_v32("bignu"), into_v32("Nu")),
        (into_v32("bigxi"), into_v32("Xi")),
        (into_v32("bigomicron"), into_v32("Omicron")),
        (into_v32("bigpi"), into_v32("Pi")),
        (into_v32("bigrho"), into_v32("Rho")),
        (into_v32("bigsigma"), into_v32("Sigma")),
        (into_v32("bigtau"), into_v32("Tau")),
        (into_v32("bigupsilon"), into_v32("Upsilon")),
        (into_v32("bigphi"), into_v32("Phi")),
        (into_v32("bigchi"), into_v32("Chi")),
        (into_v32("bigpsi"), into_v32("Psi")),
        (into_v32("bigomega"), into_v32("Omega")),
    ];

    pub static ref DIRECT_MAPPINGS: HashSet<Vec<u32>> = {
        let vec = vec![
            into_v32("bull"),
            into_v32("euro"),
            into_v32("real"),
            into_v32("trade"),
            into_v32("copy"),
            into_v32("ohm"),
            into_v32("larr"),
            into_v32("uarr"),
            into_v32("rarr"),
            into_v32("darr"),
            into_v32("forall"),
            into_v32("part"),
            into_v32("exist"),
            into_v32("empty"),
            into_v32("nabla"),
            into_v32("isin"),
            into_v32("notin"),
            into_v32("ni"),
            into_v32("prod"),
            into_v32("sum"),
            into_v32("prop"),
            into_v32("infin"),
            into_v32("and"),
            into_v32("or"),
            into_v32("cap"),
            into_v32("cup"),
            into_v32("int"),
            into_v32("there4"),
            into_v32("cong"),
            into_v32("asymp"),
            into_v32("ne"),
            into_v32("equiv"),
            into_v32("le"),
            into_v32("ge"),
            into_v32("sub"),
            into_v32("sup"),
            into_v32("nsub"),
            into_v32("sube"),
            into_v32("supe"),
            into_v32("alpha"),
            into_v32("beta"),
            into_v32("gamma"),
            into_v32("delta"),
            into_v32("epsilon"),
            into_v32("zeta"),
            into_v32("eta"),
            into_v32("theta"),
            into_v32("iota"),
            into_v32("kappa"),
            into_v32("lambda"),
            into_v32("mu"),
            into_v32("nu"),
            into_v32("xi"),
            into_v32("omicron"),
            into_v32("pi"),
            into_v32("rho"),
            into_v32("sigma"),
            into_v32("tau"),
            into_v32("upsilon"),
            into_v32("phi"),
            into_v32("chi"),
            into_v32("psi"),
            into_v32("omega"),
        ];
        let mut result = HashSet::with_capacity(vec.len());

        for d in vec.iter() {
            result.insert(d.clone());
        }

        result
    };

    static ref INDIRECT_MAPPING_NAMES: HashSet<Vec<u32>> = {
        let mut result = HashSet::with_capacity(INDIRECT_MAPPINGS_VEC.len());

        for (i, _) in INDIRECT_MAPPINGS_VEC.iter() {
            result.insert(i.clone());
        }

        result
    };

    pub static ref INDIRECT_MAPPINGS: HashMap<Vec<u32>, Vec<u32>> = {
        let mut result = HashMap::with_capacity(INDIRECT_MAPPINGS_VEC.len());

        for (key, value) in INDIRECT_MAPPINGS_VEC.iter() {
            result.insert(key.clone(), value.clone());
        }

        result
    };

    pub static ref CHAR_NAMES: HashSet<Vec<u32>> = DIRECT_MAPPINGS.union(&INDIRECT_MAPPING_NAMES).map(|name| name.clone()).collect();

}
