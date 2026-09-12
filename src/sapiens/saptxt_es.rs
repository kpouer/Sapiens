/**********************************************************************
                          TEXTES VERSION SPANISH
                        (C) DIDIER 12 Avril 1987
                            30 mars 2017
**********************************************************************/
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

// Port Rust de "Sources/SAPTXT_es.C" : textes de la langue espagnole.
// Donnees pures, aucune fonction. Chaines en MacRoman brut ; les
// caracteres accentues sont ecrits selon la substitution d'origine
// (`%`=o `^`=n `&`=a `|`=i `#`=e `_`=u `[`=? `]`=!) et convertis
// par CvtCarLissage (saptxt.rs). Ne pas convertir en UTF-8.
use super::saptxt::{nomref, sc};
use crate::sapiens::{NB_MOTS_BKMOTS, reference};
use std::os::raw::c_char;

#[unsafe(no_mangle)]
pub static mut refer1_es: [reference; 47] = [
    reference {
        nomref: nomref(b"SU PROTECCION"),
        param1: 0x3F,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"AGUA"),
        param1: 0x25,
        param2: 0xF,
    },
    reference {
        nomref: nomref(b"HIERBA"),
        param1: 0x20,
        param2: 0x11,
    },
    reference {
        nomref: nomref(b"HOJA"),
        param1: 0x20,
        param2: 0x11,
    },
    reference {
        nomref: nomref(b"MANZANA"),
        param1: 0x21,
        param2: 0x12,
    },
    reference {
        nomref: nomref(b"PERA"),
        param1: 0x21,
        param2: 0x12,
    },
    reference {
        nomref: nomref(b"ARANDANO"),
        param1: 0x21,
        param2: 0x11,
    },
    reference {
        nomref: nomref(b"LARVA"),
        param1: 0x21,
        param2: 0x20,
    },
    reference {
        nomref: nomref(b"INSECTO"),
        param1: 0x21,
        param2: 0x10,
    },
    reference {
        nomref: nomref(b"CARDO"),
        param1: 0x20,
        param2: 0x11,
    },
    reference {
        nomref: nomref(b"BELADONA"),
        param1: 0x21,
        param2: 0x20,
    },
    reference {
        nomref: nomref(b"HIEDRA"),
        param1: 0x20,
        param2: 0x10,
    },
    reference {
        nomref: nomref(b"ACHICORIA"),
        param1: 0x21,
        param2: 0x11,
    },
    reference {
        nomref: nomref(b"RAICES"),
        param1: 0x21,
        param2: 0x21,
    },
    reference {
        nomref: nomref(b"HUESO"),
        param1: 0x20,
        param2: 0x10,
    },
    reference {
        nomref: nomref(b"MELISA"),
        param1: 0x41,
        param2: 0x02,
    },
    reference {
        nomref: nomref(b"DIENTE DE LEON"),
        param1: 0x21,
        param2: 0x21,
    },
    reference {
        nomref: nomref(b"MARGARITA"),
        param1: 0x41,
        param2: 0x02,
    },
    reference {
        nomref: nomref(b"PEDERNAL"),
        param1: 0x35,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"GUIJARRO"),
        param1: 0x30,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"HACHA"),
        param1: 0x1A,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"GARROTE"),
        param1: 0x18,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"LANZA"),
        param1: 0x16,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"RASPADOR"),
        param1: 0x14,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"UNGUENTO"),
        param1: 0x48,
        param2: 0x0A,
    },
    reference {
        nomref: nomref(b"ODRE DE AGUA VACIO"),
        param1: 0x38,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"ODRE DE AGUA LLENO"),
        param1: 0x2A,
        param2: 0x0E,
    },
    reference {
        nomref: nomref(b"CUCHILLO DE PEDERNAL"),
        param1: 0x37,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"TELA"),
        param1: 0x3B,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"CESTA"),
        param1: 0x3A,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"PIEL DE OSO"),
        param1: 0x3A,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"PIEL DE LOBO"),
        param1: 0x39,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"PIEL DE CONEJO"),
        param1: 0x33,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"GARRA DE OSO"),
        param1: 0x39,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"COLMILLO DE LOBO"),
        param1: 0x38,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"CARNE SECA"),
        param1: 0x27,
        param2: 0x90,
    },
    reference {
        nomref: nomref(b"CARNE FRESCA"),
        param1: 0x26,
        param2: 0xA1,
    },
    reference {
        nomref: nomref(b"CARNE"),
        param1: 0x26,
        param2: 0x91,
    },
    reference {
        nomref: nomref(b"CARNE RANCIA"),
        param1: 0x24,
        param2: 0x61,
    },
    reference {
        nomref: nomref(b"CARNE MOHOSA"),
        param1: 0x21,
        param2: 0x30,
    },
    reference {
        nomref: nomref(b"CARNE PODRIDA"),
        param1: 0x30,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"Poci%n Magica"),
        param1: 0x3D,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"Hacha de Marfil"),
        param1: 0x3D,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"Vara Sagrada"),
        param1: 0x3D,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"Lanza rota"),
        param1: 0,
        param2: 0,
    },
    reference {
        nomref: nomref(b"* Diskette de floppy *"),
        param1: 0x2F,
        param2: 0xFF,
    },
    reference {
        nomref: nomref(b"$END$"),
        param1: 0,
        param2: 0,
    },
];

#[unsafe(no_mangle)]
pub static mut refer0_es: [reference; 12] = [
    reference {
        nomref: nomref(b"MIRAR"),
        param1: 0x03,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"CONSUMIR"),
        param1: 0x01,
        param2: 0x01,
    },
    reference {
        nomref: nomref(b"DECIR"),
        param1: 0x02,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"DEJAR"),
        param1: 0x01,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"TOMAR"),
        param1: 0x04,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"DAR"),
        param1: 0x01,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"CURAR CON"),
        param1: 0x01,
        param2: 0x02,
    },
    reference {
        nomref: nomref(b"DORMIR"),
        param1: 0x05,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"HACER"),
        param1: 0x06,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"SALUD"),
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
];

#[unsafe(no_mangle)]
pub static mut refer2_es: [reference; 29] = [
    reference {
        nomref: nomref(b"[QUIEN ERES?"),
        param1: 0x00,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"[QUE QUIERES?"),
        param1: 0x00,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"[QUE ME DARAS?"),
        param1: 0x00,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"[QUIEN ES TU JEFE?"),
        param1: 0x00,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"SOY \x82"),
        param1: 0x00,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"BUENA CACERIA."),
        param1: 0x00,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"DE RODILLAS"),
        param1: 0x00,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"VEN CONMIGO"),
        param1: 0x00,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"!NO TE ACERQUES!"),
        param1: 0x01,
        param2: 0x10,
    },
    reference {
        nomref: nomref(b"SIGUE TU CAMINO"),
        param1: 0x01,
        param2: 0x20,
    },
    reference {
        nomref: nomref(b"FUERA DE MI VISTA"),
        param1: 0x01,
        param2: 0x30,
    },
    reference {
        nomref: nomref(b"!ERES UN COBARDE!"),
        param1: 0x01,
        param2: 0x40,
    },
    reference {
        nomref: nomref(b"BAJA LA MIRADA"),
        param1: 0x01,
        param2: 0x50,
    },
    reference {
        nomref: nomref(b"!TE APLASTARE!"),
        param1: 0x01,
        param2: 0x60,
    },
    reference {
        nomref: nomref(b"]ARRASTRATE ANTE MI!"),
        param1: 0x01,
        param2: 0x70,
    },
    reference {
        nomref: nomref(b"!CORAZON DE CONEJO!"),
        param1: 0x01,
        param2: 0x80,
    },
    reference {
        nomref: nomref(b"!MONO DEGENERADO!"),
        param1: 0x01,
        param2: 0x90,
    },
    reference {
        nomref: nomref(b"!CARA DE CERDO!"),
        param1: 0x01,
        param2: 0xA0,
    },
    reference {
        nomref: nomref(b"PAZ,AMIGO."),
        param1: 0x02,
        param2: 0x40,
    },
    reference {
        nomref: nomref(b"LA GLORIA ESTE CON TU TRIBU"),
        param1: 0x02,
        param2: 0x80,
    },
    reference {
        nomref: nomref(b"TODOS ME VENERAN"),
        param1: 0x02,
        param2: 0xC0,
    },
    reference {
        nomref: nomref(b"POR PIEDAD,PERDONAME"),
        param1: 0x02,
        param2: 0xF0,
    },
    reference {
        nomref: nomref(b"!SOY EL MAS FUERTE !"),
        param1: 0x03,
        param2: 0x20,
    },
    reference {
        nomref: nomref(b"!NADIE SE ATREVE A OPONER RESISTENCIA !"),
        param1: 0x03,
        param2: 0x40,
    },
    reference {
        nomref: nomref(b" ]YO COMO PEDERNAL !"),
        param1: 0x03,
        param2: 0x60,
    },
    reference {
        nomref: nomref(b"HUYE ANTE MI"),
        param1: 0x03,
        param2: 0x80,
    },
    reference {
        nomref: nomref(b"YO COMANDO EL RELAMPAGO"),
        param1: 0x03,
        param2: 0xA0,
    },
    reference {
        nomref: nomref(b"LOS DIOSES ME GUIAN"),
        param1: 0x03,
        param2: 0xC0,
    },
    reference {
        nomref: nomref(b"$END$"),
        param1: 0,
        param2: 0,
    },
];

#[unsafe(no_mangle)]
pub static mut refer3_es: [reference; 27] = [
    reference {
        nomref: nomref(b"PAISAJE"),
        param1: 0x01,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"VISTA LATERAL"),
        param1: 0x01,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"PANORAMA"),
        param1: 0x01,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"MAPA"),
        param1: 0x01,
        param2: 0x00,
    },
    reference {
        nomref: nomref(b"SOLO UN MOMENTO"),
        param1: 0x00,
        param2: 0x20,
    },
    reference {
        nomref: nomref(b"MEDIO DIA"),
        param1: 0x00,
        param2: 0x80,
    },
    reference {
        nomref: nomref(b"UN DIA"),
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
        nomref: nomref(b"Cargar Aventura"),
        param1: 0x0,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"Guardar Aventura"),
        param1: 0x0,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"Nueva Aventura"),
        param1: 0x0,
        param2: 0x0,
    },
    reference {
        nomref: nomref(b"Auto-Ayuda"),
        param1: 0,
        param2: 0,
    },
    reference {
        nomref: nomref(b"Ayuda del teclado"),
        param1: 0,
        param2: 0,
    },
    reference {
        nomref: nomref(b"Configuracion"),
        param1: 0,
        param2: 0,
    },
    reference {
        nomref: nomref(b"Manejo del juego"),
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
        nomref: nomref(b"Ajustes"),
        param1: 0,
        param2: 0,
    },
    reference {
        nomref: nomref(b"$END$"),
        param1: 0,
        param2: 0,
    }, // Don't translate this
];

#[unsafe(no_mangle)]
pub static mut mescon_es: [*mut c_char; 138] = [
    sc(b"GRACIAS POR EL REGALO \x00"),
    sc(b"GRACIAS \x00"),
    sc(b"GRACIAS AMIGO \x00"),
    sc(b"GRACIAS A TI \x00"),
    sc(b"[QUIEN ERES?\x00"),
    sc(b"[CUAL ES TU NOMBRE?\x00"),
    sc(b"DIME TU NOMBRE.\x00"),
    sc(b"IDENTIFICATE.\x00"),
    sc(b"NO ES SUFICIENTE.\x00"),
    sc(b"[ESTAS BROMEANDO ?!\x00"),
    sc(b"]DE NINGUN MODO!\x00"),
    sc(b"]NO!\x00"),
    sc(b"]HECHO!\x00"),
    sc(b"O.K.\x00"),
    sc(b"\x83 ESTA DE ACUERDO.\x00"),
    sc(b"ESO SERVIRA\x00"),
    sc(b"\x83  TOMA TU REGALO.\x00"),
    sc(b"BIEN, \x00"), /* le C d'origine ecrivait b"BIEN, \x00x83 LO TOMA" : le NUL tronque la chaine, rien apres n'est lisible */
    sc(b"SI,\x83 ACEPTA\x00"),
    sc(b"\x83 LO TOMA.\x00"),
    sc(b" NO ES SUFICIENTE\x00"),
    sc(b" DAME MAS\x00"),
    sc(b" SE MAS GENEROSO\x00"),
    sc(b" SOLO PARA COMPLACERTE\x00"),
    sc(b"UHMM ...\x00"),
    sc(b"[ALGUNA OTRA COSA ??\x00"),
    sc(b"BUENO ...\x00"),
    sc(b"EN SERIO ...\x00"),
    sc(b"TERMINEMOS CON ESTO\x00"),
    sc(b"SUFICIENTE CHARLA\x00"),
    sc(b"[QUE MAS?\x00"),
    sc(b"VAYAMOS AL PUNTO\x00"),
    sc(b"]\x83 TE MATARA,SUCIO CERDO!\x00"),
    sc(b"]CALLATE!\x00"),
    sc(b"\x83 SE ARREPENTIRA DE SUS INSULTOS\x00"),
    sc(b"[CANSADO DE VIVIR ?!\x00"),
    sc(b"ALIMENTARE A LOS CUERVOS CON TUS HUESOS.\x00"),
    sc(b"]YO SOLO PODRE CON ESTO!\x00"),
    sc(b"\x83 TE DESTROZARA MIEMBRO A MIEMBRO\x00"),
    sc(b"\x83 BEBERA DE TU CRANEO\x00"),
    sc(b"QUE MI BRAZO ROMPA TU QUIJADA\x00"),
    sc(b"]VICTORIA O MUERTE!\x00"),
    sc(b"SIN PIEDAD.\x00"),
    sc(b"QUE LOS ESPIRITUS GUIEN A  \x83\x00"),
    sc(b"YO SOY \x83\x00"),
    sc(b"DIME \x83\x00"),
    sc(b"MI NOMBRE ES \x83\x00"),
    sc(b"ME DICEN \x83\x00"),
    sc(b"]TU VIDA !\x00"),
    sc(b"]PARA MATARTE!\x00"),
    sc(b"]TU CORAZON!\x00"),
    sc(b"TU CRANEO\x00"),
    sc(b"\x83 QUIERE \x00"),
    sc(b"\x83 LE GUSTARIA \x00"),
    sc(b"\x83 DESEA \x00"),
    sc(b"\x83 NECESITA \x00"),
    sc(b"\x83 OFRECE \x00"),
    sc(b"\x83 SUGIERE \x00"),
    sc(b"\x83 INTERCAMBIA \x00"),
    sc(b"\x83 SE RINDE \x00"),
    sc(b"\x83 NO CONTESTARA\x00"),
    sc(b"...\x00"),
    sc(b"AH AH AH!\x00"),
    sc(b"[QUIEN SABE?\x00"),
    sc(b" Y TU,\x00"),
    sc(b"NADA\x00"),
    sc(b"SAPIENS creado por Didier y Olivier Guillion.\x00"),
    sc(b"(c)1986-1996,1999,2001,2005,2017\x00"),
    sc(b"M_sica por Gilles Soulet. Traducci%n al espa^ol por Enrique Simanca.\x00"),
    sc(b"Hace mil siglos el sol se elev% ...\x00"),
    sc(b"HOLA,\x82\x00"),
    sc(b"ASI QUE TU ERES \x82 ?\x00"),
    sc(b"BUENA CACERIA \x82\x00"),
    sc(b"El aullido del lobo.\x00"),
    sc(b"El gru^ido del oso.\x00"),
    sc(b"El gorgoteo del manantial.\x00"),
    sc(b"El olor de la presa.\x00"),
    sc(b"El olor del humo.\x00"),
    sc(b"El olor de humanos.\x00"),
    sc(b"El escapa.\x00"),
    sc(b"Estas inconsciente.\x00"),
    sc(b"LA TRIBU ESTA ORGULLOSA DE TI,\x82\x00"),
    sc(b"ERES UN VALIENTE GUERRERO,\x82\x00"),
    sc(b"PARECE QUE TIENES LA FUERZA DEL TORO !\x00"),
    sc(b"Posees una nueva arma.\x00"),
    sc(b"Tu pedernal est& muy mal tallado.\x00"),
    sc(b"no tienes pedernal...\x00"),
    sc(b"Nombre? \x00"),
    sc(b"Hombre o Mujer (H,M) ?\x00"),
    sc(b"Aventura a Cargar ? \x00"),
    sc(b"Aventura a Guardar ? \x00"),
    sc(b" Error !!!!\x00"),
    sc(b"El hambre hace doler tu estomago !\x00"),
    sc(b"Tu garganta esta seca por la sed!\x00"),
    sc(b"Tus hombros caen por el cansancio!\x00"),
    sc(b"Bebes \x00"),
    sc(b"y llenas tus odres\x00"),
    sc(b"No tienes sed\x00"),
    sc(b"...ni modo\x00"),
    sc(b"]Estoy justo detr&s de ti !\x00"),
    sc(b"]Llevas demasiadas cosas !!!\x00"),
    sc(b"Un Lobo viene contigo.\x00"),
    sc(b"Un Oso es tu amigo.\x00"),
    sc(b"Un peque&o Conejo est& contigo.\x00"),
    sc(b"Un Pavo esta justo detr&s de ti .\x00"),
    sc(b" \x00"),
    sc(b"]Trag%n ! No tienes hambre ni sed !\x00"),
    sc(b"No est&s cansado.\x00"),
    sc(b"Eres la imagen de la salud.\x00"),
    sc(b"Maravilloso,\x00"),
    sc(b"Tu amigo \xAC esta inconsciente ...\x00"),
    sc(b"Lo siento \x82, \xAC Muri% !!!\x00"),
    sc(b"Solo una aventura guardada por d|a !!!\x00"),
    sc(b"Un nuevo d|a ...\x00"),
    sc(b"Salta, regresa a casa !!!\x00"),
    sc(b"[Qu# piensas de esa canci%n? ...\x00"),
    sc(b"Aqu| estamos...\x00"),
    sc(b"Un peque^o regalo...\x00"),
    sc(b"%Hola chicos!\x00"),
    sc(b"SAPIENS por D y O Guillion.\x00"),
    sc(b"SAPIENS opci%n :\x00"),
    sc(b"Tu salud :\x00"),
    sc(b"\x82 toma\x00"),
    sc(b"\x82 deja en el suelo\x00"),
    sc(b"\x82 dice :\x00"),
    sc(b"\x82 da\x00"),
    sc(b"\x82 mira\x00"),
    sc(b"\x82 consume\x00"),
    sc(b"\x82 se cura a si mismo con\x00"),
    sc(b"\x82 duerme por\x00"),
    sc(b"Con pedernal, \x82 hace\x00"),
    sc(b"]Presiona F1 por ayuda !\x00"),
    sc(b"Y el Tiempo vuela...\x00"),
    sc(b"Te golpea un rayo !!!!!\x00"),
    sc(b"[Le hablas al viento ?\x00"),
    sc(b"Has muerto ! El juego termino...\x00"),
    sc(b"Estoy construyendo el mundo...\x00"),
    sc(b"$END$\x00"), // Don't translate this
];

#[unsafe(no_mangle)]
pub static mut bkmots_es: [*mut c_char; NB_MOTS_BKMOTS as usize] = [
    sc(b"$0\x00"), // nomsecondaire // Don't translate this
    sc(b"$1\x00"), // nomjoueur// Don't translate this
    sc(b"$2\x00"), // nomadversaire// Don't translate this
    sc(b"$3\x00"), // // Don't translate this
    sc(b" JEFE\x00"),
    sc(b" DE LOS PIES AGILES\x00"),
    sc(b" DE LOS OJOS DE AGUILA\x00"),
    sc(b" DE LOS BRAZOS DE PIEDRA\x00"),
    sc(b" DE LAS HIENAS LOCAS\x00"),
    sc(b"HOGNOR\x00"),
    sc(b"SL-OOM\x00"),
    sc(b"GRUNGH\x00"),
    sc(b"KROK-G\x00"),
    sc(b" HAMBRIENTO\x00"),
    sc(b" SEDIENTO\x00"),
    sc(b"TU SALUD ES \x00"),
    sc(b"ESTAS EXTREMADAMENTE\x00"),
    sc(b"ESTAS MUY\x00"),
    sc(b"ESTAS UN POCO\x00"),
    sc(b"TU NO ERES\x00"),
    sc(b"MUY CANSADO\x00"),
    sc(b"MUY \x97\x00"),
    sc(b" CANSADO\x00"),
    sc(b"DESCANSADO\x00"),
    sc(b"ESTAS \x00"),
    sc(b"SERIAMENTE\x9C\x00"),
    sc(b"MUY\x9C\x00"),
    sc(b" HERIDO\x00"),
    sc(b"BIEN\x00"),
    sc(b" DIA\x00"),
    sc(b"NORTE\x00"),
    sc(b"SUR\x00"),
    sc(b"ESTE\x00"),
    sc(b"OESTE\x00"),
    sc(b" PIEL\x00"),
    sc(b"CARNE\x00"),
    sc(b" AVENTURA\x00"),
    sc(b"GRACIAS\x00"),
    sc(b"MUERTO !\x00"),
    sc(b"MUY VULNERABLE\x00"),
    sc(b"VULNERABLE\x00"),
    sc(b"RESISTENTE\x00"),
    sc(b"MUY RESISTENTE\x00"),
    sc(b"$4\x00"), // Nomcopain // Don't translate this
    sc(b"tu amigo\x00"),
    sc(b"$END$\x00"), // Don't translate this
];

#[unsafe(no_mangle)]
pub static mut scenme_es: [*mut c_char; 14] = [
    sc(b"Los ni^os de la tribu est&n hambrientos,\r        Trae comida.\x00"),
    sc(b"   los j%venes han comido bayas\r                 venenosas.\r        Necesitamos la medicina de los Ojos de Aguila.\x00"),
    sc(b"Los Brazos de Piedra nos han retado a\r                una pelea.\r   Ve, lucha con su jefe y roba\r                su hacha.\x00"),
    sc(b"        Guerrero, prep&rate para pelear.\r      Las Hienas Locas han robado\r             La Vara Sagrada.\rLos Esp|ritus de nuestros ancestros lo pidieron!\x00"),
    sc(b"Tus acciones de^an mi reputaci%n.\r         Que el m&s Fuerte Gane !\x00"),
    sc(b"Aqu| est& la historia hasta ahora desconocida del\rhombre que, hace cien mil a^os,\rse convirti% en el jefe de la tribu de los Pies Agiles\rpor la fuerza de sus brazos.\rEl vivi% hasta la edad de cuarenta veranos,\ry muri% en manos de un joven guerrero.\rBuena cacer|a !\x00"),
    sc(b"        Nadie podr|a sobrevivir\r         tan terribles heridas.\rAqu| yace el final de tu camino, guerrero.\x00"),
    sc(b"'Hace cien mil a^os ...\r      ...La Aventura Comenz% para los Humanos.'\r\r  SAPIENS fue dise^ado y escrito por\rDidier y Olivier Guillion para AMSTRAD,\r   PC y Compatibles y ATARI ST.\r    Gracias a Jean-Francois Cassan,\r Gilles Soulet, Jean-Michel Georges y\r      Yves Basquet de  'MYRIAD'\r   Por su ayuda, cr|ticas and consejos.\r\r     Toulouse, Julio-Agosto XXXVII.\x00"),
    sc(b"          Me atrapaste !\r        La Vara Sagrada est& escondida\r     Al Sur bajo los Tres T%tems.\x00"),
    sc(b"\x00"),
    sc(b"\x00"),
    sc(b"\x00"),
    sc(b"\x00"),
    sc(b"$END$\x00"), // Don't translate this
];

#[unsafe(no_mangle)]
pub static mut app_es: [*mut c_char; 23] = [
    sc(b"Agilidad\x00"),
    sc(b"Fuerza\x00"),
    sc(b"Carisma\x00"),
    sc(b"Vitalidad\x00"),
    sc(b"Salud\x00"),
    sc(b"Cr#ditos\x00"),
    sc(b"Jugador Secundario : \x00"),
    sc(b"Jugador Principal      : \x00"),
    sc(b"Cual tecla para \x00"),
    sc(b"Arriba\x00"),
    sc(b"Abajo\x00"),
    sc(b"Izquierda\x00"),
    sc(b"Derecha\x00"),
    sc(b"Cambiar Arma/Validaci%n\x00"),
    sc(b"Golpear\x00"),
    sc(b"Salir\x00"),
    sc(b" ?\x00"),     // Don't translate this
    sc(b"(....)\x00"), // Don't translate this
    sc(b" Lobo \x00"),
    sc(b" Oso \x00"),
    sc(b"Conejo\x00"),
    sc(b"Pavo\x00"),
    sc(b"$END$\x00"), // Don't translate this
];
