/**********************************************************************
                          TEXTES VERSION SPANISH
                        (C) DIDIER 12 Avril 1987
							30 mars 2017
Notes to the translator :

Only translated the strings between " and "
Don't change anyhting else outside strings.

For example 
{"HIS PROTECTION",0x3F,0x0}, 
will become 
{"SU PROTECCION",0x3F,0x0},

Inside strings :
Dont change tags like \r or \r\ or \n or \n\ this is cariages returns (go to next line)
Dont change tags \xnn like \x9C they will be sustituted
Please respect the space before and after the tags or at the end or the begin of strings
Either don't change tags $nn

Special Chars Substitution :

¿ [
¡ ] //Exclamation point inverted
ó %
ñ ^
á &
í |
é #
ú _

To report to AffCarBrt()
***********************************************************************/
#include "sapiens.h"
// This the list of object the user can find on the ground, win in a battle, make, and share
// Please respect uppercase and lowercase
struct reference refer1_es[]={
{"SU PROTECCION",0x3F,0x0},
{"AGUA",0x25,0xF},
{"HIERBA",0x20,0x11},
{"HOJA",0x20,0x11},
{"MANZANA",0x21,0x12},
{"PERA",0x21,0x12},
{"ARANDANO",0x21,0x11},
{"LARVA",0x21,0x20},
{"INSECTO",0x21,0x10},
{"CARDO",0x20,0x11},
{"BELADONA",0x21,0x20},
{"HIEDRA",0x20,0x10},
{"ACHICORIA",0x21,0x11},
{"RAICES",0x21,0x21},
{"HUESO",0x20,0x10},
{"MELISA",0x41,0x02},
{"DIENTE DE LEON",0x21,0x21},
{"MARGARITA",0x41,0x02},
{"PEDERNAL",0x35,0x00},
{"GUIJARRO",0x30,0x00},
{"HACHA",0x1A,0x00},
{"GARROTE",0x18,0x00},
{"LANZA",0x16,0x00},
{"RASPADOR",0x14,0x00},
{"UNGUENTO",0x48,0x0A},
{"ODRE DE AGUA VACIO",0x38,0x00},
{"ODRE DE AGUA LLENO",0x2A,0x0E},
{"CUCHILLO DE PEDERNAL",0x37,0x00},
{"TELA",0x3B,0x00},
{"CESTA",0x3A,0x00},
{"PIEL DE OSO",0x3A,0x00},
{"PIEL DE LOBO",0x39,0x00},
{"PIEL DE CONEJO",0x33,0x00},
{"GARRA DE OSO",0x39,0x00},
{"COLMILLO DE LOBO",0x38,0x00},
{"CARNE SECA",0x27,0x90},
{"CARNE FRESCA",0x26,0xA1},
{"CARNE",0x26,0x91},
{"CARNE RANCIA",0x24,0x61},
{"CARNE MOHOSA",0x21,0x30},
{"CARNE PODRIDA",0x30,0x00},
{"Poci%n Magica",0x3D,0x00},
{"Hacha de Marfil",0x3D,0x00},
{"Vara Sagrada",0x3D,0x00},
{"Lanza rota",0,0},
{"* Diskette de floppy *",0x2F,0xFF},
{"$END$",0,0}
},
// This the list of actions (the main Sapiens subwindow at the left
refer0_es[]={
{"MIRAR",0x03,0x00},
{"CONSUMIR",0x01,0x01},
{"DECIR",0x02,0x00},
{"DEJAR",0x01,0x00},
{"TOMAR",0x04,0x00},
{"DAR",0x01,0x00},
{"CURAR CON",0x01,0x02},
{"DORMIR",0x05,0x00},
{"HACER",0x06,0x00},
{"SALUD",0x07,0x00},
{"SAPIENS",0x08,0x00},
{"$END$",0,0}
},
// This the list of sentences the user can say in lateral view
// Dont change tags \xnn like \x82 they will be sustituted
// For example \x82 is the name of the player
refer2_es[]={
{"[QUIEN ERES?",0x00,0x00},
{"[QUE QUIERES?",0x00,0x00},
{"[QUE ME DARAS?",0x00,0x00},
{"[QUIEN ES TU JEFE?",0x00,0x00},
{"SOY \x82",0x00,0x00},
{"BUENA CACERIA.",0x00,0x00},
{"DE RODILLAS",0x00,0x00},
{"VEN CONMIGO",0x00,0x00},
{"!NO TE ACERQUES!",0x01,0x10},
{"SIGUE TU CAMINO",0x01,0x20},
{"FUERA DE MI VISTA",0x01,0x30},
{"!ERES UN COBARDE!",0x01,0x40},
{"BAJA LA MIRADA",0x01,0x50},
{"!TE APLASTARE!",0x01,0x60},
{"]ARRASTRATE ANTE MI!",0x01,0x70},
{"!CORAZON DE CONEJO!",0x01,0x80},
{"!MONO DEGENERADO!",0x01,0x90},
{"!CARA DE CERDO!",0x01,0xA0},
{"PAZ,AMIGO.",0x02,0x40},
{"LA GLORIA ESTE CON TU TRIBU",0x02,0x80},
{"TODOS ME VENERAN",0x02,0xC0},
{"POR PIEDAD,PERDONAME",0x02,0xF0},
{"!SOY EL MAS FUERTE !",0x03,0x20},
{"!NADIE SE ATREVE A OPONER RESISTENCIA !",0x03,0x40},
{" ]YO COMO PEDERNAL !",0x03,0x60},
{"HUYE ANTE MI",0x03,0x80},
{"YO COMANDO EL RELAMPAGO",0x03,0xA0},
{"LOS DIOSES ME GUIAN",0x03,0xC0},
{"$END$",0,0}
},
// This the list of misc sentences
// Dont change tags \xnn like \x8E they will be sustituted

refer3_es[]={
{"PAISAJE",0x01,0x00},
{"VISTA LATERAL",0x01,0x00},
{"PANORAMA",0x01,0x00},
{"MAPA",0x01,0x00},
{"SOLO UN MOMENTO",0x00,0x20},
{"MEDIO DIA",0x00,0x80},
{"UN DIA",0x0,0xFF},
{ " \x8E",0x0,0x0}, // Don't translate this
{ " \x8F",0x0,0x0}, // Don't translate this
{ "\x99 ",0x0,0x0}, // Don't translate this
{ "\x99 ",0x0,0x0}, // Don't translate this
{ "\x99 ",0x0,0x0}, // Don't translate this
{"Cargar Aventura",0x0,0x0},
{"Guardar Aventura",0x0,0x0},
{"Nueva Aventura",0x0,0x0},
{"Auto-Ayuda",0,0},
{"Ayuda del teclado",0,0},
{"Configuracion",0,0},
{"Manejo del juego",0,0},
{"'DMA' Music",0,0},  // Don't translate this
{"MIDI  Music",0,0}, // Don't translate this
{"God!",0,0}, // Don't translate this
{"God!",0,0}, // Don't translate this
{"French Version",0,0}, // Don't translate this
{"MIDI Control",0,0}, // Don't translate this
{"Ajustes",0,0},
{"$END$",0,0} // Don't translate this
};

// This the list of misc sentences the oponent can answer to the player
// Dont change tags \xnn like \x83 they will be sustituted
// For example \x83 is the name of the opponent \0x82 the player name \xAC the player's friend name
char * mescon_es[]={
"GRACIAS POR EL REGALO ",
"GRACIAS ",
"GRACIAS AMIGO ",
"GRACIAS A TI ",
"[QUIEN ERES?",
"[CUAL ES TU NOMBRE?",
"DIME TU NOMBRE.",
"IDENTIFICATE.",
"NO ES SUFICIENTE.",
"[ESTAS BROMEANDO ?!",
"]DE NINGUN MODO!",
"]NO!",
"]HECHO!",
"O.K.",
"\x83 ESTA DE ACUERDO.",
"ESO SERVIRA",
"\x83  TOMA TU REGALO.",
"BIEN, \0x83 LO TOMA",
"SI,\x83 ACEPTA",
"\x83 LO TOMA.",
" NO ES SUFICIENTE",
" DAME MAS",
" SE MAS GENEROSO",
" SOLO PARA COMPLACERTE",
"UHMM ...",
"[ALGUNA OTRA COSA ??",
"BUENO ...",
"EN SERIO ...",
"TERMINEMOS CON ESTO",
"SUFICIENTE CHARLA",
"[QUE MAS?",
"VAYAMOS AL PUNTO",
"]\x83 TE MATARA,SUCIO CERDO!",
"]CALLATE!",
"\x83 SE ARREPENTIRA DE SUS INSULTOS",
"[CANSADO DE VIVIR ?!",
"ALIMENTARE A LOS CUERVOS CON TUS HUESOS.",
"]YO SOLO PODRE CON ESTO!",
"\x83 TE DESTROZARA MIEMBRO A MIEMBRO",
"\x83 BEBERA DE TU CRANEO",
"QUE MI BRAZO ROMPA TU QUIJADA",
"]VICTORIA O MUERTE!",
"SIN PIEDAD.",
"QUE LOS ESPIRITUS GUIEN A  \x83",
"YO SOY \x83",
"DIME \x83",
"MI NOMBRE ES \x83",
"ME DICEN \x83",
"]TU VIDA !",
"]PARA MATARTE!",
"]TU CORAZON!",
"TU CRANEO",
"\x83 QUIERE ",
"\x83 LE GUSTARIA ",
"\x83 DESEA ",
"\x83 NECESITA ",
"\x83 OFRECE ",
"\x83 SUGIERE ",
"\x83 INTERCAMBIA ",
"\x83 SE RINDE ",
"\x83 NO CONTESTARA",
"...",
"AH AH AH!",
"[QUIEN SABE?",
" Y TU,",
"NADA",
"SAPIENS creado por Didier y Olivier Guillion.",
"(c)1986-1996,1999,2001,2005,2017",
"M_sica por Gilles Soulet. Traducci%n al espa^ol por Enrique Simanca.",
"Hace mil siglos el sol se elev% ...",
"HOLA,\x82",
"ASI QUE TU ERES \x82 ?",
"BUENA CACERIA \x82",
"El aullido del lobo.",
"El gru^ido del oso.",
"El gorgoteo del manantial.",
"El olor de la presa.",
"El olor del humo.",
"El olor de humanos.",
"El escapa.",
"Estas inconsciente.",
"LA TRIBU ESTA ORGULLOSA DE TI,\x82",
"ERES UN VALIENTE GUERRERO,\x82",
"PARECE QUE TIENES LA FUERZA DEL TORO !",
"Posees una nueva arma.",
"Tu pedernal est& muy mal tallado.",
"no tienes pedernal...",
"Nombre? ",
"Hombre o Mujer (H,M) ?",
"Aventura a Cargar ? ",
"Aventura a Guardar ? ",
" Error !!!!",
"El hambre hace doler tu estomago !",
"Tu garganta esta seca por la sed!",
"Tus hombros caen por el cansancio!",
"Bebes ",
"y llenas tus odres",
"No tienes sed",
"...ni modo",
"]Estoy justo detr&s de ti !",
"]Llevas demasiadas cosas !!!",
"Un Lobo viene contigo.",
"Un Oso es tu amigo.",
"Un peque&o Conejo est& contigo.",
"Un Pavo esta justo detr&s de ti .",
" ",
"]Trag%n ! No tienes hambre ni sed !",
"No est&s cansado.",
"Eres la imagen de la salud.",
"Maravilloso,",
"Tu amigo \xAC esta inconsciente ...",
"Lo siento \x82, \xAC Muri% !!!",
"Solo una aventura guardada por d|a !!!",
"Un nuevo d|a ...",
"Salta, regresa a casa !!!",
"[Qu# piensas de esa canci%n? ...",
"Aqu| estamos...",
"Un peque^o regalo...",
"%Hola chicos!",
"SAPIENS por D y O Guillion.",
"SAPIENS opci%n :",
"Tu salud :",
"\x82 toma",
"\x82 deja en el suelo",
"\x82 dice :",
"\x82 da",
"\x82 mira",
"\x82 consume",
"\x82 se cura a si mismo con",
"\x82 duerme por",
"Con pedernal, \x82 hace",
"]Presiona F1 por ayuda !",
"Y el Tiempo vuela...",
"Te golpea un rayo !!!!!",
"[Le hablas al viento ?",
"Has muerto ! El juego termino...",
"Estoy construyendo el mundo...",
"$END$" // Don't translate this
};

// This the list of misc sentences 
// Dont change tags \xnn like \x9C they will be sustituted
// Please respect the space before and after the tags or at the end or the begin of strings
// Either don't change tags $nn

 char * bkmots_es []={
"$0", //nomsecondaire // Don't translate this
"$1",//  nomjoueur// Don't translate this
"$2", //  nomadversaire// Don't translate this
"$3", //// Don't translate this
" JEFE",
" DE LOS PIES AGILES",
" DE LOS OJOS DE AGUILA",
" DE LOS BRAZOS DE PIEDRA",
" DE LAS HIENAS LOCAS",
"HOGNOR",
"SL-OOM",
"GRUNGH",
"KROK-G",
" HAMBRIENTO",
" SEDIENTO",
"TU SALUD ES ",
"ESTAS EXTREMADAMENTE",
"ESTAS MUY",
"ESTAS UN POCO",
"TU NO ERES",
"MUY CANSADO",
"MUY \x97",
" CANSADO",
"DESCANSADO",
"ESTAS ",
"SERIAMENTE\x9C",
"MUY\x9C",
" HERIDO",
"BIEN",
" DIA",
"NORTE",
"SUR",
"ESTE",
"OESTE",
" PIEL",
"CARNE",
" AVENTURA",
"GRACIAS",
"MUERTO !",
"MUY VULNERABLE",
"VULNERABLE",
"RESISTENTE",
"MUY RESISTENTE",
"$4",//Nomcopain // Don't translate this
"tu amigo",
"$END$" // Don't translate this
};

// This the list of history sentences 
// Please respect the \r\

 char *scenme_es[]={
"Los ni^os de la tribu est&n hambrientos,\r\
        Trae comida.",

"   los j%venes han comido bayas\r\
                 venenosas.\r\
        Necesitamos la medicina de los Ojos de Aguila.",

"Los Brazos de Piedra nos han retado a\r\
                una pelea.\r\
   Ve, lucha con su jefe y roba\r\
                su hacha.",

"        Guerrero, prep&rate para pelear.\r\
      Las Hienas Locas han robado\r\
             La Vara Sagrada.\r\
Los Esp|ritus de nuestros ancestros lo pidieron!",

"Tus acciones de^an mi reputaci%n.\r\
         Que el m&s Fuerte Gane !",

"Aqu| est& la historia hasta ahora desconocida del\r\
hombre que, hace cien mil a^os,\r\
se convirti% en el jefe de la tribu de los Pies Agiles\r\
por la fuerza de sus brazos.\r\
El vivi% hasta la edad de cuarenta veranos,\r\
y muri% en manos de un joven guerrero.\r\
Buena cacer|a !",

"        Nadie podr|a sobrevivir\r\
         tan terribles heridas.\r\
Aqu| yace el final de tu camino, guerrero.",


"'Hace cien mil a^os ...\r\
      ...La Aventura Comenz% para los Humanos.'\r\
\r\
  SAPIENS fue dise^ado y escrito por\r\
Didier y Olivier Guillion para AMSTRAD,\r\
   PC y Compatibles y ATARI ST.\r\
    Gracias a Jean-Francois Cassan,\r\
 Gilles Soulet, Jean-Michel Georges y\r\
      Yves Basquet de  'MYRIAD'\r\
   Por su ayuda, cr|ticas and consejos.\r\
\r\
     Toulouse, Julio-Agosto XXXVII.",

"          Me atrapaste !\r\
        La Vara Sagrada est& escondida\r\
     Al Sur bajo los Tres T%tems.",

"",
"",
"",
"",
"$END$" // Don't translate this
};
// Other miscelaneous sentences

char * app_es []=
{"Agilidad",
 "Fuerza",
 "Carisma",
 "Vitalidad",
 "Salud",
 "Cr#ditos",
 "Jugador Secundario : ",
 "Jugador Principal      : ",
 "Cual tecla para ",
 "Arriba",
 "Abajo",
 "Izquierda",
 "Derecha",
 "Cambiar Arma/Validaci%n",
 "Golpear",
 "Salir",
 " ?", // Don't translate this
 "(....)", // Don't translate this
 " Lobo ",
 " Oso ",
 "Conejo",
 "Pavo",
"$END$" // Don't translate this
 };
 //EOF


