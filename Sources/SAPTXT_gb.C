/**********************************************************************
                          TEXTES VERSION ANGLAISE
                        (C) DIDIER 12 Avril 1987
							30 mars 2017
Notes to the translator :

Only translated the strings between " and "
Don't change anyhting else outside strings.

Inside strings :
Dont change tags like \r or \r\ or \n or \n\ this is cariages returns (go to next line)
Dont change tags \xnn like \x9C they will be sustituted
Please respect the space before and after the tags or at the end or the begin of strings
Either don't change tags $nn
***********************************************************************/
#include "sapiens.h"

// This the list of object the user can find on the ground, win in a battle, make, and share
// Please respect uppercase and lowercase
struct reference refer1_gb[]={
{"HIS PROTECTION",0x3F,0x0},
{"WATER",0x25,0xF},
{"GRASS",0x20,0x11},
{"LEAF",0x20,0x11},
{"APPLE",0x21,0x12},
{"PEAR",0x21,0x12},
{"BILBERRIES",0x21,0x11},
{"LARVA",0x21,0x20},
{"INSECT",0x21,0x10},
{"THISTLE",0x20,0x11},
{"BELLADONNA",0x21,0x20},
{"NETTLE",0x20,0x10},
{"CHICORY",0x21,0x11},
{"ROOTS",0x21,0x21},
{"BONE",0x20,0x10},
{"MELISSA",0x41,0x02},
{"DANDELION",0x21,0x21},
{"PERIWINKLE",0x41,0x02},
{"FLINT",0x35,0x00},
{"PEBBLE",0x30,0x00},
{"HATCHET",0x1A,0x00},
{"CLUB",0x18,0x00},
{"ASSEGAI",0x16,0x00},
{"SCRAPER",0x14,0x00},
{"OINTMENT",0x48,0x0A},
{"EMPTY WATER SKIN",0x38,0x00},
{"FULL WATER SKIN",0x2A,0x0E},
{"FLINT KNIFE",0x37,0x00},
{"CLOTH",0x3B,0x00},
{"BASKET",0x3A,0x00},
{"BEARSKIN",0x3A,0x00},
{"WOLFSKIN",0x39,0x00},
{"RABBITSKIN",0x33,0x00},
{"BEAR CLAW",0x39,0x00},
{"WOLF FANG",0x38,0x00},
{"DRIED MEAT",0x27,0x90},
{"FRESH MEAT",0x26,0xA1},
{"MEAT",0x26,0x91},
{"STALE MEAT",0x24,0x61},
{"MOULDY MEAT",0x21,0x30},
{"ROTTEN MEAT",0x30,0x00},
{"Magic Potion",0x3D,0x00},
{"Ivory Axe",0x3D,0x00},
{"Holy Rod",0x3D,0x00},
{"Broken assegai",0,0},
{"* Floppy disk *",0x2F,0xFF},
{"$END$",0,0}
},
// This the list of actions (the main Sapiens subwindow at the left
refer0_gb[]={
{"LOOK",0x03,0x00},
{"CONSUME",0x01,0x01},
{"SAY",0x02,0x00},
{"PUT DOWN",0x01,0x00},
{"TAKE",0x04,0x00},
{"GIVE",0x01,0x00},
{"TEND WITH",0x01,0x02},
{"SLEEP",0x05,0x00},
{"MAKE",0x06,0x00},
{"HEALTH",0x07,0x00},
{"SAPIENS",0x08,0x00},
{"$END$",0,0}
},
// This the list of sentences the user can say in lateral view
// Dont change tags \xnn like \x82 they will be sustituted
// For example \x82 is the name of the player
refer2_gb[]={
{"WHO ARE YOU?",0x00,0x00},
{"WHAT DO YOU WANT?",0x00,0x00},
{"WHAT ARE YOU GIVING ME?",0x00,0x00},
{"WHERE IS YOUR CHIEF?",0x00,0x00},
{"I AM \x82",0x00,0x00},
{"GOOD HUNTING.",0x00,0x00},
{"ON YOUR KNEES",0x00,0x00},
{"COME WITH ME",0x00,0x00},
{"DON'T COME NEAR ME!",0x01,0x10},
{"KEEP ON GOING",0x01,0x20},
{"OUT OF MY SIGHT",0x01,0x30},
{"YOU'RE A COWARD!",0x01,0x40},
{"CAST YOUR EYES DOWN",0x01,0x50},
{"I'LL CRUSH YOU!",0x01,0x60},
{"GROVEL BEFORE ME!",0x01,0x70},
{"RABBIT HEART!",0x01,0x80},
{"DEGENERATE MONKEY!",0x01,0x90},
{"PIG FACE!",0x01,0xA0},
{"PEACE,FRIEND.",0x02,0x40},
{"GLORY BE TO YOUR TRIBE",0x02,0x80},
{"EVERYBODY WORSHIPS YOU",0x02,0xC0},
{"FOR PITY'S SAKE,SPARE ME",0x02,0xF0},
{"I AM THE STRONGEST !",0x03,0x20},
{"NO ONE DARES RESIST ME !",0x03,0x40},
{"I EAT FLINT !",0x03,0x60},
{"FLEE BEFORE ME",0x03,0x80},
{"I COMMAND THE LIGHTNING",0x03,0xA0},
{"THE GODS GUIDE ME",0x03,0xC0},
{"$END$",0,0}
},
// This the list of misc sentences
// Dont change tags \xnn like \x8E they will be sustituted

refer3_gb[]={
{"LANDSCAPE",0x01,0x00},
{"LATERAL VIEW",0x01,0x00},
{"PANORAMA",0x01,0x00},
{"MAP",0x01,0x00},
{"JUST A MOMENT",0x00,0x20},
{"HALF A DAY",0x00,0x80},
{"A DAY",0x0,0xFF},
{ " \x8E",0x0,0x0}, // Don't translate this
{ " \x8F",0x0,0x0}, // Don't translate this
{ "\x99 ",0x0,0x0}, // Don't translate this
{ "\x99 ",0x0,0x0}, // Don't translate this
{ "\x99 ",0x0,0x0}, // Don't translate this
{"Load Adventure",0x0,0x0},
{"Save Adventure",0x0,0x0},
{"New Adventure",0x0,0x0},
{"Auto-Help",0,0},
{"Keyboard-Help",0,0},
{"Configuration",0,0},
{"Game handling",0,0},
{"'DMA' Music",0,0},  // Don't translate this
{"MIDI  Music",0,0}, // Don't translate this
{"God!",0,0}, // Don't translate this
{"God!",0,0}, // Don't translate this
{"French Version",0,0}, // Don't translate this
{"MIDI Control",0,0}, // Don't translate this
{"Setup",0,0},
{"$END$",0,0} // Don't translate this
};

// This the list of misc sentences the oponent can answer to the player
// Dont change tags \xnn like \x83 they will be sustituted
// For example \x83 is the name of the opponent \0x82 the player name \xAC the player's friend name
char * mescon_gb[]={
"THANK YOU FOR THE GIFT ",
"THANKS ",
"THANK FRIEND ",
"THANK YOU ",
"WHO ARE YOU?",
"WHAT'S YOUR NAME",
"TELL ME YOUR NAME.",
"INTRODUCE YOURSELF.",
"IT'S NOT ENOUGH.",
"YOU 'RE JOKING ?!",
"OUT OF THE QUESTION!",
"NO!",
"DONE!",
"O.K.",
"\x83 AGREES.",
"THAT WILL DO",
"\x83  TAKES YOUR GIFT.",
"GOOD, \0x83 TAKES IT",
"YES,\x83 ACCEPTS",
"\x83 TAKES IT.",
" IT'S NOT ENOUGH",
" GIVE ME MORE",
" BE MORE GENEROUS",
" JUST TO PLEASE YOU",
"HMM ...",
"ANYTHING ELSE ??",
"WELL ...",
"REALLY ...",
"LET'S WRAP THIS UP",
"THAT'S ENOUGH TALK",
"WHAT ELSE?",
"LET'S GET TO THE POINT",
"\x83 WILL KILL YOU,DIRTY SWINE!",
"SHUT UP!",
"\x83 WILL TAKE BACK HIS INSULTS",
"TIRED OF LIVING ?!",
"I'LL FEED YOUR BONES TO THE CROWS.",
"I'LL HAVE MY OWN BACK FOR THIS!",
"\x83 WILL TEAR YOU LIMB FROM LIMB",
"\x83 WILL DRINK FROM YOUR SKULL",
"MAY MY ARM BREAK YOUR JAW",
"VICTORY OR DEATH!",
"NO PITY.",
"MAY THE SPIRITS GUIDE \x83",
"I AM \x83",
"CALL ME \x83",
"MY NAME IS \x83",
"I'M CALLED \x83",
"YOUR LIFE !",
"TO KILL YOU!",
"YOUR HEART!",
"YOUR SKULL",
"\x83 WANTS ",
"\x83 WOULD LIKE ",
"\x83 DESIRES ",
"\x83 NEEDS ",
"\x83 OFFERS ",
"\x83 SUGGESTS ",
"\x83 SWAPS ",
"\x83 GIVES IN ",
"\x83 WILL NOT ANSWER",
"...",
"AH AH AH!",
"WHO KNOWS?",
" AND YOU,",
"NOTHING",
"SAPIENS created by Didier and Olivier Guillion.",
" (c) 1986-1996,99,01,2005,2017",
"Music by Gilles Soulet.",
"A thousand centuries ago the sun rose ...",
"HELLO,\x82",
"SO YOU ARE \x82 ?",
"GOOD HUNTING \x82",
"The howling of a wolf.",
"The growling of a bear.",
"The gurgling of a spring.",
"The scent of game.",
"The smell of smoke.",
"The scent of humans.",
"He runs away.",
"You are knocked out.",
"THE TRIBE IS PROUD OF YOU,\x82",
"YOU'RE A BRAVE WARRIOR,\x82",
"YOU SEEM TO HAVE THE STRENGTH OF A BULL !",
"You possess a new weapon.",
"Your silex is too badly hewed.",
"You have no flint...",
"Name? ",
"Male or Female (M,F) ?",
"Adventure to Load ? ",
"Adventure to Save ? ",
" Error !!!!",
"Hunger knaws at your stomach !",
"Your throat is parched with thirst!",
"Your shoulders are stooped with fatigue!",
"You drink ",
"and fill your water skins",
"You are not thirsty",
"...out of the question",
"I'm right behind you !",
"You carry too many things !!!",
"A Wolf comes with you.",
"A Bear is your friend.",
"A little Rabbit is with you.",
"A Turkey is right behind you .",
" ",
"Guzzler ! You are not hungry or thirsty !",
"You are not tired.",
"You look the picture of health.",
"Wonderful,",
"Your friend \xAC is knocked out ...",
"Sorry \x82, \xAC is Dead !!!",
"Only one adventure saved per day !!!",
"A new day ...",
"Hop, go back home !!!",
"What d'you think of that tune? ...",
"Here we are...",
"A little gift...",
"Hello guys!",
"SAPIENS by D and O Guillion.",
"SAPIENS option :",
"Your health :",
"\x82 takes",
"\x82 puts down",
"\x82 says :",
"\x82 gives",
"\x82 looks",
"\x82 consumes",
"\x82 tends himself with",
"\x82 sleeps for",
"With a flint, \x82 makes",
"F1 key for Help !",
"And the Time runs away...",
"You are struck by lightning !!!!!",
"Are you talking to the wind ?",
"You are dead ! The game is over...",
"I am building the world....",
"$END$" // Don't translate this
};

// This the list of misc sentences 
// Dont change tags \xnn like \x9C they will be sustituted
// Please respect the space before and after the tags or at the end or the begin of strings
// Either don't change tags $nn

 char * bkmots_gb []={
"$0", //nomsecondaire // Don't translate this
"$1",//  nomjoueur// Don't translate this
"$2", //  nomadversaire// Don't translate this
"$3", //// Don't translate this
" CHIEF",
" OF THE AGILE FEET",
" OF THE EAGLE EYES",
" OF THE STONE ARMS",
" OF THE CRAZY HYENAS",
"HOGNOR",
"SL-OOM",
"GRUNGH",
"KROK-G",
" HUNGRY",
" THIRSTY",
"YOUR HEALTH IS ",
"YOU ARE EXTREMELY",
"YOU ARE VERY",
"YOU ARE SLIGHTLY",
"YOU ARE NOT",
"TIRED OUT",
"VERY \x97",
" TIRED",
"RESTED",
"YOU ARE ",
"SERIOUSLY\x9C",
"VERY\x9C",
" WOUNDED",
"WELL",
" DAY",
"NORTH",
"SOUTH",
"EAST",
"WEST",
" SKIN",
"MEAT",
" ADVENTURE",
"THANK YOU",
"DEAD !",
"VERY VULNERABLE",
"VULNERABLE",
"STURDY",
"VERY STURDY",
"$4",//Nomcopain // Don't translate this
"your friend",
"$END$" // Don't translate this
};

// This the list of history sentences 
// Please respect the \r\

 char *scenme_gb[]={
"The tribe's children are hungry,\r\
        bring back food.",

"   Young people have eaten poisonous\r\
                 berries.\r\
        We need the Eagle-Eyes medecine.",

"The Stone-Arms have challenged us to\r\
                a fight.\r\
   Go,fight with their chief and steal\r\
                his axe.",

"        Warrior,prepare to fight.\r\
      The Crazy-Hyenas have stolen\r\
             The Holy Rod.\r\
The Spirits of our ancestor ask for it!",

"Your actions damage my reputation.\r\
         May the Strongest Win !",

"Here is the story up to now unknown to\r\
the man who,a hundred thousand years ago,\r\
became chief of the Agile-Feet tribe\r\
by the strength of arms.\r\
He lived to the age of forty summers,\r\
and died by the hand of a young warrior.\r\
Good hunting !",

"        Nobody could survive\r\
         such terrible  wounds.\r\
Here lies the end of your road,warrior.",


"'A hundred thousand years ago ...\r\
      ...Adventure Began For Human.'\r\
\r\
  SAPIENS was designed and written by\r\
Didier and Olivier Guillion for AMSTRAD,\r\
   PC and Compatibles and ATARI ST.\r\
    Thanks to Jean-Francois Cassan,\r\
 Gilles Soulet, Jean-Michel Georges and\r\
      Yves Basquet from  'MYRIAD'\r\
   for their help,criticism and advise.\r\
\r\
     Toulouse, July-August XXXVII.",

"          You 've got me !\r\
        The Holy Stick is hidden\r\
     to the South under the Three Totems.",

"",
"",
"",
"",
"$END$" // Don't translate this
};
// Other miscelaneous sentences

char * app_gb []=
{"Agility",
 "Strength",
 "Charisma",
 "Vitality",
 "Health",
 "Credit",
 "Secondary Player : ",
 "Main Player      : ",
 "Which key for ",
 "Up",
 "Down",
 "Left",
 "Right",
 "Swapping Weapon/Validation",
 "Hitting",
 "Exit",
 " ?", // Don't translate this
 "(....)", // Don't translate this
 " Wolf ",
 " Bear ",
 "Rabbit",
 "Turkey",
"$END$" // Don't translate this
 };
 
 //EOF