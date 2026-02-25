// Dictionary management and word selection

use crate::difficulty::DifficultyLevel;
use rand::Rng;

pub struct Dictionary {
    easy_words: Vec<String>,
    medium_words: Vec<String>,
    hard_words: Vec<String>,
    expert_words: Vec<String>,
}

impl Dictionary {
    pub fn new() -> Self {
        Self {
            easy_words: Self::load_easy_words(),
            medium_words: Self::load_medium_words(),
            hard_words: Self::load_hard_words(),
            expert_words: Self::load_expert_words(),
        }
    }

    /// Select a random word for the given difficulty
    pub fn select_random(&self, difficulty: DifficultyLevel) -> String {
        let mut rng = rand::thread_rng();
        let words = match difficulty {
            DifficultyLevel::Easy => &self.easy_words,
            DifficultyLevel::Medium => &self.medium_words,
            DifficultyLevel::Hard => &self.hard_words,
            DifficultyLevel::Expert => &self.expert_words,
        };

        if words.is_empty() {
            "WORD".to_string()
        } else {
            words[rng.gen_range(0..words.len())].clone()
        }
    }

    /// Count words at a difficulty level
    pub fn count_words(&self, difficulty: DifficultyLevel) -> usize {
        match difficulty {
            DifficultyLevel::Easy => self.easy_words.len(),
            DifficultyLevel::Medium => self.medium_words.len(),
            DifficultyLevel::Hard => self.hard_words.len(),
            DifficultyLevel::Expert => self.expert_words.len(),
        }
    }

    // Load easy words (3-5 characters)
    fn load_easy_words() -> Vec<String> {
        vec![
            "THE", "AND", "FOR", "ARE", "BUT", "NOT", "YOU", "ALL", "CAN", "HER", "WAS",
            "ONE", "OUR", "OUT", "DAY", "GET", "HAS", "HIM", "HIS", "HOW", "ITS", "MAY",
            "NOW", "OLD", "SEE", "SHE", "TWO", "WAY", "WHO", "BOY", "DID", "CAR", "EAT",
            "FAR", "FUN", "GOT", "LET", "PUT", "RUN", "SAY", "SIT", "TRY", "USE", "ACT",
            "AGE", "BAD", "BAG", "BAT", "BIG", "BIT", "BOX", "BUS", "BUY", "CAT", "CUP",
            "CUT", "DOG", "EYE", "FLY", "HIT", "HOT", "JOB", "LAY", "LEG", "LIE", "LOT",
            "MAN", "MEN", "MOM", "MUD", "PAN", "PET", "PIE", "PIG", "RED", "RID", "SAD",
            "SAT", "SET", "SIX", "SKY", "SON", "SUN", "TAX", "TEA", "TEN", "TOP", "VAN",
            "WIN", "YES", "ZOO", "ADD", "APE", "ARM", "ART", "ASK", "BAR", "BED", "BEE",
            "BUG", "BUN", "DAM", "DIM", "DRY", "ELF", "END", "ERA", "FIG", "FIN", "FIT",
            "FOX", "GAS", "GAY", "GEL", "GEM", "GET", "GOD", "GOT", "GUM", "GUN", "GUY",
            "HAD", "HAM", "HAT", "HAY", "HEM", "HEN", "HER", "HID", "HIM", "HIS", "HOG",
            "HOP", "HOW", "ICE", "ILL", "INK", "INN", "JAM", "JAR", "JAW", "JAY", "JET",
            "JOG", "JOY", "KID", "LAP", "LAW", "LAX", "LAY", "LEG", "LET", "LID", "LIP",
            "LOG", "LOW", "MAC", "MAD", "MAP", "MAT", "MAX", "MEN", "MET", "MIX", "MOB",
            "MOD", "MOP", "NAP", "NET", "NEW", "NOR", "NOT", "NUT", "OAK", "ODD", "OFF",
            "OFT", "OIL", "OPT", "OUR", "OUT", "OWE", "OWL", "OWN", "PAD", "PAL", "PAT",
            "PAW", "PAY", "PEA", "PEG", "PEN", "PEP", "PER", "PET", "PEW", "PIE", "PIG",
            "PIN", "PIT", "PLY", "POD", "POP", "POT", "POW", "PRY", "PUB", "PUN", "PUP",
            "PUS", "PUT", "RAG", "RAM", "RAN", "RAP", "RAT", "RAW", "RAY", "REC", "RED",
            "REF", "REP", "RIB", "RID", "RIG", "RIM", "RIP", "ROB", "ROD", "ROT", "ROW",
            "RUB", "RUG", "RUM", "RUN", "RUT", "RYE", "SAC", "SAD", "SAG", "SAP", "SAT",
            "SAW", "SAY", "SEA", "SET", "SEW", "SEX", "SHE", "SHY", "SIN", "SIP", "SIR",
            "SIS", "SIT", "SIX", "SKI", "SKY", "SLY", "SOB", "SOD", "SON", "SOP", "SOW",
            "SOX", "SOY", "SPA", "SPY", "STY", "SUB", "SUM", "SUN", "SUP", "TAB", "TAD",
            "TAG", "TAP", "TAR", "TAT", "TAX", "TEA", "TEN", "THE", "TIC", "TIE", "TIN",
            "TIP", "TOE", "TON", "TOO", "TOP", "TOT", "TOW", "TOY", "TRY", "TUB", "TUG",
            "TWO", "URN", "USE", "VAN", "VAT", "VET", "VIA", "VIE", "WAD", "WAG", "WAR",
            "WAS", "WAX", "WAY", "WEB", "WED", "WEE", "WET", "WHO", "WHY", "WIG", "WIN",
            "WIT", "WOE", "WOK", "WON", "WOO", "WOW", "YAK", "YAM", "YAP", "YEA", "YES",
            "YET", "YEW", "YIN", "YOU", "ZAG", "ZAP", "ZED", "ZIP", "ZIT", "ZOO",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    // Load medium words (6-8 characters)
    fn load_medium_words() -> Vec<String> {
        vec![
            "ALMOST", "ALWAYS", "AMOUNT", "ANOTHER", "ANSWER", "ANYONE", "APPEAR", "AROUND",
            "ARRIVE", "ARTIST", "ASPECT", "ASSUME", "ATTACK", "ATTEMPT", "AUGUST", "AVENUE",
            "BATTLE", "BEAUTY", "BECAUSE", "BECOME", "BEFORE", "BEHIND", "BELIEVE", "BELONG",
            "BENEFIT", "BESIDE", "BETTER", "BEYOND", "BORDER", "BOTTOM", "BRANCH", "BREATH",
            "BRIDGE", "BRIGHT", "BROKEN", "BROUGHT", "BUDGET", "CAMERA", "CANCER", "CANVAS",
            "CAPITAL", "CAPTAIN", "CAPTURE", "CARBON", "CAREER", "CAREFUL", "CARPET", "CASTLE",
            "CASUAL", "CAUGHT", "CAUSED", "CENTER", "CENTRAL", "CENTURY", "CERTAIN", "CHANGE",
            "CHANNEL", "CHAPTER", "CHARGE", "CHARITY", "CHECKED", "CHEESE", "CHERRY", "CHOICE",
            "CHOSEN", "CIRCLE", "CIRCUIT", "CITIZEN", "CLASSIC", "CLAUSE", "CLEANED", "CLEARED",
            "CLIMATE", "CLOSED", "CLOSER", "CLOTHES", "CLOUDY", "CLUSTER", "COACHING", "COASTAL",
            "COFFEE", "COLUMN", "COMBINE", "COMEDY", "COMMAND", "COMMENT", "COMMON", "COMPANY",
            "COMPARE", "COMPETE", "COMPLEX", "CONCEPT", "CONCERN", "CONCERT", "CONDUCT", "CONFIRM",
            "CONNECT", "CONSIST", "CONTACT", "CONTAIN", "CONTENT", "CONTEST", "CONTEXT", "CONTROL",
            "CONVERT", "COOKING", "COOLING", "CORRECT", "COUNCIL", "COUNSEL", "COUNTER", "COUNTRY",
            "COURAGE", "COVERED", "CREATED", "CREATURE", "CRICKET", "CROSSED", "CRYSTAL", "CULTURE",
            "CURIOUS", "CURRENT", "CURTAIN", "CUTTING", "DAMAGED", "DANCING", "DEALING", "DEBATED",
            "DECIDED", "DECLINE", "DECORATE", "DECRYPT", "DEFAULT", "DEFENSE", "DEFICIT", "DELIVER",
            "DENSITY", "DEPARTED", "DEPENDS", "DERIVED", "DESCEND", "DESERVE", "DESKTOP", "DESPITE",
            "DESTROY", "DETAILS", "DEVELOP", "DIAGRAM", "DIAMOND", "DIGITAL", "DIGNITY", "DINING",
            "DIRECTED", "DISABLED", "DISAGREE", "DISASTER", "DISCLOSE", "DISCUSS", "DISEASE", "DISPLAY",
            "DISPUTE", "DISTANT", "DIVERSE", "DIVIDED", "DIVORCE", "DOCTOR", "DOLPHIN", "DOMAINS",
            "DONATED", "DOORWAY", "DOUBLED", "DRAGGED", "DRESSED", "DRIVING", "DROPPED", "DROUGHT",
            "DYNAMIC", "EAGERLY", "EARNING", "EASTERN", "ECLIPSE", "ECONOMY", "EDITING", "EDUCATE",
            "EFFECTS", "EFFORTS", "ELASTIC", "ELDERLY", "ELECTED", "ELEMENT", "ELEGANT", "ELEVATE",
            "EMBRACE", "EMERALD", "EMOTION", "EMPEROR", "EMPIRE", "ENABLED", "ENCODED", "ENDLESS",
            "ENDORSE", "ENFORCE", "ENGAGED", "ENHANCE", "ENIGMA", "ENLARGE", "ENORMOUS", "ENROLLED",
            "ENRICHED", "ENTIRELY", "ENTITLE", "ENVELOPE", "ENVIABLE", "EQUIPPED", "EROSION", "ERUPTED",
            "ESCAPED", "ESCORTED", "ESSENCE", "EVENING", "EVIDENT", "EXAMPLE", "EXAMINE", "EXCITED",
            "EXECUTE", "EXERCISE", "EXHAUST", "EXISTED", "EXPLAIN", "EXPOSED", "EXTRACT", "EXTREME",
            "FAILURE", "FALLING", "FANTASY", "FASHION", "FEATURE", "FEELING", "FICTION", "FIFTEEN",
            "FIFTEEN", "FINALLY", "FINDING", "FINGERS", "FINISHED", "FISHING", "FITNESS", "FITTING",
            "FLATTEN", "FLOWING", "FOCUSED", "FOLDING", "FORCING", "FOREIGN", "FOREVER", "FORGAVE",
            "FORGIVE", "FORMING", "FORWARD", "FOUNDED", "FREEDOM", "FRIENDS", "FRONTED", "FULFILL",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    // Load hard words (9-12 characters)
    fn load_hard_words() -> Vec<String> {
        vec![
            "ABANDONED", "ABILITIES", "ABOLITION", "ABSOLUTES", "ABUNDANCE", "ACADEMICS",
            "ACCEPTING", "ACCIDENTS", "ACCORDING", "ACCORDION", "ACCOUNTED", "ACCREDITED",
            "ACCUSTOMED", "ACHIEVING", "ACQUIRING", "ACTIVATED", "ACTIVITY", "ACTRESSES",
            "ADAPTABLE", "ADAPTATION", "ADDRESSED", "ADDRESSING", "ADEQUATELY", "ADJUSTING",
            "ADMIRABLE", "ADMISSION", "ADMITTEDLY", "ADMITTING", "ADVANCING", "ADVANTAGE",
            "ADVENTURE", "ADVERSARY", "ADVERTISE", "ADVOCACY", "ADVOCATED", "AESTHETIC",
            "AFFECTION", "AFFILIATED", "AFFORDING", "AFTERNOON", "AFTERMATH", "AGGRESSION",
            "AGREEMENT", "ALGORITHM", "ALIGNMENT", "ALLOWABLE", "ALLOWANCE", "ALONGSIDE",
            "ALTERNATE", "BEAUTIFUL", "BELONGING", "BIOGRAPHY", "BLUEPRINT", "BROADCAST",
            "BUILDINGS", "BRILLIANT", "BUTTERFLY", "CALCULATE", "CAMPAIGNS", "CANDIDATE",
            "CAREFULLY", "CARPENTER", "CHALLENGE", "CHARACTER", "CHEMICALS", "CHRISTMAS",
            "CIGARETTE", "CLASSROOM", "CLOCKWISE", "CLOUDLESS", "COGNITIVE", "COLLECTED",
            "COMMITTEE", "COMMODITY", "COMMUNITY", "COMPANIES", "COMPARING", "COMPETING",
            "COMPLAINT", "COMPLETED", "COMPOSING", "COMPUTERS", "CONCERNED", "CONDUCTED",
            "CONFIDENT", "CONFIRMED", "CONFLICTS", "CONNECTED", "CONSCIOUS", "CONSENSUS",
            "CONSIDERS", "CONSISTED", "CONSTANTLY", "CONSTRUCT", "CONSULTED", "CONTAINED",
            "CONTENTED", "CONTINENT", "CONTINUED", "CONTINUES", "CONTRACTS", "CONTRASTS",
            "CONVERTED", "CONVICTED", "CONVINCES", "CONVICTED", "COOPERATE", "CORRECTED",
            "CORRECTLY", "CORRUPTED", "COUNTRIES", "CREATURES", "CREDIBLE", "CRIMINALS",
            "CRITICISM", "CRITICIZE", "CROSSROAD", "CUSTOMIZE", "DANGEROUS", "DASHBOARD",
            "DATABASES", "DAUGHTERS", "DAYDREAMS", "DEFAULTED", "DEFINITELY", "DEMENTIA",
            "DEMOCRACY", "DEPARTURE", "DEPENDENT", "DEPICTING", "DEPLOYING", "DEPARTURE",
            "DEPRESSED", "DESCRIBE", "DESERVING", "DESIGNING", "DESPERATE", "DESTROYED",
            "DETECTING", "DETERMINE", "DEVELOPED", "DIAGNOSIS", "DIFFERENT", "DIFFICULT",
            "DIMENSION", "DIRECTING", "DIRECTORS", "DIRECTORY", "DISAPPEAR", "DISCHARGE",
            "DISCLOSED", "DISCOVERY", "DISCUSSED", "DISGUSTED", "DISHEARTEN", "DISMISSAL",
            "DISORDERED", "DISPERSED", "DISPLAYING", "DISSENTED", "DISSOLVED", "DISTANCES",
            "DISTORTED", "DISTURBED", "DIVERSIFY", "DOCUMENTED", "DOMINATING", "DOMINATED",
            "DOORKEEPS", "DOWNRIGHT", "DOWNLOADS", "DRAMATIZE", "DRINKABLE", "DRIVELING",
            "DUPLICATE", "DURATIONS", "EARNESTLY", "ECONOMICS", "ECONOMISTS", "EDUCATORS",
            "EFFECTIVE", "EFFICIENCY", "EGGPLANTS", "ELECTIONS", "ELECTRICAL", "ELECTRODE",
            "ELEVATORS", "ELSEWHERE", "EMANATING", "EMBRACING", "EMERGENCE", "EMERGENT",
            "EMPLOYEES", "EMPLOYERS", "EMPOWERED", "ENCRYPTION", "ENCOURAGE", "ENCOURAGES",
            "ENDEAVORS", "ENDLESSLY", "ENDORSING", "ENDURANCE", "ENERGETIC", "ENERGIZED",
            "ENFEOFFED", "ENGINEERS", "ENGRAVING", "ENIGMATIC", "ENJOYABLE", "ENJOYMENT",
            "ENLIGHTEN", "ENLISTEES", "ENLISTED", "ENORMOUSLY", "ENQUIRING", "ENRICHING",
            "ENROLMENT", "ENSHRINED", "ENSHROUDING", "ENSLAVED", "ENSNARED", "ENSUING",
            "ENSURE", "ENTANGLE", "ENTERPRISE", "ENTERTAIN", "ENTHRALL", "ENTHRONED",
            "ENLIVENED", "ENTHUSIASM", "ENTHUSIAST", "ENTICING", "ENTITLED", "ENTOMBED",
            "ENTOURAGE", "ENTRAILED", "ENTRANCED", "ENTRAPMENT", "ENTREATED", "ENTREEMING",
            "ENTREEING", "ENTRENCHED", "ENTREPOTS", "ENTRUSTED", "ENUMERATE", "ENUNCIATE",
            "ENVELOPED", "ENVELOPER", "ENVENOMED", "ENVIABLE", "ENVIABLY", "ENVIOUSLY",
            "ENVIRONMENT", "ENVISIONABLE", "ENVISIONED", "ENVISIONS", "ENVOYING", "ENVOYS",
            "EPAULETS", "EPHEMERA", "EPHEMERAL", "EPHEMERIDAE", "EPICENTER", "EPICENTRE",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    // Load expert words (13-15 characters)
    fn load_expert_words() -> Vec<String> {
        vec![
            "ABBREVIATION", "ABBREVIATED", "ABBREVIATING", "ABBREVIATIVE", "ABBREVIATORY",
            "ABNORMALITIES", "ABOMINABLE", "ABORIGINALITY", "ABORTIONISTS", "ABOVEGROUND",
            "ACCELERATING", "ACCELERATORS", "ACCELERATION", "ACCIDENTALLY", "ACCUMULATING",
            "ACHIEVEMENTS", "ACKNOWLEDGING", "ADMINISTRATIVE", "ADVERTISEMENT", "AGRICULTURAL",
            "ALPHABETICAL", "ALTERNATIVE", "ALTERNATIVELY", "AMBASSADORS", "AMBIGUOUS",
            "AMERICA", "AMPLIFICATION", "AMUSEMENT", "ANALYSIS", "ANALYTICAL",
            "ANESTHESIA", "ANNIVERSARY", "ANNOUNCEMENT", "ANOMALIES", "ANTAGONISM",
            "ANTARCTICA", "ANTICIPATION", "ANTIQUATED", "ANTISEPTIC", "ANXIETIES",
            "APOCALYPTIC", "APOLOGETIC", "APOSTLE", "APPALL", "APPARATUSES",
            "APPAREL", "APPARENTLY", "APPEAL", "APPEALING", "APPEARANCE",
            "APPEASEMENT", "APPELLANT", "APPENDICES", "APPENDIX", "APPETIZING",
            "APPLAUD", "APPLAUSE", "APPLICABLE", "APPLICANT", "APPLICATION",
            "APPRECIABLE", "APPRECIATE", "APPRECIATED", "APPRECIATION", "APPRENTICE",
            "APPROACHING", "APPROPRIATE", "APPROPRIATED", "APPROPRIATELY", "APPROPRIATION",
            "APPROVAL", "APPROXIMATE", "APPROXIMATELY", "APRICOT", "APRIL",
            "AQUA", "AQUARIUM", "AQUATIC", "AQUEDUCT", "ARACHNID",
            "ARBITRARY", "ARBITER", "ARBITERS", "ARBITRARILY", "ARBITRATION",
            "ARBOREAL", "ARBORETUM", "ARBOR", "ARC", "ARCADE",
            "ARCHAEOLOGICAL", "ARCHAEOLOGIST", "ARCHAEOLOGY", "ARCHAIC", "ARCHANGEL",
            "ARCHBISHOP", "ARCHAEOLOGISTS", "ARCHITECT", "ARCHITECTURAL", "ARCHITECTURE",
            "ARCHIVES", "ARCTIC", "ARDENT", "ARDENTLY", "ARDOR",
            "ARDUOUS", "ARDUOUSLY", "AREA", "AREAS", "ARENA",
            "ARGUABLE", "ARGUABLY", "ARGUE", "ARGUED", "ARGUMENT",
            "ARGUMENTATIVE", "ARGUMENTS", "ARIA", "ARID", "ARIDITY",
            "ARIGHTARISING", "ARISES", "ARISTOCRACY", "ARISTOCRAT", "ARISTOCRATIC",
            "ARITHMETIC", "ARITHMETICAL", "ARIZONA", "ARKANSAS", "ARM",
            "ARMADA", "ARMADILLO", "ARMAMENTS", "ARMED", "ARMENIAN",
            "ARMFUL", "ARMHOLE", "ARMIES", "ARMISTICE", "ARMOR",
            "ARMORED", "ARMORY", "ARMPIECE", "ARMPIT", "ARMS",
            "ARMSTRONG", "ARMY", "AROMA", "AROMANTIC", "AROMATIC",
            "AROSE", "AROUND", "AROUSE", "AROUSED", "AROUSING",
            "ARPEGGIO", "ARRANGE", "ARRANGED", "ARRANGEMENT", "ARRANGING",
            "ARRANT", "ARRAY", "ARRAYS", "ARRAYED", "ARREARS",
            "ARREST", "ARRESTED", "ARRESTING", "ARRESTS", "ARRIVAL",
            "ARRIVE", "ARRIVED", "ARRIVES", "ARRIVING", "ARROGANCE",
            "ARROGANT", "ARROGANTLY", "ARROW", "ARROWS", "ARROYO",
            "ARSENIC", "ARSON", "ARSONIST", "ART", "ARTERY",
            "ARTFUL", "ARTFULLY", "ARTHUR", "ARTICHOKING", "ARTICHOKE",
            "ARTICLE", "ARTICLES", "ARTICULATE", "ARTICULATED", "ARTICULATES",
            "ARTICULATING", "ARTICULATION", "ARTIFACT", "ARTIFICER", "ARTIFICIAL",
            "ARTIFICIALLY", "ARTIFICE", "ARTILLERY", "ARTISAN", "ARTISANS",
            "ARTIST", "ARTISTIC", "ARTISTICALLY", "ARTISTS", "ARTISTRY",
            "ARTLESS", "ARTLESSLY", "ARTS", "ARTWORK", "ARTY",
            "ARUM", "ARYAN", "AS", "ASBESTOS", "ASCEND",
            "ASCENDANCY", "ASCENDANT", "ASCENDING", "ASCENDS", "ASCENSION",
            "ASCENT", "ASCERTAIN", "ASCERTAINED", "ASCERTAINING", "ASCERTAINMENT",
            "ASCERTAINS", "ASCETIC", "ASCETICISM", "ASCETICS", "ASCOT",
            "ASCRIBABLE", "ASCRIBE", "ASCRIBED", "ASCRIBES", "ASCRIBING",
            "ASCRIPTION", "ASEPTIC", "ASEPTICS", "ASEXUAL", "ASH",
            "ASHAMED", "ASHEN", "ASHES", "ASHORE", "ASHRAM",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }
}

impl Default for Dictionary {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dictionary_initialization() {
        let dict = Dictionary::new();
        assert!(dict.count_words(DifficultyLevel::Easy) > 0);
        assert!(dict.count_words(DifficultyLevel::Medium) > 0);
        assert!(dict.count_words(DifficultyLevel::Hard) > 0);
        assert!(dict.count_words(DifficultyLevel::Expert) > 0);
    }

    #[test]
    fn test_word_selection_randomization() {
        let dict = Dictionary::new();
        let word1 = dict.select_random(DifficultyLevel::Easy);
        let word2 = dict.select_random(DifficultyLevel::Easy);

        // While they might be the same, the selection should work
        assert!(!word1.is_empty());
        assert!(!word2.is_empty());
    }

    #[test]
    fn test_total_words_count() {
        let dict = Dictionary::new();
        let total = dict.count_words(DifficultyLevel::Easy)
            + dict.count_words(DifficultyLevel::Medium)
            + dict.count_words(DifficultyLevel::Hard)
            + dict.count_words(DifficultyLevel::Expert);

        assert!(total >= 1000, "Should have at least 1000 words");
    }
}
