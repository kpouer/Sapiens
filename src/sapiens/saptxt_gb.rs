/**********************************************************************
                          TEXTES VERSION ANGLAISE
                        (C) DIDIER 12 Avril 1987
                            30 mars 2017
**********************************************************************/
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// Port Rust de "Sources/SAPTXT_gb.C" : textes de la langue anglaise.
// Donnees pures, aucune fonction. Chaines en MacRoman brut ; les
// caracteres accentues sont ecrits selon la substitution d'origine
// (`%`=o `^`=n `&`=a `|`=i `#`=e `_`=u `[`=? `]`=!) et convertis
// par CvtCarLissage (saptxt.rs). Ne pas convertir en UTF-8.
use super::saptxt::nomref;
use crate::sapiens::{NB_MOTS_BKMOTS, reference};

pub(crate) struct Saptxt_gb {
    pub(crate) refer0: [reference; 12],
    pub(crate) refer1: [reference; 47],
    pub(crate) refer2: [reference; 29],
    pub(crate) refer3: [reference; 27],
    pub(crate) mescon: [&'static [u8]; 138],
    pub(crate) bkmots: [&'static [u8]; NB_MOTS_BKMOTS as usize],
    pub(crate) scenme: [&'static [u8]; 14],
    pub(crate) app: [&'static [u8]; 23],
}

impl Default for Saptxt_gb {
    fn default() -> Self {
        Self {
            refer1: [
    reference {
        nomref: nomref(b"HIS PROTECTION"),
        param1: 0x3F,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"WATER"),
        param1: 0x25,
        param2: 0xF,
    },
    reference {
        nomref: nomref(b"GRASS"),
        param1: 0x20,
        param2: 0x11,
    },
    reference {
        nomref: nomref(b"LEAF"),
        param1: 0x20,
        param2: 0x11,
    },
    reference {
        nomref: nomref(b"APPLE"),
        param1: 0x21,
        param2: 0x12,
    },
    reference {
        nomref: nomref(b"PEAR"),
        param1: 0x21,
        param2: 0x12,
    },
    reference {
        nomref: nomref(b"BILBERRIES"),
        param1: 0x21,
        param2: 0x11,
    },
    reference {
        nomref: nomref(b"LARVA"),
        param1: 0x21,
        param2: 0x20,
    },
    reference {
        nomref: nomref(b"INSECT"),
        param1: 0x21,
        param2: 0x10,
    },
    reference {
        nomref: nomref(b"THISTLE"),
        param1: 0x20,
        param2: 0x11,
    },
    reference {
        nomref: nomref(b"BELLADONNA"),
        param1: 0x21,
        param2: 0x20,
    },
    reference {
        nomref: nomref(b"NETTLE"),
        param1: 0x20,
        param2: 0x10,
    },
    reference {
        nomref: nomref(b"CHICORY"),
        param1: 0x21,
        param2: 0x11,
    },
    reference {
        nomref: nomref(b"ROOTS"),
        param1: 0x21,
        param2: 0x21,
    },
    reference {
        nomref: nomref(b"BONE"),
        param1: 0x20,
        param2: 0x10,
    },
    reference {
        nomref: nomref(b"MELISSA"),
        param1: 0x41,
        param2: 0x02,
    },
    reference {
        nomref: nomref(b"DANDELION"),
        param1: 0x21,
        param2: 0x21,
    },
    reference {
        nomref: nomref(b"PERIWINKLE"),
        param1: 0x41,
        param2: 0x02,
    },
    reference {
        nomref: nomref(b"FLINT"),
        param1: 0x35,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"PEBBLE"),
        param1: 0x30,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"HATCHET"),
        param1: 0x1A,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"CLUB"),
        param1: 0x18,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"ASSEGAI"),
        param1: 0x16,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"SCRAPER"),
        param1: 0x14,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"OINTMENT"),
        param1: 0x48,
        param2: 0x0A,
    },
    reference {
        nomref: nomref(b"EMPTY WATER SKIN"),
        param1: 0x38,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"FULL WATER SKIN"),
        param1: 0x2A,
        param2: 0x0E,
    },
    reference {
        nomref: nomref(b"FLINT KNIFE"),
        param1: 0x37,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"CLOTH"),
        param1: 0x3B,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"BASKET"),
        param1: 0x3A,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"BEARSKIN"),
        param1: 0x3A,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"WOLFSKIN"),
        param1: 0x39,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"RABBITSKIN"),
        param1: 0x33,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"BEAR CLAW"),
        param1: 0x39,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"WOLF FANG"),
        param1: 0x38,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"DRIED MEAT"),
        param1: 0x27,
        param2: 0x90,
    },
    reference {
        nomref: nomref(b"FRESH MEAT"),
        param1: 0x26,
        param2: 0xA1,
    },
    reference {
        nomref: nomref(b"MEAT"),
        param1: 0x26,
        param2: 0x91,
    },
    reference {
        nomref: nomref(b"STALE MEAT"),
        param1: 0x24,
        param2: 0x61,
    },
    reference {
        nomref: nomref(b"MOULDY MEAT"),
        param1: 0x21,
        param2: 0x30,
    },
    reference {
        nomref: nomref(b"ROTTEN MEAT"),
        param1: 0x30,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"Magic Potion"),
        param1: 0x3D,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"Ivory Axe"),
        param1: 0x3D,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"Holy Rod"),
        param1: 0x3D,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"Broken assegai"),
        param1: 0,
        param2: 0,
    },
    reference {
        nomref: nomref(b"* Floppy disk *"),
        param1: 0x2F,
        param2: 0xFF,
    },
    reference {
        nomref: nomref(b"$END$"),
        param1: 0,
        param2: 0,
    },
            ],
            refer0: [
    reference {
        nomref: nomref(b"LOOK"),
        param1: 0x03,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"CONSUME"),
        param1: 0x01,
        param2: 0x01,
    },
    reference {
        nomref: nomref(b"SAY"),
        param1: 0x02,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"PUT DOWN"),
        param1: 0x01,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"TAKE"),
        param1: 0x04,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"GIVE"),
        param1: 0x01,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"TEND WITH"),
        param1: 0x01,
        param2: 0x02,
    },
    reference {
        nomref: nomref(b"SLEEP"),
        param1: 0x05,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"MAKE"),
        param1: 0x06,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"HEALTH"),
        param1: 0x07,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"SAPIENS"),
        param1: 0x08,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"$END$"),
        param1: 0,
        param2: 0,
    },
            ],
            refer2: [
    reference {
        nomref: nomref(b"WHO ARE YOU?"),
        param1: 0x00,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"WHAT DO YOU WANT?"),
        param1: 0x00,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"WHAT ARE YOU GIVING ME?"),
        param1: 0x00,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"WHERE IS YOUR CHIEF?"),
        param1: 0x00,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"I AM \x82"),
        param1: 0x00,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"GOOD HUNTING."),
        param1: 0x00,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"ON YOUR KNEES"),
        param1: 0x00,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"COME WITH ME"),
        param1: 0x00,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"DON'T COME NEAR ME!"),
        param1: 0x01,
        param2: 0x10,
    },
    reference {
        nomref: nomref(b"KEEP ON GOING"),
        param1: 0x01,
        param2: 0x20,
    },
    reference {
        nomref: nomref(b"OUT OF MY SIGHT"),
        param1: 0x01,
        param2: 0x30,
    },
    reference {
        nomref: nomref(b"YOU'RE A COWARD!"),
        param1: 0x01,
        param2: 0x40,
    },
    reference {
        nomref: nomref(b"CAST YOUR EYES DOWN"),
        param1: 0x01,
        param2: 0x50,
    },
    reference {
        nomref: nomref(b"I'LL CRUSH YOU!"),
        param1: 0x01,
        param2: 0x60,
    },
    reference {
        nomref: nomref(b"GROVEL BEFORE ME!"),
        param1: 0x01,
        param2: 0x70,
    },
    reference {
        nomref: nomref(b"RABBIT HEART!"),
        param1: 0x01,
        param2: 0x80,
    },
    reference {
        nomref: nomref(b"DEGENERATE MONKEY!"),
        param1: 0x01,
        param2: 0x90,
    },
    reference {
        nomref: nomref(b"PIG FACE!"),
        param1: 0x01,
        param2: 0xA0,
    },
    reference {
        nomref: nomref(b"PEACE,FRIEND."),
        param1: 0x02,
        param2: 0x40,
    },
    reference {
        nomref: nomref(b"GLORY BE TO YOUR TRIBE"),
        param1: 0x02,
        param2: 0x80,
    },
    reference {
        nomref: nomref(b"EVERYBODY WORSHIPS YOU"),
        param1: 0x02,
        param2: 0xC0,
    },
    reference {
        nomref: nomref(b"FOR PITY'S SAKE,SPARE ME"),
        param1: 0x02,
        param2: 0xF0,
    },
    reference {
        nomref: nomref(b"I AM THE STRONGEST !"),
        param1: 0x03,
        param2: 0x20,
    },
    reference {
        nomref: nomref(b"NO ONE DARES RESIST ME !"),
        param1: 0x03,
        param2: 0x40,
    },
    reference {
        nomref: nomref(b"I EAT FLINT !"),
        param1: 0x03,
        param2: 0x60,
    },
    reference {
        nomref: nomref(b"FLEE BEFORE ME"),
        param1: 0x03,
        param2: 0x80,
    },
    reference {
        nomref: nomref(b"I COMMAND THE LIGHTNING"),
        param1: 0x03,
        param2: 0xA0,
    },
    reference {
        nomref: nomref(b"THE GODS GUIDE ME"),
        param1: 0x03,
        param2: 0xC0,
    },
    reference {
        nomref: nomref(b"$END$"),
        param1: 0,
        param2: 0,
    },
            ],
            refer3: [
    reference {
        nomref: nomref(b"LANDSCAPE"),
        param1: 0x01,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"LATERAL VIEW"),
        param1: 0x01,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"PANORAMA"),
        param1: 0x01,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"MAP"),
        param1: 0x01,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"JUST A MOMENT"),
        param1: 0x00,
        param2: 0x20,
    },
    reference {
        nomref: nomref(b"HALF A DAY"),
        param1: 0x00,
        param2: 0x80,
    },
    reference {
        nomref: nomref(b"A DAY"),
        param1: 0x0,
        param2: 0xFF,
    },
    reference {
        nomref: nomref(b" \x8E"),
        param1: 0x0,
        param2: 0x0,
    }, // Don't translate this
    reference {
        nomref: nomref(b" \x8F"),
        param1: 0x0,
        param2: 0x0,
    }, // Don't translate this
    reference {
        nomref: nomref(b"\x99 "),
        param1: 0x0,
        param2: 0x0,
    }, // Don't translate this
    reference {
        nomref: nomref(b"\x99 "),
        param1: 0x0,
        param2: 0x0,
    }, // Don't translate this
    reference {
        nomref: nomref(b"\x99 "),
        param1: 0x0,
        param2: 0x0,
    }, // Don't translate this
    reference {
        nomref: nomref(b"Load Adventure"),
        param1: 0x0,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"Save Adventure"),
        param1: 0x0,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"New Adventure"),
        param1: 0x0,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"Auto-Help"),
        param1: 0,
        param2: 0,
    },
    reference {
        nomref: nomref(b"Keyboard-Help"),
        param1: 0,
        param2: 0,
    },
    reference {
        nomref: nomref(b"Configuration"),
        param1: 0,
        param2: 0,
    },
    reference {
        nomref: nomref(b"Game handling"),
        param1: 0,
        param2: 0,
    },
    reference {
        nomref: nomref(b"'DMA' Music"),
        param1: 0,
        param2: 0,
    }, // Don't translate this
    reference {
        nomref: nomref(b"MIDI  Music"),
        param1: 0,
        param2: 0,
    }, // Don't translate this
    reference {
        nomref: nomref(b"God!"),
        param1: 0,
        param2: 0,
    }, // Don't translate this
    reference {
        nomref: nomref(b"God!"),
        param1: 0,
        param2: 0,
    }, // Don't translate this
    reference {
        nomref: nomref(b"French Version"),
        param1: 0,
        param2: 0,
    }, // Don't translate this
    reference {
        nomref: nomref(b"MIDI Control"),
        param1: 0,
        param2: 0,
    }, // Don't translate this
    reference {
        nomref: nomref(b"Setup"),
        param1: 0,
        param2: 0,
    },
    reference {
        nomref: nomref(b"$END$"),
        param1: 0,
        param2: 0,
    }, // Don't translate this
            ],
            mescon: [
    b"THANK YOU FOR THE GIFT \x00",
    b"THANKS \x00",
    b"THANK FRIEND \x00",
    b"THANK YOU \x00",
    b"WHO ARE YOU?\x00",
    b"WHAT'S YOUR NAME\x00",
    b"TELL ME YOUR NAME.\x00",
    b"INTRODUCE YOURSELF.\x00",
    b"IT'S NOT ENOUGH.\x00",
    b"YOU 'RE JOKING ?!\x00",
    b"OUT OF THE QUESTION!\x00",
    b"NO!\x00",
    b"DONE!\x00",
    b"O.K.\x00",
    b"\x83 AGREES.\x00",
    b"THAT WILL DO\x00",
    b"\x83  TAKES YOUR GIFT.\x00",
    b"GOOD, \x00", /* le C d'origine ecrivait b"GOOD, \x00x83 TAKES IT" : le NUL tronque la chaine, rien apres n'est lisible */
    b"YES,\x83 ACCEPTS\x00",
    b"\x83 TAKES IT.\x00",
    b" IT'S NOT ENOUGH\x00",
    b" GIVE ME MORE\x00",
    b" BE MORE GENEROUS\x00",
    b" JUST TO PLEASE YOU\x00",
    b"HMM ...\x00",
    b"ANYTHING ELSE ??\x00",
    b"WELL ...\x00",
    b"REALLY ...\x00",
    b"LET'S WRAP THIS UP\x00",
    b"THAT'S ENOUGH TALK\x00",
    b"WHAT ELSE?\x00",
    b"LET'S GET TO THE POINT\x00",
    b"\x83 WILL KILL YOU,DIRTY SWINE!\x00",
    b"SHUT UP!\x00",
    b"\x83 WILL TAKE BACK HIS INSULTS\x00",
    b"TIRED OF LIVING ?!\x00",
    b"I'LL FEED YOUR BONES TO THE CROWS.\x00",
    b"I'LL HAVE MY OWN BACK FOR THIS!\x00",
    b"\x83 WILL TEAR YOU LIMB FROM LIMB\x00",
    b"\x83 WILL DRINK FROM YOUR SKULL\x00",
    b"MAY MY ARM BREAK YOUR JAW\x00",
    b"VICTORY OR DEATH!\x00",
    b"NO PITY.\x00",
    b"MAY THE SPIRITS GUIDE \x83\x00",
    b"I AM \x83\x00",
    b"CALL ME \x83\x00",
    b"MY NAME IS \x83\x00",
    b"I'M CALLED \x83\x00",
    b"YOUR LIFE !\x00",
    b"TO KILL YOU!\x00",
    b"YOUR HEART!\x00",
    b"YOUR SKULL\x00",
    b"\x83 WANTS \x00",
    b"\x83 WOULD LIKE \x00",
    b"\x83 DESIRES \x00",
    b"\x83 NEEDS \x00",
    b"\x83 OFFERS \x00",
    b"\x83 SUGGESTS \x00",
    b"\x83 SWAPS \x00",
    b"\x83 GIVES IN \x00",
    b"\x83 WILL NOT ANSWER\x00",
    b"...\x00",
    b"AH AH AH!\x00",
    b"WHO KNOWS?\x00",
    b" AND YOU,\x00",
    b"NOTHING\x00",
    b"SAPIENS created by Didier and Olivier Guillion.\x00",
    b" (c) 1986-1996,99,01,2005,2017\x00",
    b"Music by Gilles Soulet.\x00",
    b"A thousand centuries ago the sun rose ...\x00",
    b"HELLO,\x82\x00",
    b"SO YOU ARE \x82 ?\x00",
    b"GOOD HUNTING \x82\x00",
    b"The howling of a wolf.\x00",
    b"The growling of a bear.\x00",
    b"The gurgling of a spring.\x00",
    b"The scent of game.\x00",
    b"The smell of smoke.\x00",
    b"The scent of humans.\x00",
    b"He runs away.\x00",
    b"You are knocked out.\x00",
    b"THE TRIBE IS PROUD OF YOU,\x82\x00",
    b"YOU'RE A BRAVE WARRIOR,\x82\x00",
    b"YOU SEEM TO HAVE THE STRENGTH OF A BULL !\x00",
    b"You possess a new weapon.\x00",
    b"Your silex is too badly hewed.\x00",
    b"You have no flint...\x00",
    b"Name? \x00",
    b"Male or Female (M,F) ?\x00",
    b"Adventure to Load ? \x00",
    b"Adventure to Save ? \x00",
    b" Error !!!!\x00",
    b"Hunger knaws at your stomach !\x00",
    b"Your throat is parched with thirst!\x00",
    b"Your shoulders are stooped with fatigue!\x00",
    b"You drink \x00",
    b"and fill your water skins\x00",
    b"You are not thirsty\x00",
    b"...out of the question\x00",
    b"I'm right behind you !\x00",
    b"You carry too many things !!!\x00",
    b"A Wolf comes with you.\x00",
    b"A Bear is your friend.\x00",
    b"A little Rabbit is with you.\x00",
    b"A Turkey is right behind you .\x00",
    b" \x00",
    b"Guzzler ! You are not hungry or thirsty !\x00",
    b"You are not tired.\x00",
    b"You look the picture of health.\x00",
    b"Wonderful,\x00",
    b"Your friend \xAC is knocked out ...\x00",
    b"Sorry \x82, \xAC is Dead !!!\x00",
    b"Only one adventure saved per day !!!\x00",
    b"A new day ...\x00",
    b"Hop, go back home !!!\x00",
    b"What d'you think of that tune? ...\x00",
    b"Here we are...\x00",
    b"A little gift...\x00",
    b"Hello guys!\x00",
    b"SAPIENS by D and O Guillion.\x00",
    b"SAPIENS option :\x00",
    b"Your health :\x00",
    b"\x82 takes\x00",
    b"\x82 puts down\x00",
    b"\x82 says :\x00",
    b"\x82 gives\x00",
    b"\x82 looks\x00",
    b"\x82 consumes\x00",
    b"\x82 tends himself with\x00",
    b"\x82 sleeps for\x00",
    b"With a flint, \x82 makes\x00",
    b"F1 key for Help !\x00",
    b"And the Time runs away...\x00",
    b"You are struck by lightning !!!!!\x00",
    b"Are you talking to the wind ?\x00",
    b"You are dead ! The game is over...\x00",
    b"I am building the world....\x00",
    b"$END$\x00", // Don't translate this
            ],
            bkmots: [
    b"$0\x00", // nomsecondaire // Don't translate this
    b"$1\x00", // nomjoueur// Don't translate this
    b"$2\x00", // nomadversaire// Don't translate this
    b"$3\x00", // // Don't translate this
    b" CHIEF\x00",
    b" OF THE AGILE FEET\x00",
    b" OF THE EAGLE EYES\x00",
    b" OF THE STONE ARMS\x00",
    b" OF THE CRAZY HYENAS\x00",
    b"HOGNOR\x00",
    b"SL-OOM\x00",
    b"GRUNGH\x00",
    b"KROK-G\x00",
    b" HUNGRY\x00",
    b" THIRSTY\x00",
    b"YOUR HEALTH IS \x00",
    b"YOU ARE EXTREMELY\x00",
    b"YOU ARE VERY\x00",
    b"YOU ARE SLIGHTLY\x00",
    b"YOU ARE NOT\x00",
    b"TIRED OUT\x00",
    b"VERY \x97\x00",
    b" TIRED\x00",
    b"RESTED\x00",
    b"YOU ARE \x00",
    b"SERIOUSLY\x9C\x00",
    b"VERY\x9C\x00",
    b" WOUNDED\x00",
    b"WELL\x00",
    b" DAY\x00",
    b"NORTH\x00",
    b"SOUTH\x00",
    b"EAST\x00",
    b"WEST\x00",
    b" SKIN\x00",
    b"MEAT\x00",
    b" ADVENTURE\x00",
    b"THANK YOU\x00",
    b"DEAD !\x00",
    b"VERY VULNERABLE\x00",
    b"VULNERABLE\x00",
    b"STURDY\x00",
    b"VERY STURDY\x00",
    b"$4\x00", // Nomcopain // Don't translate this
    b"your friend\x00",
    b"$END$\x00", // Don't translate this
            ],
            scenme: [
    b"The tribe's children are hungry,\r        bring back food.\x00",
    b"   Young people have eaten poisonous\r                 berries.\r        We need the Eagle-Eyes medecine.\x00",
    b"The Stone-Arms have challenged us to\r                a fight.\r   Go,fight with their chief and steal\r                his axe.\x00",
    b"        Warrior,prepare to fight.\r      The Crazy-Hyenas have stolen\r             The Holy Rod.\rThe Spirits of our ancestor ask for it!\x00",
    b"Your actions damage my reputation.\r         May the Strongest Win !\x00",
    b"Here is the story up to now unknown to\rthe man who,a hundred thousand years ago,\rbecame chief of the Agile-Feet tribe\rby the strength of arms.\rHe lived to the age of forty summers,\rand died by the hand of a young warrior.\rGood hunting !\x00",
    b"        Nobody could survive\r         such terrible  wounds.\rHere lies the end of your road,warrior.\x00",
    b"'A hundred thousand years ago ...\r      ...Adventure Began For Human.'\r\r  SAPIENS was designed and written by\rDidier and Olivier Guillion for AMSTRAD,\r   PC and Compatibles and ATARI ST.\r    Thanks to Jean-Francois Cassan,\r Gilles Soulet, Jean-Michel Georges and\r      Yves Basquet from  'MYRIAD'\r   for their help,criticism and advise.\r\r     Toulouse, July-August XXXVII.\x00",
    b"          You 've got me !\r        The Holy Stick is hidden\r     to the South under the Three Totems.\x00",
    b"\x00",
    b"\x00",
    b"\x00",
    b"\x00",
    b"$END$\x00", // Don't translate this
            ],
            app: [
    b"Agility\x00",
    b"Strength\x00",
    b"Charisma\x00",
    b"Vitality\x00",
    b"Health\x00",
    b"Credit\x00",
    b"Secondary Player : \x00",
    b"Main Player      : \x00",
    b"Which key for \x00",
    b"Up\x00",
    b"Down\x00",
    b"Left\x00",
    b"Right\x00",
    b"Swapping Weapon/Validation\x00",
    b"Hitting\x00",
    b"Exit\x00",
    b" ?\x00",     // Don't translate this
    b"(....)\x00", // Don't translate this
    b" Wolf \x00",
    b" Bear \x00",
    b"Rabbit\x00",
    b"Turkey\x00",
    b"$END$\x00", // Don't translate this
            ],
        }
    }
}
